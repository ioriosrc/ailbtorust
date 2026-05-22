The provided test suite for the `HoverTooltip` component ensures that various functionalities of the tooltip are working correctly, such as showing and hiding content, handling mouse events like hover and click, adjusting position based on canvas bounds, and more. Here's a breakdown of some key aspects of the test suite:

1. **Mocking Element Size**:
   - The `offsetWidth` and `offsetHeight` properties of the mock canvas are set to 200 pixels, which is used instead of the default constants (`MAX_WIDTH`, `MAX_HEIGHT`) for measuring the tooltip size.
   - This setup ensures that the tooltip can be positioned accurately inside the canvas bounds without relying on hardcoded constants.

2. **Hover and Click Event Handling**:
   - The test checks if the tooltip displays content when hovered over an entity and hides it when clicked.
   - These event handlers are tested for correct functionality in both hover-pinned and settled modes to ensure smooth user interaction with the tooltip.

3. **Position Adjustment**:
   - When the mouse is near the edge of the canvas, the tooltip should be flipped by moving it closer to the center.
   - This test case specifically checks if the tooltip correctly flips when near the left or top edges of the canvas, maintaining its alignment within the bounds.

4. **Tooltip Content and Display**:
   - The test verifies that the tooltip displays the correct entity information, including topic, entityId, and metadata keys and values.
   - It also checks if the tooltip content is cleared when a new entity is provided.

5. **Escape Key Handling**:
   - The test ensures that pressing the Escape key dismisses the hover-pinned tooltip correctly.

6. **Dwell Mode and Update Delay**:
   - The test simulates a dwell effect by waiting for 700 milliseconds before switching to pedestrian-1, and checks if the content of the tooltip remains unchanged during this period.
   - It also tests if the tooltip updates to the new entity after the grace period has passed.

These tests collectively cover various aspects of the `HoverTooltip` component's behavior, ensuring it functions correctly across different scenarios and edge cases.