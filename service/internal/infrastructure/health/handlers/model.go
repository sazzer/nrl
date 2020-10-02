package handlers

// ComponentHealth represents the API model of the health of a single component.
type ComponentHealth struct {
	Error   string `json:"error,omitempty"`
	Healthy bool   `json:"healthy"`
}

// SystemHealth represenst the API model of the health of the entire system.
type SystemHealth struct {
	Components map[string]ComponentHealth `json:"components"`
	Healthy    bool                       `json:"healthy"`
}
