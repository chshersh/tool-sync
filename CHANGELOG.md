# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning][1]. The changelog is also
available [on GitHub][2].

## [Unreleased]

### Added

* [#92](https://github.com/chshersh/tool-sync/issues/92):
  Adds the `tool sync <tool-name>` command to install only one tool
  from the configuration file
  (by [@zixuanzhang-x][zixuanzhang-x])
* [#82](https://github.com/chshersh/tool-sync/issues/82):
  Adds proxy as a flag and config option
  (by [@MitchellBerend][MitchellBerend])
* [#111](https://github.com/chshersh/tool-sync/issues/111):
  Adds repo URLs to the output of `default-config` and `install` commands
  (by [@crudiedo][crudiedo])
* [#110](https://github.com/chshersh/tool-sync/issues/110):
  Adds the 'OS' enum and improves error messages
  (by [@crudiedo][crudiedo])
* [#109](https://github.com/chshersh/tool-sync/issues/109)
  Adds a `--path` option to the `default-config` command to print
  default config location intead
  (by [@zixuanzhang-x][zixuanzhang-x])
* [#128](https://github.com/chshersh/tool-sync/issues/128)
  Adds new error message RepoError::NotFound when fetch_release_info
  returns 404
  (by [@crudiedo][crudiedo])
* [#133](https://github.com/chshersh/tool-sync/issues/133)
  Added shell completion
  (by [@MitchellBerend][MitchellBerend])
* [#147](https://github.com/chshersh/tool-sync/issues/147) [#160](https://github.com/chshersh/tool-sync/issues/160)
  Supports casey/just, dalance/procs, derailed/k9s, and
  sharkdp/hyperfine natively.
  (by [@hdhoang][hdhoang])


### Fixed

* [#84](https://github.com/chshersh/tool-sync/issues/84):
  Refactor of hardcoded tools
  (by [@MitchellBerend][MitchellBerend])
* [#119](https://github.com/chshersh/tool-sync/issues/119):
  De-pluralize success message when only 1 tool is installed
  (by [@zixuanzhang-x][zixuanzhang-x])
* [#108](https://github.com/chshersh/tool-sync/issues/108):
  Check prefetched `tool_assets` is not empty before passing
  it to `SyncProgress::new`
  (by [@zixuanzhang-x][zixuanzhang-x])


## [0.2.0] — 2022-09-20 🔃

### Added

* [#57](https://github.com/chshersh/tool-sync/issues/57):
  Adds the `install` command to install known tools
  (by [@MitchellBerend][MitchellBerend])
  **#CLI**
* [#52](https://github.com/chshersh/tool-sync/issues/52),
  [#73](https://github.com/chshersh/tool-sync/issues/73):
  Add the `default-config` command to generate the default `.tool.toml` file
  with the tool version and all natively supported tools
  (by [@MitchellBerend][MitchellBerend])
  **#CLI** **#CONFIG**
* [#49](https://github.com/chshersh/tool-sync/issues/49),
  [#63](https://github.com/chshersh/tool-sync/issues/63):
  Output the exact tag of the downloaded tool
  (by [@MitchellBerend][MitchellBerend])
  **#OUTPUT**
* [#32](https://github.com/chshersh/tool-sync/issues/32):
  Add tag option to download a specific tool version
  (by [@FrancisMurillo][FrancisMurillo])
  **#CONFIG**
* [#33](https://github.com/chshersh/tool-sync/issues/33):
  Better `sync` output with estimated size, installation directory and early
  errors
  (by [@chshersh][chshersh])
  **#OUTPUT**
* [#46](https://github.com/chshersh/tool-sync/issues/46):
  Support syncing `tool-sync` by `tool-sync`
  (by [@chshersh][chshersh])
  **#CONFIG**
* [#87](https://github.com/chshersh/tool-sync/issues/87):
  Produce statically linked binaries for linux
  (by [@chshersh][chshersh])
  **#DX** **#CD**
* [PR #73](https://github.com/chshersh/tool-sync/pull/73):
  Adding the pre-commit config
  (by [@MitchellBerend][MitchellBerend])
  **#DX**
* [PR #74](https://github.com/chshersh/tool-sync/pull/74):
  Added pull request template
  (by [@MitchellBerend][MitchellBerend])
  **#DX**
* [PR #81](https://github.com/chshersh/tool-sync/pull/81):
  Better CI caching
  (by [@chshersh][chshersh])
  **#DX** **#CI**
* [#83](https://github.com/chshersh/tool-sync/issues/83):
  Create `AssetError` and refactor `select_asset` function
  (by [@zixuanzhang-x][zixuanzhang-x])
  **#REFACTORING** **#OUTPUT**
* [#88](https://github.com/chshersh/tool-sync/issues/88):
  Replace custom `display()` method with `std::fmt::Display` trait implementation
  (by [@zixuanzhang-x][zixuanzhang-x])
  **#REFACTORING**
* [#98](https://github.com/chshersh/tool-sync/issues/98):
  Changed all formatting functions so they accept a type that implements `Display`
  (by [@MitchellBerend][MitchellBerend])
  **#REFACTORING**
* [PR #94](https://github.com/chshersh/tool-sync/pull/94):
  Refactor commands into separate modules
  (by [@chshersh][chshersh])
  **#REFACTORING**

### Fixed

* [#54](https://github.com/chshersh/tool-sync/issues/54):
  Adding extra lookup path inside asset. Additionally, added a new error message for the case where multiple assets are found
  (by [@MitchellBerend][MitchellBerend])
  **#SYNC** **#OUTPUT**
* [#55](https://github.com/chshersh/tool-sync/issues/55):
  Make `exe_name` optional in the config and use the value of `repo` by default
  (by [@mmohammadi9812][mmohammadi9812], [@chshersh][chshersh])
  **#CONFIG**

## [0.1.0] — 2022-08-29 🌇

Initial release prepared by [@chshersh][chshersh].

<!-- Contributors -->

[chshersh]: https://github.com/chshersh
[FrancisMurillo]: https://github.com/FrancisMurillo
[MitchellBerend]: https://github.com/MitchellBerend
[mmohammadi9812]: https://github.com/mmohammadi9812
[zixuanzhang-x]: https://github.com/zixuanzhang-x
[crudiedo]: https://github.com/crudiedo

<!-- Header links -->

[1]: https://semver.org/
[2]: https://github.com/chshersh/tool-sync

<!-- Versions -->

[Unreleased]: https://github.com/chshersh/tool-sync/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/chshersh/tool-sync/releases/tag/v0.2.0
[0.1.0]: https://github.com/chshersh/tool-sync/releases/tag/v0.1.0
