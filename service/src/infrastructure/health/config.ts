import * as endpoints from "./endpoints";

import { Configurer } from "../server";
import { Express } from "express";
import { HealthService } from "./service";
import { Healthchecker } from "./healthchecker";

/**
 * Component configuration for health checks
 */
export class HealthConfig implements Configurer {
  /** The components to check the health of */
  private components: { [key: string]: Healthchecker };

  /**
   * Construct the health service
   * @param components The components to check the health of
   */
  constructor(components: { [key: string]: Healthchecker }) {
    this.components = { ...components };
  }

  /**
   * Register the appropriate routes with Express.
   * @param app The Express app
   */
  registerRoutes(app: Express): void {
    const healthService = new HealthService(this.components);

    app.get("/health", endpoints.get(healthService));
  }
}
