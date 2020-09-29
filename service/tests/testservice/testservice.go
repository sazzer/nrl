package testservice

import (
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/nrl/sazzer/internal/service"
	"github.com/sazzer/nrl/sazzer/tests/testdatabase"
)

// TestService is a wrapper around the service being tested.
type TestService struct {
	testDatabase testdatabase.TestDatabase
	service      service.Service
}

// New creates a new instance of TestService.
func New(t *testing.T) TestService {
	zerolog.SetGlobalLevel(zerolog.DebugLevel)

	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr, TimeFormat: time.RFC3339}).With().Caller().Logger()

	db := testdatabase.New(t)

	service := service.New(db.URL(t))

	return TestService{
		testDatabase: db,
		service:      service,
	}
}

// Close the TestService down, freeing up anything it's currently doing.
func (s *TestService) Close(t *testing.T) {
	s.testDatabase.Close(t)
}

// Inject will send an HTTP Request into the service and return the response.
func (s *TestService) Inject(req *http.Request) *http.Response {
	rec := httptest.NewRecorder()

	s.service.ServeHTTP(rec, req)

	return rec.Result()
}
