import { ComponentHealth, SystemHealth, SystemStatus } from "./model";

import { CheckHealthUseCase } from "./usecases";
import { Healthchecker } from "./healthchecker";

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

    Object.entries(this.components).forEach(async ([key, component]) => {
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
    });

    return {
      status,
      components,
    };
  }
}
