package sql

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"

	"github.com/rs/zerolog/log"
	"github.com/sazzer/nrl/service/internal/users"
)

// Representation of the query to use for finding a user by authentication details.
type authenticationQuery struct {
	ProviderID string `json:"provider"`
	UserID     string `json:"user_id"`
}

// Get the user that has the provided authentication details.
func (r repository) FindUserByAuthentication(ctx context.Context, providerID, userID string) (users.UserModel, error) {
	log.Debug().Str("providerID", providerID).Str("userID", userID).Msg("Finding User by Authentication details")

	query := []authenticationQuery{
		authenticationQuery{
			ProviderID: providerID,
			UserID:     userID,
		},
	}

	marshalledQuery, err := json.Marshal(query)
	if err != nil {
		log.Fatal().Err(err).Msg("Unable to marshal JSON query parameter")
	}

	row := r.db.QueryRow(ctx, "SELECT * FROM users WHERE authentications @> $1", marshalledQuery)

	var user user
	if err := row.StructScan(&user); err != nil {
		log.Info().Err(err).Msg("Error loading user")

		if errors.Is(err, sql.ErrNoRows) {
			return users.UserModel{}, users.ErrUserNotFound
		}

		return users.UserModel{}, fmt.Errorf("users: error loading user: %w", err)
	}

	result := user.UserModel()

	log.Debug().Str("providerID", providerID).Str("userID", userID).Interface("user", result).Msg("Found User by ID")

	return result, nil
}
