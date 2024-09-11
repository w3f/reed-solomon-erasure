R.<x> = FiniteField(2)[x]
ext_poly = R.irreducible_element(16,algorithm="first_lexicographic" )

print("first lexio: %s"%ext_poly)
# x^16 + x^5 + x^3 + x + 1

#sage's choice
G2Ft16.<g> = FiniteField(65536)
print("sage' choice: ", GF2t16.polynomial())
# g^16 + g^5 + g^3 + g^2 + 1: 1'0000'0000'0010'1101 

#Comment from GF-Complete code

# Allen: use the following primitive polynomial to make carryless multiply work
# more efficiently for GF(2^16).

#   h->prim_poly = 0x1002d = 1'0000'0000'0010'1101 

# The following is the traditional primitive polynomial for GF(2^16) */

#   h->prim_poly = 0x1100b = 1'0001'0000'0000'1011 ;

