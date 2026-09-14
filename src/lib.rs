
pub mod face;
pub mod ffi;
pub mod rotation;

pub use face::{Face, FaceIter};
pub use rotation::{Rotation, RotationIter};

pub use Face::*;

pub trait AxisOrientation {
    const UP: Face;
    const RIGHT: Face;
    const FORWARD: Face;

    const DOWN: Face = Self::UP.invert();
    const LEFT: Face = Self::RIGHT.invert();
    const BACKWARD: Face = Self::FORWARD.invert();
}

macro_rules! marker_type {
    ($($name:ident),+$(,)?) => {
        $(
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub enum $name {}

            impl std::fmt::Debug for $name {
                fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Ok(())
                }
            }
            
            impl std::hash::Hash for $name {
                fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
                fn hash_slice<H: std::hash::Hasher>(_: &[Self], _: &mut H) where Self: Sized {}
            }
        )*
    };
}

macro_rules! axis_orientation {
    ($($name:ident { up: $up:expr, right: $right:expr, forward: $forward:expr $(,)? }),+$(,)?) => {
        $(
            marker_type!($name);

            impl AxisOrientation for $name {
                const UP: Face = $up;
                const RIGHT: Face = $right;
                const FORWARD: Face = $forward;
            }
        )*
    };
}

axis_orientation!(
    // [https://dev.epicgames.com/documentation/unreal-engine/coordinate-system-and-spaces-in-unreal-engine]
    Unreal {
        up: PosZ,
        right: PosY,
        forward: PosX,
    },
    // [https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#coordinate-system-and-units]
    Gltf {
        up: PosY,
        right: NegX,
        forward: PosZ,
    },
    LeftHandYUp {
        up: PosY,
        right: PosX,
        forward: PosZ,
    },
    RightHandYUp {
        up: PosY,
        right: PosX,
        forward: NegZ,
    },
    LeftHandZUp {
        up: PosZ,
        right: PosX,
        forward: NegY,
    },
    RightHandZUp {
        up: PosZ,
        right: PosX,
        forward: PosY,
    }
);

// [https://docs.unity3d.com/6000.3/Documentation/ScriptReference/Vector3.html]
pub type Unity = LeftHandYUp;
// [https://docs.godotengine.org/en/stable/tutorials/assets_pipeline/importing_3d_scenes/model_export_considerations.html]
// [https://docs.godotengine.org/en/4.4/classes/class_vector3.html]
pub type Godot = RightHandYUp;
// [https://learnopengl.com/Getting-started/Coordinate-Systems]
pub type OpenGL = RightHandYUp;
// [https://learn.microsoft.com/en-us/windows/win32/direct3d9/coordinate-systems]
pub type DirectX = LeftHandYUp;

macro_rules! check_axes_features {
    ($(
        $on:literal != [$($off:literal),*$(,)?]
    ),+$(,)?) => {
        $(
            $(
                #[cfg(all(
                    feature = $on,
                    feature = $off,
                ))]
                compile_error!(concat!(
                    stringify!($on),
                    " feature conflicts with ",
                    stringify!($off),
                    " feature.",
                ));
            )*
        )*
    };
}

check_axes_features!(
    "neg_x_up" != [
        "neg_y_up", "neg_z_up", "pos_x_up", "pos_y_up", "pos_z_up",
        "neg_x_right", "pos_x_right",
        "neg_x_forward", "pos_x_forward"
    ],
    "neg_x_right" != [
        "neg_y_right", "neg_z_right", "pos_x_right", "pos_y_right", "pos_z_right",
        "pos_x_up",
        "neg_x_forward", "pos_x_forward"
    ],
    "neg_x_forward" != [
        "neg_y_forward", "neg_z_forward", "pos_x_forward", "pos_y_forward", "pos_z_forward",
        "pos_x_up",
        "pos_x_right"
    ],
    "neg_y_up" != [
        "neg_z_up", "pos_x_up", "pos_y_up", "pos_z_up",
        "neg_y_right", "pos_y_right",
        "neg_y_forward", "pos_y_forward"
    ],
    "neg_y_right" != [
        "neg_z_right", "pos_x_right", "pos_y_right", "pos_z_right",
        "pos_y_up",
        "neg_y_forward", "pos_y_forward"
    ],
    "neg_y_forward" != [
        "neg_z_forward", "pos_x_forward", "pos_y_forward", "pos_z_forward",
        "pos_y_up",
        "pos_y_right"
    ],
    "neg_z_up" != [
        "pos_x_up", "pos_y_up", "pos_z_up",
        "neg_z_right", "pos_z_right",
        "neg_z_forward", "pos_z_forward"
    ],
    "neg_z_right" != [
        "pos_x_right", "pos_y_right", "pos_z_right",
        "pos_z_up",
        "neg_z_forward", "pos_z_forward"
    ],
    "neg_z_forward" != [
        "pos_x_forward", "pos_y_forward", "pos_z_forward",
        "pos_z_up",
        "pos_z_right"
    ],
    "pos_x_up" != [
        "pos_y_up", "pos_z_up",
        "pos_x_right",
        "pos_x_forward"
    ],
    "pos_x_right" != [
        "pos_y_right", "pos_z_right",
        "pos_x_forward"
    ],
    "pos_x_forward" != [
        "pos_y_forward", "pos_z_forward"
    ],
    "pos_y_up" != [
        "pos_z_up",
        "pos_y_right",
        "pos_y_forward"
    ],
    "pos_y_right" != [
        "pos_z_right",
        "pos_y_forward"
    ],
    "pos_y_forward" != [
        "pos_z_forward"
    ],
    "pos_z_up" != [
        "pos_z_right",
        "pos_z_forward"
    ],
    "pos_z_right" != [
        "pos_z_forward"
    ],
);

#[cfg(not(any(
    feature = "neg_x_up",
    feature = "neg_y_up",
    feature = "neg_z_up",
    feature = "pos_x_up",
    feature = "pos_y_up",
    feature = "pos_z_up",
)))]
compile_error!("Must have up direction feature.");
#[cfg(not(any(
    feature = "neg_x_right",
    feature = "neg_y_right",
    feature = "neg_z_right",
    feature = "pos_x_right",
    feature = "pos_y_right",
    feature = "pos_z_right",
)))]
compile_error!("Must have right direction feature.");
#[cfg(not(any(
    feature = "neg_x_forward",
    feature = "neg_y_forward",
    feature = "neg_z_forward",
    feature = "pos_x_forward",
    feature = "pos_y_forward",
    feature = "pos_z_forward",
)))]
compile_error!("Must have forward direction feature.");
