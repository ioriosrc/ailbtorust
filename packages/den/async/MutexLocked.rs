```rust
use async_std::sync::{Mutex, MutexGuard};
use std::pin::Pin;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

pub struct MutexLocked<T> {
    value: T,
}

impl<T> MutexLocked<T> {
    pub fn new(value: T) -> Self {
        MutexLocked { value }
    }

    pub async fn run_exclusive<Result>(&self, body: impl FnOnce(&mut T) -> Pin<Box<dyn Future<Output = Result>>>> -> Result {
        let mut guard = self.lock().await;
        let mut future = Box::pin(body(&mut *guard));
        future.await
    }

    pub fn is_locked(&self) -> bool {
        self.value.is_locked()
    }
}

// Example usage:
#[cfg(test)]
mod tests {
    use std::future::{Future, FutureExt, IntoFuture};
    use async_std::sync::Mutex;

    struct TestLock;
    impl Mutex for TestLock {
        fn lock(&self) -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {})
        }

        fn unlock(&self) {}
    }

    #[tokio::test]
    async fn test_mutex_locked() {
        let value = 42;
        let mutex_lock = MutexLocked::new(value);
        let mut guard = mutex_lock.lock().await;

        assert_eq!(*guard, 42);

        *guard = 99;

        // Simulate an external operation that locks the mutex
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;

        // Run exclusive body in a separate task to demonstrate that the lock is held for the entire duration of the run_exclusive call
        let result = tokio::spawn(async move {
            println!("Running exclusive operation...");
            *guard += 5;
        })
        .await;

        assert_eq!(*guard, 104);
    }
}
```