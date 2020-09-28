package health_test

import (
	"errors"
	"testing"

	"github.com/sazzer/nrl/sazzer/internal/health"
	"github.com/stretchr/testify/assert"
)

func TestComponentHealthy(t *testing.T) {
	component := health.ComponentHealth{Error: nil}

	assert.NoError(t, component.Error)
	assert.True(t, component.Healthy())
}

func TestComponentUnhealthy(t *testing.T) {
	component := health.ComponentHealth{Error: errors.New("Oops")}

	assert.Error(t, component.Error)
	assert.False(t, component.Healthy())
}

func TestSystemEmpty(t *testing.T) {
	system := health.SystemHealth{Components: map[string]health.ComponentHealth{}}

	assert.True(t, system.Healthy())
}

func TestSystemHealthy(t *testing.T) {
	system := health.SystemHealth{Components: map[string]health.ComponentHealth{
		"healthy": health.ComponentHealth{Error: nil},
	}}

	assert.True(t, system.Healthy())
}

func TestSystemUnhealthy(t *testing.T) {
	system := health.SystemHealth{Components: map[string]health.ComponentHealth{
		"unhealthy": health.ComponentHealth{Error: errors.New("Oops")},
	}}

	assert.False(t, system.Healthy())
}

func TestSystemMixed(t *testing.T) {
	system := health.SystemHealth{Components: map[string]health.ComponentHealth{
		"healthy":   health.ComponentHealth{Error: nil},
		"unhealthy": health.ComponentHealth{Error: errors.New("Oops")},
	}}

	assert.False(t, system.Healthy())
}
