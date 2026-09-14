
// The following code uses meta-programming and other
// techniques to prepare the library for the configured
// coordinate system.

const UP_DISC: u8 = 0;
const RIGHT_DISC: u8 = 1;
const FORWARD_DISC: u8 = 2;
const LEFT_DISC: u8 = 3;
const BACKWARD_DISC: u8 = 4;
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

/// A padded Cayley table.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub(crate) struct FaceCayley<T: Copy>([T; 6]);

impl<T: Copy> FaceCayley<T> {
    #[must_use]
    #[inline(always)]
    pub const fn get(self, item: Face) -> T {
        self.0[item as usize]
    }
}

impl FaceCayley<Face> {
    #[must_use]
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
    union TableConvert<T: Copy> {
        uninit: [MaybeUninit<T>; 6],
        init: [T; 6],
    }
    FaceCayley(unsafe { TableConvert { uninit: table }.init })
}

/// The
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AngleDirection {
    /// Counter-clockwise
    #[default]
    CCW = 0,
    ///  Clockwise
    CW = 1,
}

const fn face_at_angle(
    up: FaceCayley<Face>,
    left: FaceCayley<Face>,
    down: FaceCayley<Face>,
    right: FaceCayley<Face>,
) -> [FaceCayley<Face>; 4] {
    match Face::ANGLE_DIRECTION {
        AngleDirection::CCW => {
            [
                up,
                left,
                down,
                right,
            ]
        }
        AngleDirection::CW => {
            [
                up,
                right,
                down,
                left,
            ]
        }
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

    pub const UP: Self = UP_DIRECTION;
    pub const RIGHT: Self = RIGHT_DIRECTION;
    pub const FORWARD: Self = FORWARD_DIRECTION;
    pub const DOWN: Self = UP_DIRECTION.invert();
    pub const LEFT: Self = RIGHT_DIRECTION.invert();
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
    // that orientation. You do not need to change these tables.
    pub(crate) const UP_CAYLEY:    FaceCayley<Face> = face_cayley(
        calc_face_up(NegX),
        calc_face_up(NegY),
        calc_face_up(NegZ),
        calc_face_up(PosX),
        calc_face_up(PosY),
        calc_face_up(PosZ),
    );
    pub(crate) const LEFT_CAYLEY:  FaceCayley<Face> = face_cayley(
        calc_face_left(NegX),
        calc_face_left(NegY),
        calc_face_left(NegZ),
        calc_face_left(PosX),
        calc_face_left(PosY),
        calc_face_left(PosZ),
    );
    pub(crate) const DOWN_CAYLEY:  FaceCayley<Face> = Self::UP_CAYLEY.invert();
    pub(crate) const RIGHT_CAYLEY: FaceCayley<Face> = Self::LEFT_CAYLEY.invert();

    // Within this implementation of voxel orientations, we are
    // going to use counter-clockwise angles. This means that at
    // angle `0`, a face will be have its `UP` direction facing
    // `UP`, and at angle `1`, the `UP` direction will be facing
    // `LEFT`. If you would like to use
    // counter-clockwise angles, you can change
    // Self::ANGLE_DIRECTION to AngleDirection::CW.
    pub(crate) const UP_AT_ANGLE_CAYLEY: [FaceCayley<Face>; 4] = face_at_angle(
        Self::UP_CAYLEY,
        Self::LEFT_CAYLEY,
        Self::DOWN_CAYLEY,
        Self::RIGHT_CAYLEY,
    );

    pub(crate) const LEFT_AT_ANGLE_CAYLEY: [FaceCayley<Face>; 4] = face_at_angle(
        Self::LEFT_CAYLEY,
        Self::DOWN_CAYLEY,
        Self::RIGHT_CAYLEY,
        Self::UP_CAYLEY,
    );

    pub(crate) const DOWN_AT_ANGLE_CAYLEY: [FaceCayley<Face>; 4] = face_at_angle(
        Self::DOWN_CAYLEY,
        Self::RIGHT_CAYLEY,
        Self::UP_CAYLEY,
        Self::LEFT_CAYLEY,
    );

    pub(crate) const RIGHT_AT_ANGLE_CAYLEY: [FaceCayley<Face>; 4] = face_at_angle(
        Self::RIGHT_CAYLEY,
        Self::UP_CAYLEY,
        Self::LEFT_CAYLEY,
        Self::DOWN_CAYLEY,
    );

    //                                                      Order: NegX, NegY, NegZ, PosX, PosY, PosZ
    pub(crate) const INVERT_CAYLEY: FaceCayley<Face> = face_cayley(PosX, PosY, PosZ, NegX, NegY, NegZ);

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

    #[must_use]
    #[inline(always)]
    pub const fn invert(self) -> Self {
        Self::INVERT_CAYLEY.get(self)
    }

    // --- MISCELLANEOUS ---

    #[must_use]
    #[inline(always)]
    pub const fn iter() -> FaceIter {
        FaceIter::new()
    }

    #[must_use]
    #[inline(always)]
    pub const fn eq(self, other: Self) -> bool {
        self as u8 == other as u8
    }

    #[must_use]
    #[inline(always)]
    pub const fn ne(self, other: Self) -> bool {
        self as u8 != other as u8
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Hash)]
pub struct FaceIter {
    pub(crate) face: u8,
}

impl FaceIter {
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            face: 0,
        }
    }

    #[must_use]
    #[inline]
    pub const fn current(&self) -> Option<Face> {
        if self.face >= 6 {
            return None;
        }
        Some(unsafe { Face::from_u8_unchecked(self.face) })
    }

    #[must_use]
    #[inline]
    pub const fn next(&mut self) -> Option<Face> {
        if self.face >= 6 {
            return None;
        }
        let face = unsafe { Face::from_u8_unchecked(self.face) };
        self.face += 1;
        Some(face)
    }
}

impl Iterator for FaceIter {
    type Item = Face;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
