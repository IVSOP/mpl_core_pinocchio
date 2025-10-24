Utility crate for CPI, serialization and deserialization of Metaplex Core assets in pinocchio

WARNING: this is very much a work in progress. Currently the instructions expect the user to provide a buffer of sufficient size.

In the future, I want to allow calculating the needed size or make a version that uses Vec. For now I want maximum performance and compatibility with no std and no allocator, which is why I made it this way

Features have been tested but not in this crate

**TLDR** Missing tests and polish, but it works
