import { ComponentHealth, SystemHealth, SystemStatus } from "../model";

import { CheckHealthUseCase } from "../usecases";
import { Healthchecker } from "../healthchecker";
import { createLogger } from "../../../logger";

/** The logger to use */
const LOGGER = createLogger("infrastructure:health:service");

/**
 * The actual health service
 */
export class HealthService implements CheckHealthUseCase {
  /** The components to check the health of */
  private components: { [key: string]: Healthchecker };

  /**
   * Construct the health service
   * @param components The components to check the health of
   */
  constructor(components: { [key: string]: Healthchecker }) {
    this.components = { ...components };
  }

  /**
   * Check the system health.
   * @return the system health.
   */
  async checkHealth(): Promise<SystemHealth> {
    let status: SystemStatus = "HEALTHY";
    const components: { [key: string]: ComponentHealth } = {};

    LOGGER.debug("Checking system health");

    for (const entry of Object.entries(this.components)) {
      const key = entry[0];
      const component = entry[1];

      try {
        await component.checkHealth();
        components[key] = {
          status: "HEALTHY",
        };
      } catch (e) {
        components[key] = {
          status: "UNHEALTHY",
          message: e.toString(),
        };
        status = "UNHEALTHY";
      }

      LOGGER.debug({ component: key, status: components[key] }, "Component health");
    }

    return {
      status,
      components,
    };
  }
}
