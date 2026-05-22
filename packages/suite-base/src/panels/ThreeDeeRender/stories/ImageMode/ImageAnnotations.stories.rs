This code defines several components in a React application for displaying and interacting with images, annotations, and other visualizations. Here's an overview of what each component does:

1. **ImagePanel**: A component that renders the actual image along with annotations. It takes props such as `overrideConfig` to customize how the annotations are displayed.

2. **makeRawImageAndCalibration**: A utility function that creates a sample raw image message and a calibration message for testing purposes.

3. **SyncAnnotationsStory**: A storybook component used for testing the `syncAnnotations` feature of the `ImagePanel`. It includes different scenarios where annotations should be synchronized with the camera's timestamp or without synchronization.

4. **SyncAnnotationsWaitingWithCalibration, SyncAnnotationsWaitingWithoutCalibration, SyncAnnotationsReadyWithCalibration, SyncAnnotationsReadyWithoutCalibration**: These stories test various states of the `syncAnnotations` feature, such as when both calibration and annotation data are present and ready to be synchronized, when only annotation data is available, or vice versa.

5. **SyncAnnotationsStoryArgs**: An interface that defines the props for the `SyncAnnotationsStory` component.

The components are designed to work together to simulate real-world scenarios involving images, annotations, and synchronization between these elements.