
use crate::{
    Face,
    FaceIter,
    Rot,
    RotIter,
    CartesianRotIter,
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
pub static SYM_ROT_IDENTITY: Rot = Rot::IDENTITY;

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_new(up: Face, angle: i8) -> Rot {
    Rot::new(up, angle)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_angle(rot: Rot) -> i8 {
    rot.angle()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UpAngle {
    pub up: Face,
    pub angle: i8,
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_up_angle(rot: Rot) -> UpAngle {
    let (up, angle) = rot.up_angle();
    UpAngle { up, angle }
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_face_dest(rot: Rot, face: Face) -> Face {
    rot.face_dest(face)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_face_src(rot: Rot, face: Face) -> Face {
    rot.face_src(face)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_from_up_and_forward(up: Face, forward: Face, out: &mut Rot) -> bool {
    if let Some(rot) = Rot::from_up_and_forward(up, forward) {
        *out = rot;
        true
    } else {
        false
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_rotate_by(target: Rot, rotation: Rot) -> Rot {
    target.rotate_by(rotation)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_rotate_by_inverse(target: Rot, rotation: Rot) -> Rot {
    target.rotate_by_inverse(rotation)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_invert(target: Rot) -> Rot {
    target.invert()
}

macro_rules! target_rot_func {
    ($(
        fn $fn_name:ident($target:ident : Rot) -> $ret:ty => $expr:expr
    ),+$(,)?) => {
        $(
            #[unsafe(no_mangle)]
            pub extern "C" fn $fn_name($target: Rot) -> $ret {
                $expr
            }
        )*
    };
}

target_rot_func! {
    fn sym_rot_neg_x_dest(target: Rot) -> Face => target.neg_x_dest(),
    fn sym_rot_neg_y_dest(target: Rot) -> Face => target.neg_y_dest(),
    fn sym_rot_neg_z_dest(target: Rot) -> Face => target.neg_z_dest(),
    fn sym_rot_pos_x_dest(target: Rot) -> Face => target.pos_x_dest(),
    fn sym_rot_pos_y_dest(target: Rot) -> Face => target.pos_y_dest(),
    fn sym_rot_pos_z_dest(target: Rot) -> Face => target.pos_z_dest(),
    
    fn sym_rot_neg_x_src(target: Rot) -> Face => target.neg_x_src(),
    fn sym_rot_neg_y_src(target: Rot) -> Face => target.neg_y_src(),
    fn sym_rot_neg_z_src(target: Rot) -> Face => target.neg_z_src(),
    fn sym_rot_pos_x_src(target: Rot) -> Face => target.pos_x_src(),
    fn sym_rot_pos_y_src(target: Rot) -> Face => target.pos_y_src(),
    fn sym_rot_pos_z_src(target: Rot) -> Face => target.pos_z_src(),

    fn sym_rot_up(target: Rot) -> Face => target.up(),
    fn sym_rot_down(target: Rot) -> Face => target.down(),
    fn sym_rot_left(target: Rot) -> Face => target.left(),
    fn sym_rot_right(target: Rot) -> Face => target.right(),
    fn sym_rot_forward(target: Rot) -> Face => target.forward(),
    fn sym_rot_backward(target: Rot) -> Face => target.backward(),
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_face_angle(rot: Rot, face: Face) -> i8 {
    rot.face_angle(face)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_iter() -> RotIter {
    RotIter::new()
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_iter_next(iter: &mut RotIter, out: &mut Rot) -> bool {
    match iter.next() {
        Some(next) => {
            *out = next;
            true
        }
        None => false,
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
                pub extern "C" fn [< sym_rot_cartesian_product $n _next>](it: &mut CartesianRotIter<$n>, out: &mut [Rot; $n]) -> bool {
                    match it.next() {
                        Some(result) => {
                            *out = result;
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

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_eq(lhs: Rot, rhs: Rot) -> bool {
    lhs.eq(rhs)
}

#[unsafe(no_mangle)]
pub extern "C" fn sym_rot_ne(lhs: Rot, rhs: Rot) -> bool {
    lhs.ne(rhs)
}
