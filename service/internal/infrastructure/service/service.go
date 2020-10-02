package service

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/nrl/service/internal/infrastructure/database"
	health "github.com/sazzer/nrl/service/internal/infrastructure/health/config"
	"github.com/sazzer/nrl/service/internal/infrastructure/server"
	users "github.com/sazzer/nrl/service/internal/users/config"
)

// Service represents the actual running service for the application.
type Service struct {
	server server.Server
}

// New creates a new instance of the service.
func New(databaseURL string) Service {
	db := database.New(databaseURL)

	health := health.New().WithHealthcheck("db", db)

	users := users.New(db)

	s := server.New([]server.Configurer{
		health,
		users,
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
