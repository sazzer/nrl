import request, { Request } from "supertest";

import { Service } from "../infrastructure/service";

/**
 * Wrapper around the service for testing purposes.
 */
export class TestService {
  /** The actual service to test */
  private service: Service;

  /**
   * Construct the test service.
   */
  constructor() {
    this.service = new Service();
  }

  /**
   * Make a GET request to the server
   * @param url The URL to make the request to
   */
  async get(url: string): Promise<Request> {
    return request(this.service._app()).get(url);
  }
}
