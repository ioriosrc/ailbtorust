# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: verify.3d.spec.ts >> 3D Panel E2E Validation >> installs extension, loads MCAP, plays, and verifies 3D rendering
- Location: tests/verify.3d.spec.ts:8:7

# Error details

```
Error: expect(received).not.toContain(expected) // indexOf

Expected value: not "(no frames)"
Received array:     ["(no frames)"]
```

# Page snapshot

```yaml
- generic [ref=e1]:
  - generic [ref=e3]:
    - banner [ref=e4]:
      - generic [ref=e5]:
        - button "Lichtblick" [ref=e7] [cursor=pointer]:
          - generic [ref=e8]: Lichtblick
          - img [ref=e9]
        - button "Add panel" [ref=e12] [cursor=pointer]:
          - img [ref=e13]
      - button "📄 SanDiego_san_diego_sc7_urban_splits_and_parking_lot.xosc.mcap" [ref=e16] [cursor=pointer]:
        - generic [ref=e17]: 📄
        - generic [ref=e18]: SanDiego_san_diego_sc7_urban_splits_and_parking_lot.xosc.mcap
      - generic [ref=e19]:
        - button "Hide left sidebar [" [ref=e20] [cursor=pointer]:
          - img [ref=e21]
        - button "Hide right sidebar ]" [ref=e23] [cursor=pointer]:
          - img [ref=e24]
        - button "Profile" [ref=e27] [cursor=pointer]:
          - img [ref=e28]
    - generic [ref=e30]:
      - complementary [ref=e31]:
        - generic [ref=e34]:
          - generic [ref=e35]:
            - button "Panel" [ref=e36] [cursor=pointer]
            - button "Topics" [ref=e37] [cursor=pointer]
            - button "Alerts 1" [ref=e38] [cursor=pointer]:
              - text: Alerts
              - generic [ref=e39]: "1"
            - button "Layouts" [ref=e40] [cursor=pointer]
          - generic [ref=e42]:
            - generic [ref=e43]:
              - generic [ref=e44]: 3D panel
              - generic [ref=e45]:
                - button "⋮" [ref=e47] [cursor=pointer]
                - button "✕" [ref=e48] [cursor=pointer]
            - generic [ref=e49]:
              - generic [ref=e50]:
                - generic [ref=e51]:
                  - generic [ref=e52]: 🔍
                  - textbox "Search panel settings..." [ref=e53] [cursor=pointer]
                - generic [ref=e54]:
                  - generic [ref=e55]: Title
                  - textbox [ref=e56] [cursor=pointer]: 3D
              - generic [ref=e57]:
                - heading "▼ Frame" [level=4] [ref=e59] [cursor=pointer]
                - generic [ref=e60]:
                  - generic [ref=e61]:
                    - generic [ref=e62]: Display frame
                    - combobox [ref=e63] [cursor=pointer]:
                      - option "(no frames)" [selected]
                  - generic [ref=e64]:
                    - generic [ref=e65]: Follow mode
                    - combobox [ref=e66] [cursor=pointer]:
                      - option "Pose" [selected]
                      - option "Position"
                      - option "Fixed"
              - heading "▶ Scene" [level=4] [ref=e69] [cursor=pointer]
              - heading "▶ View" [level=4] [ref=e72] [cursor=pointer]
              - heading "▶ Transforms (0)" [level=4] [ref=e75] [cursor=pointer]
              - heading "▶ Topics" [level=4] [ref=e78] [cursor=pointer]
              - generic [ref=e80]:
                - heading "▶ Custom Layers" [level=4] [ref=e81] [cursor=pointer]
                - generic [ref=e82]:
                  - button "+ Grid" [ref=e83] [cursor=pointer]
                  - button "+ URDF" [ref=e84] [cursor=pointer]
              - heading "▶ Publish" [level=4] [ref=e87] [cursor=pointer]
      - generic [ref=e92]:
        - generic [ref=e93]: 3D
        - generic [ref=e94]:
          - button "⚙" [active] [ref=e95] [cursor=pointer]
          - button "⋮" [ref=e96] [cursor=pointer]
      - complementary [ref=e100]:
        - generic [ref=e103]:
          - generic [ref=e104]:
            - heading "Variables" [level=4] [ref=e105]
            - button "+ Add variable" [ref=e106] [cursor=pointer]
          - generic [ref=e108]:
            - paragraph [ref=e109]: No variables defined.
            - paragraph [ref=e110]: Variables are key/value pairs accessible to all panels via $variable_name in message paths.
    - generic [ref=e112]:
      - slider [ref=e114] [cursor=pointer]: "19.56"
      - generic [ref=e115]:
        - generic [ref=e116]:
          - button "Data source info" [ref=e118] [cursor=pointer]:
            - img [ref=e119]:
              - generic [ref=e121]: i
          - generic [ref=e122]:
            - generic [ref=e123]: 1970-01-01 12:00:05.867 AM
            - combobox [ref=e124] [cursor=pointer]:
              - option "TOD" [selected]
              - option "SEC"
        - generic [ref=e125]:
          - button "Seek backward 100ms" [ref=e126] [cursor=pointer]:
            - img [ref=e127]
          - button "⏸" [ref=e129] [cursor=pointer]
          - button "Seek forward 100ms" [ref=e130] [cursor=pointer]:
            - img [ref=e131]
        - generic [ref=e133]:
          - 'button "Loop: OFF" [ref=e134] [cursor=pointer]':
            - img [ref=e135]
          - combobox [ref=e139]:
            - option "0.1x"
            - option "0.25x"
            - option "0.5x"
            - option "1x" [selected]
            - option "2x"
            - option "5x"
            - option "10x"
  - button "Choose File" [ref=e140]
```

# Test source

```ts
  1  | import { test, expect } from "@playwright/test";
  2  | import * as fs from "fs";
  3  | import { PNG } from "pngjs";
  4  | 
  5  | const RUST_URL = "http://127.0.0.1:8081";
  6  | 
  7  | test.describe("3D Panel E2E Validation", () => {
  8  |   test("installs extension, loads MCAP, plays, and verifies 3D rendering", async ({ page }) => {
  9  |     // 1. Load the app
  10 |     await page.goto(RUST_URL);
  11 |     await page.waitForTimeout(3000);
  12 | 
  13 |     // 2. Open extensions settings tab
  14 |     await page.locator(".user-avatar-btn").click();
  15 |     await page.locator(".user-menu-item:has-text('Extensions')").click();
  16 |     await page.locator(".settings-dialog").waitFor({ state: "visible" });
  17 | 
  18 |     // 3. Upload ASAM OSI converter extension
  19 |     const extFileChooserPromise = page.waitForEvent("filechooser");
  20 |     await page.click("button.extensions-browse-btn");
  21 |     const extFileChooser = await extFileChooserPromise;
  22 |     await extFileChooser.setFiles("/Users/CTW03722/Downloads/lichtblick.asam-osi-converter-1.0.0.foxe");
  23 | 
  24 |     // Wait for extension storage/install completion message
  25 |     await expect(page.locator(".extensions-msg-ok")).toContainText("✓ Installed", { timeout: 15000 });
  26 |     console.log("Extension installed successfully.");
  27 | 
  28 |     // Close settings dialog
  29 |     await page.click(".settings-close-btn");
  30 |     await page.locator(".settings-dialog").waitFor({ state: "hidden" });
  31 | 
  32 |     // 4. Load the MCAP file
  33 |     await page.click("button.data-source-button");
  34 |     const mcapFileChooserPromise = page.waitForEvent("filechooser");
  35 |     await page.click(".data-source-option:has-text('Open local file')");
  36 |     const mcapFileChooser = await mcapFileChooserPromise;
  37 |     await mcapFileChooser.setFiles("/Users/CTW03722/Downloads/SanDiego_san_diego_sc7_urban_splits_and_parking_lot.xosc.mcap");
  38 | 
  39 |     // Wait for data load dialog overlay to dismiss
  40 |     await page.locator(".drop-loading-overlay").waitFor({ state: "detached", timeout: 30000 });
  41 |     await expect(page.locator(".file-name-text")).toContainText("SanDiego", { timeout: 30000 });
  42 |     console.log("MCAP file loaded successfully.");
  43 | 
  44 |     // 5. Play the recording
  45 |     await page.click("button.play-btn");
  46 |     console.log("Playback started. Waiting 5s for frames to process...");
  47 |     await page.waitForTimeout(5000);
  48 | 
  49 |     // 6. Open 3D panel settings in left sidebar
  50 |     await page.locator(".panel-toolbar-btn[title='Settings']").first().click();
  51 |     await page.locator(".settings-section-body").first().waitFor({ state: "visible" });
  52 | 
  53 |     // Verify Display Frame dropdown has loaded frames
  54 |     const displayFrameSelect = page.locator(".settings-row:has-text('Display frame') select");
  55 |     const options = await displayFrameSelect.locator("option").allTextContents();
  56 |     console.log("Display Frame options found:", options);
  57 | 
  58 |     expect(options.length).toBeGreaterThan(0);
> 59 |     expect(options).not.toContain("(no frames)");
     |                         ^ Error: expect(received).not.toContain(expected) // indexOf
  60 |     expect(options).toContain("ego_vehicle_bb_center");
  61 | 
  62 |     // 7. Verify WebGL2 canvas rendering is not a black screen or blank background
  63 |     const canvas = page.locator("canvas").first();
  64 |     await canvas.waitFor({ state: "visible" });
  65 |     const screenshotBuffer = await canvas.screenshot();
  66 | 
  67 |     const png = PNG.sync.read(screenshotBuffer);
  68 |     let nonBgPixels = 0;
  69 |     const bgR = 21, bgG = 21, bgB = 26; // Background color #15151a
  70 | 
  71 |     for (let i = 0; i < png.data.length; i += 4) {
  72 |       const r = png.data[i];
  73 |       const g = png.data[i + 1];
  74 |       const b = png.data[i + 2];
  75 |       if (Math.abs(r - bgR) > 15 || Math.abs(g - bgG) > 15 || Math.abs(b - bgB) > 15) {
  76 |         nonBgPixels++;
  77 |       }
  78 |     }
  79 | 
  80 |     console.log(`WebGL Canvas: total pixels = ${png.width * png.height}, non-background pixels = ${nonBgPixels}`);
  81 |     
  82 |     // Assert that we have a significant number of non-background pixels rendered
  83 |     expect(nonBgPixels).toBeGreaterThan(100);
  84 |     console.log("3D scene verified: canvas has active rendering (not a black screen).");
  85 |   });
  86 | });
  87 | 
```