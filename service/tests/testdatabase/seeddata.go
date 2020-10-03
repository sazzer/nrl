package testdatabase

// Interface for all seedable types to implement so that they can be seeded into the database.
type SeedData interface {
	// Get the SQL needed to insert the data.
	SQL() string

	// Get the binds for the SQL.
	Binds() []interface{}
}
