package testdatabase

import (
	"context"
	"testing"

	"github.com/jackc/pgx/v4"
	"github.com/rs/zerolog/log"
	"github.com/stretchr/testify/assert"
)

// Actually seed the provided data into the database.
func (db *TestDatabase) Seed(t *testing.T, data ...SeedData) {
	ctx := context.Background()

	// Connect to the database
	conn, err := pgx.Connect(ctx, db.URL(t))
	assert.NoError(t, err)

	defer conn.Close(ctx)

	// Start a new database transaction
	tx, err := conn.Begin(ctx)
	assert.NoError(t, err)

	defer func() {
		_ = tx.Rollback(ctx)
	}()

	// Execute each piece of seed data, in order
	for _, d := range data {
		sql := d.SQL()
		binds := d.Binds()

		log.Debug().Str("sql", sql).Interface("binds", binds).Msg("Seeding data into the database")

		_, err := tx.Exec(ctx, sql, binds...)
		assert.NoError(t, err)
	}

	if t.Failed() {
		return
	}

	// Commit the transaction
	err = tx.Commit(ctx)
	assert.NoError(t, err)
}
