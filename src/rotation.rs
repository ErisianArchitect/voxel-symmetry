
use crate::{
    face::{
        Face,
        Face::*,
        AngleDirection,
        UsedAxisOrientation,
    },
    AxisOrientation,
};

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rotation {
    #[default]
    PY0 = 00, PY1 = 01, PY2 = 02, PY3 = 03,
    PX0 = 04, PX1 = 05, PX2 = 06, PX3 = 07,
    PZ0 = 08, PZ1 = 09, PZ2 = 10, PZ3 = 11,
    NY0 = 12, NY1 = 13, NY2 = 14, NY3 = 15,
    NX0 = 16, NX1 = 17, NX2 = 18, NX3 = 19,
    NZ0 = 20, NZ1 = 21, NZ2 = 22, NZ3 = 23,
}
const _: () = isit::assert_niche::<Rotation>();

macro_rules! rotate_face_table_func {
    ($(fn $name:ident(self) => $face:ident),+$(,)?) => {
        $(
            #[must_use]
            #[inline(always)]
            pub const fn $name(self) -> Face {
                const TABLE: [Face; 24] = {
                    let mut table = [UsedAxisOrientation::UP; 24];
                    let mut it = RotationIter::new();
                    while let Some(next) = it.next() {
                        table[next as usize] = next.rotate_face(UsedAxisOrientation::$face);
                    }
                    table
                };
                TABLE[self as usize]
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
    pub const fn rotate_face(self, face: Face) -> Face {
        // 6 * 24 = 144
        #[must_use]
        const fn rotate_world(world: Face, up: Face, angle: i8) -> Face {
            match world {
                UsedAxisOrientation::LEFT => up.left_at_angle(angle),
                UsedAxisOrientation::DOWN => up.invert(),
                UsedAxisOrientation::FORWARD => up.up_at_angle(angle),
                UsedAxisOrientation::RIGHT => up.right_at_angle(angle),
                UsedAxisOrientation::UP => up,
                UsedAxisOrientation::BACKWARD => up.down_at_angle(angle),
            }
        }
        const TABLE: [[Face; 6]; 24] = {
            let mut table = [[crate::face::UsedAxisOrientation::UP; 6]; 24];
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
    pub const fn from_up_and_forward(up: Face, forward: Face) -> Option<Self> {
        const TABLE: [[Option<Rotation>; 6]; 6] = {
            let mut table = [[None; 6]; 6];
            let mut up = 0;
            let mut forward = 0;
            loop {
                let mut rotation = None;
                let up_face = unsafe { Face::from_u8_unchecked(up) };
                let fwd_face = unsafe { Face::from_u8_unchecked(forward) };
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

    #[must_use]
    #[inline(always)]
    pub const fn rotate_with(self, rotation: Self) -> Self {
        const _: () = isit::assert_niche::<Rotation>();
        // 24 * 24 = 576
        const TABLE: [[Rotation; 24]; 24] = {
            let mut table = [[Rotation::IDENTITY; 24]; 24];
            let mut lhs = Rotation::iter();
            while let Some(lhs) = lhs.next() {
                let mut rhs = Rotation::iter();
                while let Some(rhs) = rhs.next() {
                    let up = lhs.up();
                    let fwd = lhs.forward();
                    let reup = rhs.rotate_face(up);
                    let refwd = rhs.rotate_face(fwd);
                    let opt_rot = Rotation::from_up_and_forward(reup, refwd);
                    table[lhs as usize][rhs as usize] = unsafe {
                        ::core::mem::transmute(opt_rot)
                    };
                }
            }
            table
        };
        TABLE[self as usize][rotation as usize]
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn iter() -> RotationIter {
        RotationIter::new()
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
