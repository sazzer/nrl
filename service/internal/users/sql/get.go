package sql

import (
	"context"
	"database/sql"
	"errors"
	"fmt"

	"github.com/google/uuid"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/nrl/service/internal/users"
)

// Get the user that has the provided unique ID.
func (r repository) GetUserByID(ctx context.Context, id users.UserID) (users.UserModel, error) {
	log.Debug().Interface("id", id).Msg("Finding User by ID")

	row := r.db.QueryRow(ctx, "SELECT * FROM users WHERE user_id = $1", uuid.UUID(id).String())

	var user user
	if err := row.StructScan(&user); err != nil {
		log.Info().Err(err).Msg("Error loading user")

		if errors.Is(err, sql.ErrNoRows) {
			return users.UserModel{}, users.ErrUserNotFound
		}

		return users.UserModel{}, fmt.Errorf("users: error loading user: %w", err)
	}

	result := user.UserModel()

	log.Debug().Interface("id", id).Interface("user", result).Msg("Found User by ID")

	return result, nil
}
