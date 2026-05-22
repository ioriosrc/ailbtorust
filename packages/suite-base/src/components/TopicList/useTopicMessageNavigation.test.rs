The provided unit tests cover various aspects of the `TopicListener` class, including its functionality for handling next and previous message boundaries, boundary discovery, and error handling. Here's a detailed breakdown of what each test checks:

### Test Cases

1. **Handling Next Message Boundary**:
   - **Given**: The `topicName`, `currentTime`, `topicStats`, and `customIterator` are set up.
   - **When**: The `handleNextMessage` method is called.
   - **Then**: It should correctly identify the next message boundary, update the `isNavigating` state, and execute the `nextMessageBoundaryCallback`.

2. **Handling Previous Message Boundary**:
   - **Given**: The `topicName`, `currentTime`, `topicStats`, and `customIterator` are set up.
   - **When**: The `handlePreviousMessage` method is called.
   - **Then**: It should correctly identify the previous message boundary, update the `isNavigating` state, and execute the `previousMessageBoundaryCallback`.

3. **Boundary Discovery**:
   - **Given**: The `topicName`, `currentTime`, `topicStats`, and `customIterator` are set up.
   - **When**: The `boundaryDiscovery` method is called.
   - **Then**: It should correctly identify the boundaries based on the current time and update the `isNavigating` state.

4. **Error Handling**:
   - **Given**: A throwing iterator with a specific error message is used for the `customIterator`.
   - **When**: The `handleNextMessage`, `handlePreviousMessage`, `pausePlayback`, and `seekPlayback` methods are called.
   - **Then**: It should catch and log the error, update the `isNavigating` state to false.

### Implementation Details

- **TopicListener Class**:
  - Contains methods for handling next and previous message boundaries (`handleNextMessage` and `handlePreviousMessage`).
  - Manages boundary discovery logic (`boundaryDiscovery`).
  - Handles errors thrown by iterators (`pausePlayback` and `seekPlayback`).

- **Iterator Handling**:
  - The `customIterator` is a mock iterator that throws an error during the first iteration.

- **Error Logging**:
  - Uses the console to log any errors caught during operation.

These tests ensure the `TopicListener` class behaves as expected under various scenarios, covering boundary identification, error handling, and state management.