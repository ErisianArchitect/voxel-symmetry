
use crate::{
    Face,
    FaceTable,
    cardinal_face_table,
    Face::*,
    Rot,
    align::*,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AchiralConjugacyClass {
    Identity = 0,
    EdgeBinary = 1,
    FaceBinary = 2,
    Ternary = 3,
    Quaternary = 4,
    InverseIdentity = 5,
    InverseEdgeBinary = 6,
    InverseFaceBinary = 7,
    InverseTernary = 8,
    InverseQuaternary = 9,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SymCycleCount {
    C1 = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C6 = 6,
}

impl SymCycleCount {
    #[must_use]
    #[inline(always)]
    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }

    #[must_use]
    #[inline]
    pub const fn from_u8(value: u8) -> Option<Self> {
        // This may seem over-the-top and unnecessary, and you would be
        // right, but this ensures correctness.
        const fn make_bits<const COUNT: usize>(items: [SymCycleCount; COUNT]) -> u8 {
            let mut bits = 0u8;
            let mut i = 0usize;
            while i < items.len() {
                bits |= 1 << (items[i] as u8);
                i += 1;
            }
            bits
        }
        const BITS: u8 = make_bits([
            SymCycleCount::C1,
            SymCycleCount::C2,
            SymCycleCount::C3,
            SymCycleCount::C4,
            SymCycleCount::C6,
        ]);
        if BITS & (1 << value) == 0 {
            return None;
        }
        Some(unsafe { Self::from_u8_unchecked(value) })
    }

    #[must_use]
    #[inline(always)]
    pub const fn count(self) -> u8 {
        self as u8
    }
}

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
    // REVIEW: Should the reflected variants also have their discriminants reflected?
    // [FACE ANGLE REFLECTION]
    // (`N` for reflection means Non-reflected, `R` means Reflected)
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

    pub const ROTATE_X: Self = Rot::ROTATE_X.base().sym();
    pub const ROTATE_Y: Self = Rot::ROTATE_Y.base().sym();
    pub const ROTATE_Z: Self = Rot::ROTATE_Z.base().sym();

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
                    let inv_face = face.invert_if(sym.is_reflected());
                    let rot_face = sym.rot().face_dest(inv_face);
                    table[sym as usize].0.set(face, rot_face);
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

    #[must_use]
    #[inline(always)]
    pub const fn transform_by(self, transform: Sym) -> Self {
        const fn transform_by(target: Sym, transform: Sym) -> Sym {
            let reflected = target.is_reflected() ^ transform.is_reflected();
            let up = target.face_dest(Face::UP);
            let fwd = target.face_dest(Face::FORWARD);
            let trans_up = transform.face_dest(up);
            let trans_fwd = transform.face_dest(fwd);
            let inv_up = trans_up.invert_if(reflected);
            let inv_fwd = trans_fwd.invert_if(reflected);
            let rot: Rot = unsafe {
                ::core::mem::transmute(Rot::from_up_and_forward(inv_up, inv_fwd))
            };
            Sym::new(rot, reflected)
        }
        // 64 * 48 = 3072
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

    #[must_use]
    #[inline(always)]
    pub const fn transform_by_inverse(self, transform: Sym) -> Self {
        const fn transform_by_inverse(target: Sym, transform: Sym) -> Sym {
            let reflected = target.is_reflected() ^ transform.is_reflected();
            let up = target.face_dest(Face::UP);
            let fwd = target.face_dest(Face::FORWARD);
            let trans_up = transform.face_src(up);
            let trans_fwd = transform.face_src(fwd);
            let inv_up = trans_up.invert_if(reflected);
            let inv_fwd = trans_fwd.invert_if(reflected);
            let rot: Rot = unsafe {
                ::core::mem::transmute(Rot::from_up_and_forward(inv_up, inv_fwd))
            };
            Sym::new(rot, reflected)
        }
        // 64 * 48 = 3072
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

    /// Determines the angle of the face positioned at the location of `face`.
    ///
    /// This angle determines which direction is the face's up. For reflected
    /// symmetries, the left and right are swapped, which means that the angle
    /// may seem inverted from the perspective of the non-reflected face.
    #[must_use]
    #[inline(always)]
    pub const fn src_face_angle(self, face: Face) -> i8 {
        const TABLE: [Align8<FaceTable<i8>>; 48] = {
            let mut table = [Align8(FaceTable([0; _])); _];
            let mut sym_it = Sym::iter();
            while let Some(sym) = sym_it.next() {
                let mut face_it = Face::iter();
                while let Some(face) = face_it.next() {
                    let src_face = sym.face_src(face);
                    let src_face_up = src_face.up();
                    let src_face_up_dest = sym.face_dest(src_face_up);
                    let angle;
                    cfg_select!{
                        not(feature = "clockwise-angles") => {
                            if src_face_up_dest.eq(face.up()) {
                                angle = 0;
                            } else if src_face_up_dest.eq(face.left()) {
                                angle = 1;
                            } else if src_face_up_dest.eq(face.down()) {
                                angle = 2;
                            } else if src_face_up_dest.eq(face.right()) {
                                angle = 3;
                            } else {
                                unreachable!();
                            }
                        }
                        feature = "clockwise-angles" => {
                            if src_face_up_dest.eq(face.up()) {
                                angle = 0;
                            } else if src_face_up_dest.eq(face.right()) {
                                angle = 1;
                            } else if src_face_up_dest.eq(face.down()) {
                                angle = 2;
                            } else if src_face_up_dest.eq(face.left()) {
                                angle = 3;
                            } else {
                                unreachable!();
                            }
                        }
                    }
                    if src_face_up_dest.ne(face.up_at_angle(angle)) {
                        panic!("Mismatch.");
                    }
                    table[sym as usize].0.set(face, angle);
                }
            }
            table
        };
        TABLE[self as usize].0.get(face)
    }

    /// Determine the angle of `face` in its destination after transformation.
    ///
    /// This angle determines which direction is the face's up. For reflected
    /// symmetries, the left and right are swapped, which means that the angle
    /// may seem inverted from the perspective of the non-reflected face.
    #[must_use]
    #[inline(always)]
    pub const fn dest_face_angle(self, face: Face) -> i8 {
        const TABLE: [Align8<FaceTable<i8>>; 48] = {
            let mut table = [Align8(FaceTable([0; _])); _];
            let mut sym_it = Sym::iter();
            while let Some(sym) = sym_it.next() {
                let mut face_it = Face::iter();
                while let Some(face) = face_it.next() {
                    let dest_face = sym.face_dest(face);
                    let face_up_dest = sym.face_dest(face.up());
                    let angle;
                    cfg_select!{
                        not(feature = "clockwise-angles") => {
                            if face_up_dest.eq(dest_face.up()) {
                                angle = 0;
                            } else if face_up_dest.eq(dest_face.left()) {
                                angle = 1;
                            } else if face_up_dest.eq(dest_face.down()) {
                                angle = 2;
                            } else if face_up_dest.eq(dest_face.right()) {
                                angle = 3;
                            } else {
                                unreachable!()
                            }
                        }
                        feature = "clockwise-angles" => {
                            if face_up_dest.eq(dest_face.up()) {
                                angle = 0;
                            } else if face_up_dest.eq(dest_face.right()) {
                                angle = 1;
                            } else if face_up_dest.eq(dest_face.down()) {
                                angle = 2;
                            } else if face_up_dest.eq(dest_face.left()) {
                                angle = 3;
                            } else {
                                unreachable!()
                            }
                        }
                    }
                    if face_up_dest.ne(dest_face.up_at_angle(angle)) {
                        panic!("Mismatch.");
                    }
                    table[sym as usize].0.set(face, angle);
                }
            }
            table
        };
        TABLE[self as usize].0.get(face)
    }

    #[must_use]
    #[inline(always)]
    pub const fn invert(self) -> Self {
        const TABLE: SymTable<Sym> = {
            let mut table = SymTable([Sym::IDENTITY; _]);
            let mut sym_it = Sym::iter();
            while let Some(sym) = sym_it.next() {
                let inverted = Sym::IDENTITY.transform_by_inverse(sym);
                table.set(sym, inverted);
            }
            table
        };
        TABLE.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn diff(self, other: Self) -> Self {
        const TABLE: [Align64<SymTable<Sym>>; 48] = {
            let mut table = [Align64(SymTable([Sym::IDENTITY; _])); _];
            let mut it = Sym::cartesian_product();
            while let Some([lhs, rhs]) = it.next() {
                table[lhs as usize].0.set(rhs, lhs.invert().transform_by(rhs));
            }
            table
        };
        TABLE[self as usize].0.get(other)
    }

    #[must_use]
    #[inline(always)]
    pub const fn conjugate(self, transform: Self) -> Self {
        const TABLE: [Align64<SymTable<Sym>>; 48] = {
            let mut table = [Align64(SymTable([Sym::IDENTITY; _])); _];
            let mut it = Sym::cartesian_product();
            while let Some([lhs, rhs]) = it.next() {
                table[lhs as usize].0.set(rhs, lhs.invert().transform_by(rhs).transform_by(lhs));
            }
            table
        };
        TABLE[self as usize].0.get(transform)
    }

    #[must_use]
    #[inline(always)]
    pub const fn count_cycles(self) -> SymCycleCount {
        const TABLE: SymTable<SymCycleCount> = {
            let mut table = SymTable([SymCycleCount::C1; _]);
            let mut it = Sym::iter();
            while let Some(sym) = it.next() {
                let mut count = 1;
                let mut current = Sym::IDENTITY;
                let count = loop {
                    current = current.transform_by(sym);
                    if current.eq(Sym::IDENTITY) {
                        match count {
                            0 | 5 | 7.. => {
                                panic!("Invalid count.");
                            }
                            _ => {}
                        }
                        break count;
                    }
                    count += 1;
                };
                table.set(sym, unsafe { SymCycleCount::from_u8_unchecked(count) });
            }
            table
        };
        TABLE.get(self)
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
        if self.it[0] > Sym::MAX as u8 { return None; }
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
            if i == 0 || self.it[i] < 47 {
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
                let dest = sym.face_dest(face);
                let src = sym.face_src(dest);
                assert_eq!(face, src);
            }
            let rot_up = sym.rot().up();
            let rot_fwd = sym.rot().forward();
            let sym_up = sym.face_dest(Face::UP);
            let sym_fwd = sym.face_dest(Face::FORWARD);
            let inv_up = sym_up.invert_if(sym.is_reflected());
            let inv_fwd = sym_fwd.invert_if(sym.is_reflected());
            assert_eq!((rot_up, rot_fwd), (inv_up, inv_fwd));
            let inv = sym.invert();
            for trans in Sym::iter() {
                let to = sym.transform_by(trans);
                let from = to.transform_by_inverse(trans);
                assert_eq!(sym, from);
                let a = trans.transform_by_inverse(sym);
                let b = trans.transform_by(inv);
                assert_eq!(a, b);
                let diff = sym.diff(trans);
                let by_diff = sym.transform_by(diff);
                assert_eq!(trans, by_diff);
                let conj = sym.conjugate(trans);
                let local1 = sym.local_transform_by(trans);
                let local2 = sym.transform_by(conj);
                assert_eq!(local1, local2);
            }
        }
    }

    #[test]
    pub fn associativity_test() {
        for [sym_x, sym_y, sym_z] in Sym::cartesian_product() {
            let a = sym_x.transform_by(sym_y).transform_by(sym_z);
            let b = sym_x.transform_by(sym_y.transform_by(sym_z));
            assert_eq!(a, b);
        }
    }
}
