package sql

import (
	"time"
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
