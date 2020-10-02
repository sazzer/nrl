package sql

import "github.com/sazzer/nrl/service/internal/infrastructure/database"

// The repository of user data.
type repository struct {
	db database.Database
}

// Create a new users repository.
// nolint:golint
// This is deliberately unexported so it can only be used by the service layer.
func New(db database.Database) repository {
	return repository{db}
}
