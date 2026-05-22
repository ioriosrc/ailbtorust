The provided test suite for the `RawMessagesPanel` component in ROS2 (Robot Operating System) focuses on various scenarios such as rendering messages, handling different expansion modes, and displaying differences between messages. Here's a breakdown of what each test case covers:

1. **Rendering Messages:**
   - Tests whether all message properties are displayed correctly.
   - Validates that the receive time is rendered in a readable format.

2. **Handling Different Expansion Modes:**
   - Ensures that nodes are expanded and collapsed as specified by the `expansion` object.
   - Verifies that the expand/collapse button accurately reflects the state of each node.

3. **Displaying Differences Between Messages:**
   - Tests whether messages with identical data appear to be different in the UI.
   - Validates that clicking "Show full message" displays a detailed view of the message structure.

4. **Testing Expansion Settings:**
   - Ensures that nodes are expanded and collapsed based on the provided expansion settings (`expandedNode` and `collapsedNode`).
   - Verifies that empty expansion objects do not affect node rendering.
   - Validates that mixed expansion states (e.g., some nodes expanded, others collapsed) are handled correctly.

The test suite is comprehensive and ensures that the `RawMessagesPanel` component behaves as expected across different scenarios.