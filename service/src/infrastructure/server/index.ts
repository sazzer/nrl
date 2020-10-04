import express, { Express } from "express";

import bodyParser from "body-parser";
import compression from "compression";
import cors from "cors";
import helmet from "helmet";
import requestId from "express-request-id";
import responseTime from "response-time";
import rtracer from "cls-rtracer";
import signale from "signale";
import timeout from "connect-timeout";

const LOGGER = signale.scope("server");

export class Server {
  private app: Express;

  constructor() {
    const app = express();
    app.use(responseTime());
    app.use(timeout("5s"));
    app.use(requestId());
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
    LOGGER.info("Starting server", { port });
    this.app.listen(port);
  }
}
