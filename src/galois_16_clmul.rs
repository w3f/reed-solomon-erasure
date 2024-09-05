//! GF(2^16) implementation using clmul cpu instruction

use std::arch::x86_64::*;
use std::convert::TryInto;
use std::ops::{Add, Div, Mul, Sub};

use crate::Field as FieldTrait;

include!(concat!(env!("OUT_DIR"), "/table_g2p16.rs"));

/// An element of `GF(2^16)`.
type Element = u16;
const EXTENSION_DEGREE: i32 = 16;
const prim_poly : u32 = 0x1002d;
//const reducing_poly : u32 = (prim_poly as u64) & 0x1ffff as u64;
static mut GF2_to_16 : Option<__m128i> = None;


use lazy_static::lazy_static;
lazy_static! {
    static ref  reduction_mask : __m128i = {
        unsafe {
            #[allow(overflowing_literals)]
            _mm_setr_epi8(0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x0B, 0x0A, 0x80, 0x80, 0x03, 0x02)
        }
    };
    static ref double_prim_poly_m : __m128i = {
        unsafe {
            _mm_set_epi32(0, 0, prim_poly as i32, prim_poly as i32)
        }
    };
}


const G2P16_MUL_GROUP_ORDER: isize = 65535;
/// The field GF(2^16).
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Field;

impl crate::Field for Field {
    const ORDER: usize = 65536;

    type Elem = u16;

    fn add(a: u16, b: u16) -> u16 {
        add(a, b)
    }

    fn mul(a: u16, b: u16) -> u16 {
        unsafe {
            mul(a, b)
        }
    }

    fn div(a: u16, b: u16) -> u16 {
        div(a, b)
        
    }

    fn zero() -> u16 {
        0
    }

    fn one() -> u16 {
        1
    }

    fn exp(elem: u16, n: usize) -> u16 {
        exp(elem, n)
    }

    fn nth_internal(n: usize) -> u16 {
        n.try_into().unwrap() //TODO: this should be mod the field poly
    }

    // fn mul_slice(c: u16, input: &[u16], out: &mut [u16]) {
    //     mul_slice(c, input, out)
    // }

    // fn mul_slice_add(c: u16, input: &[u16], out: &mut [u16]) {
    //     mul_slice_xor(c, input, out)
    // }

}


// #[cfg(feature = "simd-accel")]
// pub fn mul_slice_xor(c: u16, input: &[u16], out: &mut [u16]) {
//     unsafe {
//         let input_ptr : *mut c_void = &input[0] as *const _ as *const c_void as *mut c_void;
//         //let input_ptr : *const c_void = &input[0] as *const _ as *const c_void;
//         let out_ptr : *mut c_void = &mut out[0] as *mut _ as *mut c_void;
 
//         GF2_to_16.unwrap().multiply_region.w32.unwrap()(&mut GF2_to_16.unwrap(), input_ptr.into(), out_ptr.into(), c.into(), (input.len() * 2) as i32, 1)            
//     }
// }
            
/// Type alias of ReedSolomon over GF(2^16).
pub type ReedSolomon = crate::ReedSolomon<Field>;

/// Type alias of ShardByShard over GF(2^16).
pub type ShardByShard<'a> = crate::ShardByShard<'a, Field>;

/// Add two elements.
pub fn add(a: u16, b: u16) -> u16 {
    a ^ b
}

/// Subtract `b` from `a`.
#[cfg(test)]
pub fn sub(a: u16, b: u16) -> u16 {
    a ^ b
}

/// Multiply two elements.
#[target_feature(enable = "sse2", enable = "sse4.1", enable = "pclmulqdq")]
unsafe fn mul(a: u16, b: u16) -> u16 {
    //println!("mul {:x?} {:x?}", a, b);
    let a_m = _mm_insert_epi32 (_mm_setzero_si128(), a as i32, 0);
    let b_m = _mm_insert_epi32 (a_m, b as i32, 0);
    
    let prim_poly_m = _mm_set_epi32(0, 0, 0, prim_poly as i32);

    // /* Do the initial multiply */
    
    let mut result = _mm_clmulepi64_si128 (a_m, b_m, 0);
    //println!("mul {:x?} {:x?} {:x?}", a_m, b_m, result);

    let mut w = _mm_clmulepi64_si128 (prim_poly_m, _mm_srli_si128 (result, 2), 0);
    result = _mm_xor_si128 (result, w);
    //println!("red 1 {:x?} {:x?}",w, result);

    w = _mm_clmulepi64_si128 (prim_poly_m, _mm_srli_si128 (result, 2), 0);
    result = _mm_xor_si128 (result, w);
    //println!("red 2 {:x?} {:x?}",w, result);

    /* Extracts 32 bit value from result. */
    //println!("res: {:x?}",_mm_extract_epi32(result, 0) as u16);

    return _mm_extract_epi32(result, 0) as u16

}

#[target_feature(enable = "sse2", enable = "sse3", enable = "sse4.1", enable = "pclmulqdq")]
unsafe fn two_sim_muls(c: u16, input: &[u16], out: &mut [u16]) {
    //println!("two-mul c: {:x?} a: {:x?} b: {:x?}", c, input[0], input[1]);
    //println!("two-mul c: {:x} a: {:x} b: {:x}", c, input[0], input[1]);
    let a_b_m = _mm_set_epi32(0, 0, input[1] as i32, input[0] as i32);
    let c_c_m = _mm_set_epi32(0, 0, c as i32, c as i32);

    let mut result = _mm_clmulepi64_si128(c_c_m, a_b_m, 0);
    //println!("mul c_c: {:x?} a_b: {:x?} res: {:x?}", c_c_m, a_b_m, result);
    
    ////println!("{} {} {:x?} {:x?} {:x?}", a,b, a_m, b_m, result);

    //we need to zero the second 32bit word before every shift not to interfere with the operation.
    // result = _mm_insert_epi32 (result, 0, 1);
    // //println!("zero 1 {:x?}",result);

    // //barret 
    // let mut shifted_result = _mm_srli_si128 (result, 2);
    // //println!("shift 2 {:x?}", shifted_result);

    // //we need to copy bit 64-95 to 32-63. there should be a simd way of doing this without extarciting
    // //let mut result_0_64 = _mm_insert_epi32 (shifted_result, _mm_extract_epi32(shifted_result, 2), 1);
    //  let mut result_0_64 = _mm_shuffle_epi32 (shifted_result, 0b11101000);
    //println!("moved result 1 {:x?}", result_0_64);

    //0x808080808080808080800B0A80800302u128
    let mut result_0_64 = _mm_shuffle_epi8 (result, *reduction_mask);
    
    let mut w = _mm_clmulepi64_si128 (*double_prim_poly_m, result_0_64, 0);
    result = _mm_xor_si128 (result, w);
    //println!("red 1 w: {:x?} res: {:x?}",w, result);

    //we need to zero the second 32bit words before every shift not to interfere with the operation.
    // result = _mm_insert_epi32 (result, 0, 1);
    // //println!("zero 2 {:x?}",result);

    // let shifted_result = _mm_srli_si128 (result, 2);
    // //println!("shift 2 {:x?}", shifted_result);

    // //we need to copy bit 64-95 to 32-63. there should be a simd way of doing this without extarciting
    // //result_0_64  = _mm_insert_epi32 (shifted_result, _mm_extract_epi32(shifted_result, 2), 1);
    // result_0_64 = _mm_shuffle_epi32 (shifted_result, 0b11101000);
    //println!("moved result 2 {:x?}", result_0_64);
    result_0_64 = _mm_shuffle_epi8 (result, *reduction_mask);
    w = _mm_clmulepi64_si128 (*double_prim_poly_m, result_0_64, 0);
    result = _mm_xor_si128 (result, w);
    //println!("red 2 w: {:x?} res: {:x?}",w, result);

    /* Extracts 32 bit value from result. */
    out[0] =  _mm_extract_epi32(result, 0) as u16;
    out[1] = _mm_extract_epi32(result, 2) as u16;
    //println!("res o1: {:x?} o2: {:x?}",out[0], out[1]);

}

//     // gf.multiply_region.w32(&gf, r1, r2, a, 16, 0);
    
//     // let low: *const u8 = &MUL_TABLE_LOW[c as usize][0];
//     // let high: *const u8 = &MUL_TABLE_HIGH[c as usize][0];

//     // assert_eq!(input.len(), out.len());

//     // let input_ptr: *const u8 = &input[0];
//     // let out_ptr: *mut u8 = &mut out[0];
//     // let size: libc::size_t = input.len();

//     // let bytes_done: usize =
//     //     unsafe { reedsolomon_gal_mul(low, high, input_ptr, out_ptr, size) as usize };

//     // mul_slice_pure_rust(c, &input[bytes_done..], &mut out[bytes_done..]);
//}

/// Divide one element by another. `b`, the divisor, may not be 0.
pub fn div(a: u16, b: u16) -> u16 {
    if a == 0 {
        0
    } else if b == 0 {
        panic!("Divisor is 0")
    } else {
        let log_a = G2P16_LOG_TABLE[a as usize];
        let log_b = G2P16_LOG_TABLE[b as usize];
        let mut log_result = log_a as isize - log_b as isize;
        if log_result < 0 {
            log_result += G2P16_MUL_GROUP_ORDER;
        }
        G2P16_EXP_TABLE[log_result as usize]
    }
}

/// Compute a^n.
pub fn exp(mut elem: u16, n: usize) -> u16 {
    if n == 0 {
        1
    } else if elem == 0 {
        0
    } else {
        let log_elem = G2P16_LOG_TABLE[elem as usize];
        let mut log_result = (log_elem as usize * n) % G2P16_MUL_GROUP_ORDER as usize;
        G2P16_EXP_TABLE[log_result]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Arbitrary;
    use std::arch::x86_64::*;

    quickcheck! {
        fn qc_add_associativity(a: Element, b: Element, c: Element) -> bool {
            add(a , add(b , c)) == add(add(a, b), c)
        }

        fn qc_mul_associativity(a: Element, b: Element, c: Element) -> bool {
            unsafe {
                mul(a, mul(b, c)) == mul( mul(a, b), c)
            }
        }

        fn qc_additive_identity(a: Element) -> bool {
            let zero = 0;
            sub(a, sub(zero, a)) == zero
        }

        fn qc_multiplicative_identity(a: Element) -> bool {
            a == 0 || {
                let one = 1;
                unsafe {
                    mul(div(one, a), a) == one
                }
            }
        }

        fn qc_add_commutativity(a: Element, b: Element) -> bool {
            add(a,b) == add(b, a)
        }

        fn qc_mul_commutativity(a: Element, b: Element) -> bool {
            unsafe {
                mul(a, b) == mul(b, a)
            }
        }

        fn qc_add_distributivity(a: Element, b: Element, c: Element) -> bool {
            unsafe {
                mul(a ,add(b, c)) == add(mul(a,b), mul (a, c))
            }
        }

        fn qc_inverse(a: Element) -> bool {
            unsafe { 
                a == 0 || {
                    let inv : u16 = div(1,a);
                    mul(a, inv) == 1
                }
            }
        }

        fn qc_exponent_1(a: Element, n: u8) -> bool {
            a == 0 || n == 0 || {
                let mut b = exp(a, n as usize);
                for _ in 1..n {
                    b = div(b, a);
                }

                a == b
            }
        }

        fn qc_exponent_2(a: Element, n: u8) -> bool {
            a == 0 || {
                let mut res = true;
                let mut b = 1;

                for i in 0..n {
                    res = res && b == exp(a, i as usize);
                    unsafe {
                        b = mul(b, a);
                    }
                }

                res
            }
        }

        fn qc_exp_zero_is_one(a: Element) -> bool {
            exp(a,0) == 1
        }

        fn qc_mul_sanity_element_s_order_divides_mul_subgroup_order(a: Element) -> bool {
            let mut a_to_mul_order = Field::one();
            if a != 0 {
                for i in 0..Field::ORDER - 1 {
                    unsafe {
                        a_to_mul_order = mul(a_to_mul_order,a);
                    }
                }
            }

            a_to_mul_order == Field::one()
                        
        }

        fn qc_two_sim_muls_are_equal_two_singl_mul(a: Element, b: Element, c: Element)-> bool {
            let input_pair = [a, b];
            let mut output = [0 as u16; 2];

            unsafe {
                two_sim_muls(c, &input_pair, &mut output);
                mul(c,a) == output[0] && mul(c,b) == output[1]
            }
            
        }
          
    }

   #[test]
   fn lots_of_mul() {
        use rand::Rng;

        let mut rng = rand::thread_rng();

       let mut input_array : [u16; 2] =  [rng.gen::<u16>(), rng.gen::<u16>()];
       let mut output_array = [0, 0];
       let mut c = rng.gen();

        const number_of_mul: u32 = 500000000;

       unsafe {
        for x in 0..number_of_mul {
            two_sim_muls(c, &input_array, &mut output_array);
            c = input_array[0];
            input_array[0] = input_array[1];
            input_array[1] = output_array[0];

        }
           println!("{}", output_array[0]);
       }

   }

   #[test]
   fn lots_of_single_mul() {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        let mut a: u16 = rng.gen();
        let mut b: u16 = rng.gen();
        let mut c: u16 = 0;
        let mut d: u16 = 0;

        const number_of_mul: u32 = 1000000000;

       unsafe {
        for x in 0..number_of_mul {
            c = mul(a, b);
            a = b;
            b = c;
        }
           println!("{}", c);
       }

     }
    
    #[test]
    #[should_panic]
    fn test_div_b_is_0() {
        let result : u16 =  div(1 as u16, 0 as u16) as u16;
    }
    
    #[test]
    fn zero_to_zero_is_one() {
        assert_eq!(exp(0,0), 1)
    }
}
