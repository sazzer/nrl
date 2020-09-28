package health

import "context"

// Healthchecker represents some component that is able to check it's own health.
type Healthchecker interface {
	// Check the health of this component and return an error if it's unhealthy.
	Healthcheck(ctx context.Context) error
}
