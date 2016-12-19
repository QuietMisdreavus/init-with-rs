# Changelog for init\_with

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
