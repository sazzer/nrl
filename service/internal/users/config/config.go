package config

import (
	"github.com/labstack/echo/v4"
	"github.com/sazzer/nrl/service/internal/infrastructure/database"
	"github.com/sazzer/nrl/service/internal/users/service"
)

// Config represents the configuration of the users module.
type Config struct {
	service.Config
}

// New creates a new instance of the configuration.
func New(db database.Database) Config {
	serviceConfig := service.New(db)

	return Config{
		Config: serviceConfig,
	}
}

// ConfigureRoutes wires up the users endpoints onto the HTTP router.
func (c Config) ConfigureRoutes(e *echo.Echo) {
}
