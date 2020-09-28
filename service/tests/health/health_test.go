package health_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/sazzer/nrl/sazzer/tests"
	"gotest.tools/assert"
)

func TestHealth(t *testing.T) {
	service := tests.New()
	defer service.Close()

	res := service.Inject(httptest.NewRequest("GET", "/health", nil))
	defer res.Body.Close()

	assert.Equal(t, http.StatusOK, res.StatusCode)
	cupaloy.SnapshotT(t, res.Body)
}
