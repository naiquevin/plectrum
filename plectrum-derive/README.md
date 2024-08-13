# plectrum-derive

Provides derive macro for the plectrum crate.

The `plectrum` crate actually specifies this crate as a dependency and
re-exports it. So no need to add `plectrum-derive` separately as a
dependency. Instead it's recommended to enable the `derive` feature
for the `plectrum` dependency.
