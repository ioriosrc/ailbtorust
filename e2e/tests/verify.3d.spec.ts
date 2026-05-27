import { test, expect } from "@playwright/test";
import * as fs from "fs";
import { PNG } from "pngjs";

const RUST_URL = "http://127.0.0.1:8081";

test.describe("3D Panel E2E Validation", () => {
  test("installs extension, loads MCAP, plays, and verifies 3D rendering", async ({ page }) => {
    // 1. Load the app
    await page.goto(RUST_URL);
    await page.waitForTimeout(3000);

    // 2. Open extensions settings tab
    await page.locator(".user-avatar-btn").click();
    await page.locator(".user-menu-item:has-text('Extensions')").click();
    await page.locator(".settings-dialog").waitFor({ state: "visible" });

    // 3. Upload ASAM OSI converter extension
    const extFileChooserPromise = page.waitForEvent("filechooser");
    await page.click("button.extensions-browse-btn");
    const extFileChooser = await extFileChooserPromise;
    await extFileChooser.setFiles("/Users/CTW03722/Downloads/lichtblick.asam-osi-converter-1.0.0.foxe");

    // Wait for extension storage/install completion message
    await expect(page.locator(".extensions-msg-ok")).toContainText("✓ Installed", { timeout: 15000 });
    console.log("Extension installed successfully.");

    // Close settings dialog
    await page.click(".settings-close-btn");
    await page.locator(".settings-dialog").waitFor({ state: "hidden" });

    // 4. Load the MCAP file
    await page.click("button.data-source-button");
    const mcapFileChooserPromise = page.waitForEvent("filechooser");
    await page.click(".data-source-option:has-text('Open local file')");
    const mcapFileChooser = await mcapFileChooserPromise;
    await mcapFileChooser.setFiles("/Users/CTW03722/Downloads/SanDiego_san_diego_sc7_urban_splits_and_parking_lot.xosc.mcap");

    // Wait for data load dialog overlay to dismiss
    await page.locator(".drop-loading-overlay").waitFor({ state: "detached", timeout: 30000 });
    await expect(page.locator(".file-name-text")).toContainText("SanDiego", { timeout: 30000 });
    console.log("MCAP file loaded successfully.");

    // 5. Play the recording
    await page.click("button.play-btn");
    console.log("Playback started. Waiting 5s for frames to process...");
    await page.waitForTimeout(5000);

    // 6. Open 3D panel settings in left sidebar
    await page.locator(".panel-toolbar-btn[title='Settings']").first().click();
    await page.locator(".settings-section-body").first().waitFor({ state: "visible" });

    // Verify Display Frame dropdown has loaded frames
    const displayFrameSelect = page.locator(".settings-row:has-text('Display frame') select");
    const options = await displayFrameSelect.locator("option").allTextContents();
    console.log("Display Frame options found:", options);

    expect(options.length).toBeGreaterThan(0);
    expect(options).not.toContain("(no frames)");
    expect(options).toContain("ego_vehicle_bb_center");

    // 7. Verify WebGL2 canvas rendering is not a black screen or blank background
    const canvas = page.locator("canvas").first();
    await canvas.waitFor({ state: "visible" });
    const screenshotBuffer = await canvas.screenshot();

    const png = PNG.sync.read(screenshotBuffer);
    let nonBgPixels = 0;
    const bgR = 21, bgG = 21, bgB = 26; // Background color #15151a

    for (let i = 0; i < png.data.length; i += 4) {
      const r = png.data[i];
      const g = png.data[i + 1];
      const b = png.data[i + 2];
      if (Math.abs(r - bgR) > 15 || Math.abs(g - bgG) > 15 || Math.abs(b - bgB) > 15) {
        nonBgPixels++;
      }
    }

    console.log(`WebGL Canvas: total pixels = ${png.width * png.height}, non-background pixels = ${nonBgPixels}`);
    
    // Assert that we have a significant number of non-background pixels rendered
    expect(nonBgPixels).toBeGreaterThan(100);
    console.log("3D scene verified: canvas has active rendering (not a black screen).");
  });
});
