import express, { Express } from "express";

import bodyParser from "body-parser";
import compression from "compression";
import cors from "cors";
import { createLogger } from "../../logger";
import helmet from "helmet";
import pinoHttp from "pino-http";
import requestId from "express-request-id";
import responseTime from "response-time";
import rtracer from "cls-rtracer";
import timeout from "connect-timeout";

const LOGGER = createLogger("server");

export class Server {
  private app: Express;

  constructor() {
    const app = express();
    app.use(responseTime());
    app.use(timeout("5s"));
    app.use(requestId());
    app.use(pinoHttp());
    app.use(
      rtracer.expressMiddleware({
        useHeader: true,
      }),
    );
    app.use(bodyParser.json());
    app.use(compression());
    app.use(cors());
    app.use(helmet());

    app.get("/", (req, res) => {
      res.json("Hello World!");
    });

    this.app = app;
  }

  start(port: number): void {
    LOGGER.info({ port }, "Starting service");
    this.app.listen(port);
  }
}
