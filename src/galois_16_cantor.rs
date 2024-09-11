//! GF(2^16) implementation in cantor basis specified in [1]
//!
//! This is a `GF(2^16)` extension using a degree 16 irreducible polynomial
//! specified in [1]. The field representation is however is not in standard
//! basis { 1, x, x^2,..,x^16} but rather in cantor basis specified in [1]
//!
//! [1] JAM Grey Paper

use std::ops::{Add, Div, Mul, Sub};

// the irreducible polynomial used as a modulus for the field.
// it is in LE as it makes more sense
// x^{16} + x^5 + x^3 + x^2 + 1 
// 2^0 + 2^2 + 2^3 + 2^5 = 45
//                          6  5  4  3  2  1  0  9  8  7  6  5  4  3  2  1
//                         [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1];
const EXT_POLY: [u8; 3] = [45, 0, 1];
const SHIFTABLE_EXT_POLY : u32 = 65536 + 45;

//Cantor basis in little endian is as follows:
const CANTOR_BASIS: [u16; 16] = [
    0x0001, 0xACCA, 0x3C0E, 0x163E, 0xC582, 0xED2E, 0x914C, 0x4012, 0x6C98, 0x10D8, 0x6A72, 0xB900, 0xFDB8, 0xFB34, 0xFF38, 0x991E,
];

const STANDARD_BASIS_IN_CANTOR: [u16; 16] = [0x0001, 0x4690, 0x65D8, 0x62D0, 0x5734, 0x45F0, 0x53B8, 0x1E38, 0x7CAE, 0x4E38, 0x6708, 0xC25C, 0x7A64, 0x9EAC, 0x1124, 0x523A];

/// The field GF(2^16)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Field;

impl crate::Field for Field {
    const ORDER: usize = 65536;

    type Elem = [u8; 2];

    fn add(a: [u8; 2], b: [u8; 2]) -> [u8; 2] {
        (Element(a) + Element(b)).0
    }

    fn mul(a: [u8; 2], b: [u8; 2]) -> [u8; 2] {
        (Element(a) * Element(b)).0
    }

    fn div(a: [u8; 2], b: [u8; 2]) -> [u8; 2] {
        (Element(a) / Element(b)).0
    }

    fn exp(elem: [u8; 2], n: usize) -> [u8; 2] {
        Element(elem).exp(n).0
    }

    fn zero() -> [u8; 2] {
        [0; 2]
    }

    //first element of the cantor basis is 0x01 in standard basis.
    fn one() -> [u8; 2] {
        [1, 0]
    }

    fn nth_internal(n: usize) -> [u8; 2] {
        [n as u8, (n >> 8) as u8, ]
    }
}

/// Type alias of ReedSolomon over GF(2^16).
pub type ReedSolomon = crate::ReedSolomon<Field>;

/// Type alias of ShardByShard over GF(2^16).
pub type ShardByShard<'a> = crate::ShardByShard<'a, Field>;

/// An element of `GF(2^16)` represented in custom basis in little endian
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Element(pub [u8; 2]);

impl Element {
    // Create the zero element.
    fn zero() -> Self {
        Element([0, 0])
    }

    // A constant element evaluating to `n`.
    fn constant(n: u8) -> Element {
        Element([n, 0])
    }

    // Whether this is the zero element.
    fn is_zero(&self) -> bool {
        self.0 == [0; 2]
    }

    fn exp(mut self, n: usize) -> Element {
        if n == 0 {
            Element::constant(1)
        } else if self == Element::zero() {
            Element::zero()
        } else {
            let x = self;
            for _ in 1..n {
                self = self * x;
            }

            self
        }
    }
   

    // // reduces from some polynomial with degree <= 30.
    // #[inline]
    fn reduce_from_poly(mut x: u32) -> Self {
        for i in 0..16 {
            if (x & (1u32 << (31 - i)))!=0 {
                x ^= SHIFTABLE_EXT_POLY << (15 - i);
            }
        }

        Element::from_standard(x as u16)
        
    }
    
}

impl From<[u8; 2]> for Element {
    fn from(c: [u8; 2]) -> Self {
        Element(c)
    }
}

impl Default for Element {
    fn default() -> Self {
        Element::zero()
    }
}

impl Add for Element {
    type Output = Element;

    //Matrix multiplication distributes over vector addition Q.E.D.
    fn add(self, other: Self) -> Element {
        Element([self.0[0] ^ other.0[0], self.0[1] ^ other.0[1]])
    }
}

impl Sub for Element {
    type Output = Element;

    //Characteristic 2
    fn sub(self, other: Self) -> Element {
        self.add(other)
    }
}

impl Mul for Element {
    type Output = Element;

    //We implement the naive algorithm for multiplication as speed is not a concern i.e.
    //1. convert to standard basis.
    //2. Multiply
    //3. Compute quotient
    //4. Convert back to cantor.
    //
    // For this we need polynomial arithmetic, but somebody has implemented that
    fn mul(self, rhs: Self) -> Element {
        //multiply as polynomials        
        let mul_result = Element::poly_mul(self.into_standard(), rhs.into_standard());
        Element::reduce_from_poly(mul_result)
    }
}

impl Div for Element {
    type Output = Element;

    fn div(self, rhs: Self) -> Element {
        self * rhs.inverse()
    }
}

//Into u16 from little Endian
impl Into<u16> for Element {
        
    fn into(self) -> u16 {
        let mut result : u16 = self.0[1] as u16;
        result <<= 8;
        result += self.0[0] as u16;

        result
    }
}

//From u16 into little Endian
impl From<u16> for Element {
        
    fn from(element_as_u16 :u16) -> Element {
        [element_as_u16 as u8, (element_as_u16 >> 8) as u8].into()
    }
}
    
impl Element {

    fn into_standard(self) -> u16 {
        let mut result : u16 = 0;
        for i in 0..8 {
            if self.0[0] & (1 << i) != 0{
                result ^= CANTOR_BASIS[i] ;
            }

            if self.0[1] & (1 << i) != 0 {
                result ^= CANTOR_BASIS[i + 8];
            }
        }

        result

    }

    fn from_standard(standard_element: u16) -> Element {
        let mut result : u16 = 0;
        for i in 0..16 {
            if standard_element & (1 << i) != 0{
                result ^= STANDARD_BASIS_IN_CANTOR[i];
            }
        }

        result.into()

    }
    
    fn poly_mul(lhs: u16 , mut rhs: u16) -> u32 {
        let mut result: u32 = 0;
        let shiftable_lhs:u32 =  lhs as u32;
        for i in 0..16 {
            // Check if the current bit in rhs is set
            if (rhs & 1) != 0 {
                result ^= shiftable_lhs << i; // Add the shifted polynomial lhs
            }
            rhs >>= 1; // Move to the next bit of b
        }
        result
    }

    fn poly_mul_u32_no_carry(mut lhs: u32, mut rhs: u32) -> u32 {     
        let mut result: u32 = 0;
        for i in 0..32 {
            // Check if the current bit in b is set
            if (rhs & 1) != 0 {
                result ^= lhs << i; // Add the shifted polynomial a
            }
            rhs >>= 1; // Move to the next bit of b
        }
        result
    }


    fn poly_degree(poly: u32) -> u32 {
        32 - poly.leading_zeros()
    }

    // polynomial division (returns quotient and remainder)
    fn poly_divmod(mut a: u32, mut b: u32) -> (u32, u32) {
        let mut quotient = 0;
        let deg_b = Self::poly_degree(b);

        while Self::poly_degree(a) >= deg_b {
            let shift = Self::poly_degree(a) - deg_b;
            quotient ^= 1 << shift; 
            a ^= b << shift;        
        }
        (quotient, a)
    }

    /// Compute the inverse of this field element. Panics if zero.
    // It computies the inverse using Extended Euclidean Algorithm
    fn inverse(self) -> Element {

        if self.is_zero() {
            panic!("Cannot invert 0");
        }

        // Initial values for s and t
        let mut s0 = 1u32;
        let mut s1 = 0u32;
        let mut t0 = 0u32;
        let mut t1 = 1u32;

        let mut a: u32 = SHIFTABLE_EXT_POLY;
        let mut b: u16 = self.into_standard();
        
        while b != 0 {
            // Perform division a / b
            let (quotient, remainder) = Self::poly_divmod(a, b as u32).into();

            // Update a and b for the next iteration
            a = b as u32;
            // we know the remainder has degree less than 16 because b has degree less than 16
            b = remainder as u16;

            // Update s and t
            let new_s = s0 ^ Self::poly_mul_u32_no_carry(quotient, s1); // s = s0 - quotient * s1
            let new_t = t0 ^ Self::poly_mul_u32_no_carry(quotient, t1); // t = t0 - quotient * t1
            s0 = s1;
            s1 = new_s;
            t0 = t1;
            t1 = new_t;
        }

        //We know that t0 has degree 16 because it is in the field
        Element::reduce_from_poly(t0)
            
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Arbitrary;


    #[test]
    fn test_convert_from_cantor_works() {
        //1 x 1 = 1
        let a = Element([1,0]);
        let a_standard :u16 = 1;
        let b = Element([2,0]);
        let b_standard :u16 = CANTOR_BASIS[1];

        assert_eq!(a.into_standard(), a_standard);
        assert_eq!(b.into_standard(), b_standard);
        assert_eq!((a+b).into_standard(), a_standard^b_standard);
    }

    #[test]
    fn test_convert_from_standard_works() {
        //1 x 1 = 1
        let a = Element([1,0]);
        let a_standard :u16 = 1;
        let b = Element([2,0]);
        let b_standard :u16 = CANTOR_BASIS[1];

        assert_eq!(Element::from_standard(a_standard), a);
        assert_eq!(Element::from_standard(b_standard), b);
        assert_eq!(Element::from_standard(a_standard ^ b_standard), a+b);


    }

    #[test]
    fn test_basis_conversion_round_trip_works() {
        let a : Element = CANTOR_BASIS[1].into();
        assert_eq!(Element::from_standard(a.into_standard()), a);
    }

    #[test]
    fn reduce_from_poly_works() {
        let a_standard :u16 = 1;
        let a = Element([1,0]);

        assert_eq!(Element::reduce_from_poly(a_standard as u32), a);
    }
        
    #[test]
    fn test_known_mul() {
        //1 x 1 = 1
        let a = Element([1,0]);
        let b = Element([1,0]);
        let c = Element([1,0]);

        assert_eq!(a * b, c);

        let a: u16 = 2;

        assert_eq!(Element::from_standard(a)*Element::from_standard(a), Element::from_standard(a*a));

        let a = Element([2,0]);
        let asqrt = Element([3,0]);
        
        assert_eq!(a *a, asqrt);
    }


    #[test]
    fn poly_divmod_works() {
        let one : u32 = 1;
        assert_eq!(Element::poly_divmod(one, one), (1, 0));
                let one : u32 = 1;
        assert_eq!(Element::poly_divmod(SHIFTABLE_EXT_POLY, one), (SHIFTABLE_EXT_POLY, 0));
        
        let gen : u32 = 2;
        let (q, r) = Element::poly_divmod(SHIFTABLE_EXT_POLY, gen);
        
        assert_eq!(SHIFTABLE_EXT_POLY, Element::poly_mul_u32_no_carry(q,gen) ^ r);

    }

    #[test]
    fn test_known_inverse() {
        let one = Element([1,0]);
        assert_eq!(one.inverse(), one);

        
        let a = Element::from_standard(2);
        
        //sage: 1/a
        //a^15 + a^4 + a^2 + a
        let a_inv = 32768 + 16 + 4 + 2;
        assert_eq!(a.inverse(), Element::from_standard(a_inv));
        assert_eq!(a.inverse() * a, one);

    }
    
    impl Arbitrary for Element {
        fn arbitrary<G: quickcheck::Gen>(gen: &mut G) -> Self {
            let a = u8::arbitrary(gen);
            let b = u8::arbitrary(gen);

            Element([a, b])
        }
    }

    quickcheck! {
        fn qc_add_associativity(a: Element, b: Element, c: Element) -> bool {
            a + (b + c) == (a + b) + c
        }

        fn qc_mul_associativity(a: Element, b: Element, c: Element) -> bool {
            a * (b * c) == (a * b) * c
        }

        fn qc_additive_identity(a: Element) -> bool {
            let zero = Element::zero();
            a - (zero - a) == zero
        }

        fn qc_multiplicative_identity(a: Element) -> bool {
            a.is_zero() || {
                let one = Element([0, 1]);
                (one / a) * a == one
            }
        }

        fn qc_add_commutativity(a: Element, b: Element) -> bool {
            a + b == b + a
        }

        fn qc_mul_commutativity(a: Element, b: Element) -> bool {
            a * b == b * a
        }

        fn qc_add_distributivity(a: Element, b: Element, c: Element) -> bool {
            a * (b + c) == (a * b) + (a * c)
        }

        fn qc_inverse(a: Element) -> bool {
            a.is_zero() || {
                let inv = a.inverse();
                a * inv == Element::constant(1)
            }
        }

        fn qc_exponent_1(a: Element, n: u8) -> bool {
            a.is_zero() || n == 0 || {
                let mut b = a.exp(n as usize);
                for _ in 1..n {
                    b = b / a;
                }

                a == b
            }
        }

        fn qc_exponent_2(a: Element, n: u8) -> bool {
            a.is_zero() || {
                let mut res = true;
                let mut b = Element::constant(1);

                for i in 0..n {
                    res = res && b == a.exp(i as usize);
                    b = b * a;
                }

                res
            }
        }

        fn qc_exp_zero_is_one(a: Element) -> bool {
            a.exp(0) == Element::constant(1)
        }
    }

    // #[test]
    // fn lots_of_mul() {
    //     use rand::Rng;

    //     let mut rng = rand::thread_rng();

    //     let mut a: Element = Element([rng.gen(),rng.gen()]);
    //     let mut b: Element = Element([rng.gen(),rng.gen()]);
    //     let mut c: Element = Element([0,0]);

    //     const number_of_mul: u32 = 1000000000;

    //     for x in 0..number_of_mul {
    //         c = a * b;
    //         a = b;
    //         b = c;
    //     }
    //     println!("{:?}", c.0)

    // }

    // #[test]
    // #[should_panic]
    // fn test_div_b_is_0() {
    //     let _ = Element([1, 0]) / Element::zero();
    // }

    // #[test]
    // fn zero_to_zero_is_one() {
    //     assert_eq!(Element::zero().exp(0), Element::constant(1))
    // }
}
