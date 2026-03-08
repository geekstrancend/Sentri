const path = require("path");
const fs = require("fs");
const { getBinaryPath, isBinaryInstalled } = require("../../lib/binary-path");

describe("Binary Path Resolution", () => {
  const originalEnv = process.env;

  beforeEach(() => {
    process.env = { ...originalEnv };
    delete process.env.SENTRI_BINARY_PATH;
  });

  afterEach(() => {
    process.env = originalEnv;
  });

  test("SENTRI_BINARY_PATH environment variable overrides default", () => {
    process.env.SENTRI_BINARY_PATH = "/custom/path/to/sentri";

    const binaryPath = getBinaryPath();

    expect(binaryPath).toBe("/custom/path/to/sentri");
  });

  test("isBinaryInstalled returns false when binary missing", () => {
    process.env.SENTRI_BINARY_PATH = "/nonexistent/path/to/sentri";

    const installed = isBinaryInstalled();

    expect(installed).toBe(false);
  });

  test("isBinaryInstalled returns true when binary exists and is executable", () => {
    // Skip this test on systems without a sentri in PATH
    const which = require("child_process").spawnSync("which", ["node"]);
    if (which.status === 0) {
      process.env.SENTRI_BINARY_PATH = which.stdout.toString().trim();
      const installed = isBinaryInstalled();
      expect(installed).toBe(true);
    }
  });

  test("getBinaryPath throws helpful error when not installed", () => {
    process.env.SENTRI_BINARY_PATH = "/nonexistent/sentri";

    expect(() => {
      getBinaryPath();
    }).toThrow(/Sentri binary not found/);

    expect(() => {
      getBinaryPath();
    }).toThrow(/npm install @sentri\/cli/);
  });
});
