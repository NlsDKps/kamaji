# Idea
Small and easy to use web-service for office and sales activities.

# Crypto
Recommendations for the cryptografic standards and key lengths are based on the
BSI document "BSI - Techische Richtline", version 2020-01 (BSI TR-02102-1)[1][1]. If
nothing can be found there, the NIST standard is used
Currently TPM, Smartcard or similar is not used, since such a high security
standard is not necessary for such a small project.
For implementation of crypto, Botan is used, since this is a well-known
C++-library with a wrapper provided for Rust.

## Password hashing
Passwords are hashed with SHA3-512 (see [1][1], chapter 3).

## Key Derivation
For Key derivation Argon2id is used (see [1][1], chapter B1.2). The tradeoff of
Argon2i, Argon2d and Argon2id is explained in [2][2].

# Links
[1]: https://www.bsi.bund.de/SharedDocs/Downloads/EN/BSI/Publications/TechGuidelines/TG02102/BSI-TR-02102-1.pdf
[2]: https://crypto.stackexchange.com/questions/48935/why-use-argon2i-or-argon2d-if-argon2id-exists
