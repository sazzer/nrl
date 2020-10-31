import { render, waitFor } from "@testing-library/react";

import { AuthenticationPane } from "./authentication";
import React from "react";
import { axe } from "jest-axe";

describe("<Authentication>", () => {
  test("After some authentication providers are loaded", async () => {
    const { container } = render(<AuthenticationPane />);

    await expect(container).toMatchInlineSnapshot(`
            <div>
              <div>
                <h2>
                  Login / Register
                </h2>
                <button
                  class="btn btn-block btn-social btn-twitter"
                >
                  <span
                    class="fa fa-twitter"
                  />
                   
                  Sign in with Twitter
                </button>
                <button
                  class="btn btn-block btn-social btn-google"
                >
                  <span
                    class="fa fa-google"
                  />
                   
                  Sign in with Google
                </button>
                <button
                  class="btn btn-block btn-social btn-facebook"
                >
                  <span
                    class="fa fa-facebook"
                  />
                   
                  Sign in with Facebook
                </button>
              </div>
            </div>
          `);
    expect(await axe(container)).toHaveNoViolations();
  });
});
