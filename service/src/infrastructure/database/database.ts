import { Healthchecker } from "../health";
import { Pool } from "pg";
import { createLogger } from "../../logger";

/** The logger to use */
const LOGGER = createLogger("infrastructure:database");

/**
 * A wrapper around the database connection to use.
 */
export class Database implements Healthchecker {
  /** The connection pool */
  private pool: Pool;

  /**
   * Construct the database connection
   * @param url The URL to connect to the database with
   */
  constructor(url: string) {
    LOGGER.debug({ url }, "Connecting to database");

    this.pool = new Pool({
      connectionString: url,
    });
  }

  /**
   * Check that the database connection is healthy.
   */
  async checkHealth(): Promise<void> {
    await this.pool.query("SELECT 1");
  }
}
