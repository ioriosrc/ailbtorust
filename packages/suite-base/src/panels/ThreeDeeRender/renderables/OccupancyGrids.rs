 This code is a TypeScript file that defines classes and functions to represent the costmap, map, and raw data in an occupancy grid. It includes methods to calculate and normalize the data, as well as methods to create different palettes for rendering the costmap, map, and raw data.

### Key Components:

1. **Class Definitions:**
   - `CostmapData`: Represents the costmap data.
   - `MapData`: Represents the map data.
   - `RawData`: Represents the raw data.

2. **Methods:**
   - `normalizeHeader`, `normalizeTime`, `normalizePose`: Normalize the header, time, and pose fields of an occupancy grid message.
   - `paletteColorCached`: Maps a given value to a color using a predefined palette based on the specified colormode.
   - `createCostmapPalette`, `createMapPalette`, `createRawPalette`: Generate the palettes for costmap, map, and raw data.

3. **Constants:**
   - `costmapPalette`, `mapPalette`, `rawPalette`: Arrays to store the generated palettes.

### Usage Example:

To use this code, you would typically create an instance of one of these classes and call their methods to normalize the data fields of an occupancy grid message. You can then use these instances in your visualization application to display the costmap, map, or raw data appropriately.

Here is a simplified example of how you might use the `CostmapData` class:

```typescript
import { CostmapData } from './costmap-data';

// Create an instance of CostmapData
const costmap = new CostmapData();

// Assuming msg is an instance of PartialMessage<OccupancyGrid>
const msg = ...;

// Normalize the data fields of the occupancy grid message
msg.header = normalizeHeader(msg.header);
msg.info.map_load_time = normalizeTime(msg.info.map_load_time);
msg.info.resolution = msg.info.resolution ?? 0;
msg.info.width = msg.info.width ?? 0;
msg.info.height = msg.info.height ?? 0;
msg.data = normalizeInt8Array(msg.data);

// Set the costmap data
costmap.setData(msg.data, msg.header.stamp, msg.header.frame_id);
```

This example assumes that you have other necessary functions to handle the normalization and data representation. The actual implementation of these functions would depend on your specific application requirements and the structure of your ROS messages.