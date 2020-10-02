package server

import "github.com/labstack/echo/v4"

// Configurer represents a type that can be provided to the server to configure routes.
type Configurer interface {
	// ConfigureRoutes allows routes to be configured on the Router.
	ConfigureRoutes(r *echo.Echo)
}

// ConfigurerFunc represents a function that implements the Configurer interface.
type ConfigurerFunc func(*echo.Echo)

func (f ConfigurerFunc) ConfigureRoutes(r *echo.Echo) {
	f(r)
}
