import { GenericContainer, Wait } from "testcontainers";

/**
 * The test database, which is just a wrapper around a running Postgres.
 */
export interface TestDatabase {
  url: string;
  stop(): Promise<void>;
}

/**
 * Construct a new test database.
 */
export async function newTestDatabase(): Promise<TestDatabase> {
  const container = await new GenericContainer("postgres", "12.4-alpine")
    .withExposedPorts(5432)
    .withEnv("POSTGRES_DB", "nrl-test")
    .withEnv("POSTGRES_USER", "nrl-test")
    .withEnv("POSTGRES_PASSWORD", "nrl-test")
    .withWaitStrategy(Wait.forLogMessage("database system is ready to accept connections"))
    .start();

  const address = container.getContainerIpAddress();
  const port = container.getMappedPort(5432);

  return {
    url: `postgres://nrl-test:nrl-test@${address}:${port}/nrl-test`,
    async stop() {
      await container.stop();
    },
  };
}
