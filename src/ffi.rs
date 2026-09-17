
use crate::{
    Face,
    FaceIter,
    Rot,
    RotIter,
    CartesianRotIter,
    FaceTable,
    axial_face_table,
};

macro_rules! lambda {
    (
        $(
            $(#[$attr:meta])*
            fn $fn_name:ident($($arg_name:ident : $arg_type:ty),*$(,)?) $(-> $ret_type:ty)? => $stmt:stmt
        ),+
        $(,)?
    ) => {
        $(
            $(#[$attr])*
            #[unsafe(no_mangle)]
            pub extern "C" fn $fn_name($($arg_name : $arg_type),*) $(-> $ret_type)? {
                $stmt
            }
        )*
    };
}

#[unsafe(no_mangle)]
pub static SYM_FACE_POS_X: Face = Face::PosX;
#[unsafe(no_mangle)]
pub static SYM_FACE_POS_Y: Face = Face::PosY;
#[unsafe(no_mangle)]
pub static SYM_FACE_POS_Z: Face = Face::PosZ;
#[unsafe(no_mangle)]
pub static SYM_FACE_NEG_X: Face = Face::NegX;
#[unsafe(no_mangle)]
pub static SYM_FACE_NEG_Y: Face = Face::NegY;
#[unsafe(no_mangle)]
pub static SYM_FACE_NEG_Z: Face = Face::NegZ;

#[unsafe(no_mangle)]
pub static SYM_FACE_UP: Face = Face::UP;
#[unsafe(no_mangle)]
pub static SYM_FACE_FORWAD: Face = Face::FORWARD;
#[unsafe(no_mangle)]
pub static SYM_FACE_LEFT: Face = Face::LEFT;
#[unsafe(no_mangle)]
pub static SYM_FACE_BACKWARD: Face = Face::BACKWARD;
#[unsafe(no_mangle)]
pub static SYM_FACE_RIGHT: Face = Face::RIGHT;
#[unsafe(no_mangle)]
pub static SYM_FACE_DOWN: Face = Face::DOWN;

#[unsafe(no_mangle)]
pub static SYM_ROT_IDENTITY: Rot = Rot::IDENTITY;
#[unsafe(no_mangle)]
pub static SYM_ROT_MIN: Rot = Rot::MIN;
#[unsafe(no_mangle)]
pub static SYM_ROT_MAX: Rot = Rot::MAX;

#[unsafe(no_mangle)]
pub static SYM_ROT_NEG_X: Rot = Rot::NEG_X;
#[unsafe(no_mangle)]
pub static SYM_ROT_NEG_Y: Rot = Rot::NEG_Y;
#[unsafe(no_mangle)]
pub static SYM_ROT_NEG_Z: Rot = Rot::NEG_Z;
#[unsafe(no_mangle)]
pub static SYM_ROT_POS_X: Rot = Rot::POS_X;
#[unsafe(no_mangle)]
pub static SYM_ROT_POS_Y: Rot = Rot::POS_Y;
#[unsafe(no_mangle)]
pub static SYM_ROT_POS_Z: Rot = Rot::POS_Z;

#[unsafe(no_mangle)]
pub static SYM_ROT_UP: Rot = Rot::UP;
#[unsafe(no_mangle)]
pub static SYM_ROT_FORWARD: Rot = Rot::FORWARD;
#[unsafe(no_mangle)]
pub static SYM_ROT_LEFT: Rot = Rot::LEFT;
#[unsafe(no_mangle)]
pub static SYM_ROT_BACKWARD: Rot = Rot::BACKWARD;
#[unsafe(no_mangle)]
pub static SYM_ROT_RIGHT: Rot = Rot::RIGHT;
#[unsafe(no_mangle)]
pub static SYM_ROT_DOWN: Rot = Rot::DOWN;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UpAngle {
    pub up: Face,
    pub angle: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FFIVec3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> FFIVec3<T> {
    #[must_use]
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_array([x, y, z]: [T; 3]) -> Self {
        Self { x, y, z }
    }
}

lambda!{
    /// Obtain the default [Face]. This is likely to be the Up face, unless
    /// I changed it and forgot to update this documentation (unlikely).
    fn sym_face_default() -> Face
        => Face::default(),
    /// Create a Cayley (lookup) table for [Face] values that can be indexed with [Face].
    fn sym_face_cayley(
        neg_x: Face,
        neg_y: Face,
        neg_z: Face,
        pos_x: Face,
        pos_y: Face,
        pos_z: Face,
    ) -> FaceTable<Face>
        => axial_face_table(neg_x, neg_y, neg_z, pos_x, pos_y, pos_z),
    /// Create a Cayley (lookup) table for [Rot] values that can be indexed with [Face].
    fn sym_rot_face_cayley(
        neg_x: Rot,
        neg_y: Rot,
        neg_z: Rot,
        pos_x: Rot,
        pos_y: Rot,
        pos_z: Rot,
    ) -> FaceTable<Rot>
        => axial_face_table(neg_x, neg_y, neg_z, pos_x, pos_y, pos_z),
    /// Get a [Face] from a [FaceCayley] (lookup) table.
    fn sym_face_cayley_get(cayley: FaceTable<Face>, face: Face) -> Face
        => cayley.get(face),
    /// Get a [Rot] from a [FaceCayley] (lookup) table.
    fn sym_rot_face_cayley_get(cayley: FaceTable<Face>, face: Face) -> Face
        => cayley.get(face),
    /// Returns [Face::NegX].
    fn sym_face_neg_x() -> Face
        => Face::NegX,
    /// Returns [Face::NegY].
    fn sym_face_neg_y() -> Face
        => Face::NegY,
    /// Returns [Face::NegZ].
    fn sym_face_neg_z() -> Face
        => Face::NegZ,
    /// Returns [Face::PosX].
    fn sym_face_pos_x() -> Face
        => Face::PosX,
    /// Returns [Face::PosY].
    fn sym_face_pos_y() -> Face
        => Face::PosY,
    /// Returns [Face::PosZ].
    fn sym_face_pos_z() -> Face
        => Face::PosZ,
    /// Returns [Face::UP].
    fn sym_face_up() -> Face
        => Face::UP,
    /// Returns [Face::FORWARD].
    fn sym_face_forward() -> Face
        => Face::FORWARD,
    /// Returns [Face::LEFT].
    fn sym_face_left() -> Face
        => Face::LEFT,
    /// Returns [Face::BACKWARD].
    fn sym_face_backward() -> Face
        => Face::BACKWARD,
    /// Returns [Face::RIGHT].
    fn sym_face_right() -> Face
        => Face::RIGHT,
    /// Returns [Face::DOWN].
    fn sym_face_down() -> Face
        => Face::DOWN,
    /// Returns the Up [Face] of a [Face]. That is, the face that points in the same direction as the `up` direction of the [Face].
    fn sym_face_up_dir(face: Face) -> Face
        => face.up(),
    /// Returns the Up [Face] of a [Face] at the given `angle`. That is, the face that points in the same direction as the `up` direction of the [Face].
    fn sym_face_up_dir_at_angle(face: Face, angle: i8) -> Face
        => face.up_at_angle(angle),
    /// Returns the Left [Face] of a [Face]. That is, the face that points in the same direction as the `left` direction of the [Face].
    fn sym_face_left_dir(face: Face) -> Face
        => face.left(),
    /// Returns the Left [Face] of a [Face] at the given `angle`. That is, the face that points in the same direction as the `left` direction of the [Face].
    fn sym_face_left_dir_at_angle(face: Face, angle: i8) -> Face
        => face.left_at_angle(angle),
    /// Returns the Down [Face] of a [Face]. That is, the face that points in the same direction as the `down` direction of the [Face].
    fn sym_face_down_dir(face: Face) -> Face
        => face.down(),
    /// Returns the Down [Face] of a [Face] at the given `angle`. That is, the face that points in the same direction as the `down` direction of the [Face].
    fn sym_face_down_dir_at_angle(face: Face, angle: i8) -> Face
        => face.down_at_angle(angle),
    /// Returns the Right [Face] of a [Face]. That is, the face that points in the same direction as the `right` direction of the [Face].
    fn sym_face_right_dir(face: Face) -> Face
        => face.right(),
    /// Returns the Right [Face] of a [Face] at the given `angle`. That is, the face that points in the same direction as the `right` direction of the [Face].
    fn sym_face_right_dir_at_angle(face: Face, angle: i8) -> Face
        => face.right_at_angle(angle),
    /// Inverts a [Face].
    fn sym_face_invert(face: Face) -> Face
        => face.invert(),
    /// Inverts the X axis of a [Face].
    fn sym_face_invert_x(face: Face) -> Face
        => face.invert_x(),
    /// Inverts the Y axis of a [Face].
    fn sym_face_invert_y(face: Face) -> Face
        => face.invert_y(),
    /// Inverts the Z axis of a [Face].
    fn sym_face_invert_z(face: Face) -> Face
        => face.invert_z(),
    /// Inverts the X and Y axes of a [Face].
    fn sym_face_invert_xy(face: Face) -> Face
        => face.invert_xy(),
    /// Inverts the X and Z axes of a [Face].
    fn sym_face_invert_xz(face: Face) -> Face
        => face.invert_xz(),
    /// Inverts the Y and Z axes of a [Face].
    fn sym_face_invert_yz(face: Face) -> Face
        => face.invert_yz(),
    /// Create a [Face] iterator.
    fn sym_face_iter() -> FaceIter
        => FaceIter::new(),
    /// Check if the `lhs` [Face] is equal to the `rhs` [Face].
    fn sym_face_eq(lhs: Face, rhs: Face) -> bool => lhs.eq(rhs),
    /// Check if the `lhs` [Face] is not equal to the `rhs` [Face].
    fn sym_face_ne(lhs: Face, rhs: Face) -> bool => lhs.ne(rhs),

    /// Get the normal for the given [Face].
    fn sym_face_to_coord_i8(face: Face) -> FFIVec3<i8>
        => FFIVec3::from_array(face.to_coord_i8()),
    /// Get the normal for the given [Face].
    fn sym_face_to_coord_i16(face: Face) -> FFIVec3<i16>
        => FFIVec3::from_array(face.to_coord_i16()),
    /// Get the normal for the given [Face].
    fn sym_face_to_coord_i32(face: Face) -> FFIVec3<i32>
        => FFIVec3::from_array(face.to_coord_i32()),
    /// Get the normal for the given [Face].
    fn sym_face_to_coord_i64(face: Face) -> FFIVec3<i64>
        => FFIVec3::from_array(face.to_coord_i64()),
    /// Get the normal for the given [Face].
    fn sym_face_to_coord_f32(face: Face) -> FFIVec3<f32>
        => FFIVec3::from_array(face.to_coord_f32()),
    /// Get the normal for the given [Face].
    fn sym_face_to_coord_f64(face: Face) -> FFIVec3<f64>
        => FFIVec3::from_array(face.to_coord_f64()),

    // --- Rot ---

    /// Get [Rot::IDENTITY].
    fn sym_rot_identity() -> Rot => Rot::IDENTITY,
    /// Get default [Rot] value, which is equal to [Rot::IDENTITY].
    fn sym_rot_default() -> Rot => Rot::IDENTITY,
    /// Get the minimum [Rot] value (the [Rot] with the lowest integral representation).
    fn sym_rot_min() -> Rot => Rot::MIN,
    /// Get the maximum [Rot] value (the [Rot] with the highest integral representation).
    fn sym_rot_max() -> Rot => Rot::MAX,

    /// Create a [Rot] from the given `up` [Face] and `angle` around that [Face].
    fn sym_rot_new(up: Face, angle: i8) -> Rot
        => Rot::new(up, angle),
    /// Create a [Rot] from the given `up` [Face] with an angle of 0.
    fn sym_rot_from_up(up: Face) -> Rot
        => Rot::from_up(up),
    /// Create a new version of `rot` but with the given `up` [Face].
    fn sym_rot_with_up(rot: Rot, up: Face) -> Rot
        => rot.with_up(up),
    /// Create a new version of `rot` but with the given `angle`.
    fn sym_rot_with_angle(rot: Rot, angle: i8) -> Rot
        => rot.with_angle(angle),

    /// Get the destination of [Face::NegX] after rotation.
    fn sym_rot_neg_x_dest(rot: Rot) -> Face => rot.neg_x_dest(),
    /// Get the destination of [Face::NegY] after rotation.
    fn sym_rot_neg_y_dest(rot: Rot) -> Face => rot.neg_y_dest(),
    /// Get the destination of [Face::NegZ] after rotation.
    fn sym_rot_neg_z_dest(rot: Rot) -> Face => rot.neg_z_dest(),
    /// Get the destination of [Face::PosX] after rotation.
    fn sym_rot_pos_x_dest(rot: Rot) -> Face => rot.pos_x_dest(),
    /// Get the destination of [Face::PosY] after rotation.
    fn sym_rot_pos_y_dest(rot: Rot) -> Face => rot.pos_y_dest(),
    /// Get the destination of [Face::PosZ] after rotation.
    fn sym_rot_pos_z_dest(rot: Rot) -> Face => rot.pos_z_dest(),

    /// Get the source of [Face::NegX] before rotation.
    fn sym_rot_neg_x_src(rot: Rot) -> Face => rot.neg_x_src(),
    /// Get the source of [Face::NegY] before rotation.
    fn sym_rot_neg_y_src(rot: Rot) -> Face => rot.neg_y_src(),
    /// Get the source of [Face::NegZ] before rotation.
    fn sym_rot_neg_z_src(rot: Rot) -> Face => rot.neg_z_src(),
    /// Get the source of [Face::PosX] before rotation.
    fn sym_rot_pos_x_src(rot: Rot) -> Face => rot.pos_x_src(),
    /// Get the source of [Face::PosY] before rotation.
    fn sym_rot_pos_y_src(rot: Rot) -> Face => rot.pos_y_src(),
    /// Get the source of [Face::PosZ] before rotation.
    fn sym_rot_pos_z_src(rot: Rot) -> Face => rot.pos_z_src(),

    /// Get the destination of [Face::UP] after rotation.
    fn sym_rot_up(rot: Rot) -> Face           => rot.up(),
    /// Get the source of [Face::UP] before rotation.
    fn sym_rot_up_src(rot: Rot) -> Face       => rot.up_src(),
    /// Get the destination of [Face::DOWN] after rotation.
    fn sym_rot_down(rot: Rot) -> Face         => rot.down(),
    /// Get the source of [Face::DOWN] before rotation.
    fn sym_rot_down_src(rot: Rot) -> Face     => rot.down_src(),
    /// Get the destination of [Face::LEFT] after rotation.
    fn sym_rot_left(rot: Rot) -> Face         => rot.left(),
    /// Get the source of [Face::LEFT] before rotation.
    fn sym_rot_left_src(rot: Rot) -> Face     => rot.left_src(),
    /// Get the destination of [Face::RIGHT] after rotation.
    fn sym_rot_right(rot: Rot) -> Face        => rot.right(),
    /// Get the source of [Face::RIGHT] before rotation.
    fn sym_rot_right_src(rot: Rot) -> Face    => rot.right_src(),
    /// Get the destination of [Face::FORWARD] after rotation.
    fn sym_rot_forward(rot: Rot) -> Face      => rot.forward(),
    /// Get the source of [Face::FORWARD] before rotation.
    fn sym_rot_forward_src(rot: Rot) -> Face  => rot.forward_src(),
    /// Get the destination of [Face::BACKWARD] after rotation.
    fn sym_rot_backward(rot: Rot) -> Face     => rot.backward(),
    /// Get the source of [Face::BACKWARD] before rotation.
    fn sym_rot_backward_src(rot: Rot) -> Face => rot.backward_src(),

    /// Get the `angle` of a [Rot].
    fn sym_rot_angle(rot: Rot) -> i8
        => rot.angle(),

    /// Get the destination of the given [Face] after rotation with the given [Rot].
    fn sym_rot_face_dest(rot: Rot, face: Face) -> Face
        => rot.face_dest(face),
    /// Get the source of the given [Face] before rotation with the given [Rot].
    fn sym_rot_face_src(rot: Rot, face: Face) -> Face
        => rot.face_src(face),

    /// Rotate the `target` by the given `rotation`.
    fn sym_rot_rotate_by(target: Rot, rotation: Rot) -> Rot
        => target.rotate_by(rotation),
    /// Rotate the `target` by the inverse of the given `rotation`.
    fn sym_rot_rotate_by_inverse(target: Rot, rotation: Rot) -> Rot
        => target.rotate_by_inverse(rotation),
    /// Rotate the `target` locally by the given `rotation`.
    fn sym_rot_local_rotate_by(target: Rot, rotation: Rot) -> Rot
        => target.local_rotate_by(rotation),
    /// Rotate the `target` locally by the inverse of the given `rotation`.
    fn sym_rot_local_rotate_by_inverse(target: Rot, rotation: Rot) -> Rot
        => target.local_rotate_by_inverse(rotation),

    /// Return the inverse of `rot`.
    fn sym_rot_invert(rot: Rot) -> Rot
        => rot.invert(),
    /// Return the angle of the given `face` after rotation with the given `rot`.
    fn sym_rot_face_angle(rot: Rot, face: Face) -> i8
        => rot.face_angle(face),

    /// Get the difference between `lhs` and `rhs`.
    ///
    /// `lhs` can then be rotated by the given difference value to create `rhs`.
    fn sym_rot_diff(lhs: Rot, rhs: Rot) -> Rot
        => lhs.diff(rhs),
    /// Get the conjugation of `lhs` and `rhs`.
    fn sym_rot_conjugate(lhs: Rot, rhs: Rot) -> Rot
        => lhs.conjugate(rhs),

    fn sym_rot_iter() -> RotIter
        => RotIter::new(),

    fn sym_rot_eq(lhs: Rot, rhs: Rot) -> bool
        => lhs.eq(rhs),
    fn sym_rot_ne(lhs: Rot, rhs: Rot) -> bool
        => lhs.ne(rhs),
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_iter_next(iter: &mut FaceIter, out: Option<&mut Face>) -> bool {
    match iter.next() {
        Some(next) => {
            if let Some(out) = out {
                *out = next;
            }
            true
        }
        None => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_iter_next(iter: &mut RotIter, out: Option<&mut Rot>) -> bool {
    match iter.next() {
        Some(next) => {
            if let Some(out) = out {
                *out = next;
            }
            true
        }
        None => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_up_angle(rot: Rot) -> UpAngle {
    let (up, angle) = rot.up_angle();
    UpAngle { up, angle }
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_from_up_and_forward(up: Face, forward: Face, out: Option<&mut Rot>) -> bool {
    if let Some(rot) = Rot::from_up_and_forward(up, forward) {
        if let Some(out) = out {
            *out = rot;
        }
        true
    } else {
        false
    }
}

macro_rules! rot_cartesian_product_funcs {
    ($(
        $n:literal
    ),+$(,)?) => {
        paste::paste!{
            $(
                #[unsafe(no_mangle)]
                pub extern "C" fn [< sym_rot_cartesian_product $n >]() -> CartesianRotIter<$n> {
                    Rot::cartesian_product()
                }
                
                #[unsafe(no_mangle)]
                pub extern "C" fn [< sym_rot_cartesian_product $n _next>](it: &mut CartesianRotIter<$n>, out: Option<&mut [Rot; $n]>) -> bool {
                    match it.next() {
                        Some(result) => {
                            if let Some(out) = out {
                                *out = result;
                            }
                            true
                        }
                        None => false,
                    }
                }
            )*
        }
    };
}

rot_cartesian_product_funcs!(2, 3, 4, 5, 6, 7, 8);
