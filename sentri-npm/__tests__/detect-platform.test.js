describe("Platform Detection", () => {
  const { detectPlatform, SENTRI_VERSION, GITHUB_REPO, BINARY_DIR } = require("../../lib/detect-platform");
  const os = require("os");

  const originalPlatform = os.platform;
  const originalArch = os.arch;

  afterEach(() => {
    // Restore original functions
    os.platform = originalPlatform;
    os.arch = originalArch;
  });

  test("linux-x86_64 detected correctly", () => {
    os.platform = jest.fn(() => "linux");
    os.arch = jest.fn(() => "x64");

    const info = detectPlatform();

    expect(info.platform).toBe("linux");
    expect(info.arch).toBe("x86_64");
    expect(info.binaryName).toBe("sentri");
    expect(info.target).toBe("x86_64-unknown-linux-gnu");
    expect(info.archiveFormat).toBe("tar.gz");
    expect(info.archiveName).toContain("x86_64-unknown-linux-gnu");
  });

  test("macos-aarch64 detected correctly", () => {
    os.platform = jest.fn(() => "darwin");
    os.arch = jest.fn(() => "arm64");

    const info = detectPlatform();

    expect(info.platform).toBe("macos");
    expect(info.arch).toBe("aarch64");
    expect(info.binaryName).toBe("sentri");
    expect(info.target).toBe("aarch64-apple-darwin");
    expect(info.archiveFormat).toBe("tar.gz");
  });

  test("windows-x86_64 detected correctly", () => {
    os.platform = jest.fn(() => "win32");
    os.arch = jest.fn(() => "x64");

    const info = detectPlatform();

    expect(info.platform).toBe("windows");
    expect(info.arch).toBe("x86_64");
    expect(info.binaryName).toBe("sentri.exe");
    expect(info.target).toBe("x86_64-pc-windows-msvc");
    expect(info.archiveFormat).toBe("zip");
  });

  test("unsupported platform throws helpful error", () => {
    os.platform = jest.fn(() => "sunos");
    os.arch = jest.fn(() => "x64");

    expect(() => {
      detectPlatform();
    }).toThrow(/does not support your platform/);

    expect(() => {
      detectPlatform();
    }).toThrow(/sunos-x64/);

    expect(() => {
      detectPlatform();
    }).toThrow(/github.com\/geekstrancend\/Sentri\/issues/);
  });

  test("exports correct version", () => {
    expect(SENTRI_VERSION).toBe("0.1.3");
  });

  test("exports correct GitHub repo", () => {
    expect(GITHUB_REPO).toBe("geekstrancend/Sentri");
  });

  test("exports binary directory path", () => {
    expect(BINARY_DIR).toContain(".sentri-bin");
  });

  test("download URL is correctly formatted", () => {
    os.platform = jest.fn(() => "linux");
    os.arch = jest.fn(() => "x64");

    const info = detectPlatform();

    expect(info.downloadUrl).toContain("https://github.com");
    expect(info.downloadUrl).toContain("releases/download");
    expect(info.downloadUrl).toContain("v0.1.3");
    expect(info.downloadUrl).toContain("x86_64-unknown-linux-gnu");
  });
});
