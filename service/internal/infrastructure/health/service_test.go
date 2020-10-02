package health_test

import (
	"context"
	"errors"
	"testing"

	"github.com/sazzer/nrl/service/internal/infrastructure/health"
	"github.com/stretchr/testify/assert"
)

type healthcheckerFunc func(context.Context) error

func (f healthcheckerFunc) Healthcheck(ctx context.Context) error {
	return f(ctx)
}

func TestEmptyService(t *testing.T) {
	service := health.New(map[string]health.Healthchecker{})

	result := service.CheckHealth(context.Background())

	assert.True(t, result.Healthy())
	assert.Equal(t, 0, len(result.Components))
}

func TestHealthyService(t *testing.T) {
	service := health.New(map[string]health.Healthchecker{
		"healthy": healthcheckerFunc(func(context.Context) error { return nil }),
	})

	result := service.CheckHealth(context.Background())

	assert.True(t, result.Healthy())
	assert.Equal(t, 1, len(result.Components))

	component, ok := result.Components["healthy"]
	assert.True(t, ok)
	assert.True(t, component.Healthy())
}

func TestUnhealthyService(t *testing.T) {
	service := health.New(map[string]health.Healthchecker{
		"unhealthy": healthcheckerFunc(func(context.Context) error { return errors.New("Oops") }),
	})

	result := service.CheckHealth(context.Background())

	assert.False(t, result.Healthy())
	assert.Equal(t, 1, len(result.Components))

	component, ok := result.Components["unhealthy"]
	assert.True(t, ok)
	assert.False(t, component.Healthy())
}

func TestMixedService(t *testing.T) {
	service := health.New(map[string]health.Healthchecker{
		"healthy":   healthcheckerFunc(func(context.Context) error { return nil }),
		"unhealthy": healthcheckerFunc(func(context.Context) error { return errors.New("Oops") }),
	})

	result := service.CheckHealth(context.Background())

	assert.False(t, result.Healthy())
	assert.Equal(t, 2, len(result.Components))

	component, ok := result.Components["healthy"]
	assert.True(t, ok)
	assert.True(t, component.Healthy())

	component, ok = result.Components["unhealthy"]
	assert.True(t, ok)
	assert.False(t, component.Healthy())
}
