import signale from "signale";

export class Service {
  constructor() {
    signale.info("Building service");
    signale.info("Built service");
  }

  async start(port: number): Promise<void> {
    signale.info("Starting service", { port });
  }
}
