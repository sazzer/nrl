package health

import (
	"github.com/labstack/echo/v4"
	"github.com/sazzer/nrl/service/internal/health"
	"github.com/sazzer/nrl/service/internal/health/handlers"
)

// Config represents the module configuration for the healthchecks.
type Config struct {
	healthcheckers map[string]health.Healthchecker
}

// New creates a new healthchecker module.
func New() Config {
	return Config{
		healthcheckers: map[string]health.Healthchecker{},
	}
}

// WithHealthcheck adds a new healthcheck to the service.
func (c Config) WithHealthcheck(name string, check health.Healthchecker) Config {
	c.healthcheckers[name] = check

	return c
}

// ConfigureRoutes wires up the healthchecker endpoint onto the HTTP router.
func (c Config) ConfigureRoutes(e *echo.Echo) {
	service := health.New(c.healthcheckers)
	handler := handlers.New(service)

	e.GET("/health", handler.CheckHealth)
}
