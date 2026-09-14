
use crate::{
    face::{
        Face,
        Face::*,
        AngleDirection,
    },
};

const fn disc(face: Face, angle: i8) -> u8 {
    ((face as u8) << 2) | (angle & 3) as u8
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rotation {
    NX0 = disc(NegX, 0), NX1 = disc(NegX, 1), NX2 = disc(NegX, 2), NX3 = disc(NegX, 3),
    NY0 = disc(NegY, 0), NY1 = disc(NegY, 1), NY2 = disc(NegY, 2), NY3 = disc(NegY, 3),
    NZ0 = disc(NegZ, 0), NZ1 = disc(NegZ, 1), NZ2 = disc(NegZ, 2), NZ3 = disc(NegZ, 3),
    PX0 = disc(PosX, 0), PX1 = disc(PosX, 1), PX2 = disc(PosX, 2), PX3 = disc(PosX, 3),
    PY0 = disc(PosY, 0), PY1 = disc(PosY, 1), PY2 = disc(PosY, 2), PY3 = disc(PosY, 3),
    PZ0 = disc(PosZ, 0), PZ1 = disc(PosZ, 1), PZ2 = disc(PosZ, 2), PZ3 = disc(PosZ, 3),
}
const _: () = isit::assert_niche::<Rotation>();

impl Default for Rotation {
    #[inline(always)]
    fn default() -> Self {
        unsafe { Rotation::from_u8_unchecked(0) }
    }
}

macro_rules! rotate_face_table_func {
    ($(fn $name:ident(self) => $face:ident),+$(,)?) => {
        $(
            #[must_use]
            #[inline(always)]
            pub const fn $name(self) -> Face {
                const TABLE: [Face; 24] = {
                    let mut table = [Face::UP; 24];
                    let mut it = RotationIter::new();
                    while let Some(next) = it.next() {
                        table[next as usize] = next.face_dest(Face::$face);
                    }
                    table
                };
                TABLE[self as usize]
            }
        )*
    };
}

macro_rules! face_dest_func {
    ($(
        fn $name:ident(self) => $face:expr
    ),*$(,)?) => {
        $(
            #[must_use]
            #[inline(always)]
            pub const fn $name(self) -> Face {
                const TABLE: [Face; 24] = {
                    let mut table = [Face::UP; 24];
                    let mut rot = Rotation::iter();
                    while let Some(rot) = rot.next() {
                        table[rot as usize] = rot.face_dest($face);
                    }
                    table
                };
                TABLE[self as usize]
            }
        )*
    };
}

macro_rules! face_src_func {
    ($(
        fn $name:ident(self) => $face:expr
    ),*$(,)?) => {
        $(
            #[must_use]
            #[inline(always)]
            pub const fn $name(self) -> Face {
                const TABLE: [Face; 24] = {
                    let mut table = [Face::UP; 24];
                    let mut rot = Rotation::iter();
                    while let Some(rot) = rot.next() {
                        table[rot as usize] = rot.face_src($face);
                    }
                    table
                };
                TABLE[self as usize]
            }
        )*
    };
}

macro_rules! rotate_by_func {
    ($(fn $func_name:ident(self, $rotation:ident: Self) => $function:ident),*$(,)?) => {
        $(
            #[must_use]
            #[inline(always)]
            pub const fn $func_name(self, $rotation: Self) -> Self {
                // 24 * 24 = 576
                const TABLE: [[Rotation; 24]; 24] = {
                    let mut table = [[Rotation::IDENTITY; 24]; 24];
                    let mut lhs = Rotation::iter();
                    while let Some(lhs) = lhs.next() {
                        let mut rhs = Rotation::iter();
                        while let Some(rhs) = rhs.next() {
                            let up = lhs.up();
                            let fwd = lhs.forward();
                            let reup = rhs.$function(up);
                            let refwd = rhs.$function(fwd);
                            let opt_rot = Rotation::from_up_and_forward(reup, refwd);
                            table[rhs as usize][lhs as usize] = unsafe {
                                const _SAFETY: () = isit::assert_niche::<Rotation>();
                                ::core::mem::transmute(opt_rot)
                            };
                        }
                    }
                    table
                };
                TABLE[$rotation as usize][self as usize]
            }
        )*
    };
}

impl Rotation {
    // --- CONSTANTS ---
    pub const IDENTITY: Self = unsafe {
        Self::from_u8_unchecked(0)
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
        if value > 23 {
            return None;
        }
        Some(unsafe { Self::from_u8_unchecked(value) })
    }

    #[must_use]
    #[inline]
    pub const fn new(up: Face, angle: i8) -> Self {
        unsafe { Self::from_u8_unchecked(
            ((up as u8) << 2) | (angle & 3) as u8
        ) }
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

    // --- QUERIES ---

    #[must_use]
    #[inline(always)]
    pub const fn face_dest(self, face: Face) -> Face {
        // 6 * 24 = 144
        #[must_use]
        const fn rotate_world(world: Face, up: Face, angle: i8) -> Face {
            match world {
                Face::LEFT => up.left_at_angle(angle),
                Face::DOWN => up.invert(),
                Face::FORWARD => up.up_at_angle(angle),
                Face::RIGHT => up.right_at_angle(angle),
                Face::UP => up,
                Face::BACKWARD => up.down_at_angle(angle),
            }
        }
        const TABLE: [[Face; 6]; 24] = {
            let mut table = [[crate::face::Face::UP; 6]; 24];
            let mut face_index = 0;
            let mut rot_index = 0;
            loop {
                let rot_face = unsafe { Face::from_u8_unchecked(rot_index >> 2) };
                let rot_angle = (rot_index & 3) as i8;
                let world_face = unsafe { Face::from_u8_unchecked(face_index) };
                table[rot_index as usize][face_index as usize] = rotate_world(world_face, rot_face, rot_angle);
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
        TABLE[self as usize][face as usize]
    }

    #[must_use]
    #[inline(always)]
    pub const fn face_src(self, face: Face) -> Face {
        const TABLE: [[Face; 6]; 24] = {
            let mut table = [[Face::UP; 6]; 24];
            let mut rot = Rotation::iter();
            while let Some(rot) = rot.next() {
                let mut face = Face::iter();
                while let Some(face) = face.next() {
                    let mut src_face = Face::iter();
                    'found: {
                        while let Some(src) = src_face.next() {
                            let dest = rot.face_dest(src);
                            if dest as u8 == face as u8 {
                                table[rot as usize][face as usize] = src;
                                break 'found;
                            }
                        }
                        panic!("Not found.");
                    }
                }
            }
            table
        };
        TABLE[self as usize][face as usize]
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_up_and_forward(up: Face, forward: Face) -> Option<Self> {
        const TABLE: [[Option<Rotation>; 6]; 6] = {
            let mut table = [[None; 6]; 6];
            let mut up = 0;
            let mut forward = 0;
            loop {
                let up_face = unsafe { Face::from_u8_unchecked(up) };
                let fwd_face = unsafe { Face::from_u8_unchecked(forward) };
                let rotation;
                match Face::ANGLE_DIRECTION {
                    AngleDirection::CW => {
                        if up_face.up().as_u8() == fwd_face.as_u8() {
                            rotation = Some(Rotation::new(up_face, 0));
                        } else if up_face.right().as_u8() == fwd_face.as_u8() {
                            rotation = Some(Rotation::new(up_face, 1));
                        } else if up_face.down().as_u8() == fwd_face.as_u8() {
                            rotation = Some(Rotation::new(up_face, 2));
                        } else if up_face.left().as_u8() == fwd_face.as_u8() {
                            rotation = Some(Rotation::new(up_face, 3));
                        } else {
                            rotation = None;
                        }
                    },
                    AngleDirection::CCW => {
                        if up_face.up().as_u8() == fwd_face.as_u8() {
                            rotation = Some(Rotation::new(up_face, 0));
                        } else if up_face.left().as_u8() == fwd_face.as_u8() {
                            rotation = Some(Rotation::new(up_face, 1));
                        } else if up_face.down().as_u8() == fwd_face.as_u8() {
                            rotation = Some(Rotation::new(up_face, 2));
                        } else if up_face.right().as_u8() == fwd_face.as_u8() {
                            rotation = Some(Rotation::new(up_face, 3));
                        } else {
                            rotation = None;
                        }
                    },
                }
                table[up as usize][forward as usize] = rotation;
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
        TABLE[up as usize][forward as usize]
    }

    rotate_by_func!{
        fn rotate_by(self, rotation: Self) => face_dest,
        fn rotate_by_inverse(self, rotation: Self) => face_src,
    }

    #[must_use]
    #[inline(always)]
    pub const fn invert(self) -> Self {
        const TABLE: [Rotation; 24] = {
            let mut table = [Rotation::IDENTITY; 24];
            let mut rot = Rotation::iter();
            while let Some(rot) = rot.next() {
                table[rot as usize] = Rotation::IDENTITY.rotate_by_inverse(rot);
            }
            table
        };
        TABLE[self as usize]
    }

    // 24 * 6 = 192
    face_dest_func!{
        fn neg_x_dest(self) => Face::NegX,
        fn neg_y_dest(self) => Face::NegY,
        fn neg_z_dest(self) => Face::NegZ,
        fn pos_x_dest(self) => Face::PosX,
        fn pos_y_dest(self) => Face::PosY,
        fn pos_z_dest(self) => Face::PosZ,
    }

    // 24 * 6 = 192
    face_src_func!{
        fn neg_x_src(self) => Face::NegX,
        fn neg_y_src(self) => Face::NegY,
        fn neg_z_src(self) => Face::NegZ,
        fn pos_x_src(self) => Face::PosX,
        fn pos_y_src(self) => Face::PosY,
        fn pos_z_src(self) => Face::PosZ,
    }

    // 24 * 6 = 192
    rotate_face_table_func!{
        fn up(self) => UP,
        fn left(self) => LEFT,
        fn down(self) => DOWN,
        fn right(self) => RIGHT,
        fn forward(self) => FORWARD,
        fn backward(self) => BACKWARD,
    }

    #[must_use]
    #[inline(always)]
    pub const fn face_angle(self, face: Face) -> i8 {
        const TABLE: [[i8; 6]; 24] = {
            let mut table = [[0; 6]; 24];
            let mut rot = Rotation::iter();
            while let Some(rot) = rot.next() {
                let mut face = Face::iter();
                while let Some(face) = face.next() {
                    let src = rot.face_src(face);
                    let src_up = src.up();
                    let src_up_dest = rot.face_dest(src_up);
                    let angle;
                    match Face::ANGLE_DIRECTION {
                        AngleDirection::CW => {
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
                        },
                        AngleDirection::CCW => {
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
                        },
                    }
                    table[rot as usize][face as usize] = angle;
                }
            }
            table
        };
        TABLE[self as usize][face as usize]
    }

    // --- MISCELLANEOUS ---
    
    #[must_use]
    #[inline(always)]
    pub const fn iter() -> RotationIter {
        RotationIter::new()
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
#[derive(Debug, Clone)]
pub struct RotationIter {
    rot: u8,
}

impl RotationIter {
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self { rot: 0 }
    }
    #[must_use]
    pub const fn next(&mut self) -> Option<Rotation> {
        if self.rot < 24 {
            let next = Some(unsafe { Rotation::from_u8_unchecked(self.rot) });
            self.rot += 1;
            next
        } else {
            None
        }
    }
}

impl Iterator for RotationIter {
    type Item = Rotation;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn round_trip_test() {
        for rot_x in Rotation::iter() {
            for face in Face::iter() {
                let dest = rot_x.face_dest(face);
                let src = rot_x.face_src(dest);
                assert_eq!(src, face);
            }
            let inverted = rot_x.invert();
            let rot_inv = inverted.rotate_by(rot_x);
            assert_eq!(rot_inv, Rotation::IDENTITY);
            for rot_y in Rotation::iter() {
                let rotated = rot_x.rotate_by(rot_y);
                let derotated = rotated.rotate_by_inverse(rot_y);
                assert_eq!(rot_x, derotated);
            }
        }
    }

    #[test]
    pub fn associativity_test() {
        for rot_x in Rotation::iter() {
            for rot_y in Rotation::iter() {
                for rot_z in Rotation::iter() {
                    let a = rot_x.rotate_by(rot_y).rotate_by(rot_z);
                    let b = rot_x.rotate_by(rot_y.rotate_by(rot_z));
                    assert_eq!(a, b);
                }
            }
        }
    }
}
