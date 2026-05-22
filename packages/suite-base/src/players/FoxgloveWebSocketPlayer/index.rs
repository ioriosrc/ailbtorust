The `FoxgloveWebSocketPlayer` class is a WebSocket-based implementation of the Player API in ROS. It allows clients to subscribe to topics, publish messages, and call services through a WebSocket connection. The class manages various aspects such as connectivity with the server, data streaming, and alerts. Here's a detailed breakdown of how it works:

### Key Components

1. **WebSocket Connection**:
   - Establishes a WebSocket connection to the ROS server.
   - Handles all network communications with the server.

2. **Server Capabilities**:
   - Determines which operations are supported by the server (e.g., publishing, subscribing, calling services).

3. **Client Presence**:
   - Tracks the presence of the client on the server.

4. **Session Management**:
   - Manages the state of the session (e.g., start time, end time, clock time).
   - Handles connection reset and reconnection logic.

5. **Data Streaming**:
   - Buffers incoming messages from the server.
   - Converts binary data to human-readable formats.

6. **Alerts**:
   - Manages alerts for issues related to data streaming or server capabilities.

7. **Datatypes Handling**:
   - Tracks and updates message types defined on the server.

### Constructor

- **Initialization**: Initializes properties like `presence`, `serverCapabilities`, `connectionStatus`, etc.
- **Event Listeners**: Registers event listeners for WebSocket events such as messages, disconnections, and connection resets.

### Methods

1. **Reset Session State**:
   - Clears all session-related state including message buffers, timestamps, and alerts.

2. **Setup Publishers**:
   - Configures the client to publish data based on server capabilities and options.
   - Advertises topics with appropriate encoding and datatypes.

3. **Advertise Channel**:
   - Registers a new channel for publishing messages on the specified topic.
   - Handles any errors or warnings related to datatype mismatches.

4. **Unadvertise Channel**:
   - Removes an existing channel from the list of active channels.

5. **Publish Data**:
   - Sends data through the WebSocket connection to the server.
   - Converts JavaScript objects to binary format if necessary.

6. **Call Service**:
   - Initiates a service call to the specified server.
   - Handles service response callbacks and potential errors.

7. **Fetch Asset**:
   - Asynchronously fetches an asset from the ROS server.
   - Manages the lifecycle of asset requests and responses.

8. **Get Batch Iterator**:
   - Returns `undefined` as FoxgloveWebSocketPlayer does not support batch iteration.

### Example Usage

```typescript
const player = new FoxgloveWebSocketPlayer("ws://localhost:11345");

player.onMessage((message) => {
  console.log("Received message:", message);
});

player.connect();
```

### Important Notes

- **Error Handling**: The class includes error handling for network failures, datatype mismatches, and other issues.
- **Performance**: Since it uses WebSocket for real-time data streaming, it can handle high-frequency updates efficiently.
- **Dependencies**: Ensure that the `ws` library is installed (`npm install ws`) to use this player.

This implementation provides a robust framework for ROS clients over WebSocket, facilitating seamless interaction with servers that support the Player API.