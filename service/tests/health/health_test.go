package health_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/sazzer/nrl/service/tests/testservice"
	"gotest.tools/assert"
)

func TestHealth(t *testing.T) {
	service := testservice.New(t)
	defer service.Close(t)

	res := service.Inject(httptest.NewRequest("GET", "/health", nil))
	defer res.Body.Close()

	assert.Equal(t, http.StatusOK, res.StatusCode)
	cupaloy.SnapshotT(t, res.Body)
}
