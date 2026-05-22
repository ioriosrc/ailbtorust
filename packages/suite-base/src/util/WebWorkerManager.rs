```rust
use std::collections::{HashSet, VecDeque};
use std::future::{self, FutureExt};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use crate::util::{Rpc, RpcMainThreadUtils};

/// This file provides a convenient way to set up and tear down workers as needed. It will create only a single worker
/// of each class, and terminate the worker when all listeners are unregistered.
///
/// # Examples
///
/// ```
/// use lichtblick_suite_base::util::{Rpc, RpcMainThreadUtils};
///
/// fn main() {
///     let worker = WorkerManager::new(|| panic!("Worker creation failed"));
///     worker.register_worker_listener("listener1");
///     worker.unregister_worker_listener("listener1");
/// }
/// ```
pub struct WebWorkerManager<W> {
    create_worker: Box<dyn FnOnce() -> W>,
    max_worker_count: usize,
    worker_states: Vec<Option<(W, Rpc, Vec<String>)>>,
    all_listeners: HashSet<String>,
}

impl<W> WebWorkerManager<W>
where
    W: Channel,
{
    pub fn new(create_worker: impl FnOnce() -> W) -> Self {
        let create_worker = Box::new(create_worker);
        Self {
            create_worker,
            max_worker_count: 1, // Assuming only one worker per class is needed for simplicity
            worker_states: vec![None],
            all_listeners: HashSet::new(),
        }
    }

    pub fn testing_worker_count(&self) -> usize {
        self.worker_states.iter().filter(|x| x.is_some()).count()
    }

    pub fn testing_get_worker_state(&self, id: &str) -> Option<&(W, Rpc, Vec<String>)> {
        self.worker_states
            .iter()
            .find_map(|x| x.as_ref())
            .and_then(|(worker, rpc, listener_ids)| if listener_ids.contains(id) { Some((worker, rpc, listener_ids)) } else { None })
    }

    pub fn register_worker_listener(&mut self, id: &str) -> Rpc {
        if self.all_listeners.contains(id) {
            panic!("cannot register the same listener id twice");
        }
        self.all_listeners.insert(id.to_string());

        let worker_count = self.worker_states.iter().filter(|x| x.is_some()).count();
        if worker_count < self.max_worker_count {
            let worker = (self.create_worker)();
            let rpc = Rpc::new(worker);
            setupMainThreadRpc(rpc);

            let mut index = VecDeque::from_iter((0..self.worker_states.len()).rev());
            let worker = self.worker_states[index.pop_front().unwrap()].take().unwrap();
            let listener_ids = vec![id.to_string()];
            self.worker_states.push(Some((worker, rpc, listener_ids)));
            return rpc;
        }
        let mut worker_state = None;
        for (i, &mut Some((ref mut worker, ref mut rpc, ref mut listener_ids))) in self.worker_states.iter_mut().enumerate() {
            if !listener_ids.contains(id) {
                listener_ids.push(id.to_string());
                worker_state = Some((worker.clone(), rpc.clone(), listener_ids));
                break;
            }
        }
        match worker_state {
            Some((worker, rpc, _)) => {
                let mut index = VecDeque::from_iter((0..self.worker_states.len()).rev());
                self.worker_states[index.pop_front().unwrap()].take().unwrap();
                worker.terminate();
                rpc.terminate();
            }
            None => panic!("no worker state"),
        }

        rpc
    }

    pub fn unregister_worker_listener(&mut self, id: &str) {
        if !self.all_listeners.contains(id) {
            panic!("Cannot find listener to unregister");
        }
        self.all_listeners.remove(id);

        let mut found = false;
        for (i, &mut Some((ref mut worker, ref mut rpc, ref mut listener_ids))) in self.worker_states.iter_mut().enumerate() {
            if listener_ids.contains(id) {
                listener_ids.remove(listener_ids.iter().position(|&x| x == id).unwrap());
                if listener_ids.is_empty() {
                    let mut index = VecDeque::from_iter((0..self.worker_states.len()).rev());
                    self.worker_states[index.pop_front().unwrap()].take().unwrap();
                    worker.terminate();
                    rpc.terminate();
                }
                found = true;
                break;
            }
        }
        if !found {
            panic!("listener not found");
        }
    }
}
```