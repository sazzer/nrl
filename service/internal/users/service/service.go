package service

import (
	"context"

	"github.com/sazzer/nrl/service/internal/users"
)

// The actual implementation of the Users Service.
type service struct {
	repository repository
}

// GetUserByID allows us to load the user with the given ID.
func (s service) GetUserByID(ctx context.Context, id users.UserID) (users.UserModel, error) {
	return s.repository.GetUserByID(ctx, id)
}

// Get the user that has the provided authentication details.
func (s service) FindUserByAuthentication(ctx context.Context, providerID, userID string) (users.UserModel, error) {
	return s.repository.FindUserByAuthentication(ctx, providerID, userID)
}
