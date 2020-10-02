package handlers

import "github.com/sazzer/nrl/service/internal/infrastructure/health"

// Handler represents the HTTP Handlers for working with healthchecks.
type Handler struct {
	service health.Service
}

// New creates a new Handler instance for handling health calls.
func New(service health.Service) Handler {
	return Handler{service}
}
