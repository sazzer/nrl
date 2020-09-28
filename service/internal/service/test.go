package service

import "net/http"

// Inject a request into the server and get the response.
//
// This is used for integration testing.
func (s *Service) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	s.server.ServeHTTP(w, r)
}
