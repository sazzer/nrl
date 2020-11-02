import { render, waitFor } from "@testing-library/react";

import { AuthenticationPane } from "./authentication";
import React from "react";
import { axe } from "jest-axe";
import { useProviders } from "../../api/authentication";

jest.mock("../../api/authentication");

describe("<Authentication>", () => {
  const useProvidersMock = useProviders as jest.Mock;

  beforeEach(() => {
    useProvidersMock.mockClear();
  });

  test("After some authentication providers are loaded", async () => {
    useProvidersMock.mockReturnValue(["facebook", "twitter"]);

    const { container, findByText } = render(<AuthenticationPane />);
    await waitFor(() => expect(useProvidersMock).toHaveBeenCalledTimes(1), {
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
