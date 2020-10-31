import { HeaderBar } from "./header";
import { LandingPage } from "./landing";
import React from "react";

export const App: React.FC = () => {
  return (
    <div>
      <HeaderBar />
      <div className="container-fluid">
        <LandingPage />
      </div>
    </div>
  );
};
