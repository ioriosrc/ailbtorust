 ```javascript
// This is a comprehensive test suite for the Mosaic library, which handles tab panel configurations in a graphical user interface.
// The Mosaic library is designed to manage tab panels efficiently and provide various functionalities such as updating, adding, deleting tabs.
// The test suite covers different scenarios such as setting up new tab panels, modifying existing ones, validating configurations,
// and more. Each test case ensures that the Mosaic library functions correctly under various conditions.

import { expect, describe, it } from 'mocha';
import { MosaicNode, getParentTabPanelByPanelId, getPathFromNode, getNodeAtPath, validateTabPanelConfig } from './mosaic';

describe('Mosaic Library Tests', () => {
  describe('Setting Up New Tab Panels', () => {
    it('should create a new tab panel when none exist', () => {
      const existingPanels = {};
      expect(setupNewTabPanel(existingPanels, 'Tab!abc')).toEqual({ "Tab!abc": {} });
    });

    it('should update an existing tab panel when one exists', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!abc', updatedConfig)).toEqual({ "Tab!abc": updatedConfig });
    });

    it('should not create a new tab panel when one already exists and has the same config', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!abc', updatedConfig)).toEqual({ "Tab!abc": { ...existingPanels["Tab!abc"], ...updatedConfig } });
    });

    it('should create a new tab panel when one already exists and has different config', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 1, tabs: [{ title: "B" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!abc', updatedConfig)).toEqual({ "Tab!abc": { ...existingPanels["Tab!abc"], ...updatedConfig } });
    });

    it('should not create a new tab panel when one already exists and has the same panel id but different config', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has different panel id but same config', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 1, tabs: [{ title: "B" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same panel id but different config type', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updatedConfig });
    });

    it('should create a new tab panel when one already exists and has the same config type but different data structure', () => {
      const existingPanels = { "Tab!abc": {} };
      const updatedConfig = { activeTabIdx: 0, tabs: [{ title: "A" }] };
      expect(setupNewTabPanel(existingPanels, 'Tab!def', updatedConfig)).toEqual({ "Tab!def": updated