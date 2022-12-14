# QuantLib in Rust

This is an implementation of the excellent [QuantLib](https://www.quantlib.org) library in Rust.  QuantLib is an outstanding piece of work by 
a team of dedicated professionals in the field. Many thanks for their continuing work.

This is a work in progress.  At present the focus is on implementing the essential building blocks of the library,
closely following the QuantLib implementations.

# Examples 
Examples can be found in the `examples` directory.  Use `cargo run --example <name>` where 
`<name>` is the name of the example binary (without the `.rs` suffix), e.g.:
```
cargo run --example bonds
```

# References

* [Implementing QuantLib, Luigi Ballabio](https://leanpub.com/implementingquantlib)
* https://people.maths.ox.ac.uk/trefethen/barycentric.pdf
* https://github.com/higham/what-is
