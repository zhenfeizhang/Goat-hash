# modulus
q = 2^64 - 2^32 + 1

def get_params(n, m, q, c):
    # ============================
    # bound from MR09 paper
    # ============================
    # if the infinity norm of SIS is larger than this bound, SIS is easy

    # dimension of the lattice
    dim = n * m
    
    upper_bound_1 = ZZ(round(2^(2*sqrt(dim * log(q) * log(c)))))

    # ============================
    # bound from SVP solvers
    # ============================
    dim = n * (m + 1)
    det = q^n
    # infinite norm of first minima
    lambda_1 = det^(1/dim)

    # upper bound form gap SVP
    # if the infinity norm of SIS is larger than this bound, gap SVP is easy
    upper_bound_2 = ZZ(round(c^dim * lambda_1))

    # lower bound from unique SVP
    # if the infinity norm of SIS is smaller than this bound, gap SVP is easy
    lower_bound = ZZ(round(lambda_1/c^dim))

    upper_bound = min(upper_bound_1, upper_bound_2)
    print("(", n, m, c, "): range ", lower_bound, ", ", upper_bound_1)


# dimension of the ring
ns = [128, 256, 512]

# number of ring elements
ms = [3, 4, 5, 6, 7, 8]

# bkz parameter
cs = [1.003, 1.004, 1.005]

for n in ns:
    for m in ms:
        for c in cs:
            get_params(n, m, q, c) 