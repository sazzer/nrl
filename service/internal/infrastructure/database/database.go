package database

import (
	"context"

	"github.com/jackc/pgx/v4"
	"github.com/jackc/pgx/v4/log/zerologadapter"
	"github.com/jackc/pgx/v4/pgxpool"
	"github.com/rs/zerolog/log"
)

// Database is the wrapper around the database connections.
type Database struct {
	db *pgxpool.Pool
}

// New creates a new database connection to the provided URL.
func New(url string) Database {
	log.Debug().Str("url", url).Msg("Connecting to database")

	config, err := pgxpool.ParseConfig(url)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to parse database connection string")
	}

	config.ConnConfig.LogLevel = pgx.LogLevelDebug
	config.ConnConfig.Logger = zerologadapter.NewLogger(log.Logger)

	db, err := pgxpool.ConnectConfig(context.Background(), config)
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
func (d Database) Tx(ctx context.Context) pgx.Tx {
	log.Debug().Msg("Starting transaction")

	tx, err := d.db.Begin(ctx)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to start transaction")
	}

	return tx
}
