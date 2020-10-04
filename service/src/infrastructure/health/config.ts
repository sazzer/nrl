import { Configurer } from "../server";
import { Express } from "express";

export class HealthConfig implements Configurer {
  registerRoutes(app: Express): void {
    app.get("/health", (req, res) => res.json("Hello"));
  }
}
