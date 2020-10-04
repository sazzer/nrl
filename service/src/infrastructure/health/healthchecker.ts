/**
 * Interface that components can implement if they can report on their health status.
 */
export interface Healthchecker {
  /**
   * Check the health of the component.
   * If the component is healthy then return a promise for nothing.
   * If the component is unhealthy then return a rejected promise for the error.
   */
  checkHealth(): Promise<void>;
}
