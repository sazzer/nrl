package database_test

import (
	"context"
	"testing"

	"github.com/sazzer/nrl/sazzer/internal/database"
	"github.com/sazzer/nrl/sazzer/tests/testdatabase"
	"github.com/stretchr/testify/assert"
)

func TestHealth(t *testing.T) {
	testDatabase := testdatabase.New(t)
	defer testDatabase.Close(t)

	database := database.New(testDatabase.URL(t))

	health := database.Healthcheck(context.Background())
	assert.NoError(t, health)
}
