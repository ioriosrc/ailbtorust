```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;

use lightray_rs::msg;
use lightray_rs::msg::MessageEvent;
use lightray_rs::msg::{Frame, Layer};
use lightray_rs::msg::Transform;
use lightray_rs::msg::{TransformStamped, Topic};

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

mod common;
use self::common::*;

const RED = make_color_attribute("#f44336");
const GREEN = make_color_attribute("#4caf50");
const BLUE = make_color_attribute("#2196f3");
const YELLOW = make_color_attribute("#ffeb3b");

const URDF = r#"<?xml version="1.0"?>
<robot name="URDF Test2">
  <joint name="mesh-no-material_T_layer" type="fixed">
    <parent link="mesh-no-material"/>
    <child link="layer"/>
    <origin xyz="1 -2 0"/>
  </joint>
</robot>"#;

const URDF2 = r#"<?xml version="1.0"?>
<robot name="URDF Test">
  <material name="box-material"><color rgba="${RED}"/></material>
  <material name="cylinder-material"><color rgba="${GREEN}"/></material>
  <material name="sphere-material"><color rgba="${BLUE}"/></material>
  <material name="mesh-material"><color rgba="${YELLOW}"/></material>

  <link name="box">
    <visual>
      <geometry><box size="1 0.5 0.025"/></geometry>
      <origin rpy="0 0 0" xyz="0 0 0"/>
      <material name="box-material"/>
    </visual>
  </link>

  <link name="cylinder">
    <visual>
      <geometry><cylinder length="2" radius="0.2"/></geometry>
      <origin rpy="0 1.5708 0" xyz="0 0 0"/>
      <material name="cylinder-material"/>
    </visual>
  </link>

  <joint name="box_T_cylinder" type="fixed">
    <parent link="box"/>
    <child link="cylinder"/>
    <origin rpy="0 0 0.785398" xyz="0 2 0"/>
  </joint>

  <link name="sphere">
    <visual>
      <geometry><sphere radius="0.2"/></geometry>
      <origin rpy="0 0 0" xyz="0 0 0"/>
      <material name="sphere-material"/>
    </visual>
  </link>

  <joint name="cylinder_T_sphere" type="fixed">
    <parent link="cylinder"/>
    <child link="sphere"/>
    <origin rpy="0 0 0" xyz="-0.70710678118 0.70710678118 0"/>
  </joint>

  <link name="mesh">
    <visual>
      <geometry><mesh filename="${STL_CUBE_MESH_RESOURCE}" scale="0.25 0.25 -0.1" /></geometry>
      <origin rpy="1.5708 0 0" xyz="0 0 0"/>
      <material name="mesh-material"/>
    </visual>
  </link>

  <joint name="box_T_mesh" type="fixed">
    <parent link="box"/>
    <child link="mesh"/>
    <origin rpy="0 0 0" xyz="1 1 0"/>
  </joint>

  <link name="mesh-no-material">
    <visual>
      <geometry><mesh filename="${STL_CUBE_MESH_RESOURCE}" scale="0.25 0.25 0.5"/></geometry>
      <origin rpy="0 0 0" xyz="0 0 -0.25"/>
    </visual>
  </link>

  <joint name="mesh_T_mesh-no-material" type="fixed">
    <parent link="mesh"/>
    <child link="mesh-no-material"/>
    <origin rpy="0 0 0" xyz="1 2 0"/>
  </joint>
</robot>"#;

const URDF3 = r#"<?xml version="1.0"?>
<robot name="URDF Test3">
  <material name="base-sphere-material"><color rgba="${BLUE}"/></material>
  <material name="sphere-material"><color rgba="${RED}"/></material>
  <link name="base_link">
    <visual>
      <geometry><sphere radius="0.2"/></geometry>
      <material name="base-sphere-material"/>
    </visual>
  </link>
  <joint name="base_sphere_box_joint" type="fixed">
    <parent link="base_link"/>
    <child link="sphere_link"/>
    <origin rpy="0 0 0" xyz="0 0 0.3"/>
  </joint>
  <link name="sphere_link">
    <visual>
      <geometry><sphere radius="0.1"/></geometry>
      <material name="sphere-material"/>
    </visual>
  </link>
</robot>"#;

pub fn make_color_attribute(hex: &str, alpha: f32) -> String {
  let c = make_color(hex, alpha);
  format!("{}, {}, {}, {}", c.red, c.green, c.blue, c.alpha)
}

mod common;
use self::common::*;

const RED = make_color_attribute("#f44336");
const GREEN = make_color_attribute("#4caf50");
const BLUE = make_color_attribute("#2196f3");
const YELLOW = make_color_attribute("#ffeb3b");

const URDF = r#"<?xml version="1.0"?>
<robot name="URDF Test2">
  <joint name="mesh-no-material_T_layer" type="fixed">
    <parent link="mesh-no-material"/>
    <child link="layer"/>
    <origin xyz="1 -2 0"/>
  </joint>
</robot>"#;

const URDF2 = r#"<?xml version="1.0"?>
<robot name="URDF Test">
  <material name="box-material"><color rgba="${RED}"/></material>
  <material name="cylinder-material"><color rgba="${GREEN}"/></material>
  <material name="sphere-material"><color rgba="${BLUE}"/></material>
  <material name="mesh-material"><color rgba="${YELLOW}"/></material>

  <link name="box">
    <visual>
      <geometry><box size="1 0.5 0.025"/></geometry>
      <origin rpy="0 0 0" xyz="0 0 0"/>
      <material name="box-material"/>
    </visual>
  </link>

  <link name="cylinder">
    <visual>
      <geometry><cylinder length="2" radius="0.2"/></geometry>
      <origin rpy="0 1.5708 0" xyz="0 0 0"/>
      <material name="cylinder-material"/>
    </visual>
  </link>

  <joint name="box_T_cylinder" type="fixed">
    <parent link="box"/>
    <child link="cylinder"/>
    <origin rpy="0 0 0.785398" xyz="0 2 0"/>
  </joint>

  <link name="sphere">
    <visual>
      <geometry><sphere radius="0.2"/></geometry>
      <origin rpy="0 0 0" xyz="0 0 0"/>
      <material name="sphere-material"/>
    </visual>
  </link>

  <joint name="cylinder_T_sphere" type="fixed">
    <parent link="cylinder"/>
    <child link="sphere"/>
    <origin rpy="0 0 0" xyz="-0.70710678118 0.70710678118 0"/>
  </joint>

  <link name="mesh">
    <visual>
      <geometry><mesh filename="${STL_CUBE_MESH_RESOURCE}" scale="0.25 0.25 -0.1" /></geometry>
      <origin rpy="1.5708 0 0" xyz="0 0 0"/>
      <material name="mesh-material"/>
    </visual>
  </link>

  <joint name="box_T_mesh" type="fixed">
    <parent link="box"/>
    <child link="mesh"/>
    <origin rpy="0 0 0" xyz="1 1 0"/>
  </joint>

  <link name="mesh-no-material">
    <visual>
      <geometry><mesh filename="${STL_CUBE_MESH_RESOURCE}" scale="0.25 0.25 0.5"/></geometry>
      <origin rpy="0 0 0" xyz="0 0 -0.25"/>
    </visual>
  </link>

  <joint name="mesh_T_mesh-no-material" type="fixed">
    <parent link="mesh"/>
    <child link="mesh-no-material"/>
    <origin rpy="0 0 0" xyz="1 2 0"/>
  </joint>
</robot>"#;

const URDF3 = r#"<?xml version="1.0"?>
<robot name="URDF Test3">
  <material name="base-sphere-material"><color rgba="${BLUE}"/></material>
  <material name="sphere-material"><color rgba="${RED}"/></material>
  <link name="base_link">
    <visual>
      <geometry><sphere radius="0.2"/></geometry>
      <material name="base-sphere-material"/>
    </visual>
  </link>
  <joint name="base_sphere_box_joint" type="fixed">
    <parent link="base_link"/>
    <child link="sphere_link"/>
    <origin rpy="0 0 0" xyz="0 0 0.3"/>
  </joint>
  <link name="sphere_link">
    <visual>
      <geometry><sphere radius="0.1"/></geometry>
      <material name="sphere-material"/>
    </visual>
  </link>
</robot>"#;

pub fn make_color_attribute(hex: &str, alpha: f32) -> String {
  let c = make_color(hex, alpha);
  format!("{}, {}, {}, {}", c.red, c.green, c.blue, c.alpha)
}
```