package database_test

import (
	"context"
	"testing"

	"github.com/sazzer/nrl/service/internal/infrastructure/database"
	"github.com/sazzer/nrl/service/tests/testdatabase"
	"github.com/stretchr/testify/assert"
)

func TestMigrate(t *testing.T) {
	testDatabase := testdatabase.New(t)
	defer testDatabase.Close(t)

	db := database.New(testDatabase.URL(t))

	err := database.Migrate(context.Background(), db)
	assert.NoError(t, err)
}

func TestMigrateTwice(t *testing.T) {
	testDatabase := testdatabase.New(t)
	defer testDatabase.Close(t)

	db := database.New(testDatabase.URL(t))

	err := database.Migrate(context.Background(), db)
	assert.NoError(t, err)

	err = database.Migrate(context.Background(), db)
	assert.NoError(t, err)
}
