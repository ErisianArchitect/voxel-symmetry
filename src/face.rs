
// The following code uses meta-programming and other
// techniques to prepare the library for the configured
// coordinate system.

// The following directional constants determine the discriminant values for the `Face`enum.
// These values ensure that orientations look the same regardless of coordinate system.
// The order ensures that rotations increase in a certain logical order.

use crate::{
    axis::Axis,
};

const UP_DISC: u8 = 0;
const FORWARD_DISC: u8 = 1;
const LEFT_DISC: u8 = 2;
const BACKWARD_DISC: u8 = 3;
const RIGHT_DISC: u8 = 4;
const DOWN_DISC: u8 = 5;

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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Face {
    NegX = NEG_X_DISC,
    NegY = NEG_Y_DISC,
    NegZ = NEG_Z_DISC,
    PosX = POS_X_DISC,
    PosY = POS_Y_DISC,
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

/// A padded Cayley table for values associated with each [Face].
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FaceCayley<T: Copy = Face>(pub [T; 6]);

impl<T: Copy> FaceCayley<T> {
    #[must_use]
    #[inline(always)]
    pub const fn new(arr: [T; 6]) -> Self {
        Self(arr)
    }
    
    /// Get the value stored for the given [Face].
    #[must_use]
    #[inline(always)]
    pub const fn get(&self, face: Face) -> T {
        self.0[face as usize]
    }

    #[inline(always)]
    pub const fn set(&mut self, face: Face, value: T) {
        self.0[face as usize] = value;
    }
}

impl<T: Copy> std::ops::Index<Face> for FaceCayley<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, index: Face) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T: Copy> std::ops::IndexMut<Face> for FaceCayley<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: Face) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl FaceCayley<Face> {
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaceCayleyBits(u8);

impl FaceCayleyBits {
    #[must_use]
    #[inline(always)]
    pub const fn get(self, face: Face) -> bool {
        self.0 & (1 << face as u8) != 0
    }
}

pub(crate) const fn face_caley_bits(
    neg_x: bool,
    neg_y: bool,
    neg_z: bool,
    pos_x: bool,
    pos_y: bool,
    pos_z: bool,
) -> FaceCayleyBits {
    const fn set_bit_if(bits: u8, bit: u8, condition: bool) -> u8 {
        if condition {
            bits | bit
        } else {
            bits
        }
    }
    let mut bits = 0u8;
    bits = set_bit_if(bits, 1 << Face::NegX as u8, neg_x);
    bits = set_bit_if(bits, 1 << Face::NegY as u8, neg_y);
    bits = set_bit_if(bits, 1 << Face::NegZ as u8, neg_z);
    bits = set_bit_if(bits, 1 << Face::PosX as u8, pos_x);
    bits = set_bit_if(bits, 1 << Face::PosY as u8, pos_y);
    bits = set_bit_if(bits, 1 << Face::PosZ as u8, pos_z);
    FaceCayleyBits(bits)
}

/// Create a new face Cayley table.
/// This function ensures that each value ends up in the right slot.
pub(crate) const fn face_cayley<T: Copy>(
    neg_x: T,
    neg_y: T,
    neg_z: T,
    pos_x: T,
    pos_y: T,
    pos_z: T,
) -> FaceCayley<T> {
    // This is a somewhat convoluted way to ensure that changing
    // the discriminant ordering of Face does not break the
    // Cayley tables.
    use ::core::mem::MaybeUninit;
    let mut table = [MaybeUninit::<T>::uninit(); 6];
    table[Face::NegX as usize].write(neg_x);
    table[Face::NegY as usize].write(neg_y);
    table[Face::NegZ as usize].write(neg_z);
    table[Face::PosX as usize].write(pos_x);
    table[Face::PosY as usize].write(pos_y);
    table[Face::PosZ as usize].write(pos_z);
    #[repr(C)]
    #[derive(Clone, Copy)]
    union TableConvert<T: Copy> {
        uninit: [MaybeUninit<T>; 6],
        init: [T; 6],
    }
    FaceCayley(unsafe { TableConvert { uninit: table }.init })
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

/// Create a Cayley table for face at angle tables, configured for the AngleDirection.
const fn face_at_angle(
    up: FaceCayley<Face>,
    left: FaceCayley<Face>,
    down: FaceCayley<Face>,
    right: FaceCayley<Face>,
) -> [FaceCayley<Face>; 4] {
    match Face::ANGLE_DIRECTION {
        AngleDirection::CCW => [up, left, down, right],
        AngleDirection::CW => [up, right, down, left],
    }
}

const UP_DIRECTION: Face = cfg_select!(
    feature = "neg_x_up" => Face::NegX,
    feature = "neg_y_up" => Face::NegY,
    feature = "neg_z_up" => Face::NegZ,
    feature = "pos_x_up" => Face::PosX,
    feature = "pos_y_up" => Face::PosY,
    feature = "pos_z_up" => Face::PosZ,
    _ => compile_error!("Must have up direction feature enabled."),
);

const RIGHT_DIRECTION: Face = cfg_select!(
    feature = "neg_x_right" => Face::NegX,
    feature = "neg_y_right" => Face::NegY,
    feature = "neg_z_right" => Face::NegZ,
    feature = "pos_x_right" => Face::PosX,
    feature = "pos_y_right" => Face::PosY,
    feature = "pos_z_right" => Face::PosZ,
    _ => compile_error!("Must have right direction feature enabled."),
);

const FORWARD_DIRECTION: Face = cfg_select!(
    feature = "neg_x_forward" => Face::NegX,
    feature = "neg_y_forward" => Face::NegY,
    feature = "neg_z_forward" => Face::NegZ,
    feature = "pos_x_forward" => Face::PosX,
    feature = "pos_y_forward" => Face::PosY,
    feature = "pos_z_forward" => Face::PosZ,
    _ => compile_error!("Must have forward direction feature enabled."),
);

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
    
    /// The angle direction determines which direction that
    /// angles increase, whether clockwise or counter-clockwise.
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
    pub(crate) const UP_CAYLEY:    FaceCayley<Face> = face_cayley(
        calc_face_up(NegX),
        calc_face_up(NegY),
        calc_face_up(NegZ),
        calc_face_up(PosX),
        calc_face_up(PosY),
        calc_face_up(PosZ),
    );
    /// Determines which direction points leftward relative to each face.
    pub(crate) const LEFT_CAYLEY:  FaceCayley<Face> = face_cayley(
        calc_face_left(NegX),
        calc_face_left(NegY),
        calc_face_left(NegZ),
        calc_face_left(PosX),
        calc_face_left(PosY),
        calc_face_left(PosZ),
    );
    /// Determines which direction points downward relative to each face.
    pub(crate) const DOWN_CAYLEY:  FaceCayley<Face> = Self::UP_CAYLEY.invert();
    /// Determines which direction points rightward relative to each face.
    pub(crate) const RIGHT_CAYLEY: FaceCayley<Face> = Self::LEFT_CAYLEY.invert();

    /// Determines which direction points upward for each face at each angle.
    pub(crate) const UP_AT_ANGLE_CAYLEY: [FaceCayley<Face>; 4] = face_at_angle(
        Self::UP_CAYLEY,
        Self::LEFT_CAYLEY,
        Self::DOWN_CAYLEY,
        Self::RIGHT_CAYLEY,
    );

    /// Determines which direction points leftward for each face at each angle.
    pub(crate) const LEFT_AT_ANGLE_CAYLEY: [FaceCayley<Face>; 4] = face_at_angle(
        Self::LEFT_CAYLEY,
        Self::DOWN_CAYLEY,
        Self::RIGHT_CAYLEY,
        Self::UP_CAYLEY,
    );

    /// Determines which direction points downward for each face at each angle.
    pub(crate) const DOWN_AT_ANGLE_CAYLEY: [FaceCayley<Face>; 4] = face_at_angle(
        Self::DOWN_CAYLEY,
        Self::RIGHT_CAYLEY,
        Self::UP_CAYLEY,
        Self::LEFT_CAYLEY,
    );

    /// Determines which direction points rightward for each face at each angle.
    pub(crate) const RIGHT_AT_ANGLE_CAYLEY: [FaceCayley<Face>; 4] = face_at_angle(
        Self::RIGHT_CAYLEY,
        Self::UP_CAYLEY,
        Self::LEFT_CAYLEY,
        Self::DOWN_CAYLEY,
    );

    //                                                      Order: NegX, NegY, NegZ, PosX, PosY, PosZ
    /// The inversion of each face.
    pub(crate) const INVERT_CAYLEY: FaceCayley<Face> = face_cayley(PosX, PosY, PosZ, NegX, NegY, NegZ);

    //                                                         Order: NegX, NegY, NegZ, PosX, PosY, PosZ
    pub(crate) const INVERT_X_CAYLEY: FaceCayley<Face> = face_cayley(PosX, NegY, NegZ, NegX, PosY, PosZ);
    pub(crate) const INVERT_Y_CAYLEY: FaceCayley<Face> = face_cayley(NegX, PosY, NegZ, PosX, NegY, PosZ);
    pub(crate) const INVERT_Z_CAYLEY: FaceCayley<Face> = face_cayley(NegX, NegY, PosZ, PosX, PosY, NegZ);

    pub(crate) const INVERT_XY_CAYLEY: FaceCayley<Face> = face_cayley(PosX, PosY, NegZ, NegX, NegY, PosZ);
    pub(crate) const INVERT_XZ_CAYLEY: FaceCayley<Face> = face_cayley(PosX, NegY, PosZ, NegX, PosY, NegZ);
    pub(crate) const INVERT_YZ_CAYLEY: FaceCayley<Face> = face_cayley(NegX, PosY, PosZ, PosX, NegY, NegZ);

    pub(crate) const INVERT_VERTICAL_CAYLEY: FaceCayley<Face> = {
        const fn calc(face: Face) -> Face {
            match face {
                Face::UP => Face::DOWN,
                Face::DOWN => Face::UP,
                other => other,
            }
        }
        face_cayley(
            calc(NegX), calc(NegY), calc(NegZ),
            calc(PosX), calc(PosY), calc(PosZ),
        )
    };

    pub(crate) const INVERT_LEFT_RIGHT_CAYLEY: FaceCayley<Face> = {
        const fn calc(face: Face) -> Face {
            match face {
                Face::LEFT => Face::RIGHT,
                Face::RIGHT => Face::LEFT,
                other => other,
            }
        }
        face_cayley(
            calc(NegX), calc(NegY), calc(NegZ),
            calc(PosX), calc(PosY), calc(PosZ),
        )
    };

    pub(crate) const INVERT_FRONT_BACK_CAYLEY: FaceCayley<Face> = {
        const fn calc(face: Face) -> Face {
            match face {
                Face::FORWARD => Face::BACKWARD,
                Face::BACKWARD => Face::FORWARD,
                other => other,
            }
        }
        face_cayley(
            calc(NegX), calc(NegY), calc(NegZ),
            calc(PosX), calc(PosY), calc(PosZ),
        )
    };

    // --- CONSTRUCTORS ---

    #[must_use]
    #[inline(always)]
    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Option<Self> {
        if value > 5 {
            return None;
        }
        Some(unsafe { Self::from_u8_unchecked(value) })
    }

    // --- ACCESSORS ---

    #[must_use]
    #[inline(always)]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    #[must_use]
    #[inline(always)]
    pub const fn axis(self) -> Axis {
        const TABLE: FaceCayley<Axis> = face_cayley(Axis::X, Axis::Y, Axis::Z, Axis::X, Axis::Y, Axis::Z);
        TABLE.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_negative(self) -> bool {
        const TABLE: FaceCayleyBits = face_caley_bits(
            true, true, true, false, false, false,
        );
        TABLE.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        const TABLE: FaceCayleyBits = face_caley_bits(
            false, false, false, true, true, true,
        );
        TABLE.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn as_negative(self) -> Self {
        const TABLE: FaceCayley = face_cayley(NegX, NegY, NegZ, NegX, NegY, NegZ);
        TABLE.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn as_positive(self) -> Self {
        const TABLE: FaceCayley = face_cayley(PosX, PosY, PosZ, PosX, PosY, PosZ);
        TABLE.get(self)
    }

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

    #[must_use]
    #[inline(always)]
    pub const fn up(self) -> Self {
        Self::UP_CAYLEY.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn up_at_angle(self, angle: i8) -> Self {
        Self::UP_AT_ANGLE_CAYLEY[(angle & 3) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn left(self) -> Self {
        Self::LEFT_CAYLEY.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn left_at_angle(self, angle: i8) -> Self {
        Self::LEFT_AT_ANGLE_CAYLEY[(angle & 3) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn down(self) -> Self {
        Self::DOWN_CAYLEY.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn down_at_angle(self, angle: i8) -> Self {
        Self::DOWN_AT_ANGLE_CAYLEY[(angle & 3) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn right(self) -> Self {
        Self::RIGHT_CAYLEY.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn right_at_angle(self, angle: i8) -> Self {
        Self::RIGHT_AT_ANGLE_CAYLEY[(angle & 3) as usize].get(self)
    }

    /// Invert all axes.
    #[must_use]
    #[inline(always)]
    pub const fn invert(self) -> Self {
        Self::INVERT_CAYLEY.get(self)
    }

    /// Invert the `X` axis.
    #[must_use]
    #[inline(always)]
    pub const fn invert_x(self) -> Self {
        Self::INVERT_X_CAYLEY.get(self)
    }

    /// Invert the `Y` axis.
    #[must_use]
    #[inline(always)]
    pub const fn invert_y(self) -> Self {
        Self::INVERT_Y_CAYLEY.get(self)
    }

    /// Invert the `Z` axis.
    #[must_use]
    #[inline(always)]
    pub const fn invert_z(self) -> Self {
        Self::INVERT_Z_CAYLEY.get(self)
    }

    /// Invert the `X` and `Y` axes.
    #[must_use]
    #[inline(always)]
    pub const fn invert_xy(self) -> Self {
        Self::INVERT_XY_CAYLEY.get(self)
    }

    /// Invert the `X` and `Z` axes.
    #[must_use]
    #[inline(always)]
    pub const fn invert_xz(self) -> Self {
        Self::INVERT_XZ_CAYLEY.get(self)
    }

    /// Invert the `Y` and `Z` axes.
    #[must_use]
    #[inline(always)]
    pub const fn invert_yz(self) -> Self {
        Self::INVERT_YZ_CAYLEY.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn invert_vertical(self) -> Self {
        Self::INVERT_VERTICAL_CAYLEY.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn invert_left_right(self) -> Self {
        Self::INVERT_LEFT_RIGHT_CAYLEY.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn invert_front_back(self) -> Self {
        Self::INVERT_FRONT_BACK_CAYLEY.get(self)
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

    #[must_use]
    #[inline(always)]
    pub fn lexicographic_iter() -> <[Face; 6] as IntoIterator>::IntoIter {
        [Face::NegX, Face::NegY, Face::NegZ, Face::PosX, Face::PosY, Face::PosZ].into_iter()
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
