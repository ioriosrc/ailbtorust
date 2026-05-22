This code defines a class `CoordinateFrame` that represents a frame in a scene graph. It allows for the addition of child frames, the interpolation between two [time, transform] pairs, and the display-friendly rendition of frame identifiers. The class also includes methods for applying transforms, interpolating between poses, and generating transformation matrices.

Here's a breakdown of the key components:

1. **Class Definition**:
   - `CoordinateFrame` is defined with properties like `id`, `offsetEulerDegrees`, `offsetPosition`, `parent`, and methods to manipulate these properties.

2. **Methods for Adding Child Frames**:
   - `addChild`: Adds a child frame to the current frame.
   - `findClosestTransforms`: Finds the closest [time, transform] pairs between two frames.

3. **Methods for Interpolating Transformations**:
   - `Interpolate`: Interpolates between two [time, transform] pairs.
   - `InterpolateTransform`: Interpolates a single transform between two [time, transform] pairs.

4. **Methods for Applying Transforms**:
   - `GetTransformMatrix`: Calculates the transformation matrix from one frame to another at a given time.
   - `Apply`: Applies a transformation from one frame to another at a given time to a pose, optionally inverting it if needed.

5. **Helper Functions**:
   - `copyPose`: Copies the position and orientation of a pose into another pose.
   - `quaternionFromEuler`: Converts XYZ Euler angles from degrees to a quaternion.

6. **Constants**:
   - `DEG2RAD`: Converts degrees to radians.
   - `FALLBACK_FRAME_ID`: A constant representing an empty frame identifier.

This class provides a comprehensive framework for managing and manipulating frames in a scene graph, enabling efficient transformations and interpolation between different frames.