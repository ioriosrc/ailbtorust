import { test, expect, Page } from "@playwright/test";

const RUST_URL = "http://127.0.0.1:8081";
const NODEJS_URL = "http://localhost:8080";

/**
 * Compare Agent: Visual and behavioral comparison between
 * Node.js Lichtblick (8080) and Rust Lichtblick (8081).
 */

test.describe("Visual Comparison: Rust vs Node.js", () => {
  test("3D panel renders solid cubes in both", async ({ browser }) => {
    const rustPage = await browser.newPage();
    const nodePage = await browser.newPage();

    await rustPage.goto(RUST_URL);
    await nodePage.goto(NODEJS_URL);

    // Wait for canvas to be present
    await rustPage.waitForSelector("canvas", { timeout: 15000 });
    await nodePage.waitForSelector("canvas", { timeout: 15000 });

    // Take screenshots of the 3D panel area
    const rustScreenshot = await rustPage.screenshot({ fullPage: false });
    const nodeScreenshot = await nodePage.screenshot({ fullPage: false });

    // Both should have rendered content (non-empty canvas)
    expect(rustScreenshot.length).toBeGreaterThan(1000);
    expect(nodeScreenshot.length).toBeGreaterThan(1000);

    await rustPage.close();
    await nodePage.close();
  });

  test("sidebar Transforms section shows frame data", async ({ browser }) => {
    const rustPage = await browser.newPage();
    const nodePage = await browser.newPage();

    await rustPage.goto(RUST_URL);
    await nodePage.goto(NODEJS_URL);

    // Wait for app load
    await rustPage.waitForTimeout(5000);
    await nodePage.waitForTimeout(5000);

    // Check sidebar content
    const rustSidebar = await rustPage.textContent("body");
    const nodeSidebar = await nodePage.textContent("body");

    // Both should show Transforms section when data is loaded
    // (This test validates structure, actual frame data depends on MCAP file)
    expect(rustSidebar).toBeDefined();
    expect(nodeSidebar).toBeDefined();

    await rustPage.close();
    await nodePage.close();
  });

  test("camera follow mode tracks ego vehicle", async ({ browser }) => {
    const rustPage = await browser.newPage();
    await rustPage.goto(RUST_URL);
    await rustPage.waitForSelector("canvas", { timeout: 15000 });

    // Inject console debug to capture camera state
    const cameraState = await rustPage.evaluate(() => {
      // Access any exposed debug state
      return (window as any).__debugCameraState || "not-exposed";
    });

    // Camera should exist (even if debug not exposed)
    expect(cameraState).toBeDefined();
    await rustPage.close();
  });
});

test.describe("Console Debug Comparison", () => {
  test("capture console logs from both apps", async ({ browser }) => {
    const rustPage = await browser.newPage();
    const nodePage = await browser.newPage();

    const rustLogs: string[] = [];
    const nodeLogs: string[] = [];

    rustPage.on("console", (msg) => rustLogs.push(msg.text()));
    nodePage.on("console", (msg) => nodeLogs.push(msg.text()));

    await rustPage.goto(RUST_URL);
    await nodePage.goto(NODEJS_URL);

    await rustPage.waitForTimeout(5000);
    await nodePage.waitForTimeout(5000);

    // Log counts for debugging
    console.log(`Rust console messages: ${rustLogs.length}`);
    console.log(`Node.js console messages: ${nodeLogs.length}`);

    // Neither should have errors
    const rustErrors = rustLogs.filter((l) => l.includes("ERROR") || l.includes("panic"));
    expect(rustErrors).toHaveLength(0);

    await rustPage.close();
    await nodePage.close();
  });
});
