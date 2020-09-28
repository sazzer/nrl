package service

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/nrl/sazzer/internal/health"
	"github.com/sazzer/nrl/sazzer/internal/server"
)

// Service represents the actual running service for the application.
type Service struct {
	server server.Server
}

// New creates a new instance of the service.
func New() Service {
	health := health.New(map[string]health.Healthchecker{})

	s := server.New([]server.Configurer{
		health,
	})

	return Service{
		server: s,
	}
}

// Start starts the service listening on the provided HTTP port.
func (s Service) Start(port uint16) {
	log.Info().Uint16("port", port).Msg("Starting Service")
	s.server.Start(port)
}
