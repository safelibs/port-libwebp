# libwebp Validator Report — Through Phase 4 (Usage-Case Runtime Regressions)

## Run summary

Validator commit: 87b321fe728340d6fc6dd2f638583cca82c667c3
Safe source commit tested: e98031dac96b1ee74e8a4b62165b12f11a12ec9c

Mode: port
Library: libwebp
Validator URL: https://github.com/safelibs/validator
Local release tag: local-e98031dac96b
Override deb root: validator/artifacts/debs/local
Override leaf: validator/artifacts/debs/local/libwebp/
Artifact root: validator/artifacts/libwebp-safe/
Proof JSON: validator/artifacts/libwebp-safe/proof/libwebp-safe-port-proof.json
Lock JSON: validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json
Per-case results dir: validator/artifacts/libwebp-safe/port/results/libwebp/
Per-case logs dir: validator/artifacts/libwebp-safe/port/logs/libwebp/
Per-case casts dir: validator/artifacts/libwebp-safe/port/casts/libwebp/

The `Safe source commit tested` value above equals
`libraries[0].commit` in `local-port-debs-lock.json` and
`libraries[0].port_commit` in `libwebp-safe-port-proof.json`.

## Testcase totals

Source cases: 5
Usage cases: 171
Total cases: 176
Passed: 176
Failed: 0
Casts recorded: 176

Checks executed: validator unit suite (make -C validator unit, 110 tests); full-inventory testcase manifest check (make -C validator check-testcases); libwebp-only manifest check with live thresholds (python3 validator/tools/testcases.py --library libwebp --check --min-source-cases 5 --min-usage-cases 171 --min-cases 176); full port matrix run (bash test.sh --mode port --library libwebp --record-casts) producing 176 per-case JSON results, 177 per-case logs (including dependent-package install logs), and 176 per-case asciinema casts; proof verification (python3 tools/verify_proof_artifacts.py).

Failures found: 0

Waived testcase ids:

## Per-case override install status

Every per-case result JSON under
`validator/artifacts/libwebp-safe/port/results/libwebp/` reports
`override_debs_installed == true`. The
`dpkg --install --force-downgrade` in
`validator/tests/_shared/install_override_debs.sh` unpacks all eight
local override packages over the stock noble apt versions before each
testcase runs.

## Safe Debian packages (built via `cargo run -p xtask -- package-deb`)

All eight packages were produced from `safe/` at commit
`e98031dac96b1ee74e8a4b62165b12f11a12ec9c` using
`cargo run -p xtask --manifest-path safe/Cargo.toml -- package-deb
--output-dir safe/dist`, then copied into
`validator/artifacts/debs/local/libwebp/`.

| Package | Filename | Architecture | SHA-256 | Size (bytes) |
|---|---|---|---|---|
| libwebp7 | libwebp7_1.3.2-0.4build3+safelibs1_amd64.deb | amd64 | `fe95290b0d8fece7d9daa6e105d7bc93fcbf4f502b0286d4ffb4c26a570dff93` | 271926 |
| libwebpdemux2 | libwebpdemux2_1.3.2-0.4build3+safelibs1_amd64.deb | amd64 | `ee831697509f1594e7ff489d3271dbf8ba2180db7799760ef738d3aa3ec5b571` | 139234 |
| libwebpmux3 | libwebpmux3_1.3.2-0.4build3+safelibs1_amd64.deb | amd64 | `d7e0d3cd735ae2b3688cc6ad5a6f4537d15d8bcb97e3e6b145416f33308cea6b` | 292306 |
| libwebpdecoder3 | libwebpdecoder3_1.3.2-0.4build3+safelibs1_amd64.deb | amd64 | `3050ba41234d5eb92e85236d98498edbee544ceb6c7011f0f6c306e525a1a35d` | 131772 |
| libsharpyuv0 | libsharpyuv0_1.3.2-0.4build3+safelibs1_amd64.deb | amd64 | `0d7997f9c888e3bc69d8b9e1b64e52e0ae0c4a676a6cc4f8ed65d4d3148e5e61` | 91776 |
| libwebp-dev | libwebp-dev_1.3.2-0.4build3+safelibs1_amd64.deb | amd64 | `13475fbb7edf0a2766f3f13681de83c1ee7e64ad22ee38ccc5fda27ba16afc9c` | 444842 |
| libsharpyuv-dev | libsharpyuv-dev_1.3.2-0.4build3+safelibs1_amd64.deb | amd64 | `db118f662844c623f4d7ca9ffa15e7161564a5e2e152e46fc3926aa0b05c7b19` | 94194 |
| webp | webp_1.3.2-0.4build3+safelibs1_amd64.deb | amd64 | `11bf556e7d1ae98928823ab830660b76dc9179c4a9f1d8f99bfec1a0c883f30d` | 111160 |

Package version (Debian field): `1.3.2-0.4build3+safelibs1`.

These eight `.deb` files exist on disk under
`validator/artifacts/debs/local/libwebp/` and the SHA-256 and size
columns above are exactly the values recorded in
`validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json`
under `libraries[0].debs[]`. The `.deb` files were not rebuilt during
this phase 1 re-run because `git log -1 --format=%H -- safe` already
points at `e98031dac96b1ee74e8a4b62165b12f11a12ec9c`, which equals the
`libraries[0].commit` value in the lock and the
`libraries[0].port_commit` value in the proof; per the workflow plan,
"do not rebuild packages merely to make the lock commit equal a
report-only commit; rebuild only when `safe/` changes."

## Commands executed

From `/home/yans/safelibs/pipeline/ports/port-libwebp/`:

```bash
# Validator checkout setup (idempotent; already excluded and clean).
grep -qxF '/validator/' .git/info/exclude || printf '\n/validator/\n' >> .git/info/exclude
git -C validator diff --quiet
git -C validator diff --cached --quiet
git -C validator rev-parse HEAD
# -> 87b321fe728340d6fc6dd2f638583cca82c667c3

# Validator tooling and live-inventory checks.
make -C validator unit
make -C validator check-testcases
SOURCE_COUNT=$(find validator/tests/libwebp/tests/cases/source -maxdepth 1 -name '*.sh' | wc -l)
USAGE_COUNT=$(find validator/tests/libwebp/tests/cases/usage  -maxdepth 1 -name '*.sh' | wc -l)
TOTAL_COUNT=$((SOURCE_COUNT + USAGE_COUNT))
python3 validator/tools/testcases.py \
  --config validator/repositories.yml \
  --tests-root validator/tests \
  --library libwebp \
  --check \
  --min-source-cases "$SOURCE_COUNT" \
  --min-usage-cases "$USAGE_COUNT" \
  --min-cases "$TOTAL_COUNT"
```

From `/home/yans/safelibs/pipeline/ports/port-libwebp/validator/`
(reuses already-built artifacts; no `cargo run -p xtask -- package-deb`
re-run because `safe/` HEAD was unchanged):

```bash
bash test.sh \
  --config repositories.yml \
  --tests-root tests \
  --artifact-root artifacts/libwebp-safe \
  --mode port \
  --override-deb-root artifacts/debs/local \
  --port-deb-lock artifacts/libwebp-safe/proof/local-port-debs-lock.json \
  --library libwebp \
  --record-casts

python3 tools/verify_proof_artifacts.py \
  --config repositories.yml \
  --tests-root tests \
  --artifact-root artifacts/libwebp-safe \
  --proof-output proof/libwebp-safe-port-proof.json \
  --mode port \
  --library libwebp \
  --require-casts \
  --min-source-cases 5 \
  --min-usage-cases 171 \
  --min-cases 176 \
  --ports-root /home/yans/safelibs/pipeline/ports
```

## Failures and triage

No validator testcases failed. Every one of the 5 source/CLI cases
(`cwebp-dwebp-roundtrip`, `decode-c-api-smoke`,
`malformed-webp-rejection`, `webpinfo-inspection`, `webpmux-metadata`)
and all 171 usage cases across `ffmpeg`, `ffprobe`, `libsdl2-image`,
`python3-pil`, `vips`, and `webp-pixbuf-loader` reports `passed` in
`validator/artifacts/libwebp-safe/port/results/libwebp/<case>.json`.
There are no triage entries (no source/CLI bucket, no usage/runtime
bucket, no package/install bucket). Casts and logs are present for
every case under
`validator/artifacts/libwebp-safe/port/casts/libwebp/` and
`validator/artifacts/libwebp-safe/port/logs/libwebp/`.

## Validator-bug waivers

None. `Waived testcase ids:` is empty.

## Phase 2 (Package Override And Waiver Gate) adjudication

Phase 2 reviewed the phase 1 baseline artifacts in place and confirmed
no package, install, symbol, dependency, or validator-bug waiver work
is required:

- All 176 per-case JSON files under
  `validator/artifacts/libwebp-safe/port/results/libwebp/` report
  `override_debs_installed == true`. None report install-related
  failure.
- `summary.json` reports `passed=176, failed=0`. No testcase needs a
  validator-bug waiver, so the original-package matrix was not
  re-run; `Waived testcase ids:` remains empty and no original-vs-safe
  evidence rows are needed.
- `cargo run -p xtask -- package-deb` already produced the eight
  expected packages from the phase-2 packaging fix at safe commit
  `e98031dac96b1ee74e8a4b62165b12f11a12ec9c`
  (`ci: enable PNG/JPEG/TIFF in webp tools`), and those `.deb` files
  remain on disk under `validator/artifacts/debs/local/libwebp/`.
- `validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json`
  has eight `libraries[0].debs[]` entries whose `filename`, `size`,
  and `sha256` exactly match the on-disk `.deb` files (verified by
  hashing each file in `validator/artifacts/debs/local/libwebp/` and
  comparing against the lock). `libraries[0].commit` is
  `e98031dac96b1ee74e8a4b62165b12f11a12ec9c`, which is also the output
  of `git log -1 --format=%H -- safe`, and equals
  `libraries[0].port_commit` in
  `validator/artifacts/libwebp-safe/proof/libwebp-safe-port-proof.json`.
- No new `safe/` source, packaging, ABI, or include changes were made
  in phase 2; the phase commit is therefore an empty commit named for
  `impl_package_provenance_waiver_gate`. Per the plan, packages are
  not rebuilt merely to advance the lock commit when `safe/` is
  unchanged.

## Phase 3 (Source/CLI Regressions) adjudication

No source/CLI failures. Phase 3 reviewed the same in-place artifacts:

- All five source/CLI cases (`cwebp-dwebp-roundtrip`,
  `decode-c-api-smoke`, `malformed-webp-rejection`,
  `webpinfo-inspection`, `webpmux-metadata`) report `status == "passed"`
  and `exit_code == 0` in
  `validator/artifacts/libwebp-safe/port/results/libwebp/<case>.json`,
  with `override_debs_installed == true` on every per-case JSON. The
  override `.deb` set under `validator/artifacts/debs/local/libwebp/`
  is the eight phase-2 packages tabulated above; SHA-256 and size
  values still match `libraries[0].debs[]` in
  `validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json`,
  and `libraries[0].commit` /
  `libraries[0].port_commit` remain
  `e98031dac96b1ee74e8a4b62165b12f11a12ec9c`.
- `validator/artifacts/libwebp-safe/port/results/libwebp/summary.json`
  reports `cases=176, source_cases=5, usage_cases=171, passed=176,
  failed=0`.
- No new safe regression test was added because there is no
  source/CLI failure to reproduce. The existing safe-side regressions
  (`safe/tests/c/decode_api_test.c`,
  `safe/tests/c/demux_animdecode_test.c`,
  `safe/tests/c/encode_api_test.c`,
  `safe/tests/c/runtime_abi_test.c`,
  `safe/tests/c/upstream_public_api_test.c`,
  `safe/crates/webp-core/tests/encode_security.rs`,
  `safe/crates/webp-core/tests/cve_2020_36332.rs`,
  `safe/crates/webp-core/tests/malformed_huffman_tables.rs`)
  continue to gate the source-facing surface that the validator
  exercises through `cwebp`, `dwebp`, `webpinfo`, `webpmux`, and the
  `WebPGetInfo` / `WebPDecode*` C API entry points.
- No `safe/` source, ABI, or include changes were made in phase 3, so
  no rebuild of the override packages was required and the safe
  source commit tested is unchanged. The phase commit is therefore
  an empty commit named for `impl_source_cli_failures`.
- `Waived testcase ids:` remains empty: there is no source/CLI
  failure to waive and no original-vs-safe original-package matrix
  re-run is required.

## Phase 4 (Usage-Case Runtime Regressions) adjudication

No usage-case failures. Phase 4 reviewed the same in-place artifacts:

- All 171 usage cases under
  `validator/artifacts/libwebp-safe/port/results/libwebp/usage-*.json`
  report `status == "passed"`. The breakdown matches the phase 1
  baseline: 41 `usage-ffmpeg-*`, 7 `usage-ffprobe-*`, 7
  `usage-libsdl2-image-*`, 90 `usage-python3-pil-*`, 17 `usage-vips-*`,
  and 9 `usage-webp-pixbuf-loader-*` cases all pass against the eight
  override `.deb` files in `validator/artifacts/debs/local/libwebp/`.
- Every per-case JSON still reports `override_debs_installed == true`,
  so each Pillow, FFmpeg/FFprobe, libvips, SDL2_image, and
  webp-pixbuf-loader case is exercising the safe libwebp/libsharpyuv
  builds rather than the stock Ubuntu noble packages.
- `validator/artifacts/libwebp-safe/port/results/libwebp/summary.json`
  still reports `cases=176, source_cases=5, usage_cases=171,
  passed=176, failed=0` and the SHA-256/size columns above still match
  `libraries[0].debs[]` in
  `validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json`.
  `libraries[0].commit` and `libraries[0].port_commit` remain
  `e98031dac96b1ee74e8a4b62165b12f11a12ec9c`, matching
  `git log -1 --format=%H -- safe`.
- No new safe-side regression test was added in phase 4 because there
  is no usage failure to reproduce. The existing C and Rust regressions
  registered in `safe/tests/c/CMakeLists.txt`
  (`safe/tests/c/decode_api_test.c`,
  `safe/tests/c/demux_animdecode_test.c`,
  `safe/tests/c/encode_api_test.c`,
  `safe/tests/c/runtime_abi_test.c`,
  `safe/tests/c/upstream_public_api_test.c`) and under
  `safe/crates/webp-core/tests/`
  (`encode_security.rs`, `cve_2020_36332.rs`,
  `malformed_huffman_tables.rs`) continue to gate the
  `WebPDecode*` / `WebPEncode*` / `WebPMux*` / `WebPDemux*` C ABI
  surface that the validator drives through Pillow, FFmpeg/FFprobe,
  libvips, SDL2_image, and webp-pixbuf-loader.
- No `safe/` source, ABI, include, or packaging changes were made in
  phase 4, so no rebuild of the override `.deb` files was required and
  the safe source commit tested is unchanged. The phase commit is
  therefore an empty commit named for `impl_usage_runtime_failures`.
- `Waived testcase ids:` remains empty: there is no usage-case failure
  to waive and no original-vs-safe original-package matrix re-run is
  required.

## Unsafe-block snapshot (from proof)

- `rs_loc`: 92749
- Total unsafe blocks: 194 (abi-shaped 133, voluntary 61, no-enclosing 0)
- Blocks with any flagged op: 12 (3 voluntary)
- Op buckets: `from_raw`=9, `static_mut`=4, `zeroed_uninit`=1

## Notes

- The nested `validator/` checkout remains excluded from the parent
  repo via `.git/info/exclude` and is not added to git.
- The `.deb` files in the parent worktree's top level (e.g.
  `libwebp7_1.3.2-0.4build3+safelibs1_amd64.deb`) are CI-build
  byproducts left over from earlier work and are not tracked by git.
  The validator override packages live under
  `validator/artifacts/debs/local/libwebp/` and are the only copies
  consumed by `bash test.sh`.
- This phase 1 re-run did not modify `safe/`. The single `safe/` change
  required to reach the all-pass baseline (the `WEBP_HAVE_PNG`,
  `WEBP_HAVE_JPEG`, `WEBP_HAVE_TIFF`, and `WEBP_HAVE_GIF` defines in
  `safe/xtask/src/package.rs`) is already part of
  `e98031dac96b1ee74e8a4b62165b12f11a12ec9c` from the prior phase 2
  commit `e98031d ci: enable PNG/JPEG/TIFF in webp tools`.
