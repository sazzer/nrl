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
        <a
          className="nav-link dropdown-toggle"
          href="#"
          id="navbarDropdown"
          role="button"
          data-toggle="dropdown"
          aria-haspopup="true"
          aria-expanded="false"
        >
          {user.displayName}
        </a>
      </li>
    );
  } else {
    return null;
  }
};
