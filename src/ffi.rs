
use crate::{
    Face,
    FaceIter,
    Rotation,
    RotationIter,
};

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
pub extern "C" fn sym_face_pos_x() -> Face {
    Face::PosX
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_pos_y() -> Face {
    Face::PosY
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_pos_z() -> Face {
    Face::PosZ
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_neg_x() -> Face {
    Face::NegX
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_neg_y() -> Face {
    Face::NegY
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_neg_z() -> Face {
    Face::NegZ
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_up(face: Face) -> Face {
    face.up()
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_up_at_angle(face: Face, angle: i8) -> Face {
    face.up_at_angle(angle)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_down(face: Face) -> Face {
    face.down()
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_down_at_angle(face: Face, angle: i8) -> Face {
    face.down_at_angle(angle)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_left(face: Face) -> Face {
    face.left()
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_left_at_angle(face: Face, angle: i8) -> Face {
    face.left_at_angle(angle)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_right(face: Face) -> Face {
    face.right()
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_right_at_angle(face: Face, angle: i8) -> Face {
    face.right_at_angle(angle)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_invert(face: Face) -> Face {
    face.invert()
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_iter() -> FaceIter {
    FaceIter::new()
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_iter_next(iter: &mut FaceIter, out: &mut Face) -> bool {
    match iter.next() {
        Some(next) => {
            *out = next;
            true
        }
        None => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_eq(lhs: Face, rhs: Face) -> bool {
    lhs.eq(rhs)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_face_ne(lhs: Face, rhs: Face) -> bool {
    lhs.ne(rhs)
}

#[unsafe(no_mangle)]
pub static SYM_ROT_IDENTITY: Rotation = Rotation::IDENTITY;

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_new(up: Face, angle: i8) -> Rotation {
    Rotation::new(up, angle)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_angle(rot: Rotation) -> i8 {
    rot.angle()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UpAngle {
    pub up: Face,
    pub angle: i8,
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_up_angle(rot: Rotation) -> UpAngle {
    let (up, angle) = rot.up_angle();
    UpAngle { up, angle }
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_face_dest(rot: Rotation, face: Face) -> Face {
    rot.face_dest(face)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_face_src(rot: Rotation, face: Face) -> Face {
    rot.face_src(face)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_from_up_and_forward(up: Face, forward: Face, out: &mut Rotation) -> bool {
    if let Some(rot) = Rotation::from_up_and_forward(up, forward) {
        *out = rot;
        true
    } else {
        false
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_rotate_by(target: Rotation, rotation: Rotation) -> Rotation {
    target.rotate_by(rotation)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_rotate_by_inverse(target: Rotation, rotation: Rotation) -> Rotation {
    target.rotate_by_inverse(rotation)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_invert(target: Rotation) -> Rotation {
    target.invert()
}

macro_rules! target_rot_func {
    ($(
        fn $fn_name:ident($target:ident : Rotation) -> $ret:ty => $expr:expr
    ),+$(,)?) => {
        $(
            #[unsafe(no_mangle)]
            pub extern "C" fn $fn_name($target: Rotation) -> $ret {
                $expr
            }
        )*
    };
}

target_rot_func! {
    fn sym_rot_neg_x_dest(target: Rotation) -> Face => target.neg_x_dest(),
    fn sym_rot_neg_y_dest(target: Rotation) -> Face => target.neg_y_dest(),
    fn sym_rot_neg_z_dest(target: Rotation) -> Face => target.neg_z_dest(),
    fn sym_rot_pos_x_dest(target: Rotation) -> Face => target.pos_x_dest(),
    fn sym_rot_pos_y_dest(target: Rotation) -> Face => target.pos_y_dest(),
    fn sym_rot_pos_z_dest(target: Rotation) -> Face => target.pos_z_dest(),
    
    fn sym_rot_neg_x_src(target: Rotation) -> Face => target.neg_x_src(),
    fn sym_rot_neg_y_src(target: Rotation) -> Face => target.neg_y_src(),
    fn sym_rot_neg_z_src(target: Rotation) -> Face => target.neg_z_src(),
    fn sym_rot_pos_x_src(target: Rotation) -> Face => target.pos_x_src(),
    fn sym_rot_pos_y_src(target: Rotation) -> Face => target.pos_y_src(),
    fn sym_rot_pos_z_src(target: Rotation) -> Face => target.pos_z_src(),

    fn sym_rot_up(target: Rotation) -> Face => target.up(),
    fn sym_rot_down(target: Rotation) -> Face => target.down(),
    fn sym_rot_left(target: Rotation) -> Face => target.left(),
    fn sym_rot_right(target: Rotation) -> Face => target.right(),
    fn sym_rot_forward(target: Rotation) -> Face => target.forward(),
    fn sym_rot_backward(target: Rotation) -> Face => target.backward(),
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_face_angle(rot: Rotation, face: Face) -> i8 {
    rot.face_angle(face)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_iter() -> RotationIter {
    RotationIter::new()
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_iter_next(iter: &mut RotationIter, out: &mut Rotation) -> bool {
    match iter.next() {
        Some(next) => {
            *out = next;
            true
        }
        None => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_eq(lhs: Rotation, rhs: Rotation) -> bool {
    lhs.eq(rhs)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_ne(lhs: Rotation, rhs: Rotation) -> bool {
    lhs.ne(rhs)
}
