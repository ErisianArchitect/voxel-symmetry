
use ::core::{
    mem::{
        MaybeUninit,
    },
};

use crate::{
    axis::{
        Axis,
    },
    face::{
        Face,
        FaceTable,
        axial_face_table,
        Face::*,
        AngleDirection,
    },
    symmetry::{
        Sym,
    },
    align::*,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RotTable<T: Copy = Rot>(pub [T; 24]);

impl<T: Copy> RotTable<T> {
    #[must_use]
    #[inline(always)]
    pub const fn new(table: [T; 24]) -> Self {
        Self(table)
    }

    #[must_use]
    #[inline(always)]
    pub const fn get(&self, rot: Rot) -> T {
        self.0[rot as usize]
    }

    #[inline(always)]
    pub const fn set(&mut self, rot: Rot, value: T) {
        self.0[rot as usize] = value;
    }
}

impl<T: Copy> std::ops::Index<Rot> for RotTable<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Rot) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T: Copy> std::ops::IndexMut<Rot> for RotTable<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: Rot) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

/// Calculates the rotation discriminant.
///
/// This function is important to ensure that rotation discriminants
/// are accurate to their name.
const fn disc(face: Face, angle: i8) -> u8 {
    ((face as u8) << 2) | (angle & 3) as u8
}

/// Represents a group of 24 voxel rotations.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rot {
    NX0 = disc(NegX, 0), NX1 = disc(NegX, 1), NX2 = disc(NegX, 2), NX3 = disc(NegX, 3),
    NY0 = disc(NegY, 0), NY1 = disc(NegY, 1), NY2 = disc(NegY, 2), NY3 = disc(NegY, 3),
    NZ0 = disc(NegZ, 0), NZ1 = disc(NegZ, 1), NZ2 = disc(NegZ, 2), NZ3 = disc(NegZ, 3),
    PX0 = disc(PosX, 0), PX1 = disc(PosX, 1), PX2 = disc(PosX, 2), PX3 = disc(PosX, 3),
    PY0 = disc(PosY, 0), PY1 = disc(PosY, 1), PY2 = disc(PosY, 2), PY3 = disc(PosY, 3),
    PZ0 = disc(PosZ, 0), PZ1 = disc(PosZ, 1), PZ2 = disc(PosZ, 2), PZ3 = disc(PosZ, 3),
}
// Make sure that there are at least 8 niches. There will be many more, but this ensures it.
const _: () = isit::assert_niche::<Option<Option<Option<Option<Option<Option<Option<Option<Rot>>>>>>>>>();

impl Default for Rot {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Produces a function for rotating a specific face.
macro_rules! rotate_face_func {
    ($(
        $(#[$attr:meta])*
        fn $name:ident(self) => $function:ident($face:expr)
    ),*$(,)?) => {
        $(
            $(#[$attr])*
            #[must_use]
            #[inline(always)]
            pub const fn $name(self) -> Face {
                const TABLE: RotTable<Face> = {
                    let mut table = RotTable([Face::UP; 24]);
                    let mut rot = Rot::iter();
                    while let Some(rot) = rot.next() {
                        table.set(rot, rot.$function($face));
                    }
                    table
                };
                TABLE.get(self)
            }
        )*
    };
}

/// Produces a rotate_by function using a specific face transformation function.
macro_rules! rotate_by_func {
    ($(
        $(#[$attr:meta])*
        fn $func_name:ident(self, $rotation:ident: Self) => $function:ident
    ),*$(,)?) => {
        $(
            $(#[$attr])*
            #[must_use]
            #[inline(always)]
            pub const fn $func_name(self, $rotation: Self) -> Self {
                // 24 * 24 = 576
                const TABLE: [Align32<RotTable<Rot>>; 24] = {
                    let mut table = [Align32(RotTable([Rot::IDENTITY; 24])); 24];
                    let mut it = Rot::cartesian_product();
                    while let Some([lhs, rhs]) = it.next() {
                        let up = lhs.up();
                        let fwd = lhs.forward();
                        let reup = rhs.$function(up);
                        let refwd = rhs.$function(fwd);
                        let opt_rot = Rot::from_up_and_forward(reup, refwd);
                        table[rhs as usize].0.set(lhs, unsafe {
                            const _SAFETY: () = isit::assert_niche::<Rot>();
                            ::core::mem::transmute(opt_rot)
                        });
                    }
                    table
                };
                TABLE[$rotation as usize].0.get(self)
            }
        )*
    };
}

macro_rules! quarter_turn_func {
    ($(
        $(#[$attr:meta])*
        fn $fn_name:ident(self, angle: i8) -> Self => $const_name:ident
    ),+$(,)*) => {
        $(
            $(#[$attr])*
            #[must_use]
            #[inline]
            pub const fn $fn_name(self, angle: i8) -> Self {
                self.rotate_by(Self::$const_name.get(angle))
            }
        )*
    };
}

macro_rules! local_quarter_turn_func {
    ($(
        $(#[$attr:meta])*
        fn $fn_name:ident(self, angle: i8) -> Self => $const_name:ident
    ),+$(,)*) => {
        $(
            $(#[$attr])*
            #[must_use]
            #[inline]
            pub const fn $fn_name(self, angle: i8) -> Self {
                self.local_rotate_by(Self::$const_name.get(angle))
            }
        )*
    };
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RotCycleCount {
    C1 = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
}

impl RotCycleCount {
    #[must_use]
    #[inline(always)]
    pub const unsafe fn from_u8_unchecked(count: u8) -> Self {
        unsafe { ::core::mem::transmute(count) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_u8(count: u8) -> Option<Self> {
        if count > 3 {
            return None;
        }
        Some(unsafe { Self::from_u8_unchecked(count) })
    }

    #[must_use]
    #[inline(always)]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    #[must_use]
    #[inline(always)]
    pub const fn count(self) -> usize {
        self as usize
    }
}

#[repr(u8, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RotAnglesKind {
    Identity = 0,
    Binary(Rot) = 1,
    Ternary([Rot; 2]) = 2,
    Quaternary([Rot; 3]) = 3,
}
const _: () = isit::assert_same_size_align::<RotAnglesKind, u32>();

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RotAngles(RotAnglesKind);
const _: () = isit::assert_same_size_align::<RotAngles, u32>();

impl RotAngles {
    #[must_use]
    #[inline(always)]
    pub const fn from_rot(rot: Rot) -> Self {
        rot.angles()
    }

    #[must_use]
    #[inline]
    pub const fn base(self) -> Rot {
        match self.0 {
            RotAnglesKind::Identity => Rot::IDENTITY,
            RotAnglesKind::Binary(rot) => rot,
            RotAnglesKind::Ternary([rot, _]) => rot,
            RotAnglesKind::Quaternary([rot, _, _]) => rot,
        }
    }

    #[must_use]
    #[inline]
    pub const fn get(self, angle: i8) -> Rot {
        match self.0 {
            RotAnglesKind::Identity => Rot::IDENTITY,
            RotAnglesKind::Binary(rot) => {
                [Rot::IDENTITY, rot][(angle & 1) as usize]
            },
            RotAnglesKind::Ternary([a, b]) => {
                [Rot::IDENTITY, a, b][angle.rem_euclid(3) as usize]
            },
            RotAnglesKind::Quaternary([a, b, c]) => {
                [Rot::IDENTITY, a, b, c][(angle & 3) as usize]
            },
        }

    }

    #[must_use]
    #[inline]
    pub const fn invert(self) -> Self {
        let base = self.base();
        Self::from_rot(base.invert())
    }

    #[must_use]
    #[inline(always)]
    pub const fn cycle_count(self) -> RotCycleCount {
        match self.0 {
            RotAnglesKind::Identity => RotCycleCount::C1,
            RotAnglesKind::Binary(_) => RotCycleCount::C2,
            RotAnglesKind::Ternary(_) => RotCycleCount::C3,
            RotAnglesKind::Quaternary(_) => RotCycleCount::C4,
        }
    }
}

#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuarterTurns([Rot; 4]);

impl QuarterTurns {
    const fn build(rot: Rot) -> Self {
        if rot.cycle_count() as u8 != RotCycleCount::C4 as u8 {
            panic!("Not a quaternary rotation.");
        }
        let next1 = rot.rotate_by(rot);
        let next2 = next1.rotate_by(rot);
        Self([Rot::IDENTITY, rot, next1, next2])
    }

    #[must_use]
    #[inline(always)]
    pub const fn get(self, angle: i8) -> Rot {
        self.0[(angle & 3) as usize]
    }

    #[must_use]
    #[inline(always)]
    pub const fn base(self) -> Rot {
        self.0[1]
    }

    #[must_use]
    #[inline]
    pub const fn invert(self) -> Self {
        Self::build(self.base().invert())
    }
}

/// Represents the conjugacy class of a [Rot].
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConjugacyClass {
    /// The Identity class. One element ([Rot::IDENTITY]).
    Identity = 0,
    /// Edge binary. 180 degree rotations for opposing edges. Six elements.
    EdgeBinary = 1,
    /// Face binary. 180 degree rotations about a face. Three elements.
    FaceBinary = 2,
    /// Ternary. 120 degree rotations around a diagonal. Eight elements.
    Ternary = 3,
    /// Quaternary. 90 degree rotations about a face. Six elements.
    Quaternary = 4,
}

const fn determine_conjugacy_class(rot: Rot) -> ConjugacyClass {
    let cycle_count = rot.cycle_count();
    match cycle_count {
        RotCycleCount::C1 => ConjugacyClass::Identity,
        RotCycleCount::C2 => {
            // Only two axes need to be checked because two axes would be flipped.
            if rot.is_orthogonal() {
                ConjugacyClass::EdgeBinary
            } else {
                ConjugacyClass::FaceBinary
            }
        },
        RotCycleCount::C3 => ConjugacyClass::Ternary,
        RotCycleCount::C4 => ConjugacyClass::Quaternary,
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RotTableBits(u32);

impl RotTableBits {
    #[inline]
    pub const fn set(&mut self, index: Rot, value: bool) {
        let bit = 1 << index as u8;
        if value {
            self.0 |= bit;
        } else {
            self.0 &= !bit;
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn get(self, index: Rot) -> bool {
        let bit = 1 << index as u8;
        self.0 & bit != 0
    }
}

impl Rot {
    // --- CONSTANTS ---
    pub const IDENTITY: Self = unsafe { Self::from_u8_unchecked(0) };
    pub const MIN: Self = Self::IDENTITY;
    pub const MAX: Self = unsafe { Self::from_u8_unchecked(23) };

    pub const UP: Self = Self::new(Face::UP, 0);
    pub const FORWARD: Self = Self::new(Face::FORWARD, 0);
    pub const LEFT: Self = Self::new(Face::LEFT, 0);
    pub const BACKWARD: Self = Self::new(Face::BACKWARD, 0);
    pub const RIGHT: Self = Self::new(Face::RIGHT, 0);
    pub const DOWN: Self = Self::new(Face::DOWN, 0);

    pub const NEG_X: Self = Self::new(NegX, 0);
    pub const NEG_Y: Self = Self::new(NegY, 0);
    pub const NEG_Z: Self = Self::new(NegZ, 0);
    pub const POS_X: Self = Self::new(PosX, 0);
    pub const POS_Y: Self = Self::new(PosY, 0);
    pub const POS_Z: Self = Self::new(PosZ, 0);

    pub const ROTATE_X: QuarterTurns = {
        const FACE: Face = Face::PosX;
        // I'm too lazy to figure out how to make this generic
        // over coordinate systems through geometric means, so
        // I'm just going to brute force it.
        let up = FACE.up();
        let target_up = match Face::ANGLE_DIRECTION {
            AngleDirection::CCW => FACE.left(),
            AngleDirection::CW => FACE.right(),
        };
        let mut it = Rot::iter();
        QuarterTurns::build('result: {
            while let Some(rot) = it.next() {
                if rot.face_dest(FACE).eq(FACE)
                && rot.face_dest(up).eq(target_up) {
                    break 'result rot;
                }
            }
            panic!("Not found.");
        })
    };
    pub const ROTATE_X_CCW: QuarterTurns = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_X.invert(),
            _ => Self::ROTATE_X,
        }
    };
    pub const ROTATE_X_CW: QuarterTurns = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_X,
            _ => Self::ROTATE_X.invert(),
        }
    };

    pub const ROTATE_Y: QuarterTurns = {
        const FACE: Face = Face::PosY;
        // I'm too lazy to figure out how to make this generic
        // over coordinate systems through geometric means, so
        // I'm just going to brute force it.
        let up = FACE.up();
        let target_up = match Face::ANGLE_DIRECTION {
            AngleDirection::CCW => FACE.left(),
            AngleDirection::CW => FACE.right(),
        };
        let mut it = Rot::iter();
        QuarterTurns::build('result: {
            while let Some(rot) = it.next() {
                if rot.face_dest(FACE).eq(FACE)
                && rot.face_dest(up).eq(target_up) {
                    break 'result rot;
                }
            }
            panic!("Not found.");
        })
    };
    pub const ROTATE_Y_CCW: QuarterTurns = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_Y.invert(),
            _ => Self::ROTATE_Y,
        }
    };
    pub const ROTATE_Y_CW: QuarterTurns = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_Y,
            _ => Self::ROTATE_Y.invert(),
        }
    };
    
    pub const ROTATE_Z: QuarterTurns = {
        const FACE: Face = Face::PosZ;
        // I'm too lazy to figure out how to make this generic
        // over coordinate systems through geometric means, so
        // I'm just going to brute force it.
        let up = FACE.up();
        let target_up = match Face::ANGLE_DIRECTION {
            AngleDirection::CCW => FACE.left(),
            AngleDirection::CW => FACE.right(),
        };
        let mut it = Rot::iter();
        QuarterTurns::build('result: {
            while let Some(rot) = it.next() {
                if rot.face_dest(FACE).eq(FACE)
                && rot.face_dest(up).eq(target_up) {
                    break 'result rot;
                }
            }
            panic!("Not found.");
        })
    };
    pub const ROTATE_Z_CCW: QuarterTurns = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_Z.invert(),
            _ => Self::ROTATE_Z,
        }
    };
    pub const ROTATE_Z_CW: QuarterTurns = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_Z,
            _ => Self::ROTATE_Z.invert(),
        }
    };

    pub const ROTATE_NEG_X: QuarterTurns = Self::ROTATE_X.invert();
    pub const ROTATE_NEG_X_CW: QuarterTurns = Self::ROTATE_X_CCW;
    pub const ROTATE_NEG_X_CCW: QuarterTurns = Self::ROTATE_X_CW;

    pub const ROTATE_NEG_Y: QuarterTurns = Self::ROTATE_Y.invert();
    pub const ROTATE_NEG_Y_CW: QuarterTurns = Self::ROTATE_Y_CCW;
    pub const ROTATE_NEG_Y_CCW: QuarterTurns = Self::ROTATE_Y_CW;

    pub const ROTATE_NEG_Z: QuarterTurns = Self::ROTATE_Z.invert();
    pub const ROTATE_NEG_Z_CW: QuarterTurns = Self::ROTATE_Z_CCW;
    pub const ROTATE_NEG_Z_CCW: QuarterTurns = Self::ROTATE_Z_CW;

    pub const ROTATE_POS_X: QuarterTurns = Self::ROTATE_X;
    pub const ROTATE_POS_X_CW: QuarterTurns = Self::ROTATE_X_CW;
    pub const ROTATE_POS_X_CCW: QuarterTurns = Self::ROTATE_X_CCW;
    
    pub const ROTATE_POS_Y: QuarterTurns = Self::ROTATE_Y;
    pub const ROTATE_POS_Y_CW: QuarterTurns = Self::ROTATE_Y_CW;
    pub const ROTATE_POS_Y_CCW: QuarterTurns = Self::ROTATE_Y_CCW;
    
    pub const ROTATE_POS_Z: QuarterTurns = Self::ROTATE_Z;
    pub const ROTATE_POS_Z_CW: QuarterTurns = Self::ROTATE_Z_CW;
    pub const ROTATE_POS_Z_CCW: QuarterTurns = Self::ROTATE_Z_CCW;

    pub const ROTATE_FACE_TABLE: FaceTable<QuarterTurns> = axial_face_table(
        Self::ROTATE_NEG_X,
        Self::ROTATE_NEG_Y,
        Self::ROTATE_NEG_Z,
        Self::ROTATE_POS_X,
        Self::ROTATE_POS_Y,
        Self::ROTATE_POS_Z,
    );

    pub const ROTATE_FACE_CW_TABLE: FaceTable<QuarterTurns> = axial_face_table(
        Self::ROTATE_NEG_X_CW,
        Self::ROTATE_NEG_Y_CW,
        Self::ROTATE_NEG_Z_CW,
        Self::ROTATE_POS_X_CW,
        Self::ROTATE_POS_Y_CW,
        Self::ROTATE_POS_Z_CW,
    );

    pub const ROTATE_FACE_CCW_TABLE: FaceTable<QuarterTurns> = axial_face_table(
        Self::ROTATE_NEG_X_CCW,
        Self::ROTATE_NEG_Y_CCW,
        Self::ROTATE_NEG_Z_CCW,
        Self::ROTATE_POS_X_CCW,
        Self::ROTATE_POS_Y_CCW,
        Self::ROTATE_POS_Z_CCW,
    );
    // --- CONSTRUCTORS ---
    
    #[must_use]
    #[inline(always)]
    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Option<Self> {
        if value > 23 {
            return None;
        }
        Some(unsafe { Self::from_u8_unchecked(value) })
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_u8_wrapping(value: u8) -> Self {
        unsafe { Self::from_u8_unchecked(value % 24) }
    }

    #[must_use]
    #[inline]
    pub const fn new(up: Face, angle: i8) -> Self {
        unsafe { Self::from_u8_unchecked(
            ((up as u8) << 2) | (angle & 3) as u8
        ) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_up(up: Face) -> Self {
        Self::new(up, 0)
    }

    #[must_use]
    #[inline(always)]
    pub const fn with_up(self, up: Face) -> Self {
        unsafe { Self::from_u8_unchecked(
            ((up as u8) << 2) | ((self as u8) & 3)
        ) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn with_angle(self, angle: i8) -> Self {
        unsafe { Self::from_u8_unchecked(
            ((self as u8) & 0b11111100) | (angle & 3) as u8
        ) }
    }

    #[must_use]
    #[inline(always)]
    pub fn update_up<F: FnOnce(Face) -> Face>(&mut self, update: F) {
        *self = self.with_up(update(self.up()))
    }

    #[must_use]
    #[inline(always)]
    pub fn update_angle<F: FnOnce(i8) -> i8>(&mut self, update: F) {
        *self = self.with_angle(update(self.angle()));
    }

    // --- ACCESSORS ---

    #[must_use]
    #[inline(always)]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    #[must_use]
    #[inline(always)]
    pub const fn angle(self) -> i8 {
        (self as i8) & 3
    }

    #[must_use]
    #[inline(always)]
    pub const fn up_angle(self) -> (Face, i8) {
        (self.up(), self.angle())
    }

    // --- BUILDERS ---

    #[must_use]
    #[inline(always)]
    pub const fn sym(self) -> Sym {
        Sym::new(self, false)
    }

    #[must_use]
    #[inline(always)]
    pub const fn reflect(self) -> Sym {
        Sym::new(self, true)
    }

    // --- QUERIES ---
    
    #[must_use]
    #[inline(always)]
    pub const fn face_dest(self, face: Face) -> Face {
        // 6 * 24 = 144
        #[must_use]
        const fn rotate_world_face(world: Face, up: Face, angle: i8) -> Face {
            match world {
                Face::UP => up,
                Face::RIGHT => up.right_at_angle(angle),
                Face::FORWARD => up.up_at_angle(angle),
                Face::LEFT => up.left_at_angle(angle),
                Face::BACKWARD => up.down_at_angle(angle),
                Face::DOWN => up.invert(),
            }
        }
        const TABLE: [Align8<FaceTable<Face>>; 24] = {
            let mut table = [Align8(FaceTable::new([Face::UP; 6])); 24];
            let mut face_index = 0;
            let mut rot_index = 0;
            loop {
                let rot_face = unsafe { Face::from_u8_unchecked(rot_index >> 2) };
                let rot_angle = (rot_index & 3) as i8;
                let world_face = unsafe { Face::from_u8_unchecked(face_index) };
                table[rot_index as usize].0.set(world_face, rotate_world_face(world_face, rot_face, rot_angle));
                if rot_index == 23 {
                    if face_index == 5 {
                        break;
                    }
                    face_index += 1;
                    rot_index = 0;
                } else {
                    rot_index += 1;
                }
            }
            table
        };
        TABLE[self as usize].0.get(face)
    }

    #[must_use]
    #[inline(always)]
    pub const fn face_src(self, face: Face) -> Face {
        const TABLE: [Align8<FaceTable<Face>>; 24] = {
            let mut table = [Align8(FaceTable::new([Face::UP; 6])); 24];
            let mut rot = Rot::iter();
            while let Some(rot) = rot.next() {
                let mut face = Face::iter();
                while let Some(face) = face.next() {
                    let dest = rot.face_dest(face);
                    table[rot as usize].0.set(dest, face);
                }
            }
            table
        };
        TABLE[self as usize].0.get(face)
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_up_and_forward(up: Face, forward: Face) -> Option<Self> {
        const TABLE: [Align8<FaceTable<Option<Rot>>>; 6] = {
            let mut table = [Align8(FaceTable::new([None; 6])); 6];
            let mut up = 0;
            let mut forward = 0;
            loop {
                let up_face = unsafe { Face::from_u8_unchecked(up) };
                let fwd_face = unsafe { Face::from_u8_unchecked(forward) };
                let rotation;
                match Face::ANGLE_DIRECTION {
                    AngleDirection::CW => {
                        if up_face.up().eq(fwd_face) {
                            rotation = Some(Rot::new(up_face, 0));
                        } else if up_face.right().eq(fwd_face) {
                            rotation = Some(Rot::new(up_face, 1));
                        } else if up_face.down().eq(fwd_face) {
                            rotation = Some(Rot::new(up_face, 2));
                        } else if up_face.left().eq(fwd_face) {
                            rotation = Some(Rot::new(up_face, 3));
                        } else {
                            rotation = None;
                        }
                    },
                    AngleDirection::CCW => {
                        if up_face.up().eq(fwd_face) {
                            rotation = Some(Rot::new(up_face, 0));
                        } else if up_face.left().eq(fwd_face) {
                            rotation = Some(Rot::new(up_face, 1));
                        } else if up_face.down().eq(fwd_face) {
                            rotation = Some(Rot::new(up_face, 2));
                        } else if up_face.right().eq(fwd_face) {
                            rotation = Some(Rot::new(up_face, 3));
                        } else {
                            rotation = None;
                        }
                    },
                }
                table[up as usize].0.set(fwd_face, rotation);
                if forward == 5 {
                    if up == 5 {
                        break;
                    }
                    up += 1;
                    forward = 0;
                } else {
                    forward += 1;
                }
            }
            table
        };
        TABLE[up as usize].0.get(forward)
    }

    // 24 * 6 = 192 + 24 * 6 = 192 == 384
    rotate_face_func!{
        /// The destination of [Face::NegX] after rotation.
        fn neg_x_dest(self) => face_dest(Face::NegX),
        /// The destination of [Face::NegY] after rotation.
        fn neg_y_dest(self) => face_dest(Face::NegY),
        /// The destination of [Face::NegZ] after rotation.
        fn neg_z_dest(self) => face_dest(Face::NegZ),
        /// The destination of [Face::PosX] after rotation.
        fn pos_x_dest(self) => face_dest(Face::PosX),
        /// The destination of [Face::PosY] after rotation.
        fn pos_y_dest(self) => face_dest(Face::PosY),
        /// The destination of [Face::PosZ] after rotation.
        fn pos_z_dest(self) => face_dest(Face::PosZ),

        /// The source of [Face::NegX] before rotation.
        fn neg_x_src(self) => face_src(Face::NegX),
        /// The source of [Face::NegY] before rotation.
        fn neg_y_src(self) => face_src(Face::NegY),
        /// The source of [Face::NegZ] before rotation.
        fn neg_z_src(self) => face_src(Face::NegZ),
        /// The source of [Face::PosX] before rotation.
        fn pos_x_src(self) => face_src(Face::PosX),
        /// The source of [Face::PosY] before rotation.
        fn pos_y_src(self) => face_src(Face::PosY),
        /// The source of [Face::PosZ] before rotation.
        fn pos_z_src(self) => face_src(Face::PosZ),
    }

    rotate_by_func!{
        /// Rotate `self` by `rotation`.
        fn rotate_by(self, rotation: Self) => face_dest,
        /// Rotate `self` by the inverse of `rotation`.
        ///
        /// Equivalent to `self.rotate_by(rotation.invert())`.
        fn rotate_by_inverse(self, rotation: Self) => face_src,
    }

    #[must_use]
    #[inline(always)]
    pub const fn rotate_by_self(self) -> Self {
        const TABLE: RotTable<Rot> = {
            let mut table = RotTable([Rot::IDENTITY; 24]);
            let mut it = Rot::iter();
            while let Some(rot) = it.next() {
                table.set(rot, rot.rotate_by(rot));
            }
            table
        };
        TABLE.get(self)
    }

    /// Rotate `self` by `rotation` within the local space of `self`.
    ///
    /// Equivalent to `rotation.rotate_by(self)`.
    #[must_use]
    #[inline(always)]
    pub const fn local_rotate_by(self, rotation: Self) -> Self {
        rotation.rotate_by(self)
    }

    /// Rotate `self` by inverse of `rotation` within the local space of `self`.
    ///
    /// Equivalent to `rotation.invert().rotate_by(self)`.
    #[must_use]
    #[inline(always)]
    pub const fn local_rotate_by_inverse(self, rotation: Self) -> Self {
        rotation.rotate_by_inverse(rotation)
    }

    /// Invert [Rotation]. This gives you a new rotation that can be used to "undo"
    /// rotations made with the source rotation.
    #[must_use]
    #[inline(always)]
    pub const fn invert(self) -> Self {
        const TABLE: [Rot; 24] = {
            let mut table = [Rot::IDENTITY; 24];
            let mut rot = Rot::iter();
            while let Some(rot) = rot.next() {
                table[rot as usize] = Rot::IDENTITY.rotate_by_inverse(rot);
            }
            table
        };
        TABLE[self as usize]
    }

    /// Determines the destination of the Up face after rotation.
    ///
    /// This value is determined by the configured coordinate system.
    #[must_use]
    #[inline(always)]
    pub const fn up(self) -> Face {
        cfg_select!(
            feature = "neg_x_up" => self.neg_x_dest(),
            feature = "neg_y_up" => self.neg_y_dest(),
            feature = "neg_z_up" => self.neg_z_dest(),
            feature = "pos_x_up" => self.pos_x_dest(),
            feature = "pos_y_up" => self.pos_y_dest(),
            feature = "pos_z_up" => self.pos_z_dest(),
        )
    }

    #[must_use]
    #[inline(always)]
    pub const fn up_src(self) -> Face {
        cfg_select!(
            feature = "neg_x_up" => self.neg_x_src(),
            feature = "neg_y_up" => self.neg_y_src(),
            feature = "neg_z_up" => self.neg_z_src(),
            feature = "pos_x_up" => self.pos_x_src(),
            feature = "pos_y_up" => self.pos_y_src(),
            feature = "pos_z_up" => self.pos_z_src(),
        )
    }

    /// Determines the destination of the Forward face after rotation.
    ///
    /// This value is determined by the configured coordinate system.
    #[must_use]
    #[inline(always)]
    pub const fn forward(self) -> Face {
        cfg_select!(
            feature = "neg_x_forward" => self.neg_x_dest(),
            feature = "neg_y_forward" => self.neg_y_dest(),
            feature = "neg_z_forward" => self.neg_z_dest(),
            feature = "pos_x_forward" => self.pos_x_dest(),
            feature = "pos_y_forward" => self.pos_y_dest(),
            feature = "pos_z_forward" => self.pos_z_dest(),
        )
    }

    #[must_use]
    #[inline(always)]
    pub const fn forward_src(self) -> Face {
        cfg_select!(
            feature = "neg_x_forward" => self.neg_x_src(),
            feature = "neg_y_forward" => self.neg_y_src(),
            feature = "neg_z_forward" => self.neg_z_src(),
            feature = "pos_x_forward" => self.pos_x_src(),
            feature = "pos_y_forward" => self.pos_y_src(),
            feature = "pos_z_forward" => self.pos_z_src(),
        )
    }

    /// Determines the destination of the Right face after rotation.
    ///
    /// This value is determined by the configured coordinate system.
    #[must_use]
    #[inline(always)]
    pub const fn right(self) -> Face {
        cfg_select!(
            feature = "neg_x_right" => self.neg_x_dest(),
            feature = "neg_y_right" => self.neg_y_dest(),
            feature = "neg_z_right" => self.neg_z_dest(),
            feature = "pos_x_right" => self.pos_x_dest(),
            feature = "pos_y_right" => self.pos_y_dest(),
            feature = "pos_z_right" => self.pos_z_dest(),
        )
    }

    #[must_use]
    #[inline(always)]
    pub const fn right_src(self) -> Face {
        cfg_select!(
            feature = "neg_x_right" => self.neg_x_src(),
            feature = "neg_y_right" => self.neg_y_src(),
            feature = "neg_z_right" => self.neg_z_src(),
            feature = "pos_x_right" => self.pos_x_src(),
            feature = "pos_y_right" => self.pos_y_src(),
            feature = "pos_z_right" => self.pos_z_src(),
        )
    }

    /// Determines the destination of the Down face after rotation.
    ///
    /// This value is determined by the configured coordinate system.
    #[must_use]
    #[inline(always)]
    pub const fn down(self) -> Face {
        cfg_select!(
            feature = "neg_x_up" => self.pos_x_dest(),
            feature = "neg_y_up" => self.pos_y_dest(),
            feature = "neg_z_up" => self.pos_z_dest(),
            feature = "pos_x_up" => self.neg_x_dest(),
            feature = "pos_y_up" => self.neg_y_dest(),
            feature = "pos_z_up" => self.neg_z_dest(),
        )
    }

    #[must_use]
    #[inline(always)]
    pub const fn down_src(self) -> Face {
        cfg_select!(
            feature = "neg_x_up" => self.pos_x_src(),
            feature = "neg_y_up" => self.pos_y_src(),
            feature = "neg_z_up" => self.pos_z_src(),
            feature = "pos_x_up" => self.neg_x_src(),
            feature = "pos_y_up" => self.neg_y_src(),
            feature = "pos_z_up" => self.neg_z_src(),
        )
    }

    /// Determines the destination of the Backward face after rotation.
    ///
    /// This value is determined by the configured coordinate system.
    #[must_use]
    #[inline(always)]
    pub const fn backward(self) -> Face {
        cfg_select!(
            feature = "neg_x_forward" => self.pos_x_dest(),
            feature = "neg_y_forward" => self.pos_y_dest(),
            feature = "neg_z_forward" => self.pos_z_dest(),
            feature = "pos_x_forward" => self.neg_x_dest(),
            feature = "pos_y_forward" => self.neg_y_dest(),
            feature = "pos_z_forward" => self.neg_z_dest(),
        )
    }

    #[must_use]
    #[inline(always)]
    pub const fn backward_src(self) -> Face {
        cfg_select!(
            feature = "neg_x_forward" => self.pos_x_src(),
            feature = "neg_y_forward" => self.pos_y_src(),
            feature = "neg_z_forward" => self.pos_z_src(),
            feature = "pos_x_forward" => self.neg_x_src(),
            feature = "pos_y_forward" => self.neg_y_src(),
            feature = "pos_z_forward" => self.neg_z_src(),
        )
    }

    /// Determines the destination of the Left face after rotation.
    ///
    /// This value is determined by the configured coordinate system.
    #[must_use]
    #[inline(always)]
    pub const fn left(self) -> Face {
        cfg_select!(
            feature = "neg_x_right" => self.pos_x_dest(),
            feature = "neg_y_right" => self.pos_y_dest(),
            feature = "neg_z_right" => self.pos_z_dest(),
            feature = "pos_x_right" => self.neg_x_dest(),
            feature = "pos_y_right" => self.neg_y_dest(),
            feature = "pos_z_right" => self.neg_z_dest(),
        )
    }

    #[must_use]
    #[inline(always)]
    pub const fn left_src(self) -> Face {
        cfg_select!(
            feature = "neg_x_right" => self.pos_x_src(),
            feature = "neg_y_right" => self.pos_y_src(),
            feature = "neg_z_right" => self.pos_z_src(),
            feature = "pos_x_right" => self.neg_x_src(),
            feature = "pos_y_right" => self.neg_y_src(),
            feature = "pos_z_right" => self.neg_z_src(),
        )
    }

    /// Determines the global angle of the source face at the position of the requested `face`.
    #[must_use]
    #[inline(always)]
    pub const fn src_face_angle(self, face: Face) -> i8 {
        // 24 * 6 = 144
        const TABLE: [Align8<FaceTable<i8>>; 24] = {
            let mut table = [Align8(FaceTable([0; 6])); 24];
            let mut rot = Rot::iter();
            while let Some(rot) = rot.next() {
                let mut face = Face::iter();
                while let Some(face) = face.next() {
                    let src = rot.face_src(face);
                    let src_up = src.up();
                    let src_up_dest = rot.face_dest(src_up);
                    let angle;
                    cfg_select!{
                        feature = "clockwise-angles" => {
                            if src_up_dest.eq(face.up()) {
                                angle = 0;
                            } else if src_up_dest.eq(face.right()) {
                                angle = 1;
                            } else if src_up_dest.eq(face.down()) {
                                angle = 2;
                            } else if src_up_dest.eq(face.left()) {
                                angle = 3;
                            } else {
                                unreachable!()
                            }
                        }
                        not(feature = "clockwise-angles") => {
                            if src_up_dest.eq(face.up()) {
                                angle = 0;
                            } else if src_up_dest.eq(face.left()) {
                                angle = 1;
                            } else if src_up_dest.eq(face.down()) {
                                angle = 2;
                            } else if src_up_dest.eq(face.right()) {
                                angle = 3;
                            } else {
                                unreachable!()
                            }
                        }
                    }
                    table[rot as usize].0.set(face, angle);
                }
            }
            table
        };
        TABLE[self as usize].0.get(face)
    }

    /// Determines the global angle of `face` in its local position.
    #[must_use]
    #[inline(always)]
    pub const fn dest_face_angle(self, face: Face) -> i8 {
        const TABLE: [Align8<FaceTable<i8>>; 24] = {
            let mut table = [Align8(FaceTable::new([0; 6])); 24];
            let mut rot_it = Rot::iter();
            while let Some(rot) = rot_it.next() {
                let mut face_it = Face::iter();
                while let Some(face) = face_it.next() {
                    table[rot as usize].0.set(face, rot.src_face_angle(rot.face_dest(face)));
                }
            }
            table
        };
        TABLE[self as usize].0.get(face)
    }

    #[must_use]
    #[inline(always)]
    pub const fn diff(self, other: Self) -> Self {
        // 24 * 24 = 576
        const TABLE: [Align32<RotTable<Rot>>; 24] = {
            let mut table = [Align32(RotTable([Rot::IDENTITY; 24])); 24];
            let mut prod = Rot::cartesian_product();
            while let Some([lhs, rhs]) = prod.next() {
                table[lhs as usize].0.set(rhs, lhs.invert().rotate_by(rhs));
            }
            table
        };
        TABLE[self as usize].0.get(other)
    }

    #[must_use]
    #[inline(always)]
    pub const fn conjugate(self, rotation: Self) -> Self {
        // 24 * 24 = 576
        const TABLE: [Align32<RotTable<Rot>>; 24] = {
            let mut table = [Align32(RotTable([Rot::IDENTITY; 24])); 24];
            let mut it = Rot::cartesian_product();
            while let Some([lhs, rhs]) = it.next() {
                table[lhs as usize].0.set(rhs, lhs.invert().rotate_by(rhs).rotate_by(lhs));
            }
            table
        };
        TABLE[self as usize].0.get(rotation)
    }

    #[must_use]
    #[inline(always)]
    pub const fn cycle_count(self) -> RotCycleCount {
        const fn count_cycle(rot: Rot) -> u8 {
            let mut count = 1;
            let mut next = Rot::IDENTITY;
            loop {
                next = next.rotate_by(rot);
                if next.ne(Rot::IDENTITY) {
                    count += 1;
                } else {
                    break;
                }
            }
            count
        }
        const TABLE: RotTable<RotCycleCount> = {
            let mut table = RotTable::new([RotCycleCount::C1; _]);
            let mut it = Rot::iter();
            let mut bits = 0u8;
            while let Some(rot) = it.next() {
                let cycle_count = count_cycle(rot);
                if !matches!(cycle_count, 1..=4) {
                    panic!("Out of range.");
                }
                table.set(rot, unsafe { RotCycleCount::from_u8_unchecked(cycle_count) });
                bits |= 1 << cycle_count;
            }
            // This check ensures that all counts from 1 to 4 are specified.
            if bits != 30 {
                panic!("Not all cycle counts that were expected were encountered.");
            }
            table
        };
        TABLE.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn angles(self) -> RotAngles {
        const TABLE: RotTable<RotAngles> = {
            let mut table = RotTable::new([RotAngles(RotAnglesKind::Identity); _]);
            let mut it = Rot::iter();
            while let Some(rot) = it.next() {
                let angles = match rot.cycle_count() {
                    RotCycleCount::C1 => RotAngles(RotAnglesKind::Identity),
                    RotCycleCount::C2 => RotAngles(RotAnglesKind::Binary(rot)),
                    RotCycleCount::C3 => RotAngles(RotAnglesKind::Ternary([rot, rot.rotate_by_self()])),
                    RotCycleCount::C4 => {
                        let next1 = rot.rotate_by_self();
                        let next2 = next1.rotate_by(rot);
                        RotAngles(RotAnglesKind::Quaternary([rot, next1, next2]))
                    },
                };
                table.set(rot, angles);
            }
            table
        };
        TABLE.get(self)
    }

    quarter_turn_func!(
        fn rotate_x(self, angle: i8) -> Self => ROTATE_X,
        fn rotate_x_cw(self, angle: i8) -> Self => ROTATE_X_CW,
        fn rotate_x_ccw(self, angle: i8) -> Self => ROTATE_X_CCW,

        fn rotate_y(self, angle: i8) -> Self => ROTATE_Y,
        fn rotate_y_cw(self, angle: i8) -> Self => ROTATE_Y_CW,
        fn rotate_y_ccw(self, angle: i8) -> Self => ROTATE_Y_CCW,

        fn rotate_z(self, angle: i8) -> Self => ROTATE_Z,
        fn rotate_z_cw(self, angle: i8) -> Self => ROTATE_Z_CW,
        fn rotate_z_ccw(self, angle: i8) -> Self => ROTATE_Z_CCW,

        fn rotate_neg_x(self, angle: i8) -> Self => ROTATE_NEG_X,
        fn rotate_neg_x_cw(self, angle: i8) -> Self => ROTATE_NEG_X_CW,
        fn rotate_neg_x_ccw(self, angle: i8) -> Self => ROTATE_NEG_X_CCW,

        fn rotate_neg_y(self, angle: i8) -> Self => ROTATE_NEG_Y,
        fn rotate_neg_y_cw(self, angle: i8) -> Self => ROTATE_NEG_Y_CW,
        fn rotate_neg_y_ccw(self, angle: i8) -> Self => ROTATE_NEG_Y_CCW,

        fn rotate_neg_z(self, angle: i8) -> Self => ROTATE_NEG_Z,
        fn rotate_neg_z_cw(self, angle: i8) -> Self => ROTATE_NEG_Z_CW,
        fn rotate_neg_z_ccw(self, angle: i8) -> Self => ROTATE_NEG_Z_CCW,

        fn rotate_pos_x(self, angle: i8) -> Self => ROTATE_POS_X,
        fn rotate_pos_x_cw(self, angle: i8) -> Self => ROTATE_POS_X_CW,
        fn rotate_pos_x_ccw(self, angle: i8) -> Self => ROTATE_POS_X_CCW,

        fn rotate_pos_y(self, angle: i8) -> Self => ROTATE_POS_Y,
        fn rotate_pos_y_cw(self, angle: i8) -> Self => ROTATE_POS_Y_CW,
        fn rotate_pos_y_ccw(self, angle: i8) -> Self => ROTATE_POS_Y_CCW,

        fn rotate_pos_z(self, angle: i8) -> Self => ROTATE_POS_Z,
        fn rotate_pos_z_cw(self, angle: i8) -> Self => ROTATE_POS_Z_CW,
        fn rotate_pos_z_ccw(self, angle: i8) -> Self => ROTATE_POS_Z_CCW,
    );
 
    local_quarter_turn_func!(
        fn local_rotate_x(self, angle: i8) -> Self => ROTATE_X,
        fn local_rotate_x_cw(self, angle: i8) -> Self => ROTATE_X_CW,
        fn local_rotate_x_ccw(self, angle: i8) -> Self => ROTATE_X_CCW,

        fn local_rotate_y(self, angle: i8) -> Self => ROTATE_Y,
        fn local_rotate_y_cw(self, angle: i8) -> Self => ROTATE_Y_CW,
        fn local_rotate_y_ccw(self, angle: i8) -> Self => ROTATE_Y_CCW,

        fn local_rotate_z(self, angle: i8) -> Self => ROTATE_Z,
        fn local_rotate_z_cw(self, angle: i8) -> Self => ROTATE_Z_CW,
        fn local_rotate_z_ccw(self, angle: i8) -> Self => ROTATE_Z_CCW,

        fn local_rotate_neg_x(self, angle: i8) -> Self => ROTATE_NEG_X,
        fn local_rotate_neg_x_cw(self, angle: i8) -> Self => ROTATE_NEG_X_CW,
        fn local_rotate_neg_x_ccw(self, angle: i8) -> Self => ROTATE_NEG_X_CCW,

        fn local_rotate_neg_y(self, angle: i8) -> Self => ROTATE_NEG_Y,
        fn local_rotate_neg_y_cw(self, angle: i8) -> Self => ROTATE_NEG_Y_CW,
        fn local_rotate_neg_y_ccw(self, angle: i8) -> Self => ROTATE_NEG_Y_CCW,

        fn local_rotate_neg_z(self, angle: i8) -> Self => ROTATE_NEG_Z,
        fn local_rotate_neg_z_cw(self, angle: i8) -> Self => ROTATE_NEG_Z_CW,
        fn local_rotate_neg_z_ccw(self, angle: i8) -> Self => ROTATE_NEG_Z_CCW,

        fn local_rotate_pos_x(self, angle: i8) -> Self => ROTATE_POS_X,
        fn local_rotate_pos_x_cw(self, angle: i8) -> Self => ROTATE_POS_X_CW,
        fn local_rotate_pos_x_ccw(self, angle: i8) -> Self => ROTATE_POS_X_CCW,

        fn local_rotate_pos_y(self, angle: i8) -> Self => ROTATE_POS_Y,
        fn local_rotate_pos_y_cw(self, angle: i8) -> Self => ROTATE_POS_Y_CW,
        fn local_rotate_pos_y_ccw(self, angle: i8) -> Self => ROTATE_POS_Y_CCW,

        fn local_rotate_pos_z(self, angle: i8) -> Self => ROTATE_POS_Z,
        fn local_rotate_pos_z_cw(self, angle: i8) -> Self => ROTATE_POS_Z_CW,
        fn local_rotate_pos_z_ccw(self, angle: i8) -> Self => ROTATE_POS_Z_CCW,
    );

    /// Determines if any of the rotated axes are orthogonal to itself.
    #[must_use]
    #[inline(always)]
    pub const fn is_orthogonal(self) -> bool {
        const TABLE: RotTableBits = {
            let mut bits = RotTableBits(0);
            let mut it = Rot::iter();
            while let Some(rot) = it.next() {
                if rot.pos_x_dest().axis().is_orthogonal_to(Axis::X)
                || rot.pos_y_dest().axis().is_orthogonal_to(Axis::Y) {
                    bits.set(rot, true);
                }
            }
            bits
        };
        TABLE.get(self)
    }
 
    // --- MISCELLANEOUS ---
    
    #[must_use]
    #[inline(always)]
    pub const fn iter() -> RotIter {
        RotIter::new()
    }

    /// Ordered iteration over all combinations of pairs.
    ///
    /// Elements on the right side increase before elements on
    /// the left side, much like how numbers increase.
    /// [https://en.wikipedia.org/wiki/Cartesian_product]
    #[must_use]
    #[inline(always)]
    pub const fn cartesian_product<const PRODUCTS: usize>() -> CartesianRotIter<PRODUCTS> {
        CartesianRotIter::new()
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
pub struct RotIter {
    rot: u8,
}

impl RotIter {
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self { rot: 0 }
    }

    #[must_use]
    #[inline(always)]
    pub const fn current(&self) -> Option<Rot> {
        Rot::from_u8(self.rot)
    }
    
    #[must_use]
    #[inline]
    pub const fn next(&mut self) -> Option<Rot> {
        match self.current() {
            None => None,
            some => {
                self.rot += 1;
                some
            }
        }
    }
}

impl Iterator for RotIter {
    type Item = Rot;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CartesianRotIter<const PRODUCTS: usize> {
    it: [u8; PRODUCTS],
}

#[repr(C)]
#[derive(Clone, Copy)]
union CartTransmuter<const PRODUCTS: usize> {
    u8_prods: [u8; PRODUCTS],
    rot_prods: [Rot; PRODUCTS],
}

impl<const PRODUCTS: usize> CartesianRotIter<PRODUCTS> {
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self { it: [0; _] }
    }

    #[must_use]
    pub const fn current(&mut self) -> Option<[Rot; PRODUCTS]> {
        if const { PRODUCTS == 0 } { return None; }
        if self.it[0] > Rot::MAX as u8 { return None; }
        Some(unsafe {
            CartTransmuter { u8_prods: self.it }.rot_prods
        })
    }

    #[must_use]
    pub const fn next(&mut self) -> Option<[Rot; PRODUCTS]> {
        if const { PRODUCTS == 0 } { return None; }
        if self.it[0] > Rot::MAX as u8 { return None; }
        let result = Some(unsafe {
            CartTransmuter { u8_prods: self.it }.rot_prods
        });
        let mut i = PRODUCTS;
        loop {
            i -= 1;
            if i == 0 || self.it[i] < 23 {
                self.it[i] += 1;
                break;
            }
            self.it[i] = 0;
        }
        result
    }
}

impl<const PRODUCTS: usize> Iterator for CartesianRotIter<PRODUCTS> {
    type Item = [Rot; PRODUCTS];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct CardinalData<T: Copy> {
    pub up: T,
    pub forward: T,
    pub left: T,
    pub backward: T,
    pub right: T,
    pub down: T,
}

macro_rules! make_axial {
    ($(
        // $($comment:literal)?
        $name:ident : $feature:literal
    ),*$(,)?) => {
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub(crate) struct AxialData<T: Copy> {
            $(
                #[cfg(feature = $feature)]
                pub $name: T,
            )*
        }
    };
}

make_axial!(
    // Up
    neg_x: "neg_x_up",
    neg_y: "neg_y_up",
    neg_z: "neg_z_up",
    pos_x: "pos_x_up",
    pos_y: "pos_y_up",
    pos_z: "pos_z_up",
    // Forward
    neg_x: "neg_x_forward",
    neg_y: "neg_y_forward",
    neg_z: "neg_z_forward",
    pos_x: "pos_x_forward",
    pos_y: "pos_y_forward",
    pos_z: "pos_z_forward",
    // Left
    neg_x: "pos_x_right",
    neg_y: "pos_y_right",
    neg_z: "pos_z_right",
    pos_x: "neg_x_right",
    pos_y: "neg_y_right",
    pos_z: "neg_z_right",
    // Backward
    neg_x: "pos_x_forward",
    neg_y: "pos_y_forward",
    neg_z: "pos_z_forward",
    pos_x: "neg_x_forward",
    pos_y: "neg_y_forward",
    pos_z: "neg_z_forward",
    // Right
    neg_x: "neg_x_right",
    neg_y: "neg_y_right",
    neg_z: "neg_z_right",
    pos_x: "pos_x_right",
    pos_y: "pos_y_right",
    pos_z: "pos_z_right",
    // Down
    neg_x: "pos_x_up",
    neg_y: "pos_y_up",
    neg_z: "pos_z_up",
    pos_x: "neg_x_up",
    pos_y: "neg_y_up",
    pos_z: "neg_z_up",
);

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union FaceData<T: Copy> {
    _align: MaybeUninit<u64>,
    pub axial: AxialData<T>,
    pub cardinal: CardinalData<T>,
    pub cayley: FaceTable<T>,
}

#[repr(C)]
pub(crate) struct RotationData {
    pub identity: Rot,
    pub min: Rot,
    pub max: Rot,
    pub face_rotations: FaceData<Rot>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn round_trip_test() {
        for rot_x in Rot::iter() {
            for face in Face::iter() {
                let dest = rot_x.face_dest(face);
                let src = rot_x.face_src(dest);
                assert_eq!(src, face);
            }
            let inverted = rot_x.invert();
            let rot_inv = inverted.rotate_by(rot_x);
            assert_eq!(rot_inv, Rot::IDENTITY);
            for rot_y in Rot::iter() {
                let rotated = rot_x.rotate_by(rot_y);
                let derotated = rotated.rotate_by_inverse(rot_y);
                assert_eq!(rot_x, derotated);
            }
        }
    }

    #[test]
    pub fn associativity_test() {
        for [rot_x, rot_y, rot_z] in Rot::cartesian_product() {
            let a = rot_x.rotate_by(rot_y).rotate_by(rot_z);
            let b = rot_x.rotate_by(rot_y.rotate_by(rot_z));
            assert_eq!(a, b);
        }
    }
}
