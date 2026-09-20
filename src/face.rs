
// The following code uses meta-programming and other
// techniques to prepare the library for the configured
// coordinate system.

// The following directional constants determine the discriminant values for the `Face`enum.
// These values ensure that orientations look the same regardless of coordinate system.
// The order ensures that rotations increase in a certain logical order.

use ::core::mem::MaybeUninit;

use crate::{
    axis::Axis,
    align::*,
};

// IMPORTANT: DO NOT CHANGE THESE VALUES, OR ELSE YOU WILL HAVE TO DIG THROUGH THE ENTIRE CODEBASE TO FIX IT

// These are the discriminats for each cardinal direction.
// By keeping these discriminants the same value for each coordinate
// system, it means that each bit representation has the same relative
// geometric transformation for each coordinate system.
const UP_DISC: u8 = 0;
const FORWARD_DISC: u8 = 1;
const LEFT_DISC: u8 = 2;
const BACKWARD_DISC: u8 = 3;
const RIGHT_DISC: u8 = 4;
const DOWN_DISC: u8 = 5;

// Here we select the discriminants for the axial directions
// based on the coordinate system feature-flags.
const NEG_X_DISC: u8 = cfg_select!(
    feature = "neg_x_up" => UP_DISC,
    feature = "pos_x_up" => DOWN_DISC,
    feature = "neg_x_right" => RIGHT_DISC,
    feature = "pos_x_right" => LEFT_DISC,
    feature = "neg_x_forward" => FORWARD_DISC,
    feature = "pos_x_forward" => BACKWARD_DISC,
);

const NEG_Y_DISC: u8 = cfg_select!(
    feature = "neg_y_up" => UP_DISC,
    feature = "pos_y_up" => DOWN_DISC,
    feature = "neg_y_right" => RIGHT_DISC,
    feature = "pos_y_right" => LEFT_DISC,
    feature = "neg_y_forward" => FORWARD_DISC,
    feature = "pos_y_forward" => BACKWARD_DISC,
);

const NEG_Z_DISC: u8 = cfg_select!(
    feature = "neg_z_up" => UP_DISC,
    feature = "pos_z_up" => DOWN_DISC,
    feature = "neg_z_right" => RIGHT_DISC,
    feature = "pos_z_right" => LEFT_DISC,
    feature = "neg_z_forward" => FORWARD_DISC,
    feature = "pos_z_forward" => BACKWARD_DISC,
);

const POS_X_DISC: u8 = cfg_select!(
    feature = "pos_x_up" => UP_DISC,
    feature = "neg_x_up" => DOWN_DISC,
    feature = "pos_x_right" => RIGHT_DISC,
    feature = "neg_x_right" => LEFT_DISC,
    feature = "pos_x_forward" => FORWARD_DISC,
    feature = "neg_x_forward" => BACKWARD_DISC,
);

const POS_Y_DISC: u8 = cfg_select!(
    feature = "pos_y_up" => UP_DISC,
    feature = "neg_y_up" => DOWN_DISC,
    feature = "pos_y_right" => RIGHT_DISC,
    feature = "neg_y_right" => LEFT_DISC,
    feature = "pos_y_forward" => FORWARD_DISC,
    feature = "neg_y_forward" => BACKWARD_DISC,
);

const POS_Z_DISC: u8 = cfg_select!(
    feature = "pos_z_up" => UP_DISC,
    feature = "neg_z_up" => DOWN_DISC,
    feature = "pos_z_right" => RIGHT_DISC,
    feature = "neg_z_right" => LEFT_DISC,
    feature = "pos_z_forward" => FORWARD_DISC,
    feature = "neg_z_forward" => BACKWARD_DISC,
);

/// Represents a face of a cube.
///
/// # Note
/// The discriminants for this enum's variants are not the same
/// across all coordinate systems. They are dependent on the
/// coordinate system, so you should not rely on the bit
/// representation of, for example, [Face::NegX], to remain
/// the same regardless of coordinate system.
///
/// The discriminant order is based on the cardinal directions
/// of the coordinate system.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Face {
    /// The `-X` face.
    NegX = NEG_X_DISC,
    /// The `-Y` face.
    NegY = NEG_Y_DISC,
    /// The `-Z` face.
    NegZ = NEG_Z_DISC,
    /// The `+X` face.
    PosX = POS_X_DISC,
    /// The `+Y` face.
    PosY = POS_Y_DISC,
    /// The `+Z` face.
    PosZ = POS_Z_DISC,
}
const _: () = isit::const_assert_all([
    Face::UP as u8 == UP_DISC,
    Face::RIGHT as u8 == RIGHT_DISC,
    Face::FORWARD as u8 == FORWARD_DISC,
    Face::LEFT as u8 == LEFT_DISC,
    Face::BACKWARD as u8 == BACKWARD_DISC,
    Face::DOWN as u8 == DOWN_DISC,
]);

impl Default for Face {
    #[inline(always)]
    fn default() -> Self {
        Face::UP
    }
}

use Face::*;

/// A lookup table for values associated with each [Face].
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FaceTable<T: Copy = Face>(pub [T; 6]);

impl<T: Copy> FaceTable<T> {
    /// Create a new [FaceTable] from the given `array`.
    #[must_use]
    #[inline(always)]
    pub const fn new(array: [T; 6]) -> Self {
        Self(array)
    }
    
    /// Get the value stored for the given `face`.
    #[must_use]
    #[inline(always)]
    pub const fn get(&self, face: Face) -> T {
        self.0[face as usize]
    }

    /// Set the `value` for the given `face`.
    #[inline(always)]
    pub const fn set(&mut self, face: Face, value: T) {
        self.0[face as usize] = value;
    }
}

impl<T: Copy> std::ops::Index<Face> for FaceTable<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, index: Face) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T: Copy> std::ops::IndexMut<Face> for FaceTable<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: Face) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl FaceTable<Face> {
    /// Return a new table based on this one but with each of the
    /// faces inverted.
    #[must_use]
    #[inline(always)]
    pub const fn invert(self) -> Self {
        Self([
            self.0[0].invert(),
            self.0[1].invert(),
            self.0[2].invert(),
            self.0[3].invert(),
            self.0[4].invert(),
            self.0[5].invert(),
        ])
    }
}

/// A [Face] bitmask table.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaceBitsTable(u8);

impl FaceBitsTable {
    /// Get the bit value for the given `face`.
    #[must_use]
    #[inline(always)]
    pub const fn get(self, face: Face) -> bool {
        self.0 & (1 << face as u8) != 0
    }

    /// Set the bit `value` for the given `face`.
    #[inline(always)]
    pub const fn set(&mut self, face: Face, value: bool) {
        if value {
            self.0 |= 1 << face as u8;
        } else {
            self.0 &= !(1 << face as u8);
        }
    }
}

/// Construct a [FaceBitsTable] from the given axial direction values.
pub(crate) const fn axial_face_bits_table(
    neg_x: bool,
    neg_y: bool,
    neg_z: bool,
    pos_x: bool,
    pos_y: bool,
    pos_z: bool,
) -> FaceBitsTable {
    let mut bits = FaceBitsTable(0u8);
    bits.set(Face::NegX, neg_x);
    bits.set(Face::NegY, neg_y);
    bits.set(Face::NegZ, neg_z);
    bits.set(Face::PosX, pos_x);
    bits.set(Face::PosY, pos_y);
    bits.set(Face::PosZ, pos_z);
    bits
}

#[repr(C)]
#[derive(Clone, Copy)]
union TableConvert<T: Copy> {
    uninit: [MaybeUninit<T>; 6],
    init: [T; 6],
}

pub(crate) const fn cardinal_face_table<T: Copy>(
    up: T,
    forward: T,
    left: T,
    backward: T,
    right: T,
    down: T,
) -> FaceTable<T> {
    // This is a somewhat convoluted way to ensure that changing
    // the discriminant ordering of Face does not break the
    // lookup tables.
    use ::core::mem::MaybeUninit;
    let mut table = [MaybeUninit::<T>::uninit(); 6];
    table[Face::UP as usize].write(up);
    table[Face::FORWARD as usize].write(forward);
    table[Face::LEFT as usize].write(left);
    table[Face::BACKWARD as usize].write(backward);
    table[Face::RIGHT as usize].write(right);
    table[Face::DOWN as usize].write(down);
    FaceTable(unsafe { TableConvert { uninit: table }.init })
}

/// Create a new [FaceTable] from the given axial values.
pub(crate) const fn axial_face_table<T: Copy>(
    neg_x: T,
    neg_y: T,
    neg_z: T,
    pos_x: T,
    pos_y: T,
    pos_z: T,
) -> FaceTable<T> {
    // This is a somewhat convoluted way to ensure that changing
    // the discriminant ordering of Face does not break the
    // lookup tables.
    let mut table = [MaybeUninit::<T>::uninit(); 6];
    table[Face::NegX as usize].write(neg_x);
    table[Face::NegY as usize].write(neg_y);
    table[Face::NegZ as usize].write(neg_z);
    table[Face::PosX as usize].write(pos_x);
    table[Face::PosY as usize].write(pos_y);
    table[Face::PosZ as usize].write(pos_z);
    FaceTable(unsafe { TableConvert { uninit: table }.init })
}

/// The angle direction indicates which direction angles increase in.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AngleDirection {
    /// Counter-clockwise
    #[default]
    CCW = 0,
    ///  Clockwise
    CW = 1,
}

/// Create a lookup table for face at angle tables.
const fn face_at_angle(
    up: FaceTable<Face>,
    left: FaceTable<Face>,
    down: FaceTable<Face>,
    right: FaceTable<Face>,
) -> [FaceTable<Face>; 4] {
    match Face::ANGLE_DIRECTION {
        AngleDirection::CCW => [up, left, down, right],
        AngleDirection::CW => [up, right, down, left],
    }
}

/// The Up direction's [Face], as configured by the coordinate
/// system feature flags.
const UP_DIRECTION: Face = cfg_select!(
    feature = "neg_x_up" => Face::NegX,
    feature = "neg_y_up" => Face::NegY,
    feature = "neg_z_up" => Face::NegZ,
    feature = "pos_x_up" => Face::PosX,
    feature = "pos_y_up" => Face::PosY,
    feature = "pos_z_up" => Face::PosZ,
    _ => compile_error!("Must have up direction feature enabled."),
);

/// The Right direction's [Face], as configured by the coordinate
/// system feature flags.
const RIGHT_DIRECTION: Face = cfg_select!(
    feature = "neg_x_right" => Face::NegX,
    feature = "neg_y_right" => Face::NegY,
    feature = "neg_z_right" => Face::NegZ,
    feature = "pos_x_right" => Face::PosX,
    feature = "pos_y_right" => Face::PosY,
    feature = "pos_z_right" => Face::PosZ,
    _ => compile_error!("Must have right direction feature enabled."),
);

/// The Forward direction's [Face], as configured by the coordinate
/// system feature flags.
const FORWARD_DIRECTION: Face = cfg_select!(
    feature = "neg_x_forward" => Face::NegX,
    feature = "neg_y_forward" => Face::NegY,
    feature = "neg_z_forward" => Face::NegZ,
    feature = "pos_x_forward" => Face::PosX,
    feature = "pos_y_forward" => Face::PosY,
    feature = "pos_z_forward" => Face::PosZ,
    _ => compile_error!("Must have forward direction feature enabled."),
);

/// Determine the `Up` face for the given face.
///
/// The `Up` face is the face that points in the same direction as
/// "Up" on the 2D plane of the [Face].
const fn calc_face_up(face: Face) -> Face {
    match face {
        Face::UP => Face::FORWARD,
        Face::LEFT => Face::UP,
        Face::RIGHT => Face::UP,
        Face::FORWARD => Face::UP,
        Face::BACKWARD => Face::UP,
        Face::DOWN => Face::BACKWARD,
    }
}

/// Determine the `Left` face for the given face.
///
/// The `Left` face is the face that points in the same direction as
/// "Left" on the 2D plane of the [Face].
const fn calc_face_left(face: Face) -> Face {
    match face {
        Face::UP => Face::LEFT,
        Face::DOWN => Face::LEFT,
        Face::LEFT => Face::FORWARD,
        Face::RIGHT => Face::BACKWARD,
        Face::FORWARD => Face::RIGHT,
        Face::BACKWARD => Face::LEFT,
    }
}

impl Face {
    // --- CONFIGURATION CONSTANTS ---

    /// The Up face within the configured coordinate system.
    pub const UP: Self = UP_DIRECTION;
    /// The Right face within the configured coordinate system.
    pub const RIGHT: Self = RIGHT_DIRECTION;
    /// The Forward face within the configured coordinate system.
    pub const FORWARD: Self = FORWARD_DIRECTION;
    /// The Down face within the configured coordinate system.
    pub const DOWN: Self = UP_DIRECTION.invert();
    /// The Left face within the configured coordinate system.
    pub const LEFT: Self = RIGHT_DIRECTION.invert();
    /// The Backward face within the configured coordinate system.
    pub const BACKWARD: Self = FORWARD_DIRECTION.invert();
    
    /// The angle direction determines which direction angles
    /// increase, whether clockwise or counter-clockwise.
    pub const ANGLE_DIRECTION: AngleDirection = cfg_select!(
        feature = "clockwise-angles" => AngleDirection::CW,
        _ => AngleDirection::CCW,
    );
    
    // --- CAYLEY TABLES ---
    
    // ========================================
    //              +--------+            ^
    //             /|       /|           /
    //            / |UP    / |          /
    //           +--------+  |       Forward
    //           |  +-----|--+      Backward
    //     Left  | /      | /Right   /
    //           |/       |/        /
    //           +--------+        v
    //              DOWN
    // ========================================
    // For each face of the cube, the face has an orientation
    // relative to the rest of the cube. These tables determine
    // that orientation. You do should not change these tables.

    /// Determines which direction points upward relative to each face.
    pub(crate) const UP_FACE_TABLE: FaceTable<Face> = axial_face_table(
        calc_face_up(NegX),
        calc_face_up(NegY),
        calc_face_up(NegZ),
        calc_face_up(PosX),
        calc_face_up(PosY),
        calc_face_up(PosZ),
    );
    /// Determines which direction points leftward relative to each face.
    pub(crate) const LEFT_FACE_TABLE: FaceTable<Face> = axial_face_table(
        calc_face_left(NegX),
        calc_face_left(NegY),
        calc_face_left(NegZ),
        calc_face_left(PosX),
        calc_face_left(PosY),
        calc_face_left(PosZ),
    );
    /// Determines which direction points downward relative to each face.
    pub(crate) const DOWN_FACE_TABLE: FaceTable<Face> = Self::UP_FACE_TABLE.invert();
    /// Determines which direction points rightward relative to each face.
    pub(crate) const RIGHT_FACE_TABLE: FaceTable<Face> = Self::LEFT_FACE_TABLE.invert();

    /// Determines which direction points upward for each face at each angle.
    pub(crate) const UP_AT_ANGLE_TABLE: [FaceTable<Face>; 4] = face_at_angle(
        Self::UP_FACE_TABLE,
        Self::LEFT_FACE_TABLE,
        Self::DOWN_FACE_TABLE,
        Self::RIGHT_FACE_TABLE,
    );

    /// Determines which direction points leftward for each face at each angle.
    pub(crate) const LEFT_AT_ANGLE_TABLE: [FaceTable<Face>; 4] = face_at_angle(
        Self::LEFT_FACE_TABLE,
        Self::DOWN_FACE_TABLE,
        Self::RIGHT_FACE_TABLE,
        Self::UP_FACE_TABLE,
    );

    /// Determines which direction points downward for each face at each angle.
    pub(crate) const DOWN_AT_ANGLE_TABLE: [FaceTable<Face>; 4] = face_at_angle(
        Self::DOWN_FACE_TABLE,
        Self::RIGHT_FACE_TABLE,
        Self::UP_FACE_TABLE,
        Self::LEFT_FACE_TABLE,
    );

    /// Determines which direction points rightward for each face at each angle.
    pub(crate) const RIGHT_AT_ANGLE_TABLE: [FaceTable<Face>; 4] = face_at_angle(
        Self::RIGHT_FACE_TABLE,
        Self::UP_FACE_TABLE,
        Self::LEFT_FACE_TABLE,
        Self::DOWN_FACE_TABLE,
    );

    //                                                      Order: NegX, NegY, NegZ, PosX, PosY, PosZ
    /// The inversion of each face.
    pub(crate) const INVERT_TABLE: FaceTable<Face> = axial_face_table(PosX, PosY, PosZ, NegX, NegY, NegZ);

    //                                                         Order: NegX, NegY, NegZ, PosX, PosY, PosZ
    pub(crate) const INVERT_X_TABLE: FaceTable<Face> = axial_face_table(PosX, NegY, NegZ, NegX, PosY, PosZ);
    pub(crate) const INVERT_Y_TABLE: FaceTable<Face> = axial_face_table(NegX, PosY, NegZ, PosX, NegY, PosZ);
    pub(crate) const INVERT_Z_TABLE: FaceTable<Face> = axial_face_table(NegX, NegY, PosZ, PosX, PosY, NegZ);

    pub(crate) const INVERT_XY_TABLE: FaceTable<Face> = axial_face_table(PosX, PosY, NegZ, NegX, NegY, PosZ);
    pub(crate) const INVERT_XZ_TABLE: FaceTable<Face> = axial_face_table(PosX, NegY, PosZ, NegX, PosY, NegZ);
    pub(crate) const INVERT_YZ_TABLE: FaceTable<Face> = axial_face_table(NegX, PosY, PosZ, PosX, NegY, NegZ);

    pub(crate) const INVERT_VERTICAL_TABLE: FaceTable<Face> = {
        const fn calc(face: Face) -> Face {
            match face {
                Face::UP => Face::DOWN,
                Face::DOWN => Face::UP,
                other => other,
            }
        }
        axial_face_table(
            calc(NegX), calc(NegY), calc(NegZ),
            calc(PosX), calc(PosY), calc(PosZ),
        )
    };

    pub(crate) const INVERT_LEFT_RIGHT_TABLE: FaceTable<Face> = {
        const fn calc(face: Face) -> Face {
            match face {
                Face::LEFT => Face::RIGHT,
                Face::RIGHT => Face::LEFT,
                other => other,
            }
        }
        axial_face_table(
            calc(NegX), calc(NegY), calc(NegZ),
            calc(PosX), calc(PosY), calc(PosZ),
        )
    };

    pub(crate) const INVERT_FRONT_BACK_TABLE: FaceTable<Face> = {
        const fn calc(face: Face) -> Face {
            match face {
                Face::FORWARD => Face::BACKWARD,
                Face::BACKWARD => Face::FORWARD,
                other => other,
            }
        }
        axial_face_table(
            calc(NegX), calc(NegY), calc(NegZ),
            calc(PosX), calc(PosY), calc(PosZ),
        )
    };

    // --- CONSTRUCTORS ---

    /// Create a [Face] from a raw [u8] value without checking
    /// if it is a valid [Face] bit representation.
    ///
    /// # SAFETY
    /// If you provide an invalid bit representation, the behavior
    /// is undefined. Valid bit representations are `0 <= n < 6`.
    #[must_use]
    #[inline(always)]
    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }

    /// Attempt to create a [Face] from a raw [u8] value.
    ///
    /// If [Face] can not be represented with that value, returns [None].
    #[must_use]
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Option<Self> {
        if value > 5 {
            return None;
        }
        Some(unsafe { Self::from_u8_unchecked(value) })
    }

    // --- ACCESSORS ---

    /// Return the [u8] discriminant value.
    #[must_use]
    #[inline(always)]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns the [Axis] of this [Face].
    #[must_use]
    #[inline(always)]
    pub const fn axis(self) -> Axis {
        const TABLE: FaceTable<Axis> = axial_face_table(Axis::X, Axis::Y, Axis::Z, Axis::X, Axis::Y, Axis::Z);
        TABLE.get(self)
    }

    /// Check if the [Face] is a negative face.
    #[must_use]
    #[inline(always)]
    pub const fn is_negative(self) -> bool {
        const TABLE: FaceBitsTable = axial_face_bits_table(
            true, true, true, false, false, false,
        );
        TABLE.get(self)
    }

    /// Check if the [Face] is a positive face.
    #[must_use]
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        const TABLE: FaceBitsTable = axial_face_bits_table(
            false, false, false, true, true, true,
        );
        TABLE.get(self)
    }

    /// Return the negative variant of this [Face].
    #[must_use]
    #[inline(always)]
    pub const fn as_negative(self) -> Self {
        const TABLE: FaceTable = axial_face_table(NegX, NegY, NegZ, NegX, NegY, NegZ);
        TABLE.get(self)
    }

    /// Returns the positive variant of this [Face].
    #[must_use]
    #[inline(always)]
    pub const fn as_positive(self) -> Self {
        const TABLE: FaceTable = axial_face_table(PosX, PosY, PosZ, PosX, PosY, PosZ);
        TABLE.get(self)
    }

    /// Returns the variant of this [Face] with the same sign as `sign`.
    #[must_use]
    #[inline(always)]
    pub const fn with_sign(self, sign: i32) -> Self {
        match sign {
            0 => self,
            1.. => self.as_positive(),
            ..0 => self.as_negative(),
        }
    }

    // --- QUERY FUNCTIONS ---

    /// Returns the Up [Face] of the given [Face].
    ///
    /// The Up [Face] is the face that points upward on the
    /// 2D face plane.
    #[must_use]
    #[inline(always)]
    pub const fn up(self) -> Self {
        Self::UP_FACE_TABLE.get(self)
    }

    /// Returns the Up [Face] of the given [Face] when that face
    /// has been rotated at the given `angle`.
    ///
    /// The Up [Face] is the face that points upward on the
    /// 2D face plane.
    #[must_use]
    #[inline(always)]
    pub const fn up_at_angle(self, angle: i8) -> Self {
        Self::UP_AT_ANGLE_TABLE[(angle & 3) as usize].get(self)
    }

    /// Returns the Left [Face] of the given [Face].
    ///
    /// The Left [Face] is the face that points leftward on the
    /// 2D face plane.
    #[must_use]
    #[inline(always)]
    pub const fn left(self) -> Self {
        Self::LEFT_FACE_TABLE.get(self)
    }

    /// Returns the Left [Face] of the given [Face] when that face
    /// has been rotated at the given `angle`.
    ///
    /// The Left [Face] is the face that points leftward on the
    /// 2D face plane.
    #[must_use]
    #[inline(always)]
    pub const fn left_at_angle(self, angle: i8) -> Self {
        Self::LEFT_AT_ANGLE_TABLE[(angle & 3) as usize].get(self)
    }

    /// Returns the Down [Face] of the given [Face].
    ///
    /// The Down [Face] is the face that points downward on the
    /// 2D face plane.
    #[must_use]
    #[inline(always)]
    pub const fn down(self) -> Self {
        Self::DOWN_FACE_TABLE.get(self)
    }

    /// Returns the Down [Face] of the given [Face] when that face
    /// has been rotated at the given `angle`.
    ///
    /// The Down [Face] is the face that points downward on the
    /// 2D face plane.
    #[must_use]
    #[inline(always)]
    pub const fn down_at_angle(self, angle: i8) -> Self {
        Self::DOWN_AT_ANGLE_TABLE[(angle & 3) as usize].get(self)
    }

    /// Returns the Right [Face] of the given [Face].
    ///
    /// The Right [Face] is the face that points rightward on the
    /// 2D face plane.
    #[must_use]
    #[inline(always)]
    pub const fn right(self) -> Self {
        Self::RIGHT_FACE_TABLE.get(self)
    }

    /// Returns the Right [Face] of the given [Face] when that face
    /// has been rotated at the given `angle`.
    ///
    /// The Right [Face] is the face that points rightward on the
    /// 2D face plane.
    #[must_use]
    #[inline(always)]
    pub const fn right_at_angle(self, angle: i8) -> Self {
        Self::RIGHT_AT_ANGLE_TABLE[(angle & 3) as usize].get(self)
    }

    /// Invert the [Face].
    #[must_use]
    #[inline(always)]
    pub const fn invert(self) -> Self {
        Self::INVERT_TABLE.get(self)
    }

    /// Invert the [Face] if the given `condition` is met.
    #[must_use]
    #[inline(always)]
    pub const fn invert_if(self, condition: bool) -> Self {
        const TABLE: [Align8<FaceTable<Face>>; 2] = [
            Align8(axial_face_table(NegX, NegY, NegZ, PosX, PosY, PosZ)),
            Align8(Face::INVERT_TABLE),
        ];
        TABLE[condition as usize].0.get(self)
    }

    /// Invert the `X` axis.
    #[must_use]
    #[inline(always)]
    pub const fn invert_x(self) -> Self {
        Self::INVERT_X_TABLE.get(self)
    }

    /// Invert the `Y` axis.
    #[must_use]
    #[inline(always)]
    pub const fn invert_y(self) -> Self {
        Self::INVERT_Y_TABLE.get(self)
    }

    /// Invert the `Z` axis.
    #[must_use]
    #[inline(always)]
    pub const fn invert_z(self) -> Self {
        Self::INVERT_Z_TABLE.get(self)
    }

    /// Invert the `X` and `Y` axes.
    #[must_use]
    #[inline(always)]
    pub const fn invert_xy(self) -> Self {
        Self::INVERT_XY_TABLE.get(self)
    }

    /// Invert the `X` and `Z` axes.
    #[must_use]
    #[inline(always)]
    pub const fn invert_xz(self) -> Self {
        Self::INVERT_XZ_TABLE.get(self)
    }

    /// Invert the `Y` and `Z` axes.
    #[must_use]
    #[inline(always)]
    pub const fn invert_yz(self) -> Self {
        Self::INVERT_YZ_TABLE.get(self)
    }

    /// Invert the vertical axis. ([Face::UP] <-> [Face::DOWN])
    #[must_use]
    #[inline(always)]
    pub const fn invert_vertical(self) -> Self {
        Self::INVERT_VERTICAL_TABLE.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn invert_vertical_if(self, condition: bool) -> Face {
        const TABLE: [Align8<FaceTable<Face>>; 2] = [
            Align8(cardinal_face_table(Face::UP, Face::FORWARD, Face::LEFT, Face::BACKWARD, Face::RIGHT, Face::DOWN)),
            Align8(cardinal_face_table(Face::DOWN, Face::FORWARD, Face::LEFT, Face::BACKWARD, Face::RIGHT, Face::UP)),
        ];
        TABLE[condition as usize].0.get(self)
    }

    /// Invert the left/right axis.
    #[must_use]
    #[inline(always)]
    pub const fn invert_left_right(self) -> Self {
        Self::INVERT_LEFT_RIGHT_TABLE.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn invert_left_right_if(self, condition: bool) -> Face {
        const TABLE: [Align8<FaceTable<Face>>; 2] = [
            Align8(cardinal_face_table(Face::UP, Face::FORWARD, Face::LEFT, Face::BACKWARD, Face::RIGHT, Face::DOWN)),
            Align8(cardinal_face_table(Face::UP, Face::FORWARD, Face::RIGHT, Face::BACKWARD, Face::LEFT, Face::DOWN)),
        ];
        TABLE[condition as usize].0.get(self)
    }

    /// Invert the front/back axis.
    #[must_use]
    #[inline(always)]
    pub const fn invert_front_back(self) -> Self {
        Self::INVERT_FRONT_BACK_TABLE.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn invert_front_back_if(self, condition: bool) -> Self {
        const TABLE: [Align8<FaceTable<Face>>; 2] = [
            Align8(cardinal_face_table(Face::UP, Face::FORWARD, Face::LEFT, Face::BACKWARD, Face::RIGHT, Face::DOWN)),
            Align8(cardinal_face_table(Face::UP, Face::BACKWARD, Face::LEFT, Face::FORWARD, Face::RIGHT, Face::DOWN)),
        ];
        TABLE[condition as usize].0.get(self)
    }

    // --- MISCELLANEOUS ---

    /// Iterate faces in the discriminant order.
    ///
    /// # Note
    /// The order is dependent on the coordinate system.
    #[must_use]
    #[inline(always)]
    pub const fn iter() -> FaceIter {
        FaceIter::new()
    }

    /// Iterate faces in lexicographic order.
    ///
    /// # Note
    /// The order is independent of the coordinate system.
    #[must_use]
    #[inline(always)]
    pub fn lexicographic_iter() -> LexFaceIter {
        LexFaceIter::new() 
    }

    /// Check `self` is equal to `other`.
    #[must_use]
    #[inline(always)]
    pub const fn eq(self, other: Self) -> bool {
        self as u8 == other as u8
    }

    /// Check if `self` is not equal to `other`.
    #[must_use]
    #[inline(always)]
    pub const fn ne(self, other: Self) -> bool {
        self as u8 != other as u8
    }

    /// The [i8] coordinate of this [Face].
    #[must_use]
    #[inline]
    pub const fn to_coord_i8(self) -> [i8; 3] {
        match self {
            NegX => [-1 as _,  0 as _,  0 as _],
            NegY => [ 0 as _, -1 as _,  0 as _],
            NegZ => [ 0 as _,  0 as _, -1 as _],
            PosX => [ 1 as _,  0 as _,  0 as _],
            PosY => [ 0 as _,  1 as _,  0 as _],
            PosZ => [ 0 as _,  0 as _,  1 as _],
        }
    }

    /// The [i16] coordinate of this [Face].
    #[must_use]
    #[inline]
    pub const fn to_coord_i16(self) -> [i16; 3] {
        match self {
            NegX => [-1 as _,  0 as _,  0 as _],
            NegY => [ 0 as _, -1 as _,  0 as _],
            NegZ => [ 0 as _,  0 as _, -1 as _],
            PosX => [ 1 as _,  0 as _,  0 as _],
            PosY => [ 0 as _,  1 as _,  0 as _],
            PosZ => [ 0 as _,  0 as _,  1 as _],
        }
    }

    /// The [i32] coordinate of this [Face].
    #[must_use]
    #[inline]
    pub const fn to_coord_i32(self) -> [i32; 3] {
        match self {
            NegX => [-1 as _,  0 as _,  0 as _],
            NegY => [ 0 as _, -1 as _,  0 as _],
            NegZ => [ 0 as _,  0 as _, -1 as _],
            PosX => [ 1 as _,  0 as _,  0 as _],
            PosY => [ 0 as _,  1 as _,  0 as _],
            PosZ => [ 0 as _,  0 as _,  1 as _],
        }
    }

    /// The [i64] coordinate of this [Face].
    #[must_use]
    #[inline]
    pub const fn to_coord_i64(self) -> [i64; 3] {
        match self {
            NegX => [-1 as _,  0 as _,  0 as _],
            NegY => [ 0 as _, -1 as _,  0 as _],
            NegZ => [ 0 as _,  0 as _, -1 as _],
            PosX => [ 1 as _,  0 as _,  0 as _],
            PosY => [ 0 as _,  1 as _,  0 as _],
            PosZ => [ 0 as _,  0 as _,  1 as _],
        }
    }

    /// The [f32] coordinate of this [Face].
    #[must_use]
    #[inline]
    pub const fn to_coord_f32(self) -> [f32; 3] {
        match self {
            NegX => [-1 as _,  0 as _,  0 as _],
            NegY => [ 0 as _, -1 as _,  0 as _],
            NegZ => [ 0 as _,  0 as _, -1 as _],
            PosX => [ 1 as _,  0 as _,  0 as _],
            PosY => [ 0 as _,  1 as _,  0 as _],
            PosZ => [ 0 as _,  0 as _,  1 as _],
        }
    }

    /// The [f64] coordinate of this [Face].
    #[must_use]
    #[inline]
    pub const fn to_coord_f64(self) -> [f64; 3] {
        match self {
            NegX => [-1 as _,  0 as _,  0 as _],
            NegY => [ 0 as _, -1 as _,  0 as _],
            NegZ => [ 0 as _,  0 as _, -1 as _],
            PosX => [ 1 as _,  0 as _,  0 as _],
            PosY => [ 0 as _,  1 as _,  0 as _],
            PosZ => [ 0 as _,  0 as _,  1 as _],
        }
    }

    /// Check if `self` is orthogonal to `other`.
    ///
    /// A face is considered orthogonal to another one when it is
    /// on a different axis.
    #[must_use]
    #[inline(always)]
    pub const fn is_orthogonal_to(self, other: Face) -> bool {
        // TODO: This can be made into a 64-bit mask `(8 * 6)`.
        self.axis().is_orthogonal_to(other.axis())
    }

}

/// An iterator of each [Face] in the order of their discriminants.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Hash)]
pub struct FaceIter {
    pub(crate) face: u8,
}

impl FaceIter {
    /// Create a new [FaceIter], starting with [Face::UP].
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            face: 0,
        }
    }

    /// Get the current [Face], unless the iterator is exhausted, in
    /// which case it will return [None].
    #[must_use]
    #[inline(always)]
    pub const fn current(&self) -> Option<Face> {
        Face::from_u8(self.face)
    }

    /// Continue iteration, returning the current [Face] in the process.
    #[must_use]
    #[inline]
    pub const fn next(&mut self) -> Option<Face> {
        match self.current() {
            None => None,
            some => {
                self.face += 1;
                some
            }
        }
    }
}

impl Iterator for FaceIter {
    type Item = Face;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[derive(Debug, Clone)]
pub struct LexFaceIter {
    index: u8,
}

impl LexFaceIter {
    const ORDER: [Face; 6] = [
        Face::NegX,
        Face::NegY,
        Face::NegZ,
        Face::PosX,
        Face::PosY,
        Face::PosZ,
    ];
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self { index: 0 }
    }

    #[must_use]
    #[inline]
    pub const fn current(&self) -> Option<Face> {
        if self.index > 5 {
            return None;
        }
        Some(Self::ORDER[self.index as usize])
    }

    #[must_use]
    #[inline]
    pub const fn next(&mut self) -> Option<Face> {
        match self.current() {
            None => None,
            some => {
                self.index += 1;
                some
            }
        }
    }
}

impl Iterator for LexFaceIter {
    type Item = Face;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
