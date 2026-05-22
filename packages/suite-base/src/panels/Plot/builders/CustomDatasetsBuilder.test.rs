The provided code snippet defines a test suite for the `CustomDatasetsBuilder` class in a TypeScript context. The tests cover various scenarios such as combining all values from arrays of messages, toggling series enabled state, leaving gaps in the `datasetsByConfigIndex`, and more.

Here's a detailed breakdown of each test:

1. **Combines All Values from Arrays (`current` type):**
   - A `CustomDatasetsBuilder` instance is created with an XPath expression `/foo.values[:].val`.
   - Two series are set: one for the `/foo.val` path and another for the `/bar.val` path.
   - Messages are sent to update the datasets based on different types of changes:
     - **Current Data:** The first message updates the value at `x=1` to 1, and the second message adds a new value to `x=2`.
     - **Message Range Update:** The third message updates the value at `x=3` to 4, and the fourth message adds another new value to `x=4`.
   - After updating the datasets, the test asserts that there are no gaps in the `datasetsByConfigIndex` array and that the correct data points are present for each series.

2. **Toggles Series Enabled State:**
   - A `CustomDatasetsBuilder` instance is created with an XPath expression `/foo.val`.
   - Two series are set: one enabled and another disabled.
   - Messages are sent to update the datasets:
     - The first message updates the value at `x=1` to 1 for the enabled series.
     - The second message updates the value at `x=2` to 2 for the enabled series, which should not change since it's already set to 1.
   - After updating the datasets, the test asserts that the disabled series has no data points and the enabled series still has a correct data point.

3. **Leaves Gaps in `datasetsByConfigIndex` for Missing Series:**
   - A `CustomDatasetsBuilder` instance is created with an XPath expression `/foo.val`.
   - One series is set at index 3.
   - Messages are sent to update the datasets:
     - The first message updates the value at `x=1` to 1 for this series.
     - The second message does not affect the value at `x=1` since the series is disabled.
   - After updating the datasets, the test asserts that there is a gap in the `datasetsByConfigIndex` array where the disabled series is missing its data points.

4. **Supports Toggling Series Enabled State:**
   - A `CustomDatasetsBuilder` instance is created with an XPath expression `/foo.val`.
   - Two series are set: one enabled and another disabled.
   - Messages are sent to update the datasets:
     - The first message updates the value at `x=1` to 1 for both series.
     - The second message disables the series, resulting in no data points being present.
   - After updating the datasets, the test asserts that the enabled series has a correct data point and the disabled series has no data points.

These tests ensure that the `CustomDatasetsBuilder` class functions as expected across various scenarios, covering different types of changes to the datasets and ensuring proper handling of series enabled state.