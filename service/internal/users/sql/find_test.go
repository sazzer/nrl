package sql_test

import (
	"context"
	"testing"

	"github.com/sazzer/nrl/service/internal/infrastructure/database"
	"github.com/sazzer/nrl/service/internal/users"
	"github.com/sazzer/nrl/service/internal/users/sql"
	"github.com/sazzer/nrl/service/tests/data"
	"github.com/sazzer/nrl/service/tests/testdatabase"
	"github.com/stretchr/testify/assert"
)

func TestFindUserByAuthentication_UnknownUser(t *testing.T) {
	db := testdatabase.New(t)
	defer db.Close(t)

	database := database.New(db.URL(t))

	repository := sql.New(database)

	_, err := repository.FindUserByAuthentication(context.Background(), "google", "googleId")

	assert.Equal(t, users.ErrUserNotFound, err)
}

func TestFindUserByAuthentication_KnownUserWithAuthentications(t *testing.T) {
	seedUser := data.NewUser()
	seedUser.WithAuthentication("twitter", "twitterId", "Twitter User")
	seedUser.WithAuthentication("google", "googleId", "Google User")

	db := testdatabase.New(t)
	defer db.Close(t)

	database := database.New(db.URL(t))

	db.Seed(t, seedUser)

	repository := sql.New(database)

	user, err := repository.FindUserByAuthentication(context.Background(), "google", "googleId")

	assert.NoError(t, err)

	assert.Equal(t, users.UserID(seedUser.UserID), user.ID)
	assert.Equal(t, seedUser.Version, user.Version)
	assert.Equal(t, seedUser.Created, user.Created)
	assert.Equal(t, seedUser.Updated, user.Updated)

	assert.Equal(t, users.Email(seedUser.Email), user.Email)
	assert.Equal(t, seedUser.DisplayName, user.DisplayName)

	expectedAuthentications := []users.Authentication{
		users.Authentication{ProviderID: "google", UserID: "googleId", DisplayName: "Google User"},
		users.Authentication{ProviderID: "twitter", UserID: "twitterId", DisplayName: "Twitter User"},
	}
	assert.Equal(t, expectedAuthentications, user.Authentications)
}

func TestFindUserByAuthentication_KnownUserWithAuthentications_CrossedOver(t *testing.T) {
	seedUser := data.NewUser()
	seedUser.WithAuthentication("twitter", "twitterId", "Twitter User")
	seedUser.WithAuthentication("google", "googleId", "Google User")

	db := testdatabase.New(t)
	defer db.Close(t)

	database := database.New(db.URL(t))

	db.Seed(t, seedUser)

	repository := sql.New(database)

	_, err := repository.FindUserByAuthentication(context.Background(), "twitter", "googleId")

	assert.Equal(t, users.ErrUserNotFound, err)
}
