import * as sut from "./token";

describe("Authentication Token", () => {
  beforeEach(() => {
    sut.clearToken();
  });

  test("Get no token", () => {
    expect(sut.getToken()).toBeUndefined();
  });

  test("Store token", () => {
    const date = new Date();
    date.setDate(date.getDate() + 1);

    sut.setToken("abc", date);

    expect(sut.getToken()).toEqual("abc");
  });

  test("Clear token", () => {
    const date = new Date();
    date.setDate(date.getDate() + 1);

    sut.setToken("abc", date);
    sut.clearToken();

    expect(sut.getToken()).toBeUndefined();
  });

  test("Expire token", () => {
    const date = new Date();
    date.setDate(date.getDate() - 1);

    sut.setToken("abc", date);

    expect(sut.getToken()).toBeUndefined();
  });
});
