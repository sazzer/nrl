import React from "react";
import { useCurrentUser } from "../../api/users";

/**
 * The user menu, to display when the user is logged in
 */
export const UserMenu: React.FC = () => {
  const user = useCurrentUser();
  if (user !== undefined) {
    return (
      <li className="nav-item">
        <button
          className="nav-link btn dropdown-toggle"
          id="navbarDropdown"
          data-toggle="dropdown"
          aria-haspopup="true"
          aria-expanded="false"
        >
          {user.displayName}
        </button>
      </li>
    );
  } else {
    return null;
  }
};
