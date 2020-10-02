package sql_test

import (
	"context"
	"testing"

	"github.com/sazzer/nrl/service/internal/infrastructure/database"
	"github.com/sazzer/nrl/service/internal/users"
	"github.com/sazzer/nrl/service/internal/users/sql"
	"github.com/sazzer/nrl/service/tests/testdatabase"
	"gotest.tools/assert"
)

func TestGetUserByID_UnknownUser(t *testing.T) {
	db := testdatabase.New(t)
	defer db.Close(t)

	database := database.New(db.URL(t))

	repository := sql.New(database)

	_, err := repository.GetUserByID(context.Background(), users.UserID("00000000-0000-0000-0000-000000000000"))

	assert.Equal(t, err, users.ErrUserNotFound)
}
