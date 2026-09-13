pub mod face;
pub mod rotation;

use face::Face;

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
