import { Database } from "../database/database";
import { Express } from "express";
import { HealthConfig } from "../health/config";
import { Server } from "../server";
import { createLogger } from "../../logger";

/** The logger to use */
const LOGGER = createLogger("infrastructure:service");

/**
 * Class representing the entire service
 */
export class Service {
  /** The HTTP Server */
  private server: Server;

  /**
   * Construct the service
   * @param databaseUrl The URL to use to connect to the database
   */
  constructor(databaseUrl: string) {
    LOGGER.info("Building service");

    const db = new Database(databaseUrl);

    const health = new HealthConfig({ db });

    this.server = new Server([health]);

    LOGGER.info("Built service");
  }

  /**
   * Start the service running
   * @param port The HTTP Port to listen on
   */
  start(port: number): void {
    LOGGER.info({ port }, "Starting service");
    this.server.start(port);
  }

  /**
   * Get the Express app, for the purposes of integration testing.
   */
  _app(): Express {
    return this.server._app();
  }
}
