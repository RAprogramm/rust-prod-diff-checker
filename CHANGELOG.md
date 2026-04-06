# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Miscellaneous

- Bump version to 1.6.0 ([569dcc3](https://github.com/RAprogramm/rust-prod-diff-checker/commit/569dcc312bb7230efda2ee5ec9815c55f6f03b31))

## [1.5.0] - 2026-04-06

### Added

- **action**: Filter commits by author from config before analysis ([3a78625](https://github.com/RAprogramm/rust-prod-diff-checker/commit/3a786258773523f22482d5888fc7ea08f5f4b5ed))

### Documentation

- Update changelog [skip ci] ([f50b84b](https://github.com/RAprogramm/rust-prod-diff-checker/commit/f50b84b43824119e7b36bb0c3e93a57d312d0afb))
- Add ignored_authors to README documentation ([f21551d](https://github.com/RAprogramm/rust-prod-diff-checker/commit/f21551db608f7b86965c59a72afbaa63bc82f604))
- Update changelog [skip ci] ([9decfdb](https://github.com/RAprogramm/rust-prod-diff-checker/commit/9decfdb8badea92324229bd47de046b0efaa159b))

### Fixed

- Filter commits by author in GitHub Action ([7fa1c89](https://github.com/RAprogramm/rust-prod-diff-checker/commit/7fa1c89b69ab6c6ca174d06f3f53db4d216c8be6))

## [1.4.1] - 2026-04-06

### CI/CD

- Revert rustsec to v2 with Node 24 opt-in ([fcf8a1d](https://github.com/RAprogramm/rust-prod-diff-checker/commit/fcf8a1db4eee47ad58986cc1f269e8326065a31a))

### Miscellaneous

- Up version in `Cargo.toml` ([9f9de21](https://github.com/RAprogramm/rust-prod-diff-checker/commit/9f9de216b8244f66f48a64c79255cb24a2d81e84))

### Merge

- From remote ([58388c5](https://github.com/RAprogramm/rust-prod-diff-checker/commit/58388c52295bbbf5eef942e03ed4179a688b63e0))

## [1.4.0] - 2026-04-06

### CI/CD

- Update rustsec to v3 and dependabot to v2.2 ([4b742d0](https://github.com/RAprogramm/rust-prod-diff-checker/commit/4b742d08b7dac666c29919a4004d4d1f21e5c0b5))
- Update codecov test-results to v2 and use report_type for unified upload ([3ecbe6e](https://github.com/RAprogramm/rust-prod-diff-checker/commit/3ecbe6e80b8f63d2b05383cc481b732c3734316a))

### Miscellaneous

- Up version in `Cargo.toml` ([f2fd0a1](https://github.com/RAprogramm/rust-prod-diff-checker/commit/f2fd0a1051890473137cb8d8d8308733b3f111e1))

## [1.3.0] - 2026-04-06

### Added

- Add ignored_authors config for filtering commits by author ([faf2b57](https://github.com/RAprogramm/rust-prod-diff-checker/commit/faf2b57cda5322e2404922fa4048af620cede07b))

### CI/CD

- Update GitHub Actions to latest versions ([fd69600](https://github.com/RAprogramm/rust-prod-diff-checker/commit/fd69600d094206142f0cde6f7f56b2149f0f7c64))
- Update dorny/paths-filter to v4 for Node 24 support ([f94e937](https://github.com/RAprogramm/rust-prod-diff-checker/commit/f94e937fd6159f9a94d76aeab7cee7bd22c56b3d))
- Bump the github-actions group with 2 updates ([275bbca](https://github.com/RAprogramm/rust-prod-diff-checker/commit/275bbca7c2fe7a92e12f0f724c3f509fe285b186))
- Auto-update major version tag on release ([7fda879](https://github.com/RAprogramm/rust-prod-diff-checker/commit/7fda879ff2a1d172dd9102a81048464c9042cf3d))

### Documentation

- Update changelog [skip ci] ([2a4521e](https://github.com/RAprogramm/rust-prod-diff-checker/commit/2a4521e74a45ff9e2e3f9be82a16d235b20998c8))

### Fixed

- Escape brackets in doc comments for rustdoc ([862d2a1](https://github.com/RAprogramm/rust-prod-diff-checker/commit/862d2a1e1c9c6738b6f3230ce35861d9e34a5f5b))
- **ci**: Check remote tags instead of local in auto-tag job ([6bc07be](https://github.com/RAprogramm/rust-prod-diff-checker/commit/6bc07be4822176ffd2438b7de58819eff7449918))

### Miscellaneous

- Up version in `Cargo.toml` ([26034e0](https://github.com/RAprogramm/rust-prod-diff-checker/commit/26034e05069c913af15da8ada9770678103b4d79))
- Update dependencies ([8097eaf](https://github.com/RAprogramm/rust-prod-diff-checker/commit/8097eafb98a896f9cb0984b352b281434d3ec598))

### Deps

- Bump the rust-dependencies group with 3 updates ([#24](https://github.com/RAprogramm/rust-prod-diff-checker/issues/24)) ([411bffb](https://github.com/RAprogramm/rust-prod-diff-checker/commit/411bffb6a3125bd5b9ded8c5974b276b683ecc68))
- Bump clap from 4.5.54 to 4.5.56 in the rust-dependencies group ([#23](https://github.com/RAprogramm/rust-prod-diff-checker/issues/23)) ([57fcb2f](https://github.com/RAprogramm/rust-prod-diff-checker/commit/57fcb2f7d701b9777ce168403d56780c5d8a6c30))
- Bump the rust-dependencies group with 2 updates ([#22](https://github.com/RAprogramm/rust-prod-diff-checker/issues/22)) ([cbc5043](https://github.com/RAprogramm/rust-prod-diff-checker/commit/cbc5043968ed122b70b9c2e33874858476d6b0cf))
- Bump the rust-dependencies group with 4 updates ([#21](https://github.com/RAprogramm/rust-prod-diff-checker/issues/21)) ([ca2a737](https://github.com/RAprogramm/rust-prod-diff-checker/commit/ca2a73723d87d45e6bcb026cde004739327dd3d9))
- Bump the rust-dependencies group with 3 updates ([#20](https://github.com/RAprogramm/rust-prod-diff-checker/issues/20)) ([3c0e349](https://github.com/RAprogramm/rust-prod-diff-checker/commit/3c0e3497ffda1425e1b18eb0cfedf2fdef00f7cd))
- Bump the rust-dependencies group with 4 updates ([#19](https://github.com/RAprogramm/rust-prod-diff-checker/issues/19)) ([b588114](https://github.com/RAprogramm/rust-prod-diff-checker/commit/b5881146bec09c69960d0a2b7225257ea4fc1b9f))
- Bump toml in the rust-dependencies group ([#18](https://github.com/RAprogramm/rust-prod-diff-checker/issues/18)) ([c251875](https://github.com/RAprogramm/rust-prod-diff-checker/commit/c25187581cd4869246425660fa74a2eb39123057))
- Bump criterion from 0.8.0 to 0.8.1 in the rust-dependencies group ([#16](https://github.com/RAprogramm/rust-prod-diff-checker/issues/16)) ([1d23848](https://github.com/RAprogramm/rust-prod-diff-checker/commit/1d23848d912a435d12fb430052786aeb25862bb6))
- Bump the rust-dependencies group with 2 updates ([#15](https://github.com/RAprogramm/rust-prod-diff-checker/issues/15)) ([34ea316](https://github.com/RAprogramm/rust-prod-diff-checker/commit/34ea316fdb2ac2ea3240221d56e0725a8ed6a49c))

## [1.2.0] - 2025-11-28

### Added

- Improve CI pipeline and PR comment formatting ([c67eec2](https://github.com/RAprogramm/rust-prod-diff-checker/commit/c67eec2d8119758c75699f3acb2f43dceb5395b0))

### CI/CD

- Bump actions/download-artifact in the github-actions group ([688d68d](https://github.com/RAprogramm/rust-prod-diff-checker/commit/688d68dae792249fe43f4ffbf190ec8a1609606d))

### Documentation

- Add small PR guidelines and Russian translation ([fcd3820](https://github.com/RAprogramm/rust-prod-diff-checker/commit/fcd38205d7d6ad049a49eeb5c23af8f49ccaa010))
- Update changelog [skip ci] ([58f860f](https://github.com/RAprogramm/rust-prod-diff-checker/commit/58f860f2ff51667d0eb70aee93512ff752386211))

## [1.1.1] - 2025-11-23

### Miscellaneous

- Bump version to 1.1.1 ([0f82a83](https://github.com/RAprogramm/rust-prod-diff-checker/commit/0f82a830a364e2a69e9c73bbad8ca628c010ebb9))

## [1.1.0] - 2025-11-23

### CI/CD

- Optimize workflow structure and add concurrency ([bc642ab](https://github.com/RAprogramm/rust-prod-diff-checker/commit/bc642abbb48a4121699357a5c64104df24cfe0ab))
- Handle existing crate version gracefully ([022b163](https://github.com/RAprogramm/rust-prod-diff-checker/commit/022b1631ba1374cfd486f366f99a3816d632e210))
- Publish only on full version tags ([b996dde](https://github.com/RAprogramm/rust-prod-diff-checker/commit/b996ddedaee259f0f1f3206ce43918aa2c5b53fd))
- Fix publish with --allow-dirty ([5c81c4c](https://github.com/RAprogramm/rust-prod-diff-checker/commit/5c81c4c11f3a7856d74cfb5f17e662f40a180b36))
- Add crates.io publish job on tag push ([be5666f](https://github.com/RAprogramm/rust-prod-diff-checker/commit/be5666f0752b5899675f7d558a83567ad1428dbf))

### Documentation

- Update changelog [skip ci] ([2003518](https://github.com/RAprogramm/rust-prod-diff-checker/commit/20035182edef804871d17cb551cd6bc887027004))
- Update changelog [skip ci] ([b8470bf](https://github.com/RAprogramm/rust-prod-diff-checker/commit/b8470bf39c1a6424aac2cf0907c7b868692373ad))
- Update changelog [skip ci] ([4fb5333](https://github.com/RAprogramm/rust-prod-diff-checker/commit/4fb5333a34d79a0e2cc318fb3c7c7e2ca7855220))
- Update changelog [skip ci] ([9a19aaa](https://github.com/RAprogramm/rust-prod-diff-checker/commit/9a19aaa71425a0370921305e62ce4f422d824b21))
- Add crates.io and docs.rs badges ([510a122](https://github.com/RAprogramm/rust-prod-diff-checker/commit/510a122d8ed07cdfdc802543d063957c4c62a7c7))
- Update changelog [skip ci] ([621d6eb](https://github.com/RAprogramm/rust-prod-diff-checker/commit/621d6ebaa13da66d98276e002402d29f72c4d090))
- Update changelog [skip ci] ([8014b73](https://github.com/RAprogramm/rust-prod-diff-checker/commit/8014b73aa54143144fb72c48f1228256ba8cdd20))
- Comprehensive README update with detailed explanations ([174960f](https://github.com/RAprogramm/rust-prod-diff-checker/commit/174960f6b05faf923f07e8e655c0211fddf40caa))
- Update changelog [skip ci] ([aa98a92](https://github.com/RAprogramm/rust-prod-diff-checker/commit/aa98a927d5f1a77e03ea9b8a8275de52605f2121))
- Update changelog [skip ci] ([d84d7d5](https://github.com/RAprogramm/rust-prod-diff-checker/commit/d84d7d5dce12d68f93e75bb013bb3a36c432b5e6))
- Update changelog [skip ci] ([1e6368c](https://github.com/RAprogramm/rust-prod-diff-checker/commit/1e6368c6b0c10377811d2d55509e28f4c32b7ff6))
- Update changelog [skip ci] ([927bc61](https://github.com/RAprogramm/rust-prod-diff-checker/commit/927bc613af921ed690dfe8388d5ab1d31aa837a0))
- Update changelog [skip ci] ([43fe9cc](https://github.com/RAprogramm/rust-prod-diff-checker/commit/43fe9cc9dd8e947db5603858b170c7f91ddc760f))
- Add contributing guidelines with RustManifest reference ([9243dce](https://github.com/RAprogramm/rust-prod-diff-checker/commit/9243dce351121eb0d5bab646b7fa0dadd7ca662f))
- Update changelog [skip ci] ([c841ee0](https://github.com/RAprogramm/rust-prod-diff-checker/commit/c841ee03def8f3d5374ee1ec425bfa94d1975330))
- Update description to emphasize PR size limiting ([d97cce8](https://github.com/RAprogramm/rust-prod-diff-checker/commit/d97cce83a1b61ead194b3cea9aba5146f7eb0bdb))

### Fixed

- Correct action repository name in README ([766ea00](https://github.com/RAprogramm/rust-prod-diff-checker/commit/766ea0047959b53959d19e7da4b95fe652501e6d))
- Correct commit message format in contributing guide ([25241ef](https://github.com/RAprogramm/rust-prod-diff-checker/commit/25241ef734eb5558d443d14d83f429d3b2b76ea1))
- Changelog footer template macro scope and first release handling ([efca381](https://github.com/RAprogramm/rust-prod-diff-checker/commit/efca3811bb95f19e04ba376f0f2f9d444b273eea))

## [1.0.0] - 2025-11-23

### Added

- Add Codecov integration with coverage and test results ([4f6ab0e](https://github.com/RAprogramm/rust-prod-diff-checker/commit/4f6ab0edc3c96e58f3c8ea752e4011254e338af0))
- Add automatic changelog generation on push ([9ded527](https://github.com/RAprogramm/rust-prod-diff-checker/commit/9ded5272d6c57bb87daa7330a30be4f8ffb4cb66))
- Add professional tooling and masterror integration ([95c9d7a](https://github.com/RAprogramm/rust-prod-diff-checker/commit/95c9d7a85f060a1ce66247f46f5ed7a1c568fa7b))
- Implement semantic analyzer for Rust PR diffs ([7da8bbb](https://github.com/RAprogramm/rust-prod-diff-checker/commit/7da8bbb4b01670f1618eaad128072bff0f1cac59))

### CI/CD

- Integrate release workflow into CI ([8d08ebb](https://github.com/RAprogramm/rust-prod-diff-checker/commit/8d08ebb04aebd12d0b2405d5658d1596b8bac450))
- Use flag_management for automatic flag detection ([e73ea87](https://github.com/RAprogramm/rust-prod-diff-checker/commit/e73ea87b02f9c4f11515271e11171a39bb288f1d))
- Changelog updates on any push to main ([b9896aa](https://github.com/RAprogramm/rust-prod-diff-checker/commit/b9896aacceeb787a4230c028df95683607d380e9))
- Optimize with path filters to skip unnecessary jobs ([228cdb5](https://github.com/RAprogramm/rust-prod-diff-checker/commit/228cdb543a13294128c591b21b19fca52ada93ac))
- Fix JUnit XML file name pattern for Codecov ([a81fd90](https://github.com/RAprogramm/rust-prod-diff-checker/commit/a81fd907ae164a2d01585b9550c84377f2f91e79))
- Professional changelog with Keep a Changelog format ([778be2b](https://github.com/RAprogramm/rust-prod-diff-checker/commit/778be2bc9f9aa539e18180233b668b071f2a712a))
- Prevent changelog CI loop with skip ci ([7a55381](https://github.com/RAprogramm/rust-prod-diff-checker/commit/7a553816a32dd09f773a9721dd72cefdfa0d3de1))
- Add rebase before changelog push ([8a37590](https://github.com/RAprogramm/rust-prod-diff-checker/commit/8a37590e66a260072d87008082393ccfc20c7464))
- Add Codecov configuration ([b0ef8d2](https://github.com/RAprogramm/rust-prod-diff-checker/commit/b0ef8d2f2b08ee08a6dc2e0689bceab55e3b07e7))
- Move changelog to main CI workflow ([0d2794b](https://github.com/RAprogramm/rust-prod-diff-checker/commit/0d2794bdf8c83d4029d5709f346adc24ad3095ca))
- Use deploy key for changelog workflows ([5ab455d](https://github.com/RAprogramm/rust-prod-diff-checker/commit/5ab455de2bf887961753da4829735d20da9ce8a1))
- Bump the github-actions group with 6 updates ([aab6b41](https://github.com/RAprogramm/rust-prod-diff-checker/commit/aab6b41b3a6c4849fa5f7ca97802665a50cb4d5c))

### Changed

- Format property tests ([33ce970](https://github.com/RAprogramm/rust-prod-diff-checker/commit/33ce970f8d1ba136eba75a5fc98115c25ceabd16))

### Documentation

- Update release badge to show version ([051b477](https://github.com/RAprogramm/rust-prod-diff-checker/commit/051b4775a7c7bf01c9508a5b677c5c3d9d18ec4a))
- Update changelog [skip ci] ([1565ad7](https://github.com/RAprogramm/rust-prod-diff-checker/commit/1565ad723dcd5aefff4116de0362b332d539e2d4))
- Update changelog [skip ci] ([fc6605a](https://github.com/RAprogramm/rust-prod-diff-checker/commit/fc6605aa7293edbd0eda15b6f2dc9c07cdc49475))
- Update changelog [skip ci] ([c1d1662](https://github.com/RAprogramm/rust-prod-diff-checker/commit/c1d1662d2ba50680ee64d1a0bc46697be745962b))
- Update changelog [skip ci] ([3308689](https://github.com/RAprogramm/rust-prod-diff-checker/commit/33086895e1f4f4fd2690e09797bd2d2febd92ded))
- Add Hits-of-Code badge ([8971ba7](https://github.com/RAprogramm/rust-prod-diff-checker/commit/8971ba7612804487a006fec6370821072d5af4d2))
- Update changelog [skip ci] ([0982176](https://github.com/RAprogramm/rust-prod-diff-checker/commit/0982176a4e2f5afe119cb1f34366954848d848d3))
- Update changelog ([35cb5f0](https://github.com/RAprogramm/rust-prod-diff-checker/commit/35cb5f073bd62f6bc03389eefa48a88845d8e20f))
- Add coverage graph descriptions ([4cb52ac](https://github.com/RAprogramm/rust-prod-diff-checker/commit/4cb52ac3e8c52497fe862d08b9e2281541e51946))
- Update changelog ([a3275e0](https://github.com/RAprogramm/rust-prod-diff-checker/commit/a3275e05a1464903253c28828ed94b21bb72128c))
- Fix badges in README ([27fc8f4](https://github.com/RAprogramm/rust-prod-diff-checker/commit/27fc8f4e3e2d26cb4fba9a87744ad7ac9b4ae18c))
- Fix license badge link to REUSE ([f6da153](https://github.com/RAprogramm/rust-prod-diff-checker/commit/f6da153a1943dd61059dd97d9c8ced455fa63ba2))
- Add status badges to README ([7a0e88b](https://github.com/RAprogramm/rust-prod-diff-checker/commit/7a0e88b4c6fd4995f3b7bc73af3c6dde6df72de9))

### Fixed

- Improve changelog and release workflow logic ([09e2feb](https://github.com/RAprogramm/rust-prod-diff-checker/commit/09e2feb7d65b2f7c4f96f1bc3a4907cd351df4e5))
- Add grouping for github-actions in dependabot ([80e7f0e](https://github.com/RAprogramm/rust-prod-diff-checker/commit/80e7f0e060d51c50ce5c5db489df54d60e61b14f))
- Fmt config ([58a283a](https://github.com/RAprogramm/rust-prod-diff-checker/commit/58a283a4544523728ee0fc1f52fb16080e1fd96e))

### Miscellaneous

- Add REUSE compliance ([b0d592a](https://github.com/RAprogramm/rust-prod-diff-checker/commit/b0d592a913354137113b4be3eaf4a0daaa33d834))

### Testing

- Filter Rust keywords in property tests ([1ee0fcc](https://github.com/RAprogramm/rust-prod-diff-checker/commit/1ee0fcca334e6cc4077b93a47ba453b6c0cab6cf))

[Unreleased]: https://github.com/RAprogramm/rust-prod-diff-checker/compare/v1.5.0...HEAD
[1.5.0]: https://github.com/RAprogramm/rust-prod-diff-checker/compare/v1.4.1...v1.5.0
[1.4.1]: https://github.com/RAprogramm/rust-prod-diff-checker/compare/v1.4.0...v1.4.1
[1.4.0]: https://github.com/RAprogramm/rust-prod-diff-checker/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/RAprogramm/rust-prod-diff-checker/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/RAprogramm/rust-prod-diff-checker/compare/v1.1.1...v1.2.0
[1.1.1]: https://github.com/RAprogramm/rust-prod-diff-checker/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/RAprogramm/rust-prod-diff-checker/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/RAprogramm/rust-prod-diff-checker/releases/tag/v1.0.0

