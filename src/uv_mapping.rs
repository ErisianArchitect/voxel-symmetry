
const fn disc(neg_x: bool, neg_y: bool, swap: bool) -> u8 {
    (neg_x as u8) | ((neg_y as u8) << 1) | ((swap as u8) << 2)
}

const PX: bool = false;
const NX: bool = true;
const PY: bool = false;
const NY: bool = true;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UvMap {
    PXPY = disc(PX, PY, false),
    NXPY = disc(NX, PY, false),
    
    PXNY = disc(PX, NY, false),
    NXNY = disc(NX, NY, false),
    
    PYPX = disc(PX, PY, true),
    PYNX = disc(NX, PY, true),
    
    NYPX = disc(PX, NY, true),
    NYNX = disc(NX, NY, true),
}



macro_rules! uv_map_function {
    (@make: $type:ty) => {
        paste::paste!{
            #[must_use]
            #[inline]
            pub const fn [< const_map_ $type >](self, [x, y]: [i8; 2]) -> [i8; 2] {
                match self {
                    UvMap::PXPY => [ x,  y],
                    UvMap::NXPY => [-x,  y],
                    UvMap::PXNY => [ x, -y],
                    UvMap::NXNY => [-x, -y],
            
                    UvMap::PYPX => [ y,  x],
                    UvMap::PYNX => [ y, -x],
                    UvMap::NYPX => [-y,  x],
                    UvMap::NYNX => [-y, -x],
                }
            }
        }
    };
    ($($type:ty),+$(,)?) => {
        $(
            uv_map_function!(@make: $type);
        )*
    };
}

impl UvMap {
    uv_map_function!(i8, i16, i32, i64, i128, f32, f64);

    #[must_use]
    #[inline(always)]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    #[must_use]
    #[inline(always)]
    pub const fn negate_x(self) -> bool {
        self.as_u8() & 1 != 0
    }

    #[must_use]
    #[inline(always)]
    pub const fn negate_y(self) -> bool {
        self.as_u8() & 2 != 0
    }

    #[must_use]
    #[inline(always)]
    pub const fn swap(self) -> bool {
        self.as_u8() & 4 != 0
    }
}
