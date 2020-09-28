package health

import "context"

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
	}

	return SystemHealth{result}
}
