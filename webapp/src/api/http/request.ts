import UrlTemplate from "url-template";
import debug from "debug";
import env from "@beam-australia/react-env";
import { getToken } from "./token";

/** The logger to use */
const LOGGER = debug("nrl:api:http:request");

/**
 * The details needed in order to make an HTTP Request
 */
export interface Request {
  /** The HTTP Method to use. Defaults to GET if not provided */
  method?: "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD";
  /** Any parameters to use for URL expansion */
  urlParams?: { [key: string]: any };
  /** Any additional headers to provide for the request */
  headers?: { [key: string]: any };
  /** Any body to submit */
  body?: any;
  /** Whether to ignore the cache when making the request */
  ignoreCache?: boolean;
}

/**
 * The details of an HTTP Response
 */
export interface Response<B> {
  /** The response status code */
  status: number;
  /** The response headers */
  headers: Headers;
  /** The parsed body of the response */
  body: B | null;
}

/**
 * Actually make an HTTP request and get the response
 * @param url The URL to call. This is a URL-Template as defined in RFC-6570.
 * @param request Any additional parameters for the request
 */
export async function request<B>(
  url: string,
  request: Request = {}
): Promise<Response<B>> {
  const template = UrlTemplate.parse(env("URL_BASE") + url);
  const finalUrl = template.expand(request.urlParams);
  LOGGER("Making request to %s: %o", finalUrl, request);

  const headers = new Headers();
  if (request.headers) {
    Object.entries(request.headers).forEach(([key, value]) =>
      headers.set(key, value)
    );
  }

  if (request.ignoreCache) {
    headers.set("cache-control", "no-cache");
  }

  if (request.body) {
    headers.set("content-type", "application/json");
  }

  const token = getToken();
  if (token) {
    headers.set("authorization", `Bearer ${token}`);
  }

  try {
    const response = await fetch(finalUrl, {
      method: request.method || "GET",
      body: request.body && JSON.stringify(request.body),
      headers,
    });
    LOGGER("Received response from %s: %o", finalUrl, response);

    const contentType = response.headers.get("content-type");

    if (contentType) {
      const body = await response.json();
      LOGGER("Response had payload: %o", body);

      if (contentType === "application/problem+json") {
        LOGGER("Response was a Problem");
        throw new Error(); // TODO: Problem response
      } else {
        LOGGER("Response was not a Problem");
        return {
          status: response.status,
          headers: response.headers,
          body,
        };
      }
    } else {
      LOGGER("Response had no payload");
      return {
        status: response.status,
        headers: response.headers,
        body: null,
      };
    }
  } catch (e) {
    LOGGER("Unexpected error making HTTP request: %o", e);
    throw e;
  }
}
