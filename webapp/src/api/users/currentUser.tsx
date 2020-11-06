import React, { useContext, useEffect, useState } from "react";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("nrl:currentUser");

/**
 * Shape of the context store when a user
 */
interface UserContext {
  /** The actual User ID */
  userId: string | null;
  /** Callback to store the ID of the User */
  setUserId: (userId: string) => void;
  /** Callback to clear the User */
  clearUserId: () => void;
}

/** The actual context type */
const userContext = React.createContext<UserContext>({
  userId: null,
  setUserId: () => {},
  clearUserId: () => {},
});

/**
 * React component to act as a context provider for the current user.
 */
export const UserProvider: React.FC = ({ children }) => {
  const [userId, setUserId] = useState<string | null>(null);

  let contextValue: UserContext = {
    userId,
    setUserId: (userId: string) => {
      LOGGER("Setting current User ID: %s", userId);
      setUserId(userId);
    },
    clearUserId: () => {
      LOGGER("Clearing current User ID");
      setUserId(null);
    },
  };

  return (
    <userContext.Provider value={contextValue}>{children}</userContext.Provider>
  );
};

/**
 * Shape of the User Hook when a User ID is present
 */
export interface UserHookWithUser {
  /** Whether a user is present */
  hasUser: true;
  /** The User ID */
  userId: string;
  /** Callback to store the ID of the User */
  setUserId: (userId: string) => void;
  /** Callback to clear the User */
  clearUserId: () => void;
}

/**
 * Shape of the User Hook when a User ID is not present
 */
export interface UserHookWithoutUser {
  /** Whether a user is present */
  hasUser: false;
  /** The User ID */
  userId: null;
  /** Callback to store the ID of the User */
  setUserId: (userId: string) => void;
}

/**
 * Shape of the User Hook
 */
export type UserHook = UserHookWithUser | UserHookWithoutUser;

/**
 * Hook to access the user details
 */
export function useUser(): UserHook {
  const context = useContext(userContext);

  if (context.userId) {
    return {
      hasUser: true,
      userId: context.userId,
      setUserId: context.setUserId,
      clearUserId: context.clearUserId,
    };
  } else {
    return {
      hasUser: false,
      userId: null,
      setUserId: context.setUserId,
    };
  }
}
