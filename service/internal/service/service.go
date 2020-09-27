package service

import "github.com/rs/zerolog/log"

// Service represents the actual running service for the application.
type Service struct {
}

// New creates a new instance of the service.
func New() Service {
	return Service{}
}

// Start starts the service listening on the provided HTTP port.
func (s Service) Start(port uint16) {
	log.Info().Uint16("port", port).Msg("Starting Service")
}
