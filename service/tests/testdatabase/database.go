package testdatabase

import (
	"context"
	"fmt"
	"testing"

	"github.com/rs/zerolog/log"
	"github.com/stretchr/testify/assert"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/wait"
)

// Wrapper around the database used for testing.
type TestDatabase struct {
	container testcontainers.Container
}

// Create a new database connection.
func New(t *testing.T) TestDatabase {
	if testing.Short() {
		t.Skip("Not running database tests")
	}

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
	assert.NoError(t, err)

	log.Debug().Msg("Started test database")

	return TestDatabase{
		container: pgC,
	}
}

// Close the database connection.
func (db TestDatabase) Close(t *testing.T) {
	ctx := context.Background()

	err := db.container.Terminate(ctx)
	assert.NoError(t, err)
}

func (db TestDatabase) URL(t *testing.T) string {
	ctx := context.Background()

	ip, err := db.container.Host(ctx)
	assert.NoError(t, err)

	port, err := db.container.MappedPort(ctx, "5432/tcp")
	assert.NoError(t, err)

	return fmt.Sprintf("postgres://nrl-test:nrl-test@%s:%d/nrl-test", ip, port.Int())
}
