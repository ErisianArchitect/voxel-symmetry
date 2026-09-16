
use crate::{
    Face,
    FaceTable,
    Face::*,
    Rot,
    RotTable,
    align::*,
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct SymTable<T: Copy = Sym>(pub [T; 48]);

impl<T: Copy> SymTable<T> {
    #[must_use]
    #[inline(always)]
    pub const fn new(table: [T; 48]) -> Self {
        Self(table)
    }

    #[inline(always)]
    pub const fn set(&mut self, sym: Sym, value: T) {
        self.0[sym as usize] = value;
    }

    #[must_use]
    #[inline(always)]
    pub const fn get(&self, sym: Sym) -> T {
        self.0[sym as usize]
    }
}

const fn disc(up: Face, angle: u8, invert: bool) -> u8 {
    ((up as u8) << 3) | (angle << 1) | (invert as u8)
}

const NX: Face = NegX;
const NY: Face = NegY;
const NZ: Face = NegZ;
const PX: Face = PosX;
const PY: Face = PosY;
const PZ: Face = PosZ;
const N: bool = false;
const R: bool = true;

macro_rules! make_sym {
    (
        $(#[$attr:meta])*
        pub enum Sym {
            $(
                [$face:ident $angle:literal $invert:ident]
            ),+$(,)?
        }
    ) => {
        paste::paste!{
            $(#[$attr])*
            #[repr(u8)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum Sym {
                $(
                    [< $face $angle $invert >] = disc($face, $angle, $invert),
                )*
            }
        }
    };
}

make_sym!{
    // [FACE ANGLE REFLECTION] (N for reflection means Non-reflected, R means Reflected.)
    pub enum Sym {
        // Non-reflected
        //     Negative
        [NX 0 N], [NX 1 N], [NX 2 N], [NX 3 N],
        [NY 0 N], [NY 1 N], [NY 2 N], [NY 3 N],
        [NZ 0 N], [NZ 1 N], [NZ 2 N], [NZ 3 N],
        //     Positive
        [PX 0 N], [PX 1 N], [PX 2 N], [PX 3 N],
        [PY 0 N], [PY 1 N], [PY 2 N], [PY 3 N],
        [PZ 0 N], [PZ 1 N], [PZ 2 N], [PZ 3 N],

        // Reflected
        //     Negative
        [NX 0 R], [NX 1 R], [NX 2 R], [NX 3 R],
        [NY 0 R], [NY 1 R], [NY 2 R], [NY 3 R],
        [NZ 0 R], [NZ 1 R], [NZ 2 R], [NZ 3 R],
        //     Positive
        [PX 0 R], [PX 1 R], [PX 2 R], [PX 3 R],
        [PY 0 R], [PY 1 R], [PY 2 R], [PY 3 R],
        [PZ 0 R], [PZ 1 R], [PZ 2 R], [PZ 3 R],
    }
}

impl Sym {
    // --- CONSTANTS ---

    pub const IDENTITY: Self = Self::new(Rot::IDENTITY, false);
    pub const REFLECTED: Self = Self::new(Rot::IDENTITY, true);
    pub const MIN: Self = Self::IDENTITY;
    pub const MAX: Self = Self::new(Rot::MAX, true);

    // --- CONSTRUCTORS ---

    #[must_use]
    #[inline(always)]
    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Option<Self> {
        if value > Self::MAX as u8 {
            return None;
        }
        Some(unsafe { Self::from_u8_unchecked(value) })
    }

    #[must_use]
    #[inline(always)]
    pub const fn new(rot: Rot, reflected: bool) -> Self {
        unsafe { Self::from_u8_unchecked(((rot as u8) << 1) | (reflected as u8)) }
    }

    // --- ACCESSORS ---

    #[must_use]
    #[inline(always)]
    pub const fn rot(self) -> Rot {
        unsafe { Rot::from_u8_unchecked(self as u8 >> 1) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_reflected(self) -> bool {
        (self as u8) & 1 != 0
    }

    // --- QUERIES ---

    #[must_use]
    #[inline(always)]
    pub const fn reflect(self) -> Self {
        unsafe { Self::from_u8_unchecked(self as u8 ^ 1) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn face_dest(self, face: Face) -> Face {
        const TABLE: [Align8<FaceTable<Face>>; 48] = {
            let mut table = [Align8(FaceTable([Face::UP; _])); 48];
            let mut sym_it = Sym::iter();
            while let Some(sym) = sym_it.next() {
                let mut face_it = Face::iter();
                while let Some(face) = face_it.next() {
                    let rot_face = sym.rot().face_dest(face);
                    table[sym as usize].0.set(face, rot_face.invert_if(sym.is_reflected()));
                }
            }
            table
        };
        TABLE[self as usize].0.get(face)
    }

    #[must_use]
    #[inline(always)]
    pub const fn face_src(self, face: Face) -> Face {
        const TABLE: [Align8<FaceTable<Face>>; 48] = {
            let mut table = [Align8(FaceTable([Face::UP; _])); 48];
            let mut sym_it = Sym::iter();
            while let Some(sym) = sym_it.next() {
                let mut face_it = Face::iter();
                while let Some(face) = face_it.next() {
                    let dest = sym.face_dest(face);
                    table[sym as usize].0.set(dest, face);
                }
            }
            table
        };
        TABLE[self as usize].0.get(face)
    }

    pub const fn transform_by(self, transform: Sym) -> Self {
        const fn transform_by(target: Sym, transform: Sym) -> Sym {
            let reflected = target.is_reflected() ^ transform.is_reflected();
            let up = target.face_dest(Face::UP);
            let fwd = target.face_dest(Face::FORWARD);
            let trans_up = transform.face_dest(up);
            let trans_fwd = transform.face_dest(fwd);
            let final_up = trans_up.invert_if(reflected);
            let final_fwd = trans_fwd.invert_if(reflected);
            let rot: Rot = unsafe {
                ::core::mem::transmute(Rot::from_up_and_forward(final_up, final_fwd))
            };
            Sym::new(rot, reflected)
        }
        const TABLE: [Align64<SymTable<Sym>>; 48] = {
            let mut table = [Align64(SymTable([Sym::IDENTITY; _])); _];
            let mut it = Sym::cartesian_product();
            while let Some([lhs, rhs]) = it.next() {
                table[lhs as usize].0.set(rhs, transform_by(lhs, rhs));
            }
            table
        };
        TABLE[self as usize].0.get(transform)
    }

    pub const fn transform_by_inverse(self, transform: Sym) -> Self {
        const fn transform_by_inverse(target: Sym, transform: Sym) -> Sym {
            let reflected = target.is_reflected() ^ transform.is_reflected();
            let up = target.face_dest(Face::UP);
            let fwd = target.face_dest(Face::FORWARD);
            let trans_up = transform.face_src(up);
            let trans_fwd = transform.face_src(fwd);
            let final_up = trans_up.invert_if(reflected);
            let final_fwd = trans_fwd.invert_if(reflected);
            let rot: Rot = unsafe {
                ::core::mem::transmute(Rot::from_up_and_forward(final_up, final_fwd))
            };
            Sym::new(rot, reflected)
        }
        const TABLE: [Align64<SymTable<Sym>>; 48] = {
            let mut table = [Align64(SymTable([Sym::IDENTITY; _])); _];
            let mut it = Sym::cartesian_product();
            while let Some([lhs, rhs]) = it.next() {
                table[lhs as usize].0.set(rhs, transform_by_inverse(lhs, rhs));
            }
            table
        };
        TABLE[self as usize].0.get(transform)
    }

    #[must_use]
    #[inline(always)]
    pub const fn local_transform_by(self, transform: Self) -> Self {
        transform.transform_by(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn local_transform_by_inverse(self, transform: Self) -> Self {
        transform.transform_by_inverse(self)
    }

    // --- MISCELLANEOUS ---

    #[must_use]
    #[inline(always)]
    pub const fn iter() -> SymIter {
        SymIter::new()
    }

    #[must_use]
    #[inline(always)]
    pub const fn cartesian_product<const PRODUCTS: usize>() -> CartesianSymIter<PRODUCTS> {
        CartesianSymIter::new()
    }

}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct SymIter {
    it: u8,
}

impl SymIter {

    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self { it: 0 }
    }

    #[must_use]
    #[inline(always)]
    pub const fn current(&self) -> Option<Sym> {
        Sym::from_u8(self.it)
    }

    #[must_use]
    #[inline]
    pub const fn next(&mut self) -> Option<Sym> {
        match self.current() {
            None => None,
            some => {
                self.it += 1;
                some
            }
        }
    }
}

impl Iterator for SymIter {
    type Item = Sym;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CartesianSymIter<const PRODUCTS: usize> {
    it: [u8; PRODUCTS],
}

#[repr(C)]
#[derive(Clone, Copy)]
union CartTransmuter<const PRODUCTS: usize> {
    u8_prods: [u8; PRODUCTS],
    sym_prods: [Sym; PRODUCTS],
}

impl<const PRODUCTS: usize> CartesianSymIter<PRODUCTS> {
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self { it: [0; _] }
    }

    #[must_use]
    pub const fn current(&mut self) -> Option<[Sym; PRODUCTS]> {
        if const { PRODUCTS == 0 } { return None; }
        if self.it[0] > Rot::MAX as u8 { return None; }
        Some(unsafe {
            CartTransmuter { u8_prods: self.it }.sym_prods
        })
    }

    #[must_use]
    pub const fn next(&mut self) -> Option<[Sym; PRODUCTS]> {
        if const { PRODUCTS == 0 } { return None; }
        if self.it[0] > Sym::MAX as u8 { return None; }
        let result = Some(unsafe {
            CartTransmuter { u8_prods: self.it }.sym_prods
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

impl<const PRODUCTS: usize> Iterator for CartesianSymIter<PRODUCTS> {
    type Item = [Sym; PRODUCTS];

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
        for sym in Sym::iter() {
            for face in Face::iter() {
                let src = sym.face_src(face);
                let dest = sym.face_dest(src);
                assert_eq!(face, dest);
            }
        }
        for [lhs, rhs] in Sym::cartesian_product() {
            let trans = lhs.transform_by(rhs);
            let back = trans.transform_by_inverse(rhs);
            assert_eq!(lhs, back);
        }
    }
}
