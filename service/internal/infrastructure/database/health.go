package database

import "context"

// Check the health of this component and return an error if it's unhealthy.
func (d Database) Healthcheck(ctx context.Context) error {
	_, err := d.db.ExecContext(ctx, "SELECT 1")

	return err
}
