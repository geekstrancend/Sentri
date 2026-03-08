"use strict";

const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");
const { BINARY_DIR } = require("./detect-platform");

/**
 * Get the path to the Sentri binary.
 * 
 * Searches in order:
 * 1. SENTRI_BINARY_PATH environment variable
 * 2. .sentri-bin/sentri in package directory
 * 3. sentri on PATH
 * 
 * @returns {string} Path to binary
 * @throws {Error} If binary not found
 */
function getBinaryPath() {
  // Check environment variable override
  if (process.env.SENTRI_BINARY_PATH) {
    return process.env.SENTRI_BINARY_PATH;
  }

  // Check installed binary in package
  const binaryName = process.platform === "win32" ? "sentri.exe" : "sentri";
  const packageBinary = path.join(BINARY_DIR, binaryName);

  if (fs.existsSync(packageBinary)) {
    return packageBinary;
  }

  // Check PATH
  const result = spawnSync("which", ["sentri"], {
    encoding: "utf8",
    stdio: ["pipe", "pipe", "pipe"],
  });

  if (result.status === 0) {
    return result.stdout.trim();
  }

  throw new Error(
    `Error: Sentri binary not found.\n\n` +
    `The Sentri binary is not installed. Try one of:\n\n` +
    `  1. Reinstall the npm package:\n` +
    `     npm install @sentri/cli\n\n` +
    `  2. Install via Rust:\n` +
    `     cargo install sentri-cli\n\n` +
    `  3. Download manually:\n` +
    `     https://github.com/geekstrancend/Sentri/releases\n\n` +
    `If the problem persists, please file an issue at:\n` +
    `https://github.com/geekstrancend/Sentri/issues`
  );
}

/**
 * Check if the Sentri binary is installed and executable.
 * 
 * @returns {boolean}
 */
function isBinaryInstalled() {
  try {
    const binaryPath = getBinaryPath();
    return fs.existsSync(binaryPath) && isExecutable(binaryPath);
  } catch (e) {
    return false;
  }
}

/**
 * Check if a file is executable.
 * 
 * @private
 * @param {string} filepath - Path to file
 * @returns {boolean}
 */
function isExecutable(filepath) {
  try {
    // On Unix, check execute bit
    if (process.platform !== "win32") {
      const stats = fs.statSync(filepath);
      return (stats.mode & 0o111) !== 0;
    }
    // On Windows, executable files are .exe
    return filepath.endsWith(".exe");
  } catch (e) {
    return false;
  }
}

module.exports = { getBinaryPath, isBinaryInstalled };
