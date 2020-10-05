import { TestService } from "./testService";

test("GET /health", async () => {
  const sut = new TestService();

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
