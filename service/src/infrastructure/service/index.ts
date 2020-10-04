import { Server } from "../server";
import signale from "signale";

const LOGGER = signale.scope("service");

export class Service {
  private server: Server;

  constructor() {
    LOGGER.info("Building service");
    this.server = new Server();
    LOGGER.info("Built service");
  }

  start(port: number): void {
    LOGGER.info("Starting service", { port });
    this.server.start(port);
  }
}
