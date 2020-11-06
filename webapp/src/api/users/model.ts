/**
 * The shape of a single set of authentication details for a user
 */
export interface Authentication {
  /** The provider the details are for */
  provider: string;
  /** The user ID of the details at this provider */
  user: string;
  /** The display name */
  displayName: string;
}

/**
 * Base shape for a user
 */
export interface BaseUser {
  /** The ID of the user */
  userId: string;
  /** The display name of the user */
  displayName: string;
}

/**
 * Shape of a user that we're not authenticated as
 */
export type BareUser = BaseUser;

/**
 * Shape of a user that we are authenticated as
 */
export interface FullUser extends BaseUser {
  /** When the user was created */
  created: Date;
  /** When the user was last updated */
  updated: Date;
  /** The users email address */
  email: string;
  /** The users authentication details */
  authentications: Authentication[];
}

export type User = BareUser | FullUser;
