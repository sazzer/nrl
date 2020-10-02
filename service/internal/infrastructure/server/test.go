package server

import "net/http"

// Inject a request into the server and get the response.
//
// This is used for integration testing.
func (s *Server) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	s.router.ServeHTTP(w, r)
}
