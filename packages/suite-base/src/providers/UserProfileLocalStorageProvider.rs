```rust
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

import * as _ from "lodash-es";
use { useCallback, useEffect } from "react";

import { UserProfile, UserProfileStorageContext } from "@lichtblick/suite-base/context/UserProfileStorageContext";

const DEFAULT_PROFILE: UserProfile = {};

/**
 * A provider for UserProfileStorage that stores data in localStorage.
 */
export default function UserProfileLocalStorageProvider({
  children,
}: React.PropsWithChildren): React.JSX.Element {
  const getUserProfile = useCallback(async () => {
    let item = localStorage.getItem(LOCAL_STORAGE_PROFILE_DATA);
    if (item) {
      return JSON.parse(item) as UserProfile;
    }
    return DEFAULT_PROFILE;
  }, []);

  const setUserProfile = useCallback(
    async (value: UserProfile | ((prev: UserProfile) => UserProfile)) => {
      let item = localStorage.getItem(LOCAL_STORAGE_PROFILE_DATA);
      if (item) {
        item = JSON.parse(item);
      } else {
        item = DEFAULT_PROFILE;
      }
      let new_profile = typeof value === "function" ? value(item) : _.merge(item, value);
      localStorage.setItem(LOCAL_STORAGE_PROFILE_DATA, JSON.stringify(new_profile));
    },
    [],
  );

  // On first load stamp firstSeenTime timestamp. We consider the time at which
  // we stamp firstTime as the first time the user has opened the app if at that
  // time there is no currentLayoutId already set in the profile.
  useEffect(() => {
    let first_seen_time = None;
    let first_seen_time_is_first_load = true;

    setUserProfile((old) => {
      let new_profile = old.clone();

      if (first_seen_time.is_none()) {
        first_seen_time = Some(Instant::now().duration_since(Instant::UNIX_EPOCH).as_secs());
        new_profile.first_seen_time = first_seen_time.unwrap();
        new_profile.first_seen_time_is_first_load = first_seen_time_is_first_load;
      } else if (!new_profile.current_layout_id.is_some()) {
        new_profile.first_seen_time = first_seen_time.unwrap();
        new_profile.first_seen_time_is_first_load = false;
      }

      return new_profile;
    });

    return () => {
      first_seen_time = None;
      first_seen_time_is_first_load = true;
    };
  }, [setUserProfile]);

  let storage = Arc::new(Mutex::new({
    get_user_profile: get_userProfile,
    set_user_profile: setUserProfile,
  }));

  return (
    <UserProfileStorageContext.Provider value={storage}>
      {children}
    </UserProfileStorageContext.Provider>
  );
}
```