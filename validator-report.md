# libwebp Validator Report - impl-final-hardening

## Run Summary

Validator URL: https://github.com/safelibs/validator

Validator checkout: `/home/yans/safelibs/pipeline/ports/port-libwebp/validator`

Validator commit: `87b321fe728340d6fc6dd2f638583cca82c667c3`

Validator update: `git -C validator pull --ff-only` reported `Already up to date.`

Safe source commit tested: `4ec01468dd5d71aa28122acfa75950a9cf7c4592`

Safe source tree: `/home/yans/safelibs/pipeline/ports/port-libwebp/safe`

Final verification date: 2026-05-05 (America/Phoenix)

Final status: clean, waiver-free. The local compatibility matrix passed all
25 final-hardening checks. The full libwebp validator port matrix passed
176/176 cases, with 5 source cases, 171 usage cases, 0 failures, and 176 casts.

Waived testcase ids: none.

## Artifact Paths

- Local compatibility logs:
  `validator/artifacts/libwebp-safe/final-hardening/local/`
- Local compatibility summary:
  `validator/artifacts/libwebp-safe/final-hardening/local/summary.tsv`
- Local override Debian packages:
  `validator/artifacts/debs/local/libwebp/`
- Local port deb lock:
  `validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json`
- Validator proof:
  `validator/artifacts/libwebp-safe/proof/libwebp-safe-port-proof.json`
- Validator summary:
  `validator/artifacts/libwebp-safe/port/results/libwebp/summary.json`
- Per-case validator results:
  `validator/artifacts/libwebp-safe/port/results/libwebp/`
- Per-case validator logs:
  `validator/artifacts/libwebp-safe/port/logs/libwebp/`
- Per-case validator casts:
  `validator/artifacts/libwebp-safe/port/casts/libwebp/`
- Rendered validator evidence site:
  `validator/artifacts/libwebp-safe/site/`

## Commands And Checks Executed

Validator setup:

```bash
git -C /home/yans/safelibs/pipeline/ports/port-libwebp/validator status --short
git -C /home/yans/safelibs/pipeline/ports/port-libwebp/validator rev-parse HEAD
git -C /home/yans/safelibs/pipeline/ports/port-libwebp/validator pull --ff-only
sed -n '1,260p' /home/yans/safelibs/pipeline/ports/port-libwebp/validator/README.md
```

Local compatibility matrix, run from
`/home/yans/safelibs/pipeline/ports/port-libwebp` with logs under
`validator/artifacts/libwebp-safe/final-hardening/local/`:

```bash
cargo test --manifest-path safe/Cargo.toml --workspace --release
cargo build --manifest-path safe/Cargo.toml -p libsharpyuv -p libwebpdecoder -p libwebp -p libwebpdemux -p libwebpmux --release
cargo run -p xtask --manifest-path safe/Cargo.toml -- verify-symbols --baseline-dir safe/abi/original --libs libsharpyuv libwebpdecoder libwebp libwebpdemux libwebpmux
cargo run -p xtask --manifest-path safe/Cargo.toml -- verify-sonames --libs libsharpyuv libwebpdecoder libwebp libwebpdemux libwebpmux
cargo run -p xtask --manifest-path safe/Cargo.toml -- verify-needed --baseline-dir safe/abi/original --libs libsharpyuv libwebpdecoder libwebp libwebpdemux libwebpmux
cargo run -p xtask --manifest-path safe/Cargo.toml -- build-c-tests --suite all
ctest --test-dir safe/build/tests --output-on-failure
cargo run -p xtask --manifest-path safe/Cargo.toml -- build-upstream-public-api-test --source original/tests/public_api_test.c
ctest --test-dir safe/build/tests -R webp_public_api_test --output-on-failure
cargo run -p xtask --manifest-path safe/Cargo.toml -- relink-original-objects --manifest safe/abi/original/relink-manifest.json --fixtures webp_public_api_test cwebp dwebp img2webp webpinfo webpmux
cargo run -p xtask --manifest-path safe/Cargo.toml -- package-deb --output-dir /tmp/libwebp-safe-debs
cargo run -p xtask --manifest-path safe/Cargo.toml -- verify-install-tree --baseline safe/abi/original/install-surface.json --package-dir /tmp/libwebp-safe-debs
LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs ./test-original.sh --variant safe --only pkg-config-libwebp
LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs ./test-original.sh --variant safe --only pkg-config-libsharpyuv
LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs ./test-original.sh --variant safe --only cmake-webp
LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs ./test-original.sh --variant safe --only tools
LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs ./test-original.sh --variant safe --only webkitgtk
LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs ./test-original.sh --variant safe --only qt6-image-formats-plugins
LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs ./test-original.sh --variant safe --only sdl2-image
LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs ./test-original.sh --variant safe --only libvips
LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs ./test-original.sh --variant safe --only sail
LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs ./test-original.sh --variant safe
cargo run -p xtask --manifest-path safe/Cargo.toml -- unsafe-audit
git diff --exit-code -- safe/docs/unsafe-audit.md
cargo run -p xtask --manifest-path safe/Cargo.toml -- build-upstream-fuzzers --source-dir original/tests/fuzzer --safe-prefix safe/dist
```

Validator package/artifact refresh:

```bash
mkdir -p validator/artifacts/debs/local/libwebp validator/artifacts/libwebp-safe/proof
cp /tmp/libwebp-safe-debs/*.deb validator/artifacts/debs/local/libwebp/
python3 <local lock regeneration script>
```

The lock regeneration used the on-disk package metadata from `dpkg-deb -f`,
the SHA-256 digest of each copied `.deb`, and the current safe commit
`4ec01468dd5d71aa28122acfa75950a9cf7c4592`.

Full libwebp validator matrix and proof, run from
`/home/yans/safelibs/pipeline/ports/port-libwebp/validator`:

```bash
bash test.sh --config repositories.yml --tests-root tests --artifact-root artifacts/libwebp-safe --mode port --override-deb-root artifacts/debs/local --port-deb-lock artifacts/libwebp-safe/proof/local-port-debs-lock.json --library libwebp --record-casts
python3 tools/verify_proof_artifacts.py --config repositories.yml --tests-root tests --artifact-root artifacts/libwebp-safe --proof-output proof/libwebp-safe-port-proof.json --mode port --library libwebp --require-casts --min-source-cases 5 --min-usage-cases 171 --min-cases 176 --ports-root /home/yans/safelibs/pipeline/ports
python3 <per-result invariant scan>
python3 tools/render_site.py --config repositories.yml --tests-root tests --artifact-root artifacts/libwebp-safe --proof-path artifacts/libwebp-safe/proof/libwebp-safe-port-proof.json --output-root artifacts/libwebp-safe/site
bash scripts/verify-site.sh --config repositories.yml --tests-root tests --artifacts-root artifacts/libwebp-safe --proof-path artifacts/libwebp-safe/proof/libwebp-safe-port-proof.json --site-root artifacts/libwebp-safe/site --library libwebp
```

The invariant scan checked all 176 per-case JSON files for:

- `status == "passed"`
- `override_debs_installed == true`
- `port_commit == 4ec01468dd5d71aa28122acfa75950a9cf7c4592`
- the installed override package set exactly matching the eight safe packages
  in `local-port-debs-lock.json`

## Validator Results

`validator/artifacts/libwebp-safe/port/results/libwebp/summary.json`:

```json
{
  "schema_version": 2,
  "library": "libwebp",
  "mode": "port",
  "cases": 176,
  "source_cases": 5,
  "usage_cases": 171,
  "passed": 176,
  "failed": 0,
  "casts": 176,
  "duration_seconds": 0.0
}
```

The proof file
`validator/artifacts/libwebp-safe/proof/libwebp-safe-port-proof.json`
records the same totals, the same safe source commit, and the unsafe-block
summary added by `--ports-root /home/yans/safelibs/pipeline/ports`.

## Package Hashes

Package source version: `1.3.2-0.4build3+safelibs1`

| Package | Filename | Architecture | SHA-256 | Size |
|---|---|---|---|---:|
| libwebp7 | `libwebp7_1.3.2-0.4build3+safelibs1_amd64.deb` | amd64 | `fe95290b0d8fece7d9daa6e105d7bc93fcbf4f502b0286d4ffb4c26a570dff93` | 271926 |
| libwebpdemux2 | `libwebpdemux2_1.3.2-0.4build3+safelibs1_amd64.deb` | amd64 | `8155a463a1d378c4cc188fcfcd308a1bbf0af4b1400058026de9588002f76fc3` | 139400 |
| libwebpmux3 | `libwebpmux3_1.3.2-0.4build3+safelibs1_amd64.deb` | amd64 | `d7e0d3cd735ae2b3688cc6ad5a6f4537d15d8bcb97e3e6b145416f33308cea6b` | 292306 |
| libwebpdecoder3 | `libwebpdecoder3_1.3.2-0.4build3+safelibs1_amd64.deb` | amd64 | `3050ba41234d5eb92e85236d98498edbee544ceb6c7011f0f6c306e525a1a35d` | 131772 |
| libsharpyuv0 | `libsharpyuv0_1.3.2-0.4build3+safelibs1_amd64.deb` | amd64 | `0d7997f9c888e3bc69d8b9e1b64e52e0ae0c4a676a6cc4f8ed65d4d3148e5e61` | 91776 |
| libwebp-dev | `libwebp-dev_1.3.2-0.4build3+safelibs1_amd64.deb` | amd64 | `dd69d6f3ab4d4ef999ee3e986569ed3d3e3e37ed77638a151cc0d8d4197b5d00` | 445532 |
| libsharpyuv-dev | `libsharpyuv-dev_1.3.2-0.4build3+safelibs1_amd64.deb` | amd64 | `db118f662844c623f4d7ca9ffa15e7161564a5e2e152e46fc3926aa0b05c7b19` | 94194 |
| webp | `webp_1.3.2-0.4build3+safelibs1_amd64.deb` | amd64 | `11bf556e7d1ae98928823ab830660b76dc9179c4a9f1d8f99bfec1a0c883f30d` | 111160 |

## Failures, Fixes, And Regression Tests

Failures found: none.

Safe-side fixes applied: none. The committed source tree already passed the
final compatibility and validator matrices.

Regression tests added or updated: none. No unwaived validator failure or local
compatibility regression required a safe-side code fix.

Validator waivers added: none.

Validator tests, shared scripts, manifests, and tools were not modified.

## Unsafe Audit And CVE Coverage

`cargo run -p xtask --manifest-path safe/Cargo.toml -- unsafe-audit` completed
successfully, and `git diff --exit-code -- safe/docs/unsafe-audit.md` confirmed
the checked-in unsafe audit is current.

`safe/crates/webp-core/src/lib.rs` still keeps the crate-level
`#![allow(unsafe_op_in_unsafe_fn)]`. The current
`safe/docs/unsafe-audit.md` explicitly justifies that remaining allowance for
the translated codec internals and records that no other crate-wide
`unsafe_op_in_unsafe_fn` allowances are present.

The checked-in CVE regression surface remains present under
`safe/crates/webp-core/tests/`, including the `oversized_picture_alloc` and
`malformed_huffman_tables` test names mapped from `relevant_cves.json`.
