
use crate::{
    face::{
        Face,
        Face::*,
        AngleDirection,
    },
};

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
                const TABLE: [Face; 24] = {
                    let mut table = [Face::UP; 24];
                    let mut rot = Rot::iter();
                    while let Some(rot) = rot.next() {
                        table[rot as usize] = rot.$function($face);
                    }
                    table
                };
                TABLE[self as usize]
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
                const TABLE: [[Rot; 24]; 24] = {
                    let mut table = [[Rot::IDENTITY; 24]; 24];
                    let mut it = Rot::cartesian_product();
                    while let Some([lhs, rhs]) = it.next() {
                        let up = lhs.up();
                        let fwd = lhs.forward();
                        let reup = rhs.$function(up);
                        let refwd = rhs.$function(fwd);
                        let opt_rot = Rot::from_up_and_forward(reup, refwd);
                        table[rhs as usize][lhs as usize] = unsafe {
                            const _SAFETY: () = isit::assert_niche::<Rot>();
                            ::core::mem::transmute(opt_rot)
                        };
                    }
                    table
                };
                TABLE[$rotation as usize][self as usize]
            }
        )*
    };
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

    pub const ROTATE_X: Self = {
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
        'result: {
            while let Some(rot) = it.next() {
                if rot.face_dest(FACE).eq(FACE)
                && rot.face_dest(up).eq(target_up) {
                    break 'result rot;
                }
            }
            panic!("Not found.");
        }
    };
    pub const ROTATE_X_CCW: Self = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_X.invert(),
            _ => Self::ROTATE_X,
        }
    };
    pub const ROTATE_X_CW: Self = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_X,
            _ => Self::ROTATE_X.invert(),
        }
    };

    pub const ROTATE_Y: Self = {
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
        'result: {
            while let Some(rot) = it.next() {
                if rot.face_dest(FACE).eq(FACE)
                && rot.face_dest(up).eq(target_up) {
                    break 'result rot;
                }
            }
            panic!("Not found.");
        }
    };
    pub const ROTATE_Y_CCW: Self = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_Y.invert(),
            _ => Self::ROTATE_Y,
        }
    };
    pub const ROTATE_Y_CW: Self = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_Y,
            _ => Self::ROTATE_Y.invert(),
        }
    };
    
    pub const ROTATE_Z: Self = {
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
        'result: {
            while let Some(rot) = it.next() {
                if rot.face_dest(FACE).eq(FACE)
                && rot.face_dest(up).eq(target_up) {
                    break 'result rot;
                }
            }
            panic!("Not found.");
        }
    };
    pub const ROTATE_Z_CCW: Self = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_Z.invert(),
            _ => Self::ROTATE_Z,
        }
    };
    pub const ROTATE_Z_CW: Self = {
        cfg_select! {
            feature = "clockwise-angles" => Self::ROTATE_Z,
            _ => Self::ROTATE_Z.invert(),
        }
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
        const TABLE: [[Face; 6]; 24] = {
            let mut table = [[crate::face::Face::UP; 6]; 24];
            let mut face_index = 0;
            let mut rot_index = 0;
            loop {
                let rot_face = unsafe { Face::from_u8_unchecked(rot_index >> 2) };
                let rot_angle = (rot_index & 3) as i8;
                let world_face = unsafe { Face::from_u8_unchecked(face_index) };
                table[rot_index as usize][face_index as usize] = rotate_world_face(world_face, rot_face, rot_angle);
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
            let mut rot = Rot::iter();
            while let Some(rot) = rot.next() {
                let mut face = Face::iter();
                while let Some(face) = face.next() {
                    let mut src_face = Face::iter();
                    'found: {
                        while let Some(src) = src_face.next() {
                            let dest = rot.face_dest(src);
                            if dest.eq(face) {
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
        const TABLE: [[Option<Rot>; 6]; 6] = {
            let mut table = [[None; 6]; 6];
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

    #[must_use]
    #[inline(always)]
    pub const fn face_angle(self, face: Face) -> i8 {
        // 24 * 6 = 144
        const TABLE: [[i8; 6]; 24] = {
            let mut table = [[0; 6]; 24];
            let mut rot = Rot::iter();
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

    #[must_use]
    #[inline(always)]
    pub const fn diff(self, other: Self) -> Self {
        // 24 * 24 = 576
        const TABLE: [[Rot; 24]; 24] = {
            let mut table = [[Rot::IDENTITY; 24]; 24];
            let mut prod = Rot::cartesian_product();
            while let Some([lhs, rhs]) = prod.next() {
                table[lhs as usize][rhs as usize] = lhs.invert().rotate_by(rhs);
            }
            table
        };
        TABLE[self as usize][other as usize]
    }

    #[must_use]
    #[inline(always)]
    pub const fn conjugate(self, rotation: Self) -> Self {
        // 24 * 24 = 576
        const TABLE: [[Rot; 24]; 24] = {
            let mut table = [[Rot::IDENTITY; 24]; 24];
            let mut it = Rot::cartesian_product();
            while let Some([lhs, rhs]) = it.next() {
                table[lhs as usize][rhs as usize] = lhs.invert().rotate_by(rhs).rotate_by(lhs);
            }
            table
        };
        TABLE[self as usize][rotation as usize]
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
                let rotated = rot_x.local_rotate_by(rot_y);
                let derotated = rotated.local_rotate_by_inverse(rot_y);
                assert_eq!(rot_x, derotated);
            }
        }
    }

    #[test]
    pub fn associativity_test() {
        for rot_x in Rot::iter() {
            for rot_y in Rot::iter() {
                for rot_z in Rot::iter() {
                    let a = rot_x.rotate_by(rot_y).rotate_by(rot_z);
                    let b = rot_x.rotate_by(rot_y.rotate_by(rot_z));
                    assert_eq!(a, b);
                }
            }
        }
    }
}
