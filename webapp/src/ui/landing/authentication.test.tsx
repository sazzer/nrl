import { fireEvent, render, waitFor } from "@testing-library/react";
import { useAuthenticate, useProviders } from "../../api/authentication";

import { AuthenticationPane } from "./authentication";
import React from "react";
import { axe } from "jest-axe";

jest.mock("../../api/authentication");

describe("<Authentication>", () => {
  const useProvidersMock = useProviders as jest.Mock;
  const useAuthenticateMock = useAuthenticate as jest.Mock;
  const authenticateMock = jest.fn();

  beforeEach(() => {
    useProvidersMock.mockClear();
    useAuthenticateMock.mockClear();
    useAuthenticateMock.mockReturnValue({ authenticate: authenticateMock });
  });

  describe("Rendering", () => {
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

      expect(authenticateMock).not.toHaveBeenCalled();
    });
  });

  test("Clicking an authentication button", async () => {
    useProvidersMock.mockReturnValue(["facebook", "twitter"]);

    const { container, findByText } = render(<AuthenticationPane />);
    await waitFor(() => expect(useProvidersMock).toHaveBeenCalledTimes(1), {
      container,
    });
    const button = await findByText("Sign in with Facebook");
    fireEvent.click(button);

    expect(authenticateMock).toHaveBeenCalledWith("facebook");
  });
});
