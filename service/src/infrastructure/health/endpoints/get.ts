import { CheckHealthUseCase } from "..";
import { RequestHandler } from "express";

/**
 * Get the health status
 * @param service The service with which to get the health status
 */
export function get(service: CheckHealthUseCase): RequestHandler {
  return async (req, res) => {
    const health = await service.checkHealth();

    res.status(health.status === "HEALTHY" ? 200 : 503);
    res.json(health);
  };
}
