package health

import (
	"context"

	"github.com/rs/zerolog/log"
)

// Service allows us to check the health of the entire system.
type Service struct {
	components map[string]Healthchecker
}

// New creates a new Health Service instance.
func New(components map[string]Healthchecker) Service {
	return Service{components}
}

// CheckHealth will check the health of the entire system and return with the status.
func (h Service) CheckHealth(ctx context.Context) SystemHealth {
	result := map[string]ComponentHealth{}

	for name, component := range h.components {
		health := component.Healthcheck(ctx)
		result[name] = ComponentHealth{health}

		if health != nil {
			log.Warn().Err(health).Str("component", name).Msg("Healthcheck failed")
		} else {
			log.Info().Str("component", name).Msg("Healthcheck passed")
		}
	}

	return SystemHealth{result}
}
