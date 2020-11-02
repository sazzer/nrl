import * as sut from "./list";

import { QueryCache, ReactQueryCacheProvider } from "react-query";
import { act, renderHook } from "@testing-library/react-hooks";

import React from "react";
import nock from "nock";

describe("List Authentication Providers", () => {
  const queryCache = new QueryCache();
  const wrapper: React.FC = ({ children }) => (
    <ReactQueryCacheProvider queryCache={queryCache}>
      {children}
    </ReactQueryCacheProvider>
  );

  beforeEach(() => {
    queryCache.clear();
  });

  test("None returned", async () => {
    const expectation = nock("http://nrl.example.com")
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

    const { result, waitFor } = renderHook(() => sut.useProviders(), {
      wrapper,
    });
    await waitFor(() => {
      return expectation.isDone();
    });
    expect(result.current).toEqual([]);
  });

  test("Some returned", async () => {
    const expectation = nock("http://nrl.example.com")
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

    const { result, waitFor } = renderHook(() => sut.useProviders(), {
      wrapper,
    });
    await waitFor(() => {
      return expectation.isDone();
    });
    expect(result.current).toEqual(["facebook", "google", "twitter"]);
  });
});
