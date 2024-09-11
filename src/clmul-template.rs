//     #[inline]
//     #[target_feature(enable = "sse2")]
//     unsafe fn new(high: u64, low: u64) -> Self {
//         Self(_mm_set_epi64x(high as i64, low as i64))
//     }

//     #[inline]
//     #[target_feature(enable = "sse2", enable = "pclmulqdq")]
//     unsafe fn fold_16(self, coeff: Self) -> Self {
//         let h = Self(_mm_clmulepi64_si128(self.0, coeff.0, 0x11));
//         let l = Self(_mm_clmulepi64_si128(self.0, coeff.0, 0x00));
//         h ^ l
//     }

//     #[inline]
//     #[target_feature(enable = "sse2", enable = "pclmulqdq")]
//     unsafe fn fold_8(self, coeff: u64) -> Self {
//         let coeff = Self::new(0, coeff);
//         let h = Self(_mm_clmulepi64_si128(self.0, coeff.0, 0x00));
//         let l = Self(_mm_srli_si128(self.0, 8));
//         h ^ l
//     }

//     #[inline]
//     #[target_feature(enable = "sse2", enable = "sse4.1", enable = "pclmulqdq")]
//     unsafe fn barrett(self, poly: u64, mu: u64) -> u64 {
//         let polymu = Self::new(poly, mu);
//         let t1 = _mm_clmulepi64_si128(self.0, polymu.0, 0x00);
//         let h = Self(_mm_slli_si128(t1, 8));
//         let l = Self(_mm_clmulepi64_si128(t1, polymu.0, 0x10));
//         let reduced = h ^ l ^ self;
//         _mm_extract_epi64(reduced.0, 1) as u64
//     }

// }


// #if defined(INTEL_SSE4_PCLMUL)
// static
// inline
// gf_val_32_t
// gf_w16_clm_multiply_2 (gf_t *gf, gf_val_32_t a16, gf_val_32_t b16)
// {
//   gf_val_32_t rv = 0;

//   __m128i         a, b;
//   __m128i         result;
//   __m128i         prim_poly;
//   __m128i         w;
//   gf_internal_t * h = gf->scratch;

//   a = _mm_insert_epi32 (_mm_setzero_si128(), a16, 0);
//   b = _mm_insert_epi32 (a, b16, 0);

//   prim_poly = _mm_set_epi32(0, 0, 0, (uint32_t)(h->prim_poly & 0x1ffffULL));

//   /* Do the initial multiply */
  
//   result = _mm_clmulepi64_si128 (a, b, 0);

//   /* Ben: Do prim_poly reduction twice. We are guaranteed that we will only
//      have to do the reduction at most twice, because (w-2)/z == 2. Where
//      z is equal to the number of zeros after the leading 1

//      _mm_clmulepi64_si128 is the carryless multiply operation. Here
//      _mm_srli_si128 shifts the result to the right by 2 bytes. This allows
//      us to multiply the prim_poly by the leading bits of the result. We
//      then xor the result of that operation back with the result.*/

//   w = _mm_clmulepi64_si128 (prim_poly, _mm_srli_si128 (result, 2), 0);
//   result = _mm_xor_si128 (result, w);
//   w = _mm_clmulepi64_si128 (prim_poly, _mm_srli_si128 (result, 2), 0);
//   result = _mm_xor_si128 (nresult, w);

//   /* Extracts 32 bit value from result. */
  
//   rv = ((gf_val_32_t)_mm_extract_epi32(result, 0));

//   return rv;
// }
// #endif
