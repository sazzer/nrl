import { User } from "./model";
import debug from "debug";
import { request } from "../http";
import { useCurrentUserId } from "./currentUser";
import { useQuery } from "react-query";

/** The logger to use */
const LOGGER = debug("nrl:api:users:load");

/**
 * Load the given user from the server
 */
async function loadUser(userId: string): Promise<User> {
  LOGGER("Loading user: %s", userId);

  const response = await request<User>("/users/{userId}", {
    urlParams: {
      userId,
    },
  });

  return response.body!;
}

/**
 * React hook to work with a user record
 * @param userId The ID of the user to use
 */
export function useUser(userId: string | null): User | undefined {
  const { data } = useQuery<User | undefined>(["providers", userId], () => {
    if (userId !== null) {
      return loadUser(userId);
    } else {
      return undefined;
    }
  });

  return data;
}

/**
 * React hook to work with a current user record
 */
export function useCurrentUser(): User | undefined {
  const currentUser = useCurrentUserId();

  return useUser(currentUser.userId);
}
