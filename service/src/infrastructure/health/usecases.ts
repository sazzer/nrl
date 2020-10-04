import { SystemHealth } from "./model";

/**
 * Use case for checking the health of the system.
 */
export interface CheckHealthUseCase {
  /**
   * Check the system health.
   * @return the system health.
   */
  checkHealth(): Promise<SystemHealth>;
}
