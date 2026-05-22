```rust
use web_sys::OffscreenCanvas;

#[derive(Debug)]
struct ChartRendererWorkerHandle {
    worker: web_sys::Worker,
}

pub struct OffscreenCanvasRenderer {
    canvas: OffscreenCanvas,
    theme: Option<web_sys::Theme>,
    worker_handle: ChartRendererWorkerHandle,
}

impl OffscreenCanvasRenderer {
    pub fn new(canvas: OffscreenCanvas, theme: web_sys::Theme) -> Self {
        let worker = web_sys::Worker::new(&"ChartRenderer.worker".into()).unwrap();
        Self {
            canvas,
            theme: Some(theme),
            worker_handle: ChartRendererWorkerHandle { worker },
        }
    }

    pub async fn update(&self, action: Immutable<UpdateAction>) -> Option<Bounds> {
        if let Ok(remote) = self.worker_handle.clone_worker() {
            remote.update(action).await
        } else {
            None
        }
    }

    pub async fn get_elements_at_pixel(&self, pixel: { x: f64; y: f64 }) -> Vec<HoverElement> {
        if let Ok(remote) = self.worker_handle.clone_worker() {
            remote.get_elements_at_pixel(pixel).await
        } else {
            vec![]
        }
    }

    pub async fn update_datasets(&self, datasets: Vec<Dataset>) -> Option<Scale> {
        if let Ok(remote) = self.worker_handle.clone_worker() {
            remote.update_datasets(datasets).await
        } else {
            None
        }
    }

    fn clone_worker(&self) -> Result<Option<web_sys::Worker>, JsValue> {
        Ok(Some(web_sys::Worker::new(&"ChartRenderer.worker".into()).unwrap()))
    }
}
```