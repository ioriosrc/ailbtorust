```rust
use std::sync::{Arc, Mutex};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use lodash::Lodash;
use zustand::{create_store, persist, StoreApi};

import { AppSetting } from "@lichtblick/suite-base/AppSetting";
import { AlertsContext } from "@lichtblick/suite-base/context/AlertsContext";
import CurrentLayoutContext, {
  LayoutState,
} from "@lichtblick/suite-base/context/CurrentLayoutContext";
import { useAppConfigurationValue } from "@lichtblick/suite-base/hooks/useAppConfigurationValue";
import { GlobalVariables } from "@lichtblick/suite-base/hooks/useGlobalVariables";
import {
  Player,
  PlayerAlert,
  PlayerState,
  SubscribePayload,
} from "@lichtblick/suite-base/players/types";

use MessageOrderTracker from "./MessageOrderTracker";
use { pauseFrameForPromises, FramePromise } from "./pauseFrameForPromise";
use {
  concat_alerts,
  create_player_listener as createPlayerListener,
};

// Given a PlayerState and a PlayerAlert array, add the alerts to any existing player alerts
fn concat_alerts(orig_state: PlayerState, alerts: Vec<PlayerAlert>) -> PlayerState {
  if alerts.is_empty() {
    return orig_state;
  }

  let mut new_player_state = orig_state.clone();
  new_player_state.alerts.append(&mut alerts);
  return new_player_state;
}

/**
 * The creation of the player listener is extracted as a separate function to prevent memory leaks.
 * When multiple closures are created inside of an outer function, V8 allocates one "context" object
 * to be shared by all the inner closures, holding the shared variables they access. As long as any
 * of the inner closures are still alive, the context and **all** the shared variables stay alive.
 *
 * In the case of MessagePipelineProvider, when the `listener` closure was created directly inside
 * the useEffect above, it would end up retaining a shared context that also retained the player
 * `state` variable returned by `usePlayerState()`, even though the listener closure didn't actually
 * use it. In particular, each time a new player was created in the useEffect, this caused it to
 * retain the old player's state (via the listener closure), creating a "linked list" effect that
 * caused the last state produced by each player (and therefore also its preloaded message blocks)
 * to be retained indefinitely as new data sources were swapped in.
 *
 * To avoid this problem, we extract the closure creation into a module-level function where it
 * won't see variables from outer scopes that are potentially retained in the shared context due to
 * their use in other closures.
 *
 * This type of leak is discussed at:
 * - https://bugs.chromium.org/p/chromium/issues/detail?id=315190
 * - http://point.davidglasser.net/2013/06/27/surprising-javascript-memory-leak.html
 * - https://stackoverflow.com/questions/53985411/understanding-javascript-closure-variable-capture-in-v8
 */
fn create_player_listener(args: {
  ms_per_frame_ref: Arc<Mutex<i32>>;
  promises_to_wait_for_ref: Arc<Mutex<Vec<FramePromise>>>;
  store: StoreApi<MessagePipelineInternalState>;
  clear_alerts: fn();
}) -> Box<dyn Fn(PlayerState) -> Result<(), String>> {
  let ms_per_frame_ref = args.ms_per_frame_ref;
  let promises_to_wait_for_ref = args.promises_to_wait_for_ref;
  let store = Arc::clone(&args.store);
  let clear_alerts = args.clear_alerts;

  return Box::new(move |listener_player_state: PlayerState| {
    if closed.get() {
      return Err("Player has been closed".to_string());
    }

    if resolve_fn.get().is_some() {
      return Err("New playerState was emitted before last playerState was rendered.");
    }

    // check for any out-of-order or out-of-sync messages
    let alerts = message_order_tracker.update(listener_player_state);
    let new_player_state = concat_alerts(listener_player_state, alerts);

    let promise = Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    let start = std::time::Instant::now();

    // Render done is invoked by a layout effect once the component has rendered.
    // After the component renders, we kick off an animation frame to give panels one
    // animation frame to invoke pause.
    let called = false;
    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    let mut closed = false;
    let prev_player_id: Option<String> = None;
    let resolve_fn: Arc<Mutex<Option<fn()>>>;

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn.get_mut().unwrap()();
    });

    Box::new(async move {
      if closed.get() {
        return Err("Player has been closed".to_string());
      }

      let promises_to_wait_for = &mut *promises_to_wait_for_ref.lock().unwrap();
      if !promises_to_wait_for.is_empty() {
        promises_to_wait_for.clear();
        await pause_frame_for_promises(promises_to_wait_for);
      }

      if !resolve_fn.get().is_some() {
        return Err("Player has been closed".to_string());
      }
      resolve_fn