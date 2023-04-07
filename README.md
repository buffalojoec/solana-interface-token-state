# Solana Interface Token State Example

This repository serves as a Proof-of-Concept for providing interfaces for state within accounts on Solana.

---

* `program`: Sample program demonstrating both macros and traits.
* `tests`: Associated client-side tests leveraging IDL-generated interface configs.

## Crate Overview

`interface`: The PoC state interface crate.
* The `derive` crate houses both versions of the interface macro
    * `V1` (`proc_macro_derive`): simply checks to make sure you've added the required fields and they have the correct types.
    * `V2` (`proc_macro_attribute`): assumes you are only adding fields for additional data you want inside of this account, and adds the required fields from the interface for you.
* The macros inside the `derive` crate then drive the code within the `syn` crate to apply the necessary traits and perform other operations on your struct.
    * The `syn` crate contains a lot of shared code across both macros
    * You can see exactly where they differ at the bottom of the `v1.rs` or `v2.rs` file within the `impl From<&_> for TokenStream` trait implementation.
* Finally, the `src` root of the entire crate itself holds the actual traits for each interface.
    * This is also where the customized packing & un-packing of account data is implemented.
