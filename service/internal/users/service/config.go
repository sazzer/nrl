package service

import (
	"github.com/sazzer/nrl/service/internal/infrastructure/database"
	"github.com/sazzer/nrl/service/internal/users/sql"
)

// Config represents the configuration of the users service.
type Config struct {
	Service service
}

// New creates a new instance of the users service configuration.
func New(db database.Database) Config {
	return Config{
		Service: service{
			repository: sql.New(db),
		},
	}
}
