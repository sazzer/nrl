import "dotenv/config";

import { Service } from "./infrastructure/service";
import config from "config";

const service = new Service(config.get("database.url"));
service.start(config.get("http.port"));
