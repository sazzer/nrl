import debug from "debug";
import { request } from "../http";
import { useQuery } from "react-query";

/** The logger to use */
const LOGGER = debug("nrl:api:authentication:list");

/** Shape of the HTTP Response for listing authentication providers */
interface ListProvidersResponse {
  entries: string[];
}

/**
 * Get a list of the authentication providers from the server
 */
async function listProviders(): Promise<string[]> {
  const response = await request<ListProvidersResponse>("/authentication");
  const providers: string[] = response.body?.entries ?? [];
  LOGGER("Authentication providers: %o", providers);

  return providers.sort();
}

export function useProviders(): string[] {
  const { data } = useQuery<string[]>("providers", listProviders);
  return data ?? [];
}
