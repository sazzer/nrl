package server

import "github.com/go-chi/chi"

// Configurer represents a type that can be provided to the server to configure routes.
type Configurer interface {
	// ConfigureRoutes allows routes to be configured on the Router.
	ConfigureRoutes(r chi.Router)
}

// ConfigurerFunc represents a function that implements the Configurer interface.
type ConfigurerFunc func(chi.Router)

func (f ConfigurerFunc) ConfigureRoutes(r chi.Router) {
	f(r)
}
