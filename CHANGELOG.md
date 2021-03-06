# Changelog

## [0.3.3] - 2021-01-20
- Fix default features for soloud-sys.

## [0.3.2] - 2021-01-19
### Changes
- [BREAKING] Wrap Handle type in a struct to avoid misuse. Thanks @toyboot4e.
- [BREAKING] backend_string now returns a String instead of a &str.
- [BREAKING] BiquadResonantFilterAttr::Frequence renamed to Freq as with all filter attributes.
- [BREAKING] Rename unsafe LoadExt::load_mem_ex to _load_mem_ex and marked interal.
- [BREAKING] Add backend parameter when initializing the soloud engine.
- Fix docs.
- Add missing backends to CMakeLists.
- Fix builds with coreaudio framework.
- Refactor boilerplate ffi handling by using a macro. Thanks @toyboot4e.
- Support more backends in soloud-rs via feature flags.
- Fix memory leak with LoadExt::load_mem and FromExt::from_path and add explicit methods to the LoadExt and FromExt traits which allows loading audio from memory. Thanks @toyboot4e.
    - load_mem_weak and from_mem_weak
    - load_mem_weak_unsafe and from_mem_weak_unsafe
- Update deps.
- Add missing docs.

## [0.2.4] - 2020-12-27
### Changes
- Add derive Debug for Soloud types. Thanks @toyboot4e.
- Update dependencies.

## [0.2.3] - 2020-12-02
### Changes
- Use bitflags for SoloudFlag;

## [0.2.2] - 2020-11-22
### Changes
- Update dependencies.
- Add example using include_bytes.

## [0.2.1] - 2020-10-02
### Changes
- Add a helper trait FromExt, thanks @toyboot4e
- Add AsRef<Path> impl fro LoadExt::load() and FromExt::from_path trait methods.

## [0.2.0] - 2020-09-29
### Changes
- Pull changes from origin.
- Redirect submodule to point to origin.
- Patch origin.

## [0.1.9] - 2020-09-01
### Changes
- Accept CMake toolchain files for cross-compilation via SOLOUD_TOOLCHAIN env var.
- Update deps.

## [0.1.8] - 2020-09-01
### Changes
- Fix typo in SoloudErrorKind.
- Remove unwraps in internal code.

## [0.1.7] - 2020-09-01
### Changes
- Remove effects module for now.

## [0.1.6] - 2020-08-31
### Changes
- Add AudioAttenuator and AudioCollider overridable methods.


## [0.1.5] - 2020-08-30
### Changes
- Add Bus sound source.
- Complete implementation of audio sources.

## [0.1.4] - 2020-08-28
### Changes
- Make the Soloud struct an RAII type with a Drop impl.
- Add a changelog.
- Add CI to check builds on windows, macos and linux.