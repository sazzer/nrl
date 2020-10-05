import { TestService, newTestService } from "./testService";

describe("Health", () => {
  let sut: TestService;

  beforeEach(async () => {
    sut = await newTestService();
  });

  afterEach(async () => {
    await new Promise((resolve) => setTimeout(resolve, 20000));

    await sut.destroy();
  });

  test("GET /health", async () => {
    const response = await sut.get("/health");
    expect(response.status).toBe(200);
    expect(response.type).toEqual("application/json");
    expect(response.body).toMatchInlineSnapshot(`
      Object {
        "components": Object {},
        "status": "HEALTHY",
      }
    `);
  });
});
