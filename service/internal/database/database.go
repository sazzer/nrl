package database

import (
	"context"

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

	db, err := pgxpool.Connect(context.Background(), url)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to connect to database")
	}

	return Database{db}
}
