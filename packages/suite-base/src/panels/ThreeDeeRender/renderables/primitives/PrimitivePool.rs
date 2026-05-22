```rust
use std::collections::{HashMap, VecDeque};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

mod renderable_arrows;
mod renderable_cubes;
mod renderable_cylinders;
mod renderable_lines;
mod renderable_models;
mod renderable_primitive;
mod renderable_spheres;
mod renderable_texts;
mod renderable_triangles;

use renderable_cubes::RenderableCubes;
use renderable_models::RenderableModels;
use renderable_lines::RenderableLines;
use renderable_cylinders::RenderableCylinders;
use renderable_arrows::RenderableArrows;
use renderable_spheres::RenderableSpheres;
use renderable_texts::RenderableTexts;
use renderable_triangles::RenderableTriangles;
use super::{PrimitiveType, IRenderer};

const CONSTRUCTORS: HashMap<PrimitiveType, fn(IRenderer) -> Box<dyn RenderablePrimitive>> = [
    (PrimitiveType::CUBES, |r| Box::new(RenderableCubes::new(r))),
    (PrimitiveType::MODELS, |r| Box::new(RenderableModels::new(r))),
    (PrimitiveType::LINES, |r| Box::new(RenderableLines::new(r))),
    (PrimitiveType::CYLINDERS, |r| Box::new(RenderableCylinders::new(r))),
    (PrimitiveType::ARROWS, |r| Box::new(RenderableArrows::new(r))),
    (PrimitiveType::SPHERES, |r| Box::new(RenderableSpheres::new(r))),
    (PrimitiveType::TEXTS, |r| Box::new(RenderableTexts::new(r))),
    (PrimitiveType::TRIANGLES, |r| Box::new(RenderableTriangles::new(r))),
];

/**
 * An object pool for RenderablePrimitive subclass objects.
 */
pub struct PrimitivePool {
    primitives_by_type: HashMap<PrimitiveType, VecDeque<Box<dyn RenderablePrimitive>>>,
    disposed: bool;

    pub fn new(renderer: IRenderer) -> Self {
        Self {
            primitives_by_type: HashMap::new(),
            disposed: false,
            renderer,
        }
    }

    pub fn acquire<T: PrimitiveType>(&mut self) -> Box<dyn RenderablePrimitive> {
        if self.disposed {
            panic!("Attempt to acquire PrimitiveType::{T} after PrimitivePool was disposed");
        }
        let primitive = self.primitives_by_type.get_mut(&T).unwrap().pop_front();
        if primitive.is_none() {
            // https://github.com/microsoft/TypeScript/issues/44049
            return Box::new(RenderablePrimitive::new(self.renderer));
        } else {
            primitive.unwrap()
        }
    }

    pub fn release<T: PrimitiveType>(&mut self, primitive: Box<dyn RenderablePrimitive>) {
        if self.disposed {
            primitive.dispose();
            return;
        }
        let primitives = self.primitives_by_type.entry(T).or_insert_with(|| VecDeque::new());
        primitives.push_back(primitive);
    }

    pub fn dispose(&mut self) {
        for primitives in self.primitives_by_type.values_mut() {
            primitives.clear();
        }
        self.primitives_by_type.clear();
        self.disposed = true;
    }

    pub fn set_color_scheme(&mut self, color_scheme: &str) {
        for primitives in self.primitives_by_type.values_mut() {
            for primitive in primitives {
                primitive.set_color_scheme(color_scheme);
            }
        }
    }
}
```