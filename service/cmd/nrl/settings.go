package main

import (
	"github.com/kelseyhightower/envconfig"
	"github.com/rs/zerolog/log"
)

// Config represents the configuration of the application.
type Config struct {
	Debug bool

	Port uint16 `default:"8000"`

	DatabaseURL string `required:"true" envconfig:"database_url"`
}

// loadConfig loads the configuration to use for the app.
func loadConfig() Config {
	var c Config

	err := envconfig.Process("", &c)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to load config")
	}

	return c
}
