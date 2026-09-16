
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy)]
struct Align4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AxisTable<T: Copy = Axis>([T; 3]);

impl<T: Copy> AxisTable<T> {
    #[must_use]
    #[inline(always)]
    pub const fn new(arr: [T; 3]) -> Self {
        Self(arr)
    }

    #[must_use]
    #[inline(always)]
    pub const fn get(&self, axis: Axis) -> T {
        self.0[axis as usize]
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Axis {
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

    #[must_use]
    #[inline(always)]
    pub const fn is_orthogonal_to(self, axis: Self) -> bool {
        self.ne(axis)
    }
}
