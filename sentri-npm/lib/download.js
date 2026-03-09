"use strict";

const fs = require("fs");
const path = require("path");
const https = require("https");
const { exec } = require("child_process");
const { detectPlatform, BINARY_DIR } = require("./detect-platform");
const HttpsProxyAgent = require("https-proxy-agent");

/**
 * Download the Sentri binary for the current platform.
 * 
 * @param {object} platformInfo - Platform info from detectPlatform()
 * @param {object} [options] - Download options
 * @param {boolean} [options.verbose] - Log progress
 * @param {boolean} [options.force] - Force re-download even if exists
 * @returns {Promise<string>} Path to downloaded binary
 */
async function downloadBinary(platformInfo, options = {}) {
  const { verbose = false, force = false } = options;

  const binaryDir = BINARY_DIR;
  const binaryPath = path.join(binaryDir, platformInfo.binaryName);

  // Check if already installed
  if (!force && fs.existsSync(binaryPath)) {
    if (verbose) {
      console.log(`✓ Sentri binary already installed at ${binaryPath}`);
    }
    return binaryPath;
  }

  // Ensure directory exists
  await fs.promises.mkdir(binaryDir, { recursive: true });

  const downloadUrl = platformInfo.downloadUrl;
  const tempDir = path.join(binaryDir, ".tmp");
  await fs.promises.mkdir(tempDir, { recursive: true });

  const archivePath = path.join(tempDir, platformInfo.archiveName);

  try {
    // Download binary
    if (verbose) {
      console.log(`Downloading Sentri v${platformInfo.version} for ${process.platform}-${process.arch}...`);
    }

    await downloadFile(downloadUrl, archivePath, verbose);

    // Extract binary
    if (verbose) {
      console.log(`Extracting ${platformInfo.archiveName}...`);
    }

    await extractArchive(archivePath, binaryDir, platformInfo);

    // Make executable on Unix
    if (process.platform !== "win32") {
      await fs.promises.chmod(binaryPath, 0o755);
    }

    // Cleanup temp directory
    await fs.promises.rm(tempDir, { recursive: true, force: true });

    if (verbose) {
      console.log(`✓ Sentri binary extracted to ${binaryPath}`);
    }

    return binaryPath;
  } catch (error) {
    // Cleanup on failure
    await fs.promises.rm(tempDir, { recursive: true, force: true });
    throw error;
  }
}

/**
 * Download a file from URL to local path with progress.
 * 
 * @private
 * @param {string} url - File URL
 * @param {string} destinationPath - Where to save
 * @param {boolean} verbose - Show progress
 * @returns {Promise<void>}
 */
async function downloadFile(url, destinationPath, verbose) {
  return new Promise((resolve, reject) => {
    const request = (url) => {
      const client = url.startsWith("https") ? https : require("http");
      const agent = process.env.HTTPS_PROXY || process.env.HTTP_PROXY
        ? new HttpsProxyAgent(process.env.HTTPS_PROXY || process.env.HTTP_PROXY)
        : undefined;

      const options = {
        agent,
        timeout: 30_000, // 30 second socket timeout
      };

      // Hard timeout for entire download (60 seconds)
      let timedOut = false;
      const downloadTimeout = setTimeout(() => {
        timedOut = true;
        req.destroy();
        reject(new Error(`Download timeout: no progress for 60 seconds (${url})`));
      }, 60_000);

      const req = client.get(url, options, (response) => {
        // Reset timeout on successful response
        clearTimeout(downloadTimeout);

        // Handle redirects
        if (response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
          response.resume(); // Consume response data to free up memory
          request(response.headers.location);
          return;
        }

        if (response.statusCode !== 200) {
          reject(new Error(
            `Failed to download from ${url}\n` +
            `HTTP ${response.statusCode}: ${response.statusMessage}\n` +
            `Check the GitHub releases page: https://github.com/geekstrancend/Sentri/releases`
          ));
          response.resume(); // drain the response
          return;
        }

        const contentLength = parseInt(response.headers["content-length"], 10);
        let downloadedBytes = 0;
        const startTime = Date.now();

        const file = fs.createWriteStream(destinationPath);

        response.on("data", (chunk) => {
          downloadedBytes += chunk.length;
          if (verbose && process.stdout.isTTY && contentLength) {
            const percent = Math.round((downloadedBytes / contentLength) * 100);
            process.stdout.write(`\rDownloading... ${percent}% (${formatBytes(downloadedBytes)}/${formatBytes(contentLength)})`);
          }
        });

        response.pipe(file);

        file.on("finish", () => {
          clearTimeout(downloadTimeout);
          file.close(() => {
            if (verbose) {
              const elapsed = ((Date.now() - startTime) / 1000).toFixed(1);
              console.log(`\nDownload complete (${elapsed}s)`);
            }
            resolve();
          });
        });

        file.on("error", (error) => {
          clearTimeout(downloadTimeout);
          fs.unlink(destinationPath, () => {
            reject(error);
          });
        });
      });

      req.on("error", (error) => {
        clearTimeout(downloadTimeout);
        if (!timedOut) {
          reject(error);
        }
      });

      req.on("timeout", () => {
        clearTimeout(downloadTimeout);
        req.destroy();
        reject(new Error(`Connection timeout: ${url}`));
      });
    };

    request(url);
  });
}

/**
 * Extract archive to binary directory.
 * 
 * @private
 * @param {string} archivePath - Path to archive file
 * @param {string} destDir - Destination directory
 * @param {object} platformInfo - Platform info
 * @returns {Promise<void>}
 */
async function extractArchive(archivePath, destDir, platformInfo) {
  return new Promise((resolve, reject) => {
    let command;

    if (platformInfo.archiveFormat === "tar.gz") {
      // Extract tar.gz
      command = `tar xzf "${archivePath}" -C "${destDir}"`;
    } else if (platformInfo.archiveFormat === "zip") {
      // Extract zip on Windows using PowerShell
      command = `powershell -Command "Expand-Archive -Path '${archivePath}' -DestinationPath '${destDir}' -Force"`;
    } else {
      reject(new Error(`Unknown archive format: ${platformInfo.archiveFormat}`));
      return;
    }

    exec(command, (error, stdout, stderr) => {
      if (error) {
        reject(new Error(
          `Failed to extract archive: ${error.message}\n` +
          `Command: ${command}\n` +
          `stderr: ${stderr}`
        ));
        return;
      }
      resolve();
    });
  });
}

/**
 * Format bytes as human-readable string.
 * 
 * @private
 */
function formatBytes(bytes) {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return (bytes / Math.pow(k, i)).toFixed(1) + " " + sizes[i];
}

module.exports = { downloadBinary };
