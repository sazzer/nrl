import * as sut from "./list";

import nock from "nock";

describe("List Authentication Providers", () => {
  test("None returned", async () => {
    nock("http://nrl.example.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Content-Type",
      })
      .get("/authentication")
      .reply(200, {
        entries: [],
        pagination: {
          offset: 0,
          total: 0,
        },
      });

    const providers = await sut.listProviders();
    expect(providers).toEqual([]);
  });

  test("Some returned", async () => {
    nock("http://nrl.example.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Content-Type",
      })
      .get("/authentication")
      .reply(200, {
        entries: ["google", "facebook", "twitter"],
        pagination: {
          offset: 0,
          total: 3,
        },
      });

    const providers = await sut.listProviders();
    expect(providers).toEqual(["facebook", "google", "twitter"]);
  });
});
