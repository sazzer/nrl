package tests

import (
	"net/http"
	"net/http/httptest"

	"github.com/sazzer/nrl/sazzer/internal/service"
)

// TestService is a wrapper around the service being tested.
type TestService struct {
	service service.Service
}

// New creates a new instance of TestService.
func New() TestService {
	service := service.New()

	return TestService{
		service: service,
	}
}

// Close the TestService down, freeing up anything it's currently doing.
func (s *TestService) Close() {

}

// Inject will send an HTTP Request into the service and return the response.
func (s *TestService) Inject(req *http.Request) *http.Response {
	rec := httptest.NewRecorder()

	s.service.ServeHTTP(rec, req)

	return rec.Result()
}
