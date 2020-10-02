package service

import (
	"context"

	"github.com/sazzer/nrl/service/internal/users"
)

// The repository of user data.
type repository interface {
	// Get the user that has the provided unique ID.
	GetUserByID(ctx context.Context, id users.UserID) (users.UserModel, error)
}
