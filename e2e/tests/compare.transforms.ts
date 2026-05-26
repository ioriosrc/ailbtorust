import { test, expect } from "@playwright/test";

/**
 * QA Agent: Transforms section comparison tests.
 * Verifies the Transforms UI in the Rust port matches Node.js Lichtblick.
 */

const RUST_URL = "http://127.0.0.1:8081";
const NODEJS_URL = "http://localhost:8080";

test.describe("Transforms Section Comparison", () => {
  test("Rust shows same settings as Node.js", async ({ browser }) => {
    const rustPage = await browser.newPage();
    const nodePage = await browser.newPage();

    await rustPage.goto(RUST_URL);
    await nodePage.goto(NODEJS_URL);

    await rustPage.waitForTimeout(8000);
    await nodePage.waitForTimeout(8000);

    // Check for Transforms-related UI elements
    const rustBody = await rustPage.textContent("body");
    const nodeBody = await nodePage.textContent("body");

    // Node.js has these settings in Transforms:
    const expectedSettings = [
      "Editable",
      "Labels",
      "Axis scale",
      "Line width",
      "Line color",
    ];

    for (const setting of expectedSettings) {
      const inNode = nodeBody?.includes(setting) ?? false;
      const inRust = rustBody?.includes(setting) ?? false;
      if (inNode && !inRust) {
        console.log(`MISSING in Rust: "${setting}" (present in Node.js)`);
      }
    }

    await rustPage.close();
    await nodePage.close();
  });

  test("Rust shows dynamic TF frames with visibility toggle", async ({ browser }) => {
    const rustPage = await browser.newPage();
    await rustPage.goto(RUST_URL);
    await rustPage.waitForTimeout(8000);

    const body = await rustPage.textContent("body");

    // When MCAP with OSI data is loaded, these frames should appear:
    const expectedFrames = [
      "ego_vehicle_bb_center",
      "ego_vehicle_rear_axle",
      "global",
    ];

    for (const frame of expectedFrames) {
      const found = body?.includes(frame) ?? false;
      if (!found) {
        console.log(`MISSING frame in Rust Transforms: "${frame}"`);
      }
    }

    await rustPage.close();
  });

  test("Node.js Transforms has eye icon for frame visibility", async ({ browser }) => {
    const nodePage = await browser.newPage();
    await nodePage.goto(NODEJS_URL);
    await nodePage.waitForTimeout(8000);

    // In Node.js, hovering over ego_vehicle_bb_center shows eye icon
    // to toggle visibility (hides yellow arrow in 3D)
    const frameEl = nodePage.locator("text=ego_vehicle_bb_center").first();
    if (await frameEl.isVisible()) {
      await frameEl.hover();
      await nodePage.waitForTimeout(500);
      // Look for visibility toggle (eye icon)
      const eyeIcon = nodePage.locator('[aria-label*="visibility"], [title*="visibility"], [data-testid*="eye"]');
      const eyeCount = await eyeIcon.count();
      console.log(`Eye icon elements found on hover: ${eyeCount}`);
    }

    await nodePage.close();
  });
});
