module.exports = {
  testEnvironment: "node",
  testMatch: ["**/__tests__/**/*.test.js"],
  collectCoverageFrom: [
    "lib/**/*.js",
    "index.js",
    "bin/sentri.js",
    "!**/*.test.js",
  ],
  coverageDirectory: "coverage",
  coverageReporters: ["text", "lcov"],
};
