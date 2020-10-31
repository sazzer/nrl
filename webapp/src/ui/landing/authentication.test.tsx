import { render, waitFor } from "@testing-library/react";

import { AuthenticationPane } from "./authentication";
import React from "react";
import { axe } from "jest-axe";
import { listProviders } from "../../api/authentication";
import { resourceCache } from "use-async-resource";

jest.mock("../../api/authentication");

describe("<Authentication>", () => {
  const listProvidersMock = listProviders as jest.Mock;

  beforeEach(() => {
    resourceCache(listProviders).clear();
    listProvidersMock.mockClear();
  });

  test("After some authentication providers are loaded", async () => {
    listProvidersMock.mockResolvedValueOnce(["facebook", "twitter"]);

    const { container, findByText } = render(<AuthenticationPane />);
    await waitFor(() => expect(listProvidersMock).toHaveBeenCalledTimes(1), {
      container,
    });
    await findByText("Sign in with Facebook");

    await expect(container).toMatchInlineSnapshot(`
            <div>
              <div>
                <h2>
                  Login / Register
                </h2>
                <button
                  class="btn btn-block btn-social btn-facebook"
                >
                  <span
                    class="fa fa-facebook"
                  />
                   
                  Sign in with Facebook
                </button>
                <button
                  class="btn btn-block btn-social btn-twitter"
                >
                  <span
                    class="fa fa-twitter"
                  />
                   
                  Sign in with Twitter
                </button>
              </div>
            </div>
          `);
    expect(await axe(container)).toHaveNoViolations();
  });
});
