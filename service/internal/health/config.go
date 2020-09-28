package health

import "github.com/go-chi/chi"

// Config represents the module configuration for the healthchecks.
type Config struct{}

// New creates a new healthchecker module.
func New(healthcheckers map[string]Healthchecker) Config {
	return Config{}
}

// ConfigureRoutes wires up the healthchecker endpoint onto the HTTP router.
func (c Config) ConfigureRoutes(r chi.Router) {

}
