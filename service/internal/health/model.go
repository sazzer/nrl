package health

// ComponentHealth represents the health of a single component.
type ComponentHealth struct {
	// Error represents the error that occurred if the component is unhealthy.
	Error error
}

// Healthy indicates if this component is healthy or not.
// Healthy components are those that did not return an error.
func (c ComponentHealth) Healthy() bool {
	return c.Error == nil
}

// SystemHealth represents the health of the entire system.
// This is the aggregate health of every component in the system.
type SystemHealth struct {
	// The map of component names to the health of that component.
	Components map[string]ComponentHealth
}

// Healthy indicates if the system is healthy or not. This is if every single component is healthy.
func (s SystemHealth) Healthy() bool {
	result := true

	for _, c := range s.Components {
		if !c.Healthy() {
			result = false
		}
	}

	return result
}
