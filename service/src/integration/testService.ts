import request, { Test } from "supertest";

import { Service } from "../infrastructure/service";
import { newTestDatabase } from "./testDatabase";

export interface TestService {
  get(url: string): Test;
  destroy(): Promise<void>;
}

export async function newTestService(): Promise<TestService> {
  const database = await newTestDatabase();
  const service = new Service(database.url);

  return {
    destroy() {
      return database.stop();
    },
    get(url: string) {
      return request(service._app()).get(url);
    },
  };
}
