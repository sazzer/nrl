package server

import (
	"fmt"
	"net/http"
	"time"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/go-chi/cors"
	"github.com/rs/zerolog/log"
)

type Server struct {
	router chi.Router
}

func New() Server {
	r := chi.NewRouter()

	r.Use(middleware.RequestID)
	r.Use(middleware.RealIP)
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)
	r.Use(middleware.Timeout(60 * time.Second))
	r.Use(middleware.GetHead)

	r.Use(cors.Handler(cors.Options{
		AllowedHeaders:   []string{"Authorization"},
		AllowCredentials: true,
	}))

	r.Post("/", func(w http.ResponseWriter, r *http.Request) {
		_, _ = w.Write([]byte("welcome"))
	})

	return Server{
		router: r,
	}
}

// Start starts the HTTP Server listening on the provided port.
func (s Server) Start(port uint16) {
	address := fmt.Sprintf(":%d", port)
	log.Info().Str("address", address).Msg("Starting Server")

	err := http.ListenAndServe(address, s.router)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to start HTTP Server")
	}
}
