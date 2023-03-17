use netherite_common::macros::EnumFields;
// serde derive...
extern crate serde as _serde;
use serde::{Deserialize, Serialize};

pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(EnumFields)]
#[enum_field(angle: f32, offset_axis: Axis, offset: i32)]
pub enum Direction {
    #[ef(angle = 180f32, offset_axis = Axis::Z, offset = -1)]
    North,
    #[ef(angle = 0f32, offset_axis = Axis::Z, offset = 1)]
    South,
    #[ef(angle = 90f32, offset_axis = Axis::X, offset = -1)]
    West,
    #[ef(angle = -90.0, offset_axis = Axis::X, offset = 1)]
    East,
}

macro vector($($name:ident -> $($field:ident: $field_type:ty),*;)*) {
    $(
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub struct $name {
            $($field: $field_type),+
        }

        impl $name {
            pub const ZERO: Self = Self::new($(0 as $field_type),+);

            pub const fn new($($field: $field_type),+) -> Self {
                Self {
                    $($field),+
                }
            }

            $(
                pub const fn $field(&self) -> $field_type {
                    self.$field
                }
            )+
        }
    )+
}

vector! {
    Vec2f -> x: f32, y: f32;
    Vec3i -> x: i32, y: i32, z: i32;
    Vec3f -> x: f32, y: f32, z: f32;
    Vec4f -> x: f32, y: f32, z: f32, w: f32;
}

impl Vec3i {
    pub fn offset(self, direction: Direction, amount: i32) -> Self {
        let (x, y, z) = (self.x, self.y, self.z);
        let offset = direction.offset() * amount;
        let (x, y, z) = match direction.offset_axis() {
            Axis::X => (x + offset, y, z),
            Axis::Z => (x, y, z + offset),
            _ => unreachable!(),
        };
        Self { x, y, z }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct VecDiff3f {
    dx: f32,
    dy: f32,
    dz: f32,
}

impl VecDiff3f {
    pub const ZERO: Self = Self::new(0f32, 0f32, 0f32);

    pub const fn new(dx: f32, dy: f32, dz: f32) -> Self {
        Self { dx, dy, dz }
    }

    pub const fn dx(&self) -> f32 {
        self.dx
    }
    pub const fn dy(&self) -> f32 {
        self.dy
    }
    pub const fn dz(&self) -> f32 {
        self.dz
    }
}

impl Into<Vec3f> for VecDiff3f {
    fn into(self) -> Vec3f {
        Vec3f {
            x: self.dx,
            y: self.dy,
            z: self.dz,
        }
    }
}

impl From<Vec3f> for VecDiff3f {
    fn from(value: Vec3f) -> Self {
        Self {
            dx: value.x,
            dy: value.y,
            dz: value.z,
        }
    }
}

pub mod vec_diff3f {
    use crate::model::pos::{Vec3f, VecDiff3f};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        value: &Option<Vec3f>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.map(|a| VecDiff3f::from(a)).serialize(serializer)
    }

    pub fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<Vec3f>, D::Error> {
        Option::<VecDiff3f>::deserialize(deserializer).map(|a| a.map(|v| v.into()))
    }
}
