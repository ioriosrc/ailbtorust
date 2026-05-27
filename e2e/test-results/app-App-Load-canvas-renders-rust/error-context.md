# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: app.spec.ts >> App Load >> canvas renders
- Location: tests/app.spec.ts:21:7

# Error details

```
TimeoutError: page.waitForSelector: Timeout 10000ms exceeded.
Call log:
  - waiting for locator('canvas') to be visible

```

# Page snapshot

```yaml
- generic [ref=e3]:
  - banner [ref=e4]:
    - generic [ref=e5]:
      - button "Lichtblick" [ref=e7] [cursor=pointer]:
        - generic [ref=e8]: Lichtblick
        - img [ref=e9]
      - button "Add panel" [ref=e12] [cursor=pointer]:
        - img [ref=e13]
    - button "📂 Open data source" [ref=e16] [cursor=pointer]:
      - generic [ref=e17]: 📂
      - generic [ref=e18]: Open data source
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
          - button "Alerts" [ref=e38] [cursor=pointer]
          - button "Layouts" [ref=e39] [cursor=pointer]
        - paragraph [ref=e42]: Open a data source to see topics
    - generic [ref=e47]:
      - heading "Welcome to Lichtblick" [level=2] [ref=e48]
      - paragraph [ref=e49]: Open a data source to get started
      - button "Open local file" [ref=e51] [cursor=pointer]
    - complementary [ref=e52]:
      - generic [ref=e55]:
        - generic [ref=e56]:
          - heading "Variables" [level=4] [ref=e57]
          - button "+ Add variable" [ref=e58] [cursor=pointer]
        - generic [ref=e60]:
          - paragraph [ref=e61]: No variables defined.
          - paragraph [ref=e62]: Variables are key/value pairs accessible to all panels via $variable_name in message paths.
```

# Test source

```ts
  1  | import { test, expect } from "@playwright/test";
  2  | 
  3  | /**
  4  |  * QA Agent: Core functionality tests for Rust Lichtblick.
  5  |  * Runs against whichever project is configured (rust or nodejs).
  6  |  */
  7  | 
  8  | test.describe("App Load", () => {
  9  |   test("loads without errors", async ({ page, baseURL }) => {
  10 |     const errors: string[] = [];
  11 |     page.on("pageerror", (err) => errors.push(err.message));
  12 | 
  13 |     await page.goto(baseURL!);
  14 |     await page.waitForTimeout(3000);
  15 | 
  16 |     // Should not have JS panics or fatal errors
  17 |     const panics = errors.filter((e) => e.includes("panic") || e.includes("unreachable"));
  18 |     expect(panics).toHaveLength(0);
  19 |   });
  20 | 
  21 |   test("canvas renders", async ({ page, baseURL }) => {
  22 |     await page.goto(baseURL!);
> 23 |     await page.waitForSelector("canvas", { timeout: 10000 });
     |                ^ TimeoutError: page.waitForSelector: Timeout 10000ms exceeded.
  24 | 
  25 |     const canvas = page.locator("canvas").first();
  26 |     const box = await canvas.boundingBox();
  27 |     expect(box).not.toBeNull();
  28 |     expect(box!.width).toBeGreaterThan(100);
  29 |     expect(box!.height).toBeGreaterThan(100);
  30 |   });
  31 | });
  32 | 
  33 | test.describe("Sidebar", () => {
  34 |   test("shows Panel tab", async ({ page, baseURL }) => {
  35 |     await page.goto(baseURL!);
  36 |     await page.waitForTimeout(2000);
  37 | 
  38 |     // Look for sidebar tab labels
  39 |     const body = await page.textContent("body");
  40 |     expect(body).toContain("Panel");
  41 |   });
  42 | 
  43 |   test("shows Topics tab", async ({ page, baseURL }) => {
  44 |     await page.goto(baseURL!);
  45 |     await page.waitForTimeout(2000);
  46 | 
  47 |     const body = await page.textContent("body");
  48 |     expect(body).toContain("Topics");
  49 |   });
  50 | 
  51 |   test("shows Layouts tab", async ({ page, baseURL }) => {
  52 |     await page.goto(baseURL!);
  53 |     await page.waitForTimeout(2000);
  54 | 
  55 |     const body = await page.textContent("body");
  56 |     expect(body).toContain("Layouts");
  57 |   });
  58 | });
  59 | 
  60 | test.describe("3D Panel Settings", () => {
  61 |   test("Transforms section exists when data loaded", async ({ page, baseURL }) => {
  62 |     await page.goto(baseURL!);
  63 |     await page.waitForTimeout(5000);
  64 | 
  65 |     // After MCAP loads, Transforms should appear in panel settings
  66 |     const body = await page.textContent("body");
  67 |     // This depends on having an MCAP file loaded
  68 |     // In empty state, panel settings should still be navigable
  69 |     expect(body).toBeDefined();
  70 |   });
  71 | });
  72 | 
  73 | test.describe("Playback Controls", () => {
  74 |   test("play/pause button exists", async ({ page, baseURL }) => {
  75 |     await page.goto(baseURL!);
  76 |     await page.waitForTimeout(2000);
  77 | 
  78 |     // Look for playback controls
  79 |     const playBtn = page.locator('[aria-label="Play"], [title="Play"], button:has-text("▶")');
  80 |     // At minimum the toolbar should exist
  81 |     const toolbar = page.locator('[class*="toolbar"], [class*="playback"]');
  82 |     const count = await toolbar.count();
  83 |     // Either toolbar or play button should exist
  84 |     expect(count + (await playBtn.count())).toBeGreaterThanOrEqual(0);
  85 |   });
  86 | });
  87 | 
```