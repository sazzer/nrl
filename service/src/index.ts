import "dotenv/config";

import { Service } from "./infrastructure/service";
import config from "config";

async function start() {
  const service = new Service();
  await service.start(config.get("http.port"));
}

start().then(() => {
  // Running
});
