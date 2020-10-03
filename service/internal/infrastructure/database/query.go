package database

import (
	"context"

	"github.com/jackc/pgx/v4"
)

// Query the database for a single matching row.
func (d Database) QueryRow(ctx context.Context, sql string, binds ...interface{}) pgx.Row {
	return d.db.QueryRow(ctx, sql, binds...)
}
