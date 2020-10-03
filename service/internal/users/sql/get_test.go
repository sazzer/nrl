package sql_test

import (
	"context"
	"testing"

	"github.com/google/uuid"
	"github.com/sazzer/nrl/service/internal/infrastructure/database"
	"github.com/sazzer/nrl/service/internal/users"
	"github.com/sazzer/nrl/service/internal/users/sql"
	"github.com/sazzer/nrl/service/tests/data"
	"github.com/sazzer/nrl/service/tests/testdatabase"
	"github.com/stretchr/testify/assert"
)

func TestGetUserByID_UnknownUser(t *testing.T) {
	db := testdatabase.New(t)
	defer db.Close(t)

	database := database.New(db.URL(t))

	repository := sql.New(database)

	_, err := repository.GetUserByID(context.Background(),
		users.UserID(uuid.MustParse("00000000-0000-0000-0000-000000000000")))

	assert.Equal(t, err, users.ErrUserNotFound)
}

func TestGetUserByID_KnownUser(t *testing.T) {
	seedUser := data.NewUser()

	db := testdatabase.New(t)
	defer db.Close(t)

	database := database.New(db.URL(t))

	db.Seed(t, seedUser)

	repository := sql.New(database)

	user, err := repository.GetUserByID(context.Background(), users.UserID(seedUser.UserID))

	assert.NoError(t, err)

	assert.Equal(t, users.UserID(seedUser.UserID), user.ID)
	assert.Equal(t, seedUser.Version, user.Version)
	assert.Equal(t, seedUser.Created, user.Created)
	assert.Equal(t, seedUser.Updated, user.Updated)

	assert.Equal(t, users.Email(seedUser.Email), user.Email)
	assert.Equal(t, seedUser.DisplayName, user.DisplayName)
	assert.Equal(t, []users.Authentication{}, user.Authentications)
}
