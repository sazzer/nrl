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
    env("URL_BASE") + "/authentication/{provider}"
  );

  return {
    authenticate: (provider: string) => {
      LOGGER("Authenticating with provider: %s", provider);
      const url = template.expand({ provider });
      window.open(url, "nrl:authentication");
    },
  };
}
