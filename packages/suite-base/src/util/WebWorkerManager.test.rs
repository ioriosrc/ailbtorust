```rust
use std::collections::HashMap;

struct WebWorker {
    worker: Box<dyn Fn() -> Box<dyn Channel>>,
}

impl WebWorker {
    fn new(worker_factory: impl Fn() -> Box<dyn Channel>) -> Self {
        Self { worker }
    }

    fn post_message(&self) -> ! {
        unreachable!("not supported");
    }
}

#[derive(Debug)]
struct WorkerState {
    rpc: Option<Box<dyn Fn() -> ()>>,
    listener_ids: Vec<String>,
}

pub struct WebWorkerManager {
    workers: HashMap<String, WorkerState>,
    worker_count: usize,
}

impl WebWorkerManager {
    pub fn new(worker_factory: impl Fn() -> Box<dyn Channel>, max_workers: usize) -> Self {
        Self {
            workers: HashMap::with_capacity(max_workers),
            worker_count: 0,
        }
    }

    pub fn register_worker_listener(&mut self, id: &str) {
        let worker = WebWorker::new(worker_factory);
        if let Some(state) = self.workers.get_mut(id) {
            state.listener_ids.push(id.to_string());
        } else {
            self.workers.insert(id.to_string(), WorkerState {
                rpc: None,
                listener_ids: vec![id.to_string()],
            });
        }
        self.worker_count += 1;
    }

    pub fn unregister_worker_listener(&mut self, id: &str) {
        if let Some(state) = self.workers.get_mut(id) {
            state.listener_ids.remove_if(|l| l != id);
            if !state.listener_ids.is_empty() {
                return;
            }
        }

        let worker = self.workers.remove(id).unwrap();
        if let Some(rpc) = worker.rpc.take() {
            rpc();
        }
        self.worker_count -= 1;
    }

    pub fn testing_worker_count(&self) -> usize {
        self.worker_count
    }

    pub fn testing_get_worker_state(&self, id: &str) -> Option<&WorkerState> {
        self.workers.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kills_the_worker_when_unregistering_it() {
        let mut web_worker_manager = WebWorkerManager::new(|| Box::new(WebWorker::new(|| {})), 1);

        let worker_id = "1";
        web_worker_manager.register_worker_listener(worker_id);
        let worker = web_worker_manager.testing_get_worker_state(worker_id).unwrap().worker;

        assert!(!worker.terminated);
        web_worker_manager.unregister_worker_listener(worker_id);
        assert!(worker.terminated);
    }

    #[test]
    fn does_not_unregister_the_worker_until_the_last_listener_stops_listening() {
        let mut web_worker_manager = WebWorkerManager::new(|| Box::new(WebWorker::new(|| {})), 2);

        let worker_id1 = "0";
        let worker_id2 = "1";

        web_worker_manager.register_worker_listener(worker_id1);
        web_worker_manager.register_worker_listener(worker_id2);

        let worker1 = web_worker_manager.testing_get_worker_state(worker_id1).unwrap().worker;
        assert!(!worker1.terminated);
        web_worker_manager.unregister_worker_listener(worker_id1);
        assert!(worker1.terminated);

        let worker2 = web_worker_manager.testing_get_worker_state(worker_id2).unwrap().worker;
        assert!(!worker2.terminated);
    }

    #[test]
    fn can_add_and_remove_multiple_listeners_to_the_same_worker() {
        let mut web_worker_manager = WebWorkerManager::new(|| Box::new(WebWorker::new(|| {})), 2);

        let worker_id = "1";
        web_worker_manager.register_worker_listener(worker_id);
        assert_eq!(web_worker_manager.testing_get_worker_state(worker_id).unwrap().listener_ids.len(), 1);

        web_worker_manager.register_worker_listener(worker_id);
        assert_eq!(web_worker_manager.testing_get_worker_state(worker_id).unwrap().listener_ids.len(), 2);

        web_worker_manager.register_worker_listener(worker_id);
        assert_eq!(web_worker_manager.testing_get_worker_state(worker_id).unwrap().listener_ids.len(), 2);

        web_worker_manager.unregister_worker_listener(worker_id);
        assert_eq!(
            web_worker_manager.testing_get_worker_state(worker_id),
            Some(&WorkerState {
                rpc: None,
                listener_ids: vec!["3"],
            })
        );

        web_worker_manager.unregister_worker_listener(worker_id);
        assert!(web_worker_manager.workers.is_empty());

        web_worker_manager.register_worker_listener(worker_id);
        assert_eq!(web_worker_manager.testing_get_worker_state(worker_id).unwrap().listener_ids.len(), 1);
    }

    #[test]
    fn throws_when_registering_an_id_twice() {
        let mut web_worker_manager = WebWorkerManager::new(|| Box::new(WebWorker::new(|| {})), 2);

        web_worker_manager.register_worker_listener("1");
        assert_eq!(
            web_worker_manager.testing_get_worker_state("1"),
            Some(&WorkerState {
                rpc: None,
                listener_ids: vec!["1"],
            })
        );

        assert!(web_worker_manager.register_worker_listener("1").is_err());
    }

    #[test]
    fn throws_when_unregistering_an_id_twice() {
        let mut web_worker_manager = WebWorkerManager::new(|| Box::new(WebWorker::new(|| {})), 2);

        web_worker_manager.register_worker_listener("1");
        assert_eq!(
            web_worker_manager.testing_get_worker_state("1"),
            Some(&WorkerState {
                rpc: None,
                listener_ids: vec!["1"],
            })
        );

        web_worker_manager.unregister_worker_listener("1");
        assert!(web_worker_manager.unregister_worker_listener("1").is_err());
    }
}
```