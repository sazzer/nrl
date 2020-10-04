/** The overall status of the system */
export type SystemStatus = "HEALTHY" | "UNHEALTHY";

/**
 * The health status of a component
 */
export type ComponentHealth =
  | {
      status: "HEALTHY";
    }
  | {
      status: "UNHEALTHY";
      message: string;
    };

/**
 * The health status of the whole system
 */
export interface SystemHealth {
  status: SystemStatus;
  components: { [key: string]: ComponentHealth };
}
