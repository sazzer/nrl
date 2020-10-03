package sql

import (
	"time"

	"github.com/google/uuid"
)

type user struct {
	UserID  uuid.UUID `db:"user_id"`
	Version uuid.UUID `db:"version"`
	Created time.Time `db:"created"`
	Updated time.Time `db:"updated"`

	Email           string `db:"email"`
	DisplayName     string `db:"display_name"`
	Authentications string `db:"authentications"`
}
