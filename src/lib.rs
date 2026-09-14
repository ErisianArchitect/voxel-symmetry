
pub mod face;
pub mod ffi;
pub mod rotation;

pub use face::*;
pub use rotation::*;

pub use Face::*;

macro_rules! check_direction_features {
    ($(
        $on:literal != [$($off:literal),*$(,)?]
    ),+$(,)?) => {
        $(
            $(
                #[cfg(all(
                    feature = $on,
                    feature = $off,
                ))]
                compile_error!(concat!(
                    stringify!($on),
                    " feature conflicts with ",
                    stringify!($off),
                    " feature.",
                ));
            )*
        )*
    };
}

check_direction_features!(
    "neg_x_up" != [
        "neg_y_up", "neg_z_up", "pos_x_up", "pos_y_up", "pos_z_up",
        "neg_x_right", "pos_x_right",
        "neg_x_forward", "pos_x_forward"
    ],
    "neg_x_right" != [
        "neg_y_right", "neg_z_right", "pos_x_right", "pos_y_right", "pos_z_right",
        "pos_x_up",
        "neg_x_forward", "pos_x_forward"
    ],
    "neg_x_forward" != [
        "neg_y_forward", "neg_z_forward", "pos_x_forward", "pos_y_forward", "pos_z_forward",
        "pos_x_up",
        "pos_x_right"
    ],
    "neg_y_up" != [
        "neg_z_up", "pos_x_up", "pos_y_up", "pos_z_up",
        "neg_y_right", "pos_y_right",
        "neg_y_forward", "pos_y_forward"
    ],
    "neg_y_right" != [
        "neg_z_right", "pos_x_right", "pos_y_right", "pos_z_right",
        "pos_y_up",
        "neg_y_forward", "pos_y_forward"
    ],
    "neg_y_forward" != [
        "neg_z_forward", "pos_x_forward", "pos_y_forward", "pos_z_forward",
        "pos_y_up",
        "pos_y_right"
    ],
    "neg_z_up" != [
        "pos_x_up", "pos_y_up", "pos_z_up",
        "neg_z_right", "pos_z_right",
        "neg_z_forward", "pos_z_forward"
    ],
    "neg_z_right" != [
        "pos_x_right", "pos_y_right", "pos_z_right",
        "pos_z_up",
        "neg_z_forward", "pos_z_forward"
    ],
    "neg_z_forward" != [
        "pos_x_forward", "pos_y_forward", "pos_z_forward",
        "pos_z_up",
        "pos_z_right"
    ],
    "pos_x_up" != [
        "pos_y_up", "pos_z_up",
        "pos_x_right",
        "pos_x_forward"
    ],
    "pos_x_right" != [
        "pos_y_right", "pos_z_right",
        "pos_x_forward"
    ],
    "pos_x_forward" != [
        "pos_y_forward", "pos_z_forward"
    ],
    "pos_y_up" != [
        "pos_z_up",
        "pos_y_right",
        "pos_y_forward"
    ],
    "pos_y_right" != [
        "pos_z_right",
        "pos_y_forward"
    ],
    "pos_y_forward" != [
        "pos_z_forward"
    ],
    "pos_z_up" != [
        "pos_z_right",
        "pos_z_forward"
    ],
    "pos_z_right" != [
        "pos_z_forward"
    ],
);

#[cfg(not(any(
    feature = "neg_x_up",
    feature = "neg_y_up",
    feature = "neg_z_up",
    feature = "pos_x_up",
    feature = "pos_y_up",
    feature = "pos_z_up",
)))]
compile_error!("Must have up direction feature.");
#[cfg(not(any(
    feature = "neg_x_right",
    feature = "neg_y_right",
    feature = "neg_z_right",
    feature = "pos_x_right",
    feature = "pos_y_right",
    feature = "pos_z_right",
)))]
compile_error!("Must have right direction feature.");
#[cfg(not(any(
    feature = "neg_x_forward",
    feature = "neg_y_forward",
    feature = "neg_z_forward",
    feature = "pos_x_forward",
    feature = "pos_y_forward",
    feature = "pos_z_forward",
)))]
compile_error!("Must have forward direction feature.");
