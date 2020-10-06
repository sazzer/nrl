package data

import (
	"encoding/json"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/rs/zerolog/log"
)

// Identical representation of the authentication details to the ones used in the database model.
type Authentication struct {
	ProviderID  string `json:"provider"`
	UserID      string `json:"user_id"`
	DisplayName string `json:"display_name"`
}

// Representation of a user ready to be seeded into the database.
type SeedUser struct {
	UserID          uuid.UUID
	Version         uuid.UUID
	Created         time.Time
	Updated         time.Time
	Email           string
	DisplayName     string
	Authentications []Authentication
}

// Create a new, random user ready to be seeded into the database.
func NewUser() SeedUser {
	return SeedUser{
		UserID:          uuid.New(),
		Version:         uuid.New(),
		Created:         time.Now().UTC().Truncate(time.Second),
		Updated:         time.Now().UTC().Truncate(time.Second),
		Email:           fmt.Sprintf("%s@example.com", uuid.New()),
		DisplayName:     fmt.Sprintf("Test User - %s", uuid.New()),
		Authentications: []Authentication{},
	}
}

// Provide some authentication details on the user to be seeded.
func (u *SeedUser) WithAuthentication(provider, user, displayName string) *SeedUser {
	u.Authentications = append(u.Authentications, Authentication{
		ProviderID:  provider,
		UserID:      user,
		DisplayName: displayName,
	})

	return u
}

// Generate the SQL needed to insert a user into the database.
func (u SeedUser) SQL() string {
	return `INSERT INTO users(user_id, version, created, updated, email, display_name, authentications)
				  VALUES ($1, $2, $3, $4, $5, $6, $7)`
}

// Generate the binds needed to insert a user into the database.
func (u SeedUser) Binds() []interface{} {
	authentications, err := json.Marshal(u.Authentications)
	if err != nil {
		log.Error().Err(err).Interface("authentications", u.Authentications).Msg("Failed to marshal Authentications")
	}

	return []interface{}{
		u.UserID.String(),
		u.Version.String(),
		u.Created,
		u.Updated,
		u.Email,
		u.DisplayName,
		authentications,
	}
}
