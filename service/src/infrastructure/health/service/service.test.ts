import { HealthService } from ".";

describe("HealthService", () => {
  test("No components", async () => {
    const sut = new HealthService({});
    const health = await sut.checkHealth();

    expect(health).toMatchInlineSnapshot(`
      Object {
        "components": Object {},
        "status": "HEALTHY",
      }
    `);
  });

  test("Healthy component", async () => {
    const sut = new HealthService({
      healthy: {
        checkHealth: () => Promise.resolve(),
      },
    });
    const health = await sut.checkHealth();

    expect(health).toMatchInlineSnapshot(`
      Object {
        "components": Object {
          "healthy": Object {
            "status": "HEALTHY",
          },
        },
        "status": "HEALTHY",
      }
    `);
  });

  test("Unhealthy component", async () => {
    const sut = new HealthService({
      unhealthy: {
        checkHealth: () => Promise.reject(new Error("Oops")),
      },
    });
    const health = await sut.checkHealth();

    expect(health).toMatchInlineSnapshot(`
      Object {
        "components": Object {
          "unhealthy": Object {
            "message": "Error: Oops",
            "status": "UNHEALTHY",
          },
        },
        "status": "HEALTHY",
      }
    `);
  });

  test("Unhealthy component", async () => {
    const sut = new HealthService({
      healthy: {
        checkHealth: () => Promise.resolve(),
      },
      unhealthy: {
        checkHealth: () => Promise.reject(new Error("Oops")),
      },
    });
    const health = await sut.checkHealth();

    expect(health).toMatchInlineSnapshot(`
      Object {
        "components": Object {
          "healthy": Object {
            "status": "HEALTHY",
          },
          "unhealthy": Object {
            "message": "Error: Oops",
            "status": "UNHEALTHY",
          },
        },
        "status": "HEALTHY",
      }
    `);
  });
});
