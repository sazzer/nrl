package main

import (
	"os"
	"time"

	_ "github.com/joho/godotenv/autoload"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/nrl/service/internal/infrastructure/service"
)

func main() {
	c := loadConfig()

	if c.Debug {
		zerolog.SetGlobalLevel(zerolog.DebugLevel)

		log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr, TimeFormat: time.RFC3339}).With().Caller().Logger()
	} else {
		zerolog.SetGlobalLevel(zerolog.InfoLevel)
	}

	zerolog.TimeFieldFormat = zerolog.TimeFormatUnixMicro

	log.Debug().Interface("config", c).Msg("Loaded Config")

	service := service.New(c.DatabaseURL)
	service.Start(c.Port)
}
