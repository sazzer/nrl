package sql

import (
	"encoding/json"
	"sort"
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

// Convert the database record into an API UserModel.
func (u user) UserModel() users.UserModel {
	// Parse the JSONB String into the correct structure.
	var authentications []authentication
	if err := json.Unmarshal(u.Authentications, &authentications); err != nil {
		log.Error().Err(err).Interface("authentications", u.Authentications).Msg("Failed to parse Authentications JSON")
	}

	// Transform the structure into the API versions.
	realAuthentications := make([]users.Authentication, len(authentications))
	for i, authentication := range authentications {
		realAuthentications[i] = users.Authentication{
			ProviderID:  authentication.ProviderID,
			UserID:      authentication.UserID,
			DisplayName: authentication.DisplayName,
		}
	}

	// Sort the authentications, first by Provider ID then User ID.
	sort.Slice(realAuthentications, func(a, b int) bool {
		if realAuthentications[a].ProviderID < realAuthentications[b].ProviderID {
			return true
		} else if realAuthentications[a].ProviderID > realAuthentications[b].ProviderID {
			return false
		}

		if realAuthentications[a].UserID < realAuthentications[b].UserID {
			return true
		} else if realAuthentications[a].UserID > realAuthentications[b].UserID {
			return false
		}

		return realAuthentications[a].DisplayName < realAuthentications[b].DisplayName
	})

	// And finally build the API UserModel from the data we've got.
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
