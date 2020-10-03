package sql

import (
	"context"
	"errors"
	"fmt"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v4"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/nrl/service/internal/users"
)

// Get the user that has the provided unique ID.
func (r repository) GetUserByID(ctx context.Context, id users.UserID) (users.UserModel, error) {
	log.Debug().Interface("id", id).Msg("Finding User by ID")

	row := r.db.QueryRow(ctx, "SELECT * FROM users WHERE user_id = $1", uuid.UUID(id).String())

	var user user
	if err := row.StructScan(&user); err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return users.UserModel{}, users.ErrUserNotFound
		}

		return users.UserModel{}, fmt.Errorf("users: error loading user: %w", err)
	}

	log.Debug().Interface("id", id).Interface("user", user).Msg("Found User by ID")

	return user.UserModel(), nil
}
