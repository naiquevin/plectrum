# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-09-22

### Added

- Example to demonstrate loading data from a csv file

### Changed

- The id corresponding to an enum variant can now be obtained through
  the `id` method of the `plectrum::Enum` trait. It's a default method
  provided by the trait so the user doesn't need to implement it. The
  `get_id` method of `plectrum::Mapping` is deprecated in favour of
  the above.
- The inner mapping is stored in `BiHashMap` from the `bimap` crate
  instead of `HashMap`. This optimizes lookups by value.
- Added `thiserror` as a dependency which takes care of implementing
  `std::error::Error` trait for the `plectrum:Error` enum
- The `plectrum::Error::DataSource` variant now takes `Box<dyn
  std::error::Error + Send + Sync>` instead of String. This way it
  retains the original error. This is a breaking change.
