# Changelog

All changes to C2PA Tool are documented in this file.  For additional information on versions 0.9.x and earlier, see the [Archived release motes](../docs/release-notes.md).

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html), except that – as is typical in the Rust community – the minimum supported Rust version may be increased without a major version increase.

Since version 0.10.0, the format of this changelog is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.19.1](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.19.0...c2patool-v0.19.1)
_18 July 2025_

### Fixed

* Clippy warnings for Rust 1.88 ([#1204](https://github.com/contentauth/c2pa-rs/pull/1204))
* Fix  c2patool --info command. ([#1190](https://github.com/contentauth/c2pa-rs/pull/1190))

## [0.19.0](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.18.0...c2patool-v0.19.0)
_17 June 2025_

### Added

* Update Validation for 2.2 spec compliance ([#1144](https://github.com/contentauth/c2pa-rs/pull/1144))

### Documented

* Doc cleanup ([#1143](https://github.com/contentauth/c2pa-rs/pull/1143))

## [0.18.0](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.17.0...c2patool-v0.18.0)
_27 May 2025_

### Added

* Make OpenSSL a default feature ([#1118](https://github.com/contentauth/c2pa-rs/pull/1118))

### Fixed

* Remove use of workspace versioning for the moment ([#1136](https://github.com/contentauth/c2pa-rs/pull/1136))
* Add CAWG support for fragmented BMFF ([#1114](https://github.com/contentauth/c2pa-rs/pull/1114))

### Other

* Remove unreleased changes to c2patool
* Integrates prebuilt library release workflow ([#1126](https://github.com/contentauth/c2pa-rs/pull/1126))

## [0.17.0](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.16.5...c2patool-v0.17.0)
_16 May 2025_

### Added

* [**breaking**] Merge CAWG identity SDK into main C2PA crate ([#1089](https://github.com/contentauth/c2pa-rs/pull/1089))

### Documented

* Replace old c2patool release notes with CHANGELOG ([#1063](https://github.com/contentauth/c2pa-rs/pull/1063))

## [0.16.6](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.16.5...c2patool-v0.16.6)
_14 May 2025_

### Documented

* Replace old c2patool release notes with CHANGELOG ([#1063](https://github.com/contentauth/c2pa-rs/pull/1063))

## [0.16.5](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.16.4...c2patool-v0.16.5)
_25 April 2025_

### Fixed

* Enable post_validate_async for WASI ([#1052](https://github.com/contentauth/c2pa-rs/pull/1052))

## [0.16.4](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.16.3...c2patool-v0.16.4)
_16 April 2025_

### Documented

* Remove instructions to install c2patool using binstall ([#1038](https://github.com/contentauth/c2pa-rs/pull/1038))

### Fixed

* Dynamic assertions should be gathered assertions ([#1005](https://github.com/contentauth/c2pa-rs/pull/1005))

## [0.16.3](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.16.2...c2patool-v0.16.3)
_07 April 2025_

### Fixed

* Adjust dependencies to avoid security warnings and yanked versions ([#1031](https://github.com/contentauth/c2pa-rs/pull/1031))

## [0.16.2](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.16.1...c2patool-v0.16.2)
_04 April 2025_

### Fixed

* Update openssl to address a recently-announced vulnerability ([#1024](https://github.com/contentauth/c2pa-rs/pull/1024))

## [0.16.1](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.16.0...c2patool-v0.16.1)
_26 March 2025_

* Update to latest c2pa-crypto crate

## [0.15.0](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.14.0...c2patool-v0.15.0)
_18 March 2025_

### Added

* Adds `reader.post_validate` method for CAWG validation support (#976)
* Add WASI to c2patool (#945)

### Fixed

* Remove circular dependency between C2PA and CAWG crates (#982)

### Updated dependencies

* Bump env_logger from 0.11.6 to 0.11.7 (#963)

## [0.13.3](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.13.1...c2patool-v0.13.3)
_11 February 2025_

### Fixed

* Trigger a release of c2patool to pick up latest c2pa-rs changes

## [0.13.1](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.13.0...c2patool-v0.13.1)
_31 January 2025_

### Fixed

* Trigger a release of c2patool to pick up latest c2pa-rs changes (#895)

## [0.13.0](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.12.0...c2patool-v0.13.0)
_29 January 2025_

### Added

* Claim v2 (#707)

## [0.12.0](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.11.1...c2patool-v0.12.0)
_22 January 2025_

### Added

* Change the definition of `Signer.raw_signer()` to return an `Option` defaulting to `None` (#869)

## [0.11.1](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.11.0...c2patool-v0.11.1)
_18 January 2025_

### Fixed

* Upload a distinct SBOM per platform (#856)

## [0.11.0](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.10.2...c2patool-v0.11.0)
_16 January 2025_

### Added

* Move COSE signing into `c2pa_crypto` crate (#807)

### Documented

* Post move cleanup (#778)

### Fixed

* Fix: Obscure glob error message for missing files

## [0.10.2](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.10.1...c2patool-v0.10.2)
_12 December 2024_

### Fixed

* No-op change to trigger new c2patool build
* Update makefile for c2patool's new location in c2pa-rs workspace

## [0.10.1](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.10.0...c2patool-v0.10.1)
_12 December 2024_

### Fixed

* No-op change to trigger new c2patool release

## [0.10.0](https://github.com/contentauth/c2pa-rs/compare/c2patool-v0.9.12...c2patool-v0.10.0)
_12 December 2024_

### Added

* Updates c2patool to use only the new Builder/Reader API (contentauth/c2patool#297)

### Documented

* Update Contributing guide, misc minor edits (contentauth/c2patool#296)

### Fixed

* Compile `c2pa-crypto` with `cargo check` (#768)

### Other

* Move c2patool source code into c2pa-rs repo (#723)
* Move profile settings to workspace Cargo.toml
* Enlarged description of c2pa command-line behavior (contentauth/c2patool[#285](https://github.com/contentauth/c2pa-rs/pull/285))

## 0.9.12
_18 October 2024_

* fix: Update c2pa-rs for RegionOfInterest support. ([contentauth/c2patool#269](https://github.com/contentauth/c2patool/pull/269))
* Fix broken link that was causing os site workflow to fail ([contentauth/c2patool#266](https://github.com/contentauth/c2patool/pull/266))
* Bump codecov/codecov-action from 3 to 4 ([contentauth/c2patool#242](https://github.com/contentauth/c2patool/pull/242))
* chore: Run all CI jobs when user is dependabot[bot]
* chore: Debug CI again
* chore: Format for consistency with c2pa-rs CI workflow ([contentauth/c2patool#265](https://github.com/contentauth/c2patool/pull/265))
* chore: Don't skip CI jobs for non-pull-request events
* chore: Retry debug
* chore: Debug action context
* chore: Skip CodeCov upload for non-member PRs ([contentauth/c2patool#263](https://github.com/contentauth/c2patool/pull/263))
* Bump EmbarkStudios/cargo-deny-action from 1 to 2 ([contentauth/c2patool#245](https://github.com/contentauth/c2patool/pull/245))
* chore: Adjust conditions for running CI jobs ([contentauth/c2patool#261](https://github.com/contentauth/c2patool/pull/261))

## 0.9.11
_16 October 2024_

* Merge hardening bug fixes ([contentauth/c2patool#260](https://github.com/contentauth/c2patool/pull/260))

## 0.9.10
_07 October 2024_

* Update c2ptool to use latest c2pa-rs ([contentauth/c2patool#258](https://github.com/contentauth/c2patool/pull/258))

## 0.9.9
_17 September 2024_

* Pull in latest bug fixes ([contentauth/c2patool#237](https://github.com/contentauth/c2patool/pull/237))
* Document fragment subcommand ([contentauth/c2patool#236](https://github.com/contentauth/c2patool/pull/236))
* Switch back to using `pull_request` instead of `pull_request_target` trigger
* Bump actions/checkout from 3 to 4 ([contentauth/c2patool#243](https://github.com/contentauth/c2patool/pull/243))
* Remove no-longer-maintained clippy-check action ([contentauth/c2patool#238](https://github.com/contentauth/c2patool/pull/238))

## 0.9.8
_30 August 2024_

* Initial fragment support ([contentauth/c2patool#230](https://github.com/contentauth/c2patool/pull/230))
* Add warning about accessing a private key directly ([contentauth/c2patool#218](https://github.com/contentauth/c2patool/pull/218))

## 0.9.7
_15 August 2024_

* Update to latest c2pa SDK ([contentauth/c2patool#222](https://github.com/contentauth/c2patool/pull/222))
* Remove rust toolchain version lock ([contentauth/c2patool#221](https://github.com/contentauth/c2patool/pull/221))
* Update security guidance to link to SECURITY.md ([contentauth/c2patool#217](https://github.com/contentauth/c2patool/pull/217))

## 0.9.6
_30 July 2024_

* Pull latest c2pa-rs bug fixes into c2patool ([contentauth/c2patool#212](https://github.com/contentauth/c2patool/pull/212))
* only run tests/clippy if labeled ([contentauth/c2patool#205](https://github.com/contentauth/c2patool/pull/205))
* Bump env_logger from 0.10.2 to 0.11.4 ([contentauth/c2patool#204](https://github.com/contentauth/c2patool/pull/204))
* Updates cargo packages and cargo.deny file. ([contentauth/c2patool#200](https://github.com/contentauth/c2patool/pull/200))

## 0.9.5
_18 July 2024_

* Update to lastest c2pa-rs ([contentauth/c2patool#197](https://github.com/contentauth/c2patool/pull/197))
* added security.md ([contentauth/c2patool#196](https://github.com/contentauth/c2patool/pull/196))

## 0.9.4
_25 June 2024_

* Update c2patool ([contentauth/c2patool#190](https://github.com/contentauth/c2patool/pull/190))
* Match c2pa-rs minimum toolchain version and test in CI ([contentauth/c2patool#188](https://github.com/contentauth/c2patool/pull/188))
* Document how to specify an icon ([contentauth/c2patool#182](https://github.com/contentauth/c2patool/pull/182))

## 0.9.3
_29 May 2024_

* Remove binary modules ([contentauth/c2patool#179](https://github.com/contentauth/c2patool/pull/179))

## 0.9.2
_24 May 2024_

* Remove integration tests for now due to extraneous binaries ([contentauth/c2patool#178](https://github.com/contentauth/c2patool/pull/178))

## 0.9.1
_22 May 2024_

* Add better support for cargo-binstall ([contentauth/c2patool#177](https://github.com/contentauth/c2patool/pull/177))

## 0.9.0
_07 May 2024_

* Integrate with c2pa-rs 0.32.0, various test case fixes. ([contentauth/c2patool#175](https://github.com/contentauth/c2patool/pull/175))
* Add HTTP source option for trust config ([contentauth/c2patool#174](https://github.com/contentauth/c2patool/pull/174))

## 0.8.2
_28 March 2024_

* fixed c2patool asset name ([contentauth/c2patool#171](https://github.com/contentauth/c2patool/pull/171))

## 0.8.1
_25 March 2024_

* use c2pa-rs 0.31.1 for actions.changes support ([contentauth/c2patool#170](https://github.com/contentauth/c2patool/pull/170))

## 0.8.0
_20 March 2024_

* allow clients to sign with a process outside of c2patool ([contentauth/c2patool#169](https://github.com/contentauth/c2patool/pull/169))
* Add trust and verification options to c2pa_tool ([contentauth/c2patool#168](https://github.com/contentauth/c2patool/pull/168))
* adds version to c2patool artifact names ([contentauth/c2patool#158](https://github.com/contentauth/c2patool/pull/158))

## 0.7.0
_22 November 2023_

* updates to c2pa-rs v0.28.2 ([contentauth/c2patool#153](https://github.com/contentauth/c2patool/pull/153))
* Update to c2pa-rs 0.28.1

## 0.6.2
_05 October 2023_

* update to c2pa 0.27.1 ([contentauth/c2patool#146](https://github.com/contentauth/c2patool/pull/146))
* Merge branch 'main' of https://github.com/contentauth/c2patool
* Add Do not train example
* Upgrade to c2pa-rs 0.26.0 ([contentauth/c2patool#143](https://github.com/contentauth/c2patool/pull/143))
* Fix issue with docusaurus styling and fix broken links ([contentauth/c2patool#138](https://github.com/contentauth/c2patool/pull/138))
* Updates to c2pa-rs 0.25.1 ([contentauth/c2patool#128](https://github.com/contentauth/c2patool/pull/128))
* Fix windows release ([contentauth/c2patool#132](https://github.com/contentauth/c2patool/pull/132))

## 0.6.1
_24 July 2023_

* use compress-archive instead of tar ([contentauth/c2patool#130](https://github.com/contentauth/c2patool/pull/130))

## 0.6.0
_22 June 2023_

* update to c2pa-rs 0.24.0 ([contentauth/c2patool#127](https://github.com/contentauth/c2patool/pull/127))

## 0.5.4
_13 June 2023_

* integrate c2pa 23.0 bump version ([contentauth/c2patool#126](https://github.com/contentauth/c2patool/pull/126))
* Merge branch 'main' of https://github.com/contentauth/c2patool
* c2pa-rs 23.0 + updated test
* Update README.md ([contentauth/c2patool#124](https://github.com/contentauth/c2patool/pull/124))

## 0.5.3
_04 May 2023_

* Parent Ingredient JSON ([contentauth/c2patool#123](https://github.com/contentauth/c2patool/pull/123))

## 0.5.2
_19 April 2023_

* Ingredient thumbnails, extension cleanup, toolkit update ([contentauth/c2patool#120](https://github.com/contentauth/c2patool/pull/120))

## 0.5.1
_10 April 2023_

* Update README.md ([contentauth/c2patool#118](https://github.com/contentauth/c2patool/pull/118))
* Update expired sample certs ([contentauth/c2patool#113](https://github.com/contentauth/c2patool/pull/113))

## 0.5.0
_28 March 2023_

* New ingredient support and c2pa file formats ([contentauth/c2patool#111](https://github.com/contentauth/c2patool/pull/111))
* Leverage new Manifest & Ingredient, add Ingredient creation. ([contentauth/c2patool#107](https://github.com/contentauth/c2patool/pull/107))

## 0.4.0
_01 March 2023_

* Add --certs and --tree options ([contentauth/c2patool#106](https://github.com/contentauth/c2patool/pull/106))
* update to cp2pa 0.17.0 ([contentauth/c2patool#105](https://github.com/contentauth/c2patool/pull/105))
* Update for Clippy in Rust 1.67 ([contentauth/c2patool#101](https://github.com/contentauth/c2patool/pull/101))

## 0.3.9
_06 December 2022_

* update to c2pa-rs 0.16.0
* allows clients to output manifest report to specified directory ([contentauth/c2patool#91](https://github.com/contentauth/c2patool/pull/91))

## 0.3.8
_09 November 2022_

* Bump c2pa from 0.13.2 to 0.15.0 ([contentauth/c2patool#87](https://github.com/contentauth/c2patool/pull/87))
* Build infrastructure improvements ([contentauth/c2patool#85](https://github.com/contentauth/c2patool/pull/85))
* Fix new Clippy warning in Rust 1.65 ([contentauth/c2patool#84](https://github.com/contentauth/c2patool/pull/84))
* Readme updates ([contentauth/c2patool#62](https://github.com/contentauth/c2patool/pull/62))

## 0.3.7
_22 September 2022_

* Treat a source asset with a manifest store as a default parent ([contentauth/c2patool#76](https://github.com/contentauth/c2patool/pull/76))
* Fetch remote manifests for --info ([contentauth/c2patool#75](https://github.com/contentauth/c2patool/pull/75))

## 0.3.6
_16 September 2022_

* Update Cargo.lock when publishing crate ([contentauth/c2patool#71](https://github.com/contentauth/c2patool/pull/71))
* [IGNORE] update readme --info ([contentauth/c2patool#70](https://github.com/contentauth/c2patool/pull/70))
* Update Cargo.lock to 0.3.5

## 0.3.5
_15 September 2022_

* Upgrade cpufeatures to non-yanked version ([contentauth/c2patool#68](https://github.com/contentauth/c2patool/pull/68))
* Add --info option  ([contentauth/c2patool#65](https://github.com/contentauth/c2patool/pull/65))
* Updated publish workflow to upload binaries to GitHub ([contentauth/c2patool#58](https://github.com/contentauth/c2patool/pull/58))
* Fix Make release script & update readme ([contentauth/c2patool#55](https://github.com/contentauth/c2patool/pull/55))
* (Some version history omitted as we worked on some release process issues)

## 0.3.0
_18 August 2022_

* Rework c2patool parameters ([contentauth/c2patool#53](https://github.com/contentauth/c2patool/pull/53))
* Update to 0.11.0 c2pa-rs ([contentauth/c2patool#38](https://github.com/contentauth/c2patool/pull/38))
* Remove Homebrew, Git installation methods, and add "update" wording ([contentauth/c2patool#33](https://github.com/contentauth/c2patool/pull/33))

## 0.2.1
_29 June 2022_

* Add BMFF support for video & etc ([contentauth/c2patool#25](https://github.com/contentauth/c2patool/pull/25))

## 0.2.0
_28 June 2022_

* Upgrade to c2pa Rust SDK version 0.6.0 ([contentauth/c2patool#24](https://github.com/contentauth/c2patool/pull/24))
* Fix an error in the README documentation ([contentauth/c2patool#23](https://github.com/contentauth/c2patool/pull/23))
* Display help if there are no arguments on the command line ([contentauth/c2patool#21](https://github.com/contentauth/c2patool/pull/21))
* Bump anyhow from 1.0.57 to 1.0.58 ([contentauth/c2patool#17](https://github.com/contentauth/c2patool/pull/17))
* Updates examples to use ta_url instead of ta ([contentauth/c2patool#15](https://github.com/contentauth/c2patool/pull/15))

## 0.1.3
_17 June 2022_

* Update to latest c2pa Rust SDK ([contentauth/c2patool#12](https://github.com/contentauth/c2patool/pull/12))
* Add built-in default certs to make getting started easier ([contentauth/c2patool#9](https://github.com/contentauth/c2patool/pull/9))

## 0.1.2
_10 June 2022_

* Update crate's description field

## 0.1.1
_10 June 2022_

* Initial public release
