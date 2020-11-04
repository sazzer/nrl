import UrlTemplate from "url-template";
import debug from "debug";
import env from "@beam-australia/react-env";

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
      const url = template.expand({ provider, redirect_url });
      window.open(url, "nrl:authentication");
    },
  };
}
