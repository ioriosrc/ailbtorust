This code snippet defines a custom extension for the Foxglove Studio application that manages the state of a camera in real-time. The extension handles various aspects such as camera positioning, orientation, and rendering based on user input and changes to the scene configuration. Here's a breakdown of the main components:

### Key Components

1. **CameraState Class**:
   - This class represents the current state of the camera, including position, orientation, fields of view, near and far clipping planes.

2. **CameraManager Class**:
   - This class manages the camera rendering logic.
     - It handles updates to the camera state based on user input and changes in the scene configuration.
     - The `startFrame` method is responsible for updating the camera position and orientation based on the selected display frame.
     - It also updates the camera based on the current mode (orthographic or perspective).

3. **Extension Class**:
   - This class extends the `RendererExtension` provided by Foxglove Studio to add custom functionality.
     - It sets up the camera manager as a child of the renderer extension.
     - The `handleResize` method is used to update the aspect ratio of the camera based on the window size.

4. **Utility Functions**:
   - These functions are used internally within the `CameraManager` class to perform various calculations, such as converting spherical coordinates to Cartesian and quaternion rotations.

### How it Works

1. **Renderer Extension Setup**:
   - The `Extension` class sets up the camera manager as a child of the renderer extension.
   - It provides methods like `startFrame`, `handleResize`, and `getActiveCamera`.

2. **Camera State Management**:
   - When the user interacts with the studio, the camera state is updated accordingly.
   - The `startFrame` method calculates the position and orientation of the camera based on the selected display frame.

3. **Rendering Logic**:
   - The `handleResize` method updates the aspect ratio of the camera to maintain correct rendering.
   - The `CameraManager` class manages the camera's state, ensuring that it is updated in real-time based on the scene configuration.

### Example Usage

To use this extension in your Foxglove Studio setup, you would include it in your project and ensure that the necessary dependencies are installed. This setup will allow for dynamic camera control within the studio interface, providing a more interactive experience for data visualization and analysis.