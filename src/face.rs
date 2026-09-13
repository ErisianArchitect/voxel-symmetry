use crate::AxisOrientation;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Face {
    // The order of the discriminants is arbitrary, but it is
    // helpful to choose an ordering that maintains that your
    // base up vector is 0.
    NegX = 4,
    NegY = 3,
    NegZ = 5,
    PosX = 1,
    #[default]
    PosY = 0,
    PosZ = 2,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleDirection {
    ///  Clockwise
    CW = 0,
    /// Counter-clockwise
    CCW = 1,
}

const fn face_at_angle(
    direction: AngleDirection,
    up: FaceCayley<Face>,
    left: FaceCayley<Face>,
    down: FaceCayley<Face>,
    right: FaceCayley<Face>,
) -> [FaceCayley<Face>; 4] {
    match direction {
        AngleDirection::CW => {
            [
                up,
                right,
                down,
                left,
            ]
        }
        AngleDirection::CCW => {
            [
                up,
                left,
                down,
                right,
            ]
        }
    }
}

pub type UsedAxisOrientation = crate::RightHandYUp;

impl Face {
    // --- Configuration Constants
    /// The angle direction determines which direction that
    /// angles increase, whether clockwise or counter-clockwise.
    pub(crate) const ANGLE_DIRECTION: AngleDirection = AngleDirection::CCW;
    
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
    // that orientation. You do not need to change these tables
    // if you change the discriminant order, `face_cayley`
    // handles rearranging the table for the correct
    // discriminant order. The only time that you need to change
    // these tables if if you want to change the orientation of
    // faces.
    //                                   Order: NegX, NegY, NegZ, PosX, PosY, PosZ | (The order determines each face's associated element)
    pub(crate) const UP:    FaceCayley<Face> = face_cayley(PosY, PosZ, PosY, PosY, NegZ, PosY);
    pub(crate) const LEFT:  FaceCayley<Face> = face_cayley(NegZ, NegX, PosX, PosZ, NegX, NegX);
    pub(crate) const DOWN:  FaceCayley<Face> = Self::UP.invert();
    pub(crate) const RIGHT: FaceCayley<Face> = Self::LEFT.invert();

    // Within this implementation of voxel orientations, we are
    // going to use counter-clockwise angles. This means that at
    // angle `0`, a face will be have its `UP` direction facing
    // `UP`, and at angle `1`, the `UP` direction will be facing
    // `LEFT`. If you would like to use
    // counter-clockwise angles, you can change
    // Self::ANGLE_DIRECTION to AngleDirection::CW.
    pub(crate) const UP_AT_ANGLE: [FaceCayley<Face>; 4] = face_at_angle(
        Self::ANGLE_DIRECTION,
        Self::UP,
        Self::LEFT,
        Self::DOWN,
        Self::RIGHT,
    );

    pub(crate) const LEFT_AT_ANGLE: [FaceCayley<Face>; 4] = face_at_angle(
        Self::ANGLE_DIRECTION,
        Self::LEFT,
        Self::DOWN,
        Self::RIGHT,
        Self::UP,
    );

    pub(crate) const DOWN_AT_ANGLE: [FaceCayley<Face>; 4] = face_at_angle(
        Self::ANGLE_DIRECTION,
        Self::DOWN,
        Self::RIGHT,
        Self::UP,
        Self::LEFT,
    );

    pub(crate) const RIGHT_AT_ANGLE: [FaceCayley<Face>; 4] = face_at_angle(
        Self::ANGLE_DIRECTION,
        Self::RIGHT,
        Self::UP,
        Self::LEFT,
        Self::DOWN,
    );


    //                                    Order: NegX, NegY, NegZ, PosX, PosY, PosZ
    pub(crate) const INVERT: FaceCayley<Face> = face_cayley(PosX, PosY, PosZ, NegX, NegY, NegZ);

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
        Self::UP.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn up_at_angle(self, angle: i8) -> Self {
        Self::UP_AT_ANGLE[(angle & 3) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn left(self) -> Self {
        Self::LEFT.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn left_at_angle(self, angle: i8) -> Self {
        Self::LEFT_AT_ANGLE[(angle & 3) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn down(self) -> Self {
        Self::DOWN.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn down_at_angle(self, angle: i8) -> Self {
        Self::DOWN_AT_ANGLE[(angle & 3) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn right(self) -> Self {
        Self::RIGHT.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn right_at_angle(self, angle: i8) -> Self {
        Self::RIGHT_AT_ANGLE[(angle & 3) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn invert(self) -> Self {
        Self::INVERT.get(self)
    }
}
