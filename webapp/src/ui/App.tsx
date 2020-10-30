import { HeaderBar } from "./header";
import { LandingPage } from "./landing";
import React from "react";

export const App: React.FC = () => {
  return (
    <>
      <HeaderBar />
      <LandingPage />
    </>
  );
};
