
# - rev(i) is the reverse bit decomposition of i, i.e.,
#   0   ->  0
#   1   ->  100 0000
#   2   ->  010 0000
#   3   ->  110 0000   ...
def reverse_bits(i, n):
    t = i.binary()[::-1]

    while len(t) < n:
        t = t + "0"

    res = 0
    for e in t:
        res *= 2
        res += ZZ(e)
    return res


q = 2^64 - 2^32 + 1   

def print_ntt():
    P.<x> = PolynomialRing(Zmod(q))
    f = P(x^1024+1)
    r = f.roots()[0][0]
    r_inv = 1/r
    print(r)

    for i in range (1024):
        e = reverse_bits(ZZ(i), 10)
        t = ZZ(r^e)
        print("Goldilocks(", t, ")", end = ', ')
        # print(i, e, r^e)
    print()

def print_inv_ntt():
    P.<x> = PolynomialRing(Zmod(q))
    f = P(x^1024+1)
    r = f.roots()[0][0]
    r_inv = 1/r
    print(r)

    for i in range (1024):
        e = reverse_bits(ZZ(i), 10)
        t = ZZ(r_inv^e)
        print("Goldilocks(", t, ")", end = ', ')
    print()

print_ntt()
print()
print_inv_ntt()
