import pino, { Logger } from "pino";

/**
 * Create a new logger to use
 * @param name The name of the logger
 */
export function createLogger(name: string): Logger {
  return pino({
    name,
  });
}
