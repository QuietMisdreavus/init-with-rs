# Changelog for init\_with

## [1.1.0] - 2917-08-25
### Added
- `InitWith::init_with_indices`, to hand an index to your init function

## [1.0.2] - 2017-06-02

This version is identical to 1.0.1, just updating the documentation link to docs.rs.

## [1.0.1] = 2017-01-20

This version is identical to 1.0.0, with some metadata updates to add a category for the crate,

## [1.0.0] - 2016-12-27

This version is identical to 0.1.2, but I'm updating the major version because I think the API is
complete. Any further work can go into tuning the implementation or adding more impls as necessary.
If the shape of the InitWith trait needs to change, that's what a major-version bump is for. In the
meantime, I don't expect to need to update this library.

## [0.1.2] - 2016-12-19
### Changed
- Use macro-expanded array literals instead of `mem::uninitialized` and `ptr::write`
  - This lets previously-initialized items properly Drop if a later init call panics
  - This also lets us get rid of the `nodrop` dependency, and of unsafe code in general

## [0.1.1] - 2016-12-13
### Changed
- Use pointer-offset calls instead of bounds-checked indexing when filling the array

## [0.1.0] - 2016-12-12
- Initial version with impls for arrays from 0 to 32
