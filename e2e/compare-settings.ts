/**
 * Comparison Script: Node.js Lichtblick vs Rust Lichtblick
 * 
 * Extracts all panel settings and runtime TF values from both apps.
 * Run: npx playwright test compare-settings.ts
 */

import { test, expect, Page } from '@playwright/test';

const NODEJS_URL = 'http://localhost:8080/?ds=mcap-local-file';
const RUST_URL = 'http://127.0.0.1:8081/';

// ============================================================
// NODE.JS LICHTBLICK SETTINGS (extracted via Playwright)
// ============================================================
const NODEJS_SETTINGS = {
  colorScheme: 'dark',
  bodyBackground: 'rgb(21, 21, 26)', // #15151a
  webglClearColor: [0.0824, 0.0824, 0.1020, 1.0], // #15151a

  camera: {
    perspective: true,
    distance: 50.350652206163964,
    phi: 60.648768703241295,        // degrees from zenith
    thetaOffset: 37.67417394014945, // degrees azimuth
    targetOffset: [-6.937588898258537, -10.148656018089, 1.785e-16],
    target: [0, 0, 0],
    targetOrientation: [0, 0, 0, 1],
    fovy: 45,                        // degrees
    near: 0.5,
    far: 5000,
  },

  followMode: 'follow-pose',
  followTf: 'ego_vehicle_bb_center',

  scene: {}, // empty = use theme default background

  transforms: {
    'frame:ego_vehicle_bb_center': { visible: true },
    'frame:ego_vehicle_rear_axle': { visible: true },
  },

  topics: {
    'ConvertedTrace': {
      caching: true,
      showAxes: true,
      showPhysicalLanes: true,
      showLogicalLanes: false,
      showReferenceLines: true,
      showBoundingBox: true,
      show3dModels: false,
      defaultModelPath: '/opt/models/vehicles/',
      visible: true,
    },
  },

  // Sidebar TF values at time=10.663s:
  tfAtTime10: {
    frame: 'ego_vehicle_bb_center',
    parent: 'global',
    age: '3.0 ms',
    historySize: 266,
    translation: { x: -279.734, y: -693.427, z: 88.9 },
    rotation: { roll: 0, pitch: 0, yaw: 165 }, // degrees
  },
};

// ============================================================
// RUST LICHTBLICK SETTINGS (from source code defaults)
// ============================================================
const RUST_SETTINGS = {
  colorScheme: 'dark',
  bgColor: [0.082, 0.082, 0.102], // #15151a
  webglClearColorExpected: [0.082, 0.082, 0.102, 1.0],

  camera: {
    perspective: true,
    distance: 50.0,        // ← DIFF: Node=50.35
    theta: 45.0,           // ← DIFF: Node=37.67°
    phi: 60.0,             // close: Node=60.65°
    target: [0, 0, 0],
    fovy: 45.0,
    near: 0.5,
    far: 5000.0,
  },

  followMode: 'pose',                    // Node uses 'follow-pose'
  displayFrame: 'ego_vehicle_bb_center',

  scene: {
    backgroundColor: '', // empty = use default #15151a
    labelScale: 1.0,
    ignoreColladaUpAxis: false,
    meshUpAxis: 'y_up',
  },
};

// ============================================================
// DIFFERENCES SUMMARY
// ============================================================
const DIFFERENCES = {
  '1_background': {
    node: '#15151a (rgb(21,21,26)) - from dark theme',
    rust: '#15151a (0.082, 0.082, 0.102) - hardcoded default',
    status: '✅ MATCH',
  },
  '2_camera_distance': {
    node: 50.35,
    rust: 50.0,
    diff: '0.35 (0.7%)',
    status: '⚠️ CLOSE - loaded from layout in both',
  },
  '3_camera_theta_azimuth': {
    node: '37.67° (thetaOffset from layout)',
    rust: '45.0° (default, should load from layout)',
    diff: '7.33°',
    status: '❌ MISMATCH - Rust not loading thetaOffset from cameraState',
  },
  '4_camera_phi': {
    node: '60.65° (phi from layout)',
    rust: '60.0° (default)',
    diff: '0.65°',
    status: '⚠️ CLOSE',
  },
  '5_camera_targetOffset': {
    node: '[-6.94, -10.15, 0] applied during follow-pose',
    rust: 'Not applied (always [0,0,0])',
    status: '❌ MISMATCH - Rust ignores targetOffset',
  },
  '6_follow_mode': {
    node: '"follow-pose" (full string)',
    rust: '"pose" (shortened)',
    status: '⚠️ CHECK mapping is correct',
  },
  '7_tf_age': {
    node: '3.0 ms (processes all messages in range)',
    rust: 'Was 482ms, NOW FIXED with range-based processing',
    status: '🔄 NEEDS VERIFICATION after reload',
  },
  '8_tf_history': {
    node: '266 entries',
    rust: 'Was 8, NOW max_buffer=500, should be ~266 after fix',
    status: '🔄 NEEDS VERIFICATION after reload',
  },
  '9_yellow_follow_line': {
    node: 'Yellow line from camera to follow target (visible in scene)',
    rust: 'Not implemented',
    status: '❌ MISSING',
  },
};

// ============================================================
// PLAYWRIGHT TEST: Capture live values from Node.js during play
// ============================================================
test.describe('Settings Comparison', () => {
  test('capture Node.js settings and TF values during playback', async ({ page }) => {
    await page.goto(NODEJS_URL);
    await page.waitForTimeout(5000); // Wait for MCAP to load

    // Get panel config
    const config = await page.evaluate(() => {
      const layout = JSON.parse(localStorage.getItem('studio.layout') || '{}');
      return layout.configById?.['3D!9p554p'];
    });

    console.log('\n========== NODE.JS PANEL CONFIG ==========');
    console.log(JSON.stringify(config, null, 2));

    // Get WebGL clear color
    const clearColor = await page.evaluate(() => {
      const canvas = document.querySelector('canvas');
      if (!canvas) return null;
      const ctx = canvas.getContext('webgl2') || canvas.getContext('webgl');
      if (!ctx) return null;
      return Array.from(ctx.getParameter(ctx.COLOR_CLEAR_VALUE));
    });

    console.log('\n========== WEBGL CLEAR COLOR ==========');
    console.log(`Clear color: [${clearColor?.map(v => v.toFixed(4)).join(', ')}]`);

    // Click play and capture TF values at intervals
    console.log('\n========== TF VALUES DURING PLAYBACK ==========');

    // Sample at current time
    const tfValues = await page.evaluate(() => {
      const inputs = document.querySelectorAll('input');
      const values: Record<string, string> = {};
      inputs.forEach((input, i) => {
        if (input.value) values[`input_${i}`] = input.value;
      });
      return values;
    });

    console.log(JSON.stringify(tfValues, null, 2));
  });

  test('capture Rust settings', async ({ page }) => {
    await page.goto(RUST_URL);
    await page.waitForTimeout(3000);

    // Check if canvas exists (file loaded)
    const hasCanvas = await page.evaluate(() => document.querySelectorAll('canvas').length > 0);

    if (hasCanvas) {
      const clearColor = await page.evaluate(() => {
        const canvas = document.querySelector('canvas');
        if (!canvas) return null;
        const ctx = canvas.getContext('webgl2') || canvas.getContext('webgl');
        if (!ctx) return null;
        return Array.from(ctx.getParameter(ctx.COLOR_CLEAR_VALUE));
      });

      console.log('\n========== RUST WEBGL CLEAR COLOR ==========');
      console.log(`Clear color: [${clearColor?.map(v => v.toFixed(4)).join(', ')}]`);
    } else {
      console.log('\n⚠️  Rust page has no canvas (no file loaded)');
      console.log('Load the MCAP file and re-run');
    }

    console.log('\n========== DIFFERENCES ==========');
    for (const [key, diff] of Object.entries(DIFFERENCES)) {
      console.log(`\n${key}:`);
      console.log(`  Status: ${diff.status}`);
      console.log(`  Node.js: ${JSON.stringify((diff as any).node)}`);
      console.log(`  Rust:    ${JSON.stringify((diff as any).rust)}`);
    }
  });
});
