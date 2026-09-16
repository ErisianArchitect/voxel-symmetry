
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy)]
struct Align4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AxisCayley<T: Copy = Axis>(Align4, [T; 3]);

impl<T: Copy> AxisCayley<T> {
    #[must_use]
    #[inline(always)]
    pub const fn new(arr: [T; 3]) -> Self {
        Self(Align4, arr)
    }

    #[must_use]
    #[inline(always)]
    pub const fn get(&self, axis: Axis) -> T {
        self.1[axis as usize]
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
    
}
