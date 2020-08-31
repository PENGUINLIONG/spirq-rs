# Change Log

## v0.4.6

- Added API for better user experience.

## v0.4.5

- Supported dynamically sized multi-binding (`SPV_EXT_descriptor_indexing`);
- Fixed tests.

## v0.4.4

- Fixed that separable sampler and image object cannot share a same binding point;
- Fixed field name typo.

## v0.4.3

- Fixed name collision caused by multiple unnamed buffer blocks;
- Fixed interface variable resolution;
- Supported buffer block root type resolution in iteration by `descs`;
- Supported name access to push constant;
- Improved entrypoint debug printing;
- Improved API provision;
- Added comprehensive API testing.

## v0.4.2

- Added a more handy manifest merging method for pipeline construction;
- Supported specialization constant reflection;
- Fixed interface variable resolution.

## v0.4.1

- Fixed that interface variables are not correctly merged;
- Significant performance improvement.

## v0.4.0

- No longer treating push constant as special cases of descriptors;
- Support multibinding for all descriptors;
- Minor clean-up.


## v0.3.0

- Fixed that built-in variables are not correctly ignored;
- Supported component number for shared-location interface variables;
- Added support for separable sampler types;
- Added descriptor access type query;
- Minor safety improvement.


## v0.2.1

- Fixed a typo (`InputAtatchment`);
- Added walking through descriptor types;
- Added an example for descriptor walk.

## v0.2.0

This is a breaking change. SPIR-Q is now more handy with better and easier reflection information accessors.

- Fix several bugs;
- Restructured project files;
- Improved API design.


## v0.1.0

This is a breaking change. Some type names has been changed to be more exact. E.g. previous `NumericType` is now `ScalarType`; and the usage of if has also changed.

- Support boolean types;
- Fix that outer-most types (the types exposed directly to a binding point) cannot be resolved by symbol;
- Added an example reflecting a fragment shader extracted from section 1.10 of the SPIR-V specification;
- Minor performance improvement.

## v0.0.3

- Add dependency to `spirv_headers` for sharable SPIR-V constants and types;
- Add Badges.
