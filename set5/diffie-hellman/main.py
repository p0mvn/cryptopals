import numpy as np
from sys import maxsize


def diffie_hellman(p, g, a, b):
    # Private key A
    a_mod_p = a % p

    # Public key A
    A = pow(g, a_mod_p, p)
    
    # Private key B
    b_mod_p = b % p
    # Public key B
    B = pow(g, b_mod_p, p)

    # Shared secret, generated with A's private key and B's public key.
    s_a = pow(B, a_mod_p, p)
    # Shared secret, generated with B's private key and A's public key.
    s_b = pow(A, b_mod_p, p)

    if s_a == s_b:
        print(f"Shared secret is generated correctly {s_a}")
    else:
        print(f"Shared secret is not generated correctly {s_a} vs {s_b}")

# https://www.cryptopals.com/sets/5/challenges/33
# Basis: discrete logarithm problem

# To generate keypair in DH:

# 1. All participants agree on a large prime p and a generator g
# 2. Each participant generates a random number x, which becomes their private key
# 3. Each participant derives their public key as $g^{x} \mod p$

# The fact that the discrete logarithm problem is hard means that no one should be able to recover the private key from the public key.

# $g, g^1,g^2,g^3, g34121132112901242,…\mod p$

# DH public key = $g^{34121132112901242} \mod p$

# DH private key = 34121132112901242

# Choosing a private key in DH is like choosing an index in a list of numbers produced by a generator g. The discrete log problem is to find the index from the number alone.

# To compute the modular exponentiation in an efficient way:

# - Square and multiply
#     - [https://blog.xojo.com/2022/05/16/square-multiply-algorithm/](https://blog.xojo.com/2022/05/16/square-multiply-algorithm/)
#     - [https://en.wikipedia.org/wiki/Exponentiation_by_squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring)
#     - [https://leetcode.com/problems/super-pow/](https://leetcode.com/problems/super-pow/)

# How do we use this math for key exchange?

# - Alice: has private key a and public key $A = g^{a} \mod p$
# - Bob: has a private key b and a public key $B = g^b \mod p$

# Alice can compute shared secret S as $B^a \mod p$

# Bob can compute shared secret S as $A^b \mod p$

# $B^a = (g^b)^a = g^ab = (g^a)^b = A^b \mod p$
def main():
    np.random.seed(1)

    print("Small numbers:")
    p = 37
    g = 5

    a = np.random.randint(maxsize)
    b = np.random.randint(maxsize)
    print("p = ", p)
    print("g = ", g) 
    print("a =", a)
    print("b =", b)
    diffie_hellman(p, g, a, b)

    print("\n")
    print("Large NIST numbers numbers:")
    p = 0xffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff
    g = 2

    diffie_hellman(p, g, a, b)

if __name__ == "__main__":
    main()
