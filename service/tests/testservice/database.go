package testservice

import (
	"context"
	"fmt"
	"testing"

	"github.com/rs/zerolog/log"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/wait"
)

// Wrapper around the database used for testing.
type TestDatabase struct {
	container testcontainers.Container
}

// Create a new database connection.
func NewDatabase(t *testing.T) TestDatabase {
	ctx := context.Background()

	log.Debug().Msg("Starting test database")

	req := testcontainers.ContainerRequest{
		Image: "postgres:12.4-alpine",
		Env: map[string]string{
			"POSTGRES_DB":       "nrl-test",
			"POSTGRES_USER":     "nrl-test",
			"POSTGRES_PASSWORD": "nrl-test",
		},
		ExposedPorts: []string{"5432/tcp"},
		WaitingFor:   wait.ForListeningPort("5432/tcp"),
	}

	pgC, err := testcontainers.GenericContainer(ctx, testcontainers.GenericContainerRequest{
		ContainerRequest: req,
		Started:          true,
	})
	if err != nil {
		log.Error().Err(err).Msg("Failed to start test database")

		t.Fatal(err)
	} else {
		log.Debug().Msg("Started test database")
	}

	return TestDatabase{
		container: pgC,
	}
}

// Close the database connection.
func (d TestDatabase) Close(t *testing.T) {
	ctx := context.Background()

	err := d.container.Terminate(ctx)
	if err != nil {
		t.Fatal(err)
	}
}

func (d TestDatabase) URL(t *testing.T) string {
	ctx := context.Background()

	ip, err := d.container.Host(ctx)
	if err != nil {
		t.Fatal(err)
	}

	port, err := d.container.MappedPort(ctx, "5432/tcp")
	if err != nil {
		t.Fatal(err)
	}

	return fmt.Sprintf("postgres://nrl-test:nrl-test@%s:%d/nrl-test", ip, port.Int())
}
