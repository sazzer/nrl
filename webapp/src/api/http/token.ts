import debug from "debug";

/** The logger to use */
const LOGGER = debug("nrl:api:http:token");

/** The shape of a stored authentication token */
interface Token {
  token: string;
  expires: Date;
}

/** The actual stored token */
let storedToken: Token | undefined = undefined;

/**
 * Store an API token to make authenticated API calls.
 * @param token The actual token
 * @param expires The expire date of the token
 */
export function setToken(token: string, expires: Date) {
  LOGGER("Storing API Token %s until %o", token, expires);
  storedToken = {
    token,
    expires,
  };
}

/**
 * Clearing the API token
 */
export function clearToken() {
  LOGGER("Clearing API Token");
  storedToken = undefined;
}

/**
 * Get the current token, or undefined if it has expired or not been stored
 */
export function getToken(): string | undefined {
  const now = new Date();
  if (storedToken === undefined || storedToken.expires < now) {
    clearToken();
  }

  return storedToken?.token;
}
