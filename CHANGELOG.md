## 2021-09-12, Version v0.2.0
### Commits
- [[`b8ae60b369`](https://github.com/bltavares/atomic-shim/commit/b8ae60b36902dac3c40b6afd3724aac9d5691bca)] (cargo-release) version 0.2.0 (Bruno Tavares)
- [[`740c4806fb`](https://github.com/bltavares/atomic-shim/commit/740c4806fbae69e6dbd2919103f89768d5dc1acb)] Add a changelog (Bruno Tavares)
- [[`7bb15a3acf`](https://github.com/bltavares/atomic-shim/commit/7bb15a3acfd879a68a3c24124e911546691b4d33)] Merge pull request #1 from paolobarbolini/master (Bruno Tavares)
- [[`719ebb7d43`](https://github.com/bltavares/atomic-shim/commit/719ebb7d433b68df09a037da3bd4fd590dfd85d4)] Bump crossbeam and switch to crossbeam-utils (Paolo Barbolini)
- [[`f9e277e900`](https://github.com/bltavares/atomic-shim/commit/f9e277e90071213520deafb2f35d523249d1cec9)] Fix urls (Bruno Tavares)

### Stats
```diff
 CHANGELOG.md | 27 +++++++++++++++++++++++++++
 Cargo.toml   | 12 ++++++------
 README.md    |  4 ++--
 src/shim.rs  |  2 +-
 4 files changed, 36 insertions(+), 9 deletions(-)
```


## 2020-05-20, Version 0.1.0
### Commits
- [[`3a0fe4e1ec`](https://github.com/bltavares/atomic-shim/commit/3a0fe4e1ec508cde094579a5c84f174d995c5bdb)] Link things to their places (Bruno Tavares)
- [[`ed24aca271`](https://github.com/bltavares/atomic-shim/commit/ed24aca2716d21bbb39b2eec7ecab45bd61bf530)] Fix ci and add badge status (Bruno Tavares)
- [[`429b1558e7`](https://github.com/bltavares/atomic-shim/commit/429b1558e7b712c801ce59d337db403ccbd24a06)] Fix cargo clippy on ci (Bruno Tavares)
- [[`00e1c69a71`](https://github.com/bltavares/atomic-shim/commit/00e1c69a718b9c34fc3b493cec872659e97603d8)] Prepare for publication (Bruno Tavares)
- [[`dcdc35dd1b`](https://github.com/bltavares/atomic-shim/commit/dcdc35dd1b3bc345fb57ceab4c40b883565e5a7b)] Rename checks.yml to main.yml (Bruno Tavares)
- [[`70744ec58a`](https://github.com/bltavares/atomic-shim/commit/70744ec58a4edd6c619b0c8ba8457b5bcbeef7b4)] Document test and fix lints (Bruno Tavares)
- [[`91584c3174`](https://github.com/bltavares/atomic-shim/commit/91584c3174c68d45884312383d338e7090817135)] Document and setup (Bruno Tavares)
- [[`74eb0ec996`](https://github.com/bltavares/atomic-shim/commit/74eb0ec996b9bbee842c187b82e9279d370b606e)] Hold lock during operations (Bruno Tavares)
- [[`66e7049125`](https://github.com/bltavares/atomic-shim/commit/66e70491256997303747f56cbee9c06b44efc58d)] Initial commit (Bruno Tavares)

### Stats
```diff
 .github/workflows/cross_compile.yml |  29 +-
 .github/workflows/main.yml          |  80 ++++-
 .gitignore                          |   2 +-
 Cargo.toml                          |  23 +-
 LICENSE-APACHE                      | 201 +++++++++-
 LICENSE-MIT                         |  21 +-
 README.md                           | 105 +++++-
 src/lib.rs                          |  62 +++-
 src/shim.rs                         | 828 +++++++++++++++++++++++++++++++++++++-
 9 files changed, 1351 insertions(+)
```


