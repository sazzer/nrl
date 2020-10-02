package users

import "time"

// UserID is the ID of the user within this system.
type UserID string

// Email is the email address of the user.
type Email string

// Authentication is the authentication details of the user at an external provider.
type Authentication struct {
	// ProviderID is the ID of the provider these details are for.
	ProviderID string
	// UserID is the ID of the user at this provider.
	UserID string
	// DisplayName is the display name of the user at this provider.
	DisplayName string
}

// UserData is the data that makes up a user.
type UserData struct {
	Email           Email
	DisplayName     string
	Authentications []Authentication
}

// UserModel is the model representation of a user that exists in the data store.
type UserModel struct {
	ID      UserID
	Version string
	Created time.Time
	Updated time.Time
	UserData
}
