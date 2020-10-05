package data

import (
	"fmt"
	"time"

	"github.com/google/uuid"
)

type SeedUser struct {
	UserID      uuid.UUID
	Version     uuid.UUID
	Created     time.Time
	Updated     time.Time
	Email       string
	DisplayName string
}

func NewUser() SeedUser {
	return SeedUser{
		UserID:      uuid.New(),
		Version:     uuid.New(),
		Created:     time.Now().UTC().Truncate(time.Second),
		Updated:     time.Now().UTC().Truncate(time.Second),
		Email:       fmt.Sprintf("%s@example.com", uuid.New()),
		DisplayName: fmt.Sprintf("Test User - %s", uuid.New()),
	}
}

func (u SeedUser) SQL() string {
	return `INSERT INTO users(user_id, version, created, updated, email, display_name, authentications)
				  VALUES ($1, $2, $3, $4, $5, $6, $7)`
}

func (u SeedUser) Binds() []interface{} {
	return []interface{}{
		u.UserID.String(),
		u.Version.String(),
		u.Created,
		u.Updated,
		u.Email,
		u.DisplayName,
		"[]",
	}
}
