// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(missing_doc)];

use std::container::Container;
use std::option::{Option, Some, None};
use std::iter::{Iterator, DoubleEndedIterator, Invert, range};
use std::ops;
use std::uint;
use std::fail;

#[deriving(Clone)]
struct SmallBitv {
    /// only the lowest nbits of this value are used. the rest is undefined.
    bits: uint
}

/// a mask that has a 1 for each defined bit in a small_bitv, assuming n bits
#[inline]
fn small_mask(nbits: uint) -> uint {
    (1 << nbits) - 1
}

impl SmallBitv {
    pub fn new(bits: uint) -> SmallBitv {
        SmallBitv {bits: bits}
    }

    #[inline]
    pub fn bits_op(&mut self,
                   right_bits: uint,
                   nbits: uint,
                   f: |uint, uint| -> uint)
                   -> bool {
        let mask = small_mask(nbits);
        let old_b: uint = self.bits;
        let new_b = f(old_b, right_bits);
        self.bits = new_b;
        mask & old_b != mask & new_b
    }

    #[inline]
    pub fn union(&mut self, s: &SmallBitv, nbits: uint) -> bool {
        self.bits_op(s.bits, nbits, |u1, u2| u1 | u2)
    }

    #[inline]
    pub fn intersect(&mut self, s: &SmallBitv, nbits: uint) -> bool {
        self.bits_op(s.bits, nbits, |u1, u2| u1 & u2)
    }

    #[inline]
    pub fn become(&mut self, s: &SmallBitv, nbits: uint) -> bool {
        self.bits_op(s.bits, nbits, |_u1, u2| u2)
    }

    #[inline]
    pub fn difference(&mut self, s: &SmallBitv, nbits: uint) -> bool {
        self.bits_op(s.bits, nbits, |u1, u2| u1 & !u2)
    }

    #[inline]
    pub fn get(&self, i: uint) -> bool {
        (self.bits & (1 << i)) != 0
    }

    #[inline]
    pub fn set(&mut self, i: uint, x: bool) {
        if x {
            self.bits |= 1<<i;
        }
        else {
            self.bits &= !(1<<i);
        }
    }

    #[inline]
    pub fn equals(&self, b: &SmallBitv, nbits: uint) -> bool {
        let mask = small_mask(nbits);
        mask & self.bits == mask & b.bits
    }

    #[inline]
    pub fn clear(&mut self) { self.bits = 0; }

    #[inline]
    pub fn set_all(&mut self) { self.bits = !0; }

    #[inline]
    pub fn is_true(&self, nbits: uint) -> bool {
        small_mask(nbits) & !self.bits == 0
    }

    #[inline]
    pub fn is_false(&self, nbits: uint) -> bool {
        small_mask(nbits) & self.bits == 0
    }

    #[inline]
    pub fn negate(&mut self) { self.bits = !self.bits; }
}

#[deriving(Clone)]
enum BitvVariant { Small(SmallBitv) }

enum Op {Union, Intersect, Assign, Difference}

/// The bitvector type
#[deriving(Clone)]
pub struct Bitv {
    /// Internal representation of the bit vector (small or large)
    priv rep: BitvVariant,
    /// The number of valid bits in the internal representation
    priv nbits: uint
}

fn die() -> ! {
    fail::abort();
}

impl Bitv {
    #[inline]
    fn do_op(&mut self, op: Op, other: &Bitv) -> bool {
        if self.nbits != other.nbits {
            die();
        }
        match self.rep {
          Small(ref mut s) => match other.rep {
            Small(ref s1) => match op {
              Union      => s.union(s1,      self.nbits),
              Intersect  => s.intersect(s1,  self.nbits),
              Assign     => s.become(s1,     self.nbits),
              Difference => s.difference(s1, self.nbits)
            }
          }
        }
    }
}

impl Bitv {
    pub fn new(nbits: uint, init: bool) -> Bitv {
        let rep = if nbits <= uint::bits {
            Small(SmallBitv::new(if init {!0} else {0}))
        }
        else { die() };
        Bitv {rep: rep, nbits: nbits}
    }

    /**
     * Calculates the union of two bitvectors
     *
     * Sets `self` to the union of `self` and `v1`. Both bitvectors must be
     * the same length. Returns `true` if `self` changed.
    */
    #[inline]
    pub fn union(&mut self, v1: &Bitv) -> bool { self.do_op(Union, v1) }

    /**
     * Calculates the intersection of two bitvectors
     *
     * Sets `self` to the intersection of `self` and `v1`. Both bitvectors
     * must be the same length. Returns `true` if `self` changed.
    */
    #[inline]
    pub fn intersect(&mut self, v1: &Bitv) -> bool {
        self.do_op(Intersect, v1)
    }

    /**
     * Assigns the value of `v1` to `self`
     *
     * Both bitvectors must be the same length. Returns `true` if `self` was
     * changed
     */
    #[inline]
    pub fn assign(&mut self, v: &Bitv) -> bool { self.do_op(Assign, v) }

    /// Retrieve the value at index `i`
    #[inline]
    pub fn get(&self, i: uint) -> bool {
        // assert!((i < self.nbits));
        match self.rep {
            Small(ref s) => s.get(i)
        }
    }

    /**
     * Set the value of a bit at a given index
     *
     * `i` must be less than the length of the bitvector.
     */
    #[inline]
    pub fn set(&mut self, i: uint, x: bool) {
      // assert!((i < self.nbits));
      match self.rep {
        Small(ref mut s) => s.set(i, x)
      }
    }

    /**
     * Compares two bitvectors
     *
     * Both bitvectors must be the same length. Returns `true` if both
     * bitvectors contain identical elements.
     */
    #[inline]
    pub fn equal(&self, v1: &Bitv) -> bool {
      if self.nbits != v1.nbits { return false; }
      match self.rep {
        Small(ref b) => match v1.rep {
          Small(ref b1) => b.equals(b1, self.nbits),
        }
      }
    }

    /// Set all bits to 0
    #[inline]
    pub fn clear(&mut self) {
        match self.rep {
            Small(ref mut b) => b.clear(),
        }
    }

    /// Set all bits to 1
    #[inline]
    pub fn set_all(&mut self) {
        match self.rep {
            Small(ref mut b) => b.set_all(),
        }
    }

    /// Invert all bits
    #[inline]
    pub fn negate(&mut self) {
        match self.rep {
            Small(ref mut s) => s.negate(),
        }
    }

    /**
     * Calculate the difference between two bitvectors
     *
     * Sets each element of `v0` to the value of that element minus the
     * element of `v1` at the same index. Both bitvectors must be the same
     * length.
     *
     * Returns `true` if `v0` was changed.
     */
    #[inline]
    pub fn difference(&mut self, v: &Bitv) -> bool {
        self.do_op(Difference, v)
    }

    /// Returns `true` if all bits are 1
    #[inline]
    pub fn is_true(&self) -> bool {
      match self.rep {
        Small(ref b) => b.is_true(self.nbits),
      }
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> BitvIterator<'a> {
        BitvIterator {bitv: self, next_idx: 0, end_idx: self.nbits}
    }

    #[inline]
    pub fn rev_iter<'a>(&'a self) -> Invert<BitvIterator<'a>> {
        self.iter().invert()
    }

    /// Returns `true` if all bits are 0
    pub fn is_false(&self) -> bool {
      match self.rep {
        Small(ref b) => b.is_false(self.nbits),
      }
    }

    pub fn init_to_vec(&self, i: uint) -> uint {
      return if self.get(i) { 1 } else { 0 };
    }

    /**
     * Compare a bitvector to a vector of `bool`.
     *
     * Both the bitvector and vector must have the same length.
     */
    pub fn eq_vec(&self, v: &[bool]) -> bool {
        // assert_eq!(self.nbits, v.len());
        let mut i = 0;
        while i < self.nbits {
            if self.get(i) != v[i] { return false; }
            i = i + 1;
        }
        true
    }

    pub fn ones(&self, f: |uint| -> bool) -> bool {
        range(0u, self.nbits).advance(|i| !self.get(i) || f(i))
    }

}

/**
 * Transform a byte-vector into a `Bitv`. Each byte becomes 8 bits,
 * with the most significant bits of each byte coming first. Each
 * bit becomes `true` if equal to 1 or `false` if equal to 0.
 */
pub fn from_bytes(bytes: &[u8]) -> Bitv {
    from_fn(bytes.len() * 8, |i| {
        let b = bytes[i / 8] as uint;
        let offset = i % 8;
        b >> (7 - offset) & 1 == 1
    })
}

/**
 * Transform a `[bool]` into a `Bitv` by converting each `bool` into a bit.
 */
pub fn from_bools(bools: &[bool]) -> Bitv {
    from_fn(bools.len(), |i| bools[i])
}

/**
 * Create a `Bitv` of the specified length where the value at each
 * index is `f(index)`.
 */
pub fn from_fn(len: uint, f: |index: uint| -> bool) -> Bitv {
    let mut bitv = Bitv::new(len, false);
    for i in range(0u, len) {
        bitv.set(i, f(i));
    }
    bitv
}

impl ops::Index<uint,bool> for Bitv {
    fn index(&self, i: &uint) -> bool {
        self.get(*i)
    }
}

/// An iterator for `Bitv`.
pub struct BitvIterator<'a> {
    priv bitv: &'a Bitv,
    priv next_idx: uint,
    priv end_idx: uint,
}

impl<'a> Iterator<bool> for BitvIterator<'a> {
    #[inline]
    fn next(&mut self) -> Option<bool> {
        if self.next_idx != self.end_idx {
            let idx = self.next_idx;
            self.next_idx += 1;
            Some(self.bitv.get(idx))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        let rem = self.end_idx - self.next_idx;
        (rem, Some(rem))
    }
}

impl<'a> DoubleEndedIterator<bool> for BitvIterator<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<bool> {
        if self.next_idx != self.end_idx {
            self.end_idx -= 1;
            Some(self.bitv.get(self.end_idx))
        } else {
            None
        }
    }
}
