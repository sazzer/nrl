import { HealthConfig } from "../health/config";
import { Server } from "../server";
import { createLogger } from "../../logger";

/** The logger to use */
const LOGGER = createLogger("service");

/**
 * Class representing the entire service
 */
export class Service {
  /** The HTTP Server */
  private server: Server;

  /**
   * Construct the service
   */
  constructor() {
    LOGGER.info("Building service");

    const health = new HealthConfig();

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
}
