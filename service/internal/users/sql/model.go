package sql

import (
	"encoding/json"
	"time"

	"github.com/google/uuid"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/nrl/service/internal/users"
)

// Database model representing a user.
type user struct {
	UserID  string    `db:"user_id"`
	Version string    `db:"version"`
	Created time.Time `db:"created"`
	Updated time.Time `db:"updated"`

	Email           string `db:"email"`
	DisplayName     string `db:"display_name"`
	Authentications []byte `db:"authentications"`
}

// Database model representing the authentication details of a user.
type authentication struct {
	ProviderID  string `json:"provider"`
	UserID      string `json:"user_id"`
	DisplayName string `json:"display_name"`
}

func (u user) UserModel() users.UserModel {
	var authentications []authentication
	if err := json.Unmarshal(u.Authentications, &authentications); err != nil {
		log.Error().Err(err).Interface("authentications", u.Authentications).Msg("Failed to parse Authentications JSON")
	}

	realAuthentications := make([]users.Authentication, len(authentications))
	for i, authentication := range authentications {
		realAuthentications[i] = users.Authentication{
			ProviderID:  authentication.ProviderID,
			UserID:      authentication.UserID,
			DisplayName: authentication.DisplayName,
		}
	}

	return users.UserModel{
		ID:      users.UserID(uuid.MustParse(u.UserID)),
		Version: uuid.MustParse(u.Version),
		Created: u.Created.UTC(),
		Updated: u.Updated.UTC(),
		UserData: users.UserData{
			Email:           users.Email(u.Email),
			DisplayName:     u.DisplayName,
			Authentications: realAuthentications,
		},
	}
}
