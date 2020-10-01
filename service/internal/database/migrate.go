//go:generate packr2 -v

package database

import (
	"context"
	"sort"

	"github.com/georgysavva/scany/pgxscan"
	"github.com/gobuffalo/packd"
	"github.com/gobuffalo/packr/v2"
	"github.com/jackc/pgx/v4"
	"github.com/rs/zerolog/log"
)

// Migrate the provided database to the latest version of the schema.
// This is guaranteed to be safe to run multiple times, and even concurrently.
func Migrate(ctx context.Context, db Database) error {
	log.Info().Msg("Migrating database")

	tx := db.Tx(ctx)

	defer func() {
		_ = tx.Rollback(ctx)
	}()

	if err := createMigrationsTable(ctx, tx); err != nil {
		return err
	}

	applied, err := listAppliedMigrations(ctx, tx)
	if err != nil {
		return err
	}

	box := packr.New("migrations", "./migrations")
	available := listAvailableMigrations(box, applied)

	appliedCount := 0

	for _, file := range available {
		if err := applyMigration(ctx, tx, box, file); err != nil {
			return err
		}
		appliedCount++
	}

	if err := tx.Commit(ctx); err != nil {
		log.Error().Err(err).Msg("Failed to commit transaction")

		return err
	}

	log.Info().Int("applied", len(applied)).
		Int("available", len(available)).
		Int("performed", appliedCount).
		Msg("Finished migrating database")

	return nil
}

// Create the __migrations table and take out an exclusive lock on it, to ensure that nobody else is running migrations
// at the same time.
func createMigrationsTable(ctx context.Context, tx pgx.Tx) error {
	_, err := tx.Exec(ctx, `CREATE TABLE IF NOT EXISTS __migrations(
			migration_file TEXT PRIMARY KEY,
			sequence SERIAL NOT NULL,
			executed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
			executed_from TEXT NOT NULL DEFAULT inet_client_addr())`)
	if err != nil {
		log.Error().Err(err).Msg("Failed to create migrations table")

		return err
	}

	_, err = tx.Exec(ctx, "LOCK TABLE __migrations IN EXCLUSIVE MODE")
	if err != nil {
		log.Error().Err(err).Msg("Failed to lock migrations table")

		return err
	}

	return nil
}

// Load the list of applied migrations from the database.
func listAppliedMigrations(ctx context.Context, tx pgxscan.Querier) ([]string, error) {
	var result []string

	if err := pgxscan.Select(ctx, tx, &result, "SELECT migration_file FROM __migrations ORDER BY sequence"); err != nil {
		log.Error().Err(err).Msg("Failed to list applied migrations")

		return []string{}, err
	}

	log.Debug().Interface("migrations", result).Msg("List of applied migrations")

	return result, nil
}

// Load the list of available migrations from the file system, ignoring ones that have already been applied.
func listAvailableMigrations(box packd.Lister, applied []string) []string {
	result := []string{}

	for _, file := range box.List() {
		if !contains(applied, file) {
			result = append(result, file)
		}
	}

	sort.Strings(result)

	log.Debug().Interface("migrations", result).Msg("List of available migrations")

	return result
}

// Apply the given migration file to the database.
func applyMigration(ctx context.Context, tx pgx.Tx, box *packr.Box, file string) error {
	log.Debug().Str("file", file).Msg("Applying migration file")

	migration, err := box.FindString(file)
	if err != nil {
		log.Error().Err(err).Str("file", file).Msg("Failed to load migration file")

		return err
	}

	if _, err := tx.Exec(ctx, migration); err != nil {
		log.Error().Err(err).Msg("Failed to apply migration")

		return err
	}

	if _, err := tx.Exec(ctx, "INSERT INTO __migrations(migration_file) VALUES ($1)", file); err != nil {
		log.Error().Err(err).Msg("Failed to record migration")

		return err
	}

	return nil
}

// Helper to see if the given slice contains the given string.
func contains(slice []string, val string) bool {
	for _, item := range slice {
		if item == val {
			return true
		}
	}

	return false
}
