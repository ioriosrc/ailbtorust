```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

type ChartData = dyn std::any::Any;

type ChartDataset<'a> = &'a dyn std::any::Any;
type ChartDatum<'a> = &'a dyn std::any::Any;

type Bounds1D = { min: f64; max: f64 };
type Bounds = {
  x: Bounds1D;
  y: Bounds1D;
};

/**
 * PlotViewport represents the visible region of a plot in terms of its axes
 * and its dimensions on the screen.
 */
struct PlotViewport {
    // the dimensions of the plot in screen space
    width: f64; // px
    height: f64; // px
    // and its axes
    bounds: Bounds;
}

pub struct ProviderState<T> {
    data: ChartData,
    // the bounds of the data contained in the `data` field
    bounds: Bounds,
}
pub type ChartProviderState = ProviderState<ObjectData>;
pub type TypedProviderState = ProviderState<TypedData[]>;

fn provider_state_setter<'a, T>(state: &mut ProviderState<T>) -> &'a mut ProviderState<T> {
    state
}

/**
 * PlotDataProvider gives the user of a TimeBasedChart more granular control
 * over the data the plot displays, including giving it access to the current
 * viewport.
 */
pub struct PlotDataProvider<'a, T> {
    set_view: fn(&mut Self, PlotViewport),
    register: fn(&mut Self, fn(&mut ProviderState<T>), fn(&mut ProviderState<T>)),
}

pub type ObjectDataProvider = PlotDataProvider<ObjectData>;
pub type TypedDataProvider = PlotDataProvider<TypedData[]>;
```