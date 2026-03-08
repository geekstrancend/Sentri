const { analyze, version, isInstalled } = require("../../index.js");

describe("Programmatic API", () => {
  test("analyze() requires path option", async () => {
    expect.assertions(1);

    try {
      await analyze({ chain: "evm" });
    } catch (error) {
      expect(error.message).toContain("requires options.path");
    }
  });

  test("analyze() requires chain option", async () => {
    expect.assertions(1);

    try {
      await analyze({ path: "./contracts" });
    } catch (error) {
      expect(error.message).toContain("requires options.chain");
    }
  });

  test("version() returns promise", async () => {
    const versionPromise = version();
    expect(versionPromise).toBeInstanceOf(Promise);
  });

  test("isInstalled() returns promise of boolean", async () => {
    const installedPromise = isInstalled();
    expect(installedPromise).toBeInstanceOf(Promise);

    const installed = await installedPromise;
    expect(typeof installed).toBe("boolean");
  });
});
