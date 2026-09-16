
macro_rules! make_align {
    ($(
        $align:literal
    ),*$(,)?) => {
        paste::paste!{
            $(
                #[repr(C, align($align))]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub struct [< Align $align >]<T>(pub T);
            )*
        }
    };
}

make_align!(4, 8, 16, 32, 64, 128);
