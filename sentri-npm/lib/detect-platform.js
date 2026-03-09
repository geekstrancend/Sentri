"use strict";

const os = require("os");
const path = require("path");

const { version: SENTRI_VERSION } = require("../package.json");
const GITHUB_REPO = "geekstrancend/Sentri";
const BINARY_DIR = path.join(__dirname, "..", ".sentri-bin");

/**
 * Detect the current platform and return information needed for binary download.
 * 
 * @returns {object} Platform information including platform, arch, binaryName, etc.
 * @throws {Error} If platform is not supported
 */
function detectPlatform() {
  const platform = os.platform();
  const arch = os.arch();
  
  const mapping = {
    "linux-x64": {
      platform: "linux",
      arch: "x86_64",
      binaryName: "sentri",
      target: "x86_64-unknown-linux-gnu",
      archiveFormat: "tar.gz",
    },
    "linux-arm64": {
      platform: "linux",
      arch: "aarch64",
      binaryName: "sentri",
      target: "aarch64-unknown-linux-gnu",
      archiveFormat: "tar.gz",
    },
    "darwin-x64": {
      platform: "macos",
      arch: "x86_64",
      binaryName: "sentri",
      target: "x86_64-apple-darwin",
      archiveFormat: "tar.gz",
    },
    "darwin-arm64": {
      platform: "macos",
      arch: "aarch64",
      binaryName: "sentri",
      target: "aarch64-apple-darwin",
      archiveFormat: "tar.gz",
    },
    "win32-x64": {
      platform: "windows",
      arch: "x86_64",
      binaryName: "sentri.exe",
      target: "x86_64-pc-windows-msvc",
      archiveFormat: "zip",
    },
  };

  // Map os.arch() to our format
  let mappingKey;
  if (platform === "linux") {
    mappingKey = arch === "x64" ? "linux-x64" : arch === "arm64" ? "linux-arm64" : null;
  } else if (platform === "darwin") {
    mappingKey = arch === "x64" ? "darwin-x64" : arch === "arm64" ? "darwin-arm64" : null;
  } else if (platform === "win32") {
    mappingKey = arch === "x64" ? "win32-x64" : null;
  }

  if (!mappingKey || !mapping[mappingKey]) {
    throw new Error(
      `Error: Sentri does not support your platform: ${platform}-${arch}\n` +
      `Supported platforms: linux-x86_64, linux-aarch64, macos-x86_64, macos-aarch64, windows-x86_64\n` +
      `Please open an issue at https://github.com/geekstrancend/Sentri/issues`
    );
  }

  const info = mapping[mappingKey];
  const archiveName = `sentri-${SENTRI_VERSION}-${info.target}.${info.archiveFormat}`;
  const downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${SENTRI_VERSION}/${archiveName}`;

  return {
    platform: info.platform,
    arch: info.arch,
    binaryName: info.binaryName,
    target: info.target,
    archiveName,
    archiveFormat: info.archiveFormat,
    downloadUrl,
  };
}

module.exports = {
  detectPlatform,
  SENTRI_VERSION,
  GITHUB_REPO,
  BINARY_DIR,
};
