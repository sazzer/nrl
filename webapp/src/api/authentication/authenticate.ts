import UrlTemplate from "url-template";
import debug from "debug";
import env from "@beam-australia/react-env";
import { setToken } from "../http/token";
import { useCurrentUserId } from "../users";

/** The logger to use */
const LOGGER = debug("nrl:api:authentication:authenticate");

export interface AuthenticateHook {
  authenticate: (provider: string) => void;
}

/**
 * Hook to support authenticationl
 * @return the hook
 */
export function useAuthenticate(): AuthenticateHook {
  const { setUserId } = useCurrentUserId();

  const template = UrlTemplate.parse(
    env("URL_BASE") + "/authentication/{provider}{?redirect_url}"
  );
  const redirect_url = `${window.location.origin}/authenticated.html`;

  return {
    authenticate: (provider: string) => {
      LOGGER(
        "Authenticating with provider %s redirecting to %s",
        provider,
        redirect_url
      );

      const eventListener = (event: MessageEvent) => {
        if (event && event.data && event.data.type === "nrlAuthenticated") {
          window.removeEventListener("message", eventListener);
          LOGGER("Authenticated with provider %s: %o", provider, event.data);

          setToken(event.data.token, event.data.expires);
          setUserId(event.data.userId);
        }
      };
      window.addEventListener("message", eventListener);

      const url = template.expand({ provider, redirect_url });
      window.open(url, "nrl:authentication");
    },
  };
}
