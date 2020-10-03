package database

import (
	"context"
	"database/sql"

	// Database drivers.
	_ "github.com/jackc/pgx/stdlib"
	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog/log"
)

// Database is the wrapper around the database connections.
type Database struct {
	db *sqlx.DB
}

// New creates a new database connection to the provided URL.
func New(url string) Database {
	log.Debug().Str("url", url).Msg("Connecting to database")

	db, err := sqlx.Connect("pgx", url)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to connect to database")
	}

	result := Database{db}

	if err := Migrate(context.Background(), result); err != nil {
		log.Fatal().Err(err).Msg("Failed to migrate database")
	}

	return result
}

// Tx will start a new database transaction for queries against the database.
func (d Database) Tx(ctx context.Context) *sqlx.Tx {
	log.Debug().Msg("Starting transaction")

	tx, err := d.db.BeginTxx(ctx, &sql.TxOptions{})
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to start transaction")
	}

	return tx
}
