package handlers

import (
	"net/http"

	"github.com/rs/zerolog/log"
)

// CheckHealth performs the healthcheck and writes the response to the client.
func (h Handler) CheckHealth(w http.ResponseWriter, r *http.Request) {
	health := h.service.CheckHealth(r.Context())

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

	_, _ = w.Write([]byte("welcome"))
}
