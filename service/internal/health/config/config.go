package health

import (
	"github.com/go-chi/chi"
	"github.com/sazzer/nrl/sazzer/internal/health"
	"github.com/sazzer/nrl/sazzer/internal/health/handlers"
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
func (c Config) ConfigureRoutes(r chi.Router) {
	service := health.New(c.healthcheckers)
	handler := handlers.New(service)

	r.Get("/health", handler.CheckHealth)
}
