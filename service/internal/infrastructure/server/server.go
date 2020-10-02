package server

import (
	"fmt"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/rs/zerolog/log"
)

type Server struct {
	router *echo.Echo
}

func New(config []Configurer) Server {
	e := echo.New()

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.Use(middleware.RequestID())

	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowCredentials: true,
		MaxAge:           300,
	}))

	for _, c := range config {
		c.ConfigureRoutes(e)
	}

	return Server{
		router: e,
	}
}

// Start starts the HTTP Server listening on the provided port.
func (s Server) Start(port uint16) {
	address := fmt.Sprintf(":%d", port)
	log.Info().Str("address", address).Msg("Starting Server")

	err := s.router.Start(address)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to start HTTP Server")
	}
}
