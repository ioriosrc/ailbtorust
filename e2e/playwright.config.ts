import { defineConfig, devices } from "@playwright/test";

const RUST_URL = "http://127.0.0.1:8081";
const NODEJS_URL = "http://localhost:8080";

export default defineConfig({
  reporter: [
    ["html", { outputFolder: "reports/html", open: "never" }],
    ["json", { outputFile: "reports/results.json" }],
    ["list"],
  ],
  timeout: 60 * 1000,
  testDir: "./tests",
  use: {
    headless: true,
    trace: "on-first-retry",
    video: "retain-on-failure",
    screenshot: "only-on-failure",
  },
  projects: [
    {
      name: "rust",
      use: {
        baseURL: RUST_URL,
        ...devices["Desktop Chrome"],
      },
    },
    {
      name: "nodejs",
      use: {
        baseURL: NODEJS_URL,
        ...devices["Desktop Chrome"],
      },
    },
    {
      name: "compare",
      testMatch: /compare\..+\.ts$/,
      use: {
        ...devices["Desktop Chrome"],
      },
    },
  ],
});
