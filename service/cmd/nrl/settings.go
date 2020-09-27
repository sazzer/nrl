package main

import (
	"log"

	"github.com/kelseyhightower/envconfig"
)

// Config represents the configuration of the application.
type Config struct {
	Debug bool
}

// loadConfig loads the configuration to use for the app.
func loadConfig() Config {
	var c Config

	err := envconfig.Process("", &c)
	if err != nil {
		log.Fatal(err.Error())
	}

	return c
}
