import { Server } from "../server";
import { createLogger } from "../../logger";

const LOGGER = createLogger("service");

export class Service {
  private server: Server;

  constructor() {
    LOGGER.info("Building service");
    this.server = new Server();
    LOGGER.info("Built service");
  }

  start(port: number): void {
    LOGGER.info({ port }, "Starting service");
    this.server.start(port);
  }
}
