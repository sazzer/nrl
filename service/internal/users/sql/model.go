package sql

import (
	"time"

	"github.com/google/uuid"
	"github.com/sazzer/nrl/service/internal/users"
)

type user struct {
	UserID  string    `db:"user_id"`
	Version string    `db:"version"`
	Created time.Time `db:"created"`
	Updated time.Time `db:"updated"`

	Email           string `db:"email"`
	DisplayName     string `db:"display_name"`
	Authentications string `db:"authentications"`
}

func (u user) UserModel() users.UserModel {
	return users.UserModel{
		ID:      users.UserID(uuid.MustParse(u.UserID)),
		Version: uuid.MustParse(u.Version),
		Created: u.Created.UTC(),
		Updated: u.Updated.UTC(),
		UserData: users.UserData{
			Email:           users.Email(u.Email),
			DisplayName:     u.DisplayName,
			Authentications: []users.Authentication{},
		},
	}
}
