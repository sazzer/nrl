import "dotenv/config";

import { Service } from "./infrastructure/service";
import config from "config";

const service = new Service();
service.start(config.get("http.port"));
