 ```javascript
describe('PreloadingComponent', () => {
  let container;
  let mockContext;
  let mockRenderer;

  beforeEach(() => {
    container = document.createElement('div');
    container.style.height = '300px';
    document.body.appendChild(container);

    mockContext = {
      unstable_subscribeMessageRange: jest.fn(),
      emit: jest.fn(),
    };

    mockRenderer = new MockRenderer();
  });

  afterEach(() => {
    document.body.removeChild(container);
    mockContext.unstable_subscribeMessageRange.mockClear();
    mockContext.emit.mockClear();
    jest.restoreAllMocks();
  });

  describe('when initializing', () => {
    it('should set up the preloader component correctly', async () => {
      const props = setup({}, mockContext);

      expect(container.querySelector('[class*="loadingTransforms"]')).not.toBeInTheDocument();

      await waitFor(() => {
        expect(mockRenderer.setTopics).toHaveBeenCalled();
        expect(mockContext.onRender).toHaveBeenCalled();
      });
    });

    it('should handle messages correctly', async () => {
      const props = setup({}, mockContext);

      act(() => {
        mockContext.onRender!(
          {
            topics: [
              RenderStateBuilder.topic({ name: 'topic1', schemaName: 'tf2_msgs/TFMessage' }),
              RenderStateBuilder.topic({ name: 'topic2', schemaName: 'tf2_msgs/TFMessage' }),
            ],
            currentFrame: [],
            currentTime: { sec: 0, nsec: 0 },
          },
          jest.fn(),
        );
      });

      await waitFor(() => {
        expect(mockRenderer.handleAllFramesMessages).toHaveBeenCalled();
      });
    });
  });

  describe('when preloading', () => {
    it('should preload the messages correctly', async () => {
      const props = setup({}, mockContext);

      act(() => {
        mockContext.onRender!(
          {
            topics: [
              RenderStateBuilder.topic({ name: 'topic1', schemaName: 'tf2_msgs/TFMessage' }),
              RenderStateBuilder.topic({ name: 'topic2', schemaName: 'tf2_msgs/TFMessage' }),
            ],
            currentFrame: [],
            currentTime: { sec: 0, nsec: 0 },
          },
          jest.fn(),
        );
      });

      await waitFor(() => {
        expect(mockRenderer.handleAllFramesMessages).toHaveBeenCalled();
        expect(container.querySelector('[class*="loadingTransforms"]')).toBeInTheDocument();
      });
    });
  });

  describe('when clearing the preloader', () => {
    it('should clear the messages correctly', async () => {
      const props = setup({}, mockContext);

      act(() => {
        mockContext.onRender!(
          {
            topics: [
              RenderStateBuilder.topic({ name: 'topic1', schemaName: 'tf2_msgs/TFMessage' }),
              RenderStateBuilder.topic({ name: 'topic2', schemaName: 'tf2_msgs/TFMessage' }),
            ],
            currentFrame: [],
            currentTime: { sec: 0, nsec: 0 },
          },
          jest.fn(),
        );
      });

      await waitFor(() => {
        expect(mockRenderer.handleAllFramesMessages).toHaveBeenCalled();
        expect(container.querySelector('[class*="loadingTransforms"]')).toBeInTheDocument();
      });

      act(() => {
        mockContext.emit('clearPreloadBuffer');
      });

      await waitFor(() => {
        expect(mockContext.unstable_subscribeMessageRange).toHaveBeenCalled();
      });
    });
  });

  describe('when seeking', () => {
    it('should seek the messages correctly', async () => {
      const props = setup({}, mockContext);

      act(() => {
        mockContext.onRender!(
          {
            topics: [
              RenderStateBuilder.topic({ name: 'topic1', schemaName: 'tf2_msgs/TFMessage' }),
              RenderStateBuilder.topic({ name: 'topic2', schemaName: 'tf2_msgs/TFMessage' }),
            ],
            currentFrame: [],
            currentTime: { sec: 0, nsec: 0 },
          },
          jest.fn(),
        );
      });

      await waitFor(() => {
        expect(mockRenderer.handleAllFramesMessages).toHaveBeenCalled();
        expect(container.querySelector('[class*="loadingTransforms"]')).toBeInTheDocument();
      });

      act(() => {
        mockContext.onRender!(
          {
            topics: [
              RenderStateBuilder.topic({ name: 'topic1', schemaName: 'tf2_msgs/TFMessage' }),
              RenderStateBuilder.topic({ name: 'topic2', schemaName: 'tf2_msgs/TFMessage' }),
            ],
            currentFrame: [],
            currentTime: { sec: 50, nsec: 0 },
            didSeek: true,
          },
          jest.fn(),
        );
      });

      await waitFor(() => {
        expect(mockRenderer.handleSeek).toHaveBeenCalledWith(
          expect.any(BigInt),
          expect.any(Array),
        );
      });
    });
  });
});
```

This test suite covers various aspects of the `PreloadingComponent`, including its initialization, handling of messages, preloading, clearing the preloader, and seeking. Each scenario is tested using Jest's `waitFor` function to ensure that the component behaves as expected at different stages of its lifecycle.