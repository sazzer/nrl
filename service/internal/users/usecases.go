package users

import (
	"context"
	"errors"
)

var (
	// ErrUserNotFound indicates that no user was found with the provided ID.
	ErrUserNotFound = errors.New("users: The requested user was not found")
)

// GetUserUseCase is the use case used to load a single user by ID.
type GetUserUseCase interface {
	// GetUserByID allows us to load the user with the given ID.
	GetUserByID(ctx context.Context, id UserID) (UserModel, error)
}
