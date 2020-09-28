package service

import (
	"net/http"

	"github.com/go-chi/chi"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/nrl/sazzer/internal/server"
)

// Service represents the actual running service for the application.
type Service struct {
	server server.Server
}

// New creates a new instance of the service.
func New() Service {
	s := server.New()

	s.Config(server.ConfigurerFunc(func(r chi.Router) {
		r.Post("/", func(w http.ResponseWriter, r *http.Request) {
			_, _ = w.Write([]byte("welcome"))
		})
	}))

	return Service{
		server: s,
	}
}

// Start starts the service listening on the provided HTTP port.
func (s Service) Start(port uint16) {
	log.Info().Uint16("port", port).Msg("Starting Service")
	s.server.Start(port)
}
