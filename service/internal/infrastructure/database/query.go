package database

import (
	"context"

	"github.com/jmoiron/sqlx"
)

// Query the database for a single matching row.
func (d Database) QueryRow(ctx context.Context, sql string, binds ...interface{}) *sqlx.Row {
	return d.db.QueryRowxContext(ctx, sql, binds...)
}
