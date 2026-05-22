This code defines a `PathPlotter` class that manages the plotting and rendering of data on a canvas. The `PathPlotter` class uses a `Renderer` instance to handle the actual rendering of the plots. It also includes a `DatasetsBuilder` instance to manage the datasets and their processing.

The `PathPlotter` class has several methods to interact with the renderer and datasets:

- `dispatchRender()`: Updates the renderer with new data and resets the scale if necessary.
- `dispatchDownsample()`: Gets the latest downsampled datasets and dispatches them to the renderer.
- `dispatchDatasetsRender(datasets: Dataset[]): void`: Renders the provided datasets on the canvas.

The class also includes methods to handle user interactions such as zooming and resetting views. It manages subscriptions to the data source using a `MessageRange` instance, which is responsible for fetching and processing new data batches.

Overall, this code provides a robust framework for plotting and rendering data using a canvas and a custom renderer.