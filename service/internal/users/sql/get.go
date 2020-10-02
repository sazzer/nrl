package sql

import (
	"context"

	"github.com/sazzer/nrl/service/internal/users"
)

// Get the user that has the provided unique ID.
func (r repository) GetUserByID(ctx context.Context, id users.UserID) (users.UserModel, error) {
	return users.UserModel{}, users.ErrUserNotFound
}
