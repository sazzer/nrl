package service

// Config represents the configuration of the users service.
type Config struct {
	Service service
}

// New creates a new instance of the users service configuration.
func New() Config {
	return Config{
		Service: service{},
	}
}
