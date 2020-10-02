package handlers

import (
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/rs/zerolog/log"
)

// CheckHealth performs the healthcheck and writes the response to the client.
func (h Handler) CheckHealth(c echo.Context) error {
	health := h.service.CheckHealth(c.Request().Context())

	components := map[string]ComponentHealth{}

	for name, component := range health.Components {
		message := ""
		if component.Error != nil {
			message = component.Error.Error()
		}

		components[name] = ComponentHealth{
			Error:   message,
			Healthy: component.Healthy(),
		}
	}

	systemHealth := SystemHealth{
		Components: components,
		Healthy:    health.Healthy(),
	}

	log.Debug().Interface("result", systemHealth).Msg("System Health")

	status := http.StatusOK
	if !health.Healthy() {
		status = http.StatusServiceUnavailable
	}

	return c.JSON(status, systemHealth)
}
