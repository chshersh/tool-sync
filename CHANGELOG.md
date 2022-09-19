# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning][1]. The changelog is also
available [on GitHub][2].

## [Unreleased]

## [0.2.0] — 2022-09-?? 🔃

### Added
* [#57](https://github.com/chshersh/tool-sync/issues/57):
  Adds the `install` command to install known tools
  (by [@MitchellBerend][MitchellBerend])
  **#CLI**
* [#52](https://github.com/chshersh/tool-sync/issues/52),
  [#73](https://github.com/chshersh/tool-sync/issues/73):
  Add the `default-config` command to generate the default `.tools.toml` file
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

## [0.1.0] — 2022-08-29 🌇

Initial release prepared by [@chshersh][chshersh].

<!-- Contributors -->

[chshersh]: https://github.com/chshersh
[FrancisMurillo]: https://github.com/FrancisMurillo
[MitchellBerend]: https://github.com/MitchellBerend
[zixuanzhang-x]: https://github.com/zixuanzhang-x

<!-- Header links -->

[1]: https://semver.org/
[2]: https://github.com/chshersh/tool-sync

<!-- Versions -->

[Unreleased]: https://github.com/chshersh/tool-sync/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/chshersh/tool-sync/releases/tag/v0.2.0
[0.1.0]: https://github.com/chshersh/tool-sync/releases/tag/v0.1.0
