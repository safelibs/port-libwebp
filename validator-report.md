# libwebp Validator Report — Phase 3 Source-Facing And CLI Regressions

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

Checks executed: validator unit suite (make -C validator unit, 110 tests); testcase manifest check (make -C validator check-testcases); libwebp-only manifest check (python3 validator/tools/testcases.py --library libwebp --check); full port matrix run (bash test.sh --mode port --library libwebp --record-casts); proof verification (python3 tools/verify_proof_artifacts.py).

Failures found: 0

Waived testcase ids:

## Per-case override install status

Every per-case result JSON under
`validator/artifacts/libwebp-safe/port/results/libwebp/` reports
`override_debs_installed == true`. The `dpkg --install --force-downgrade`
in `validator/tests/_shared/install_override_debs.sh` unpacks all eight
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

The seven library/dev packages have the same SHA-256 hashes recorded in
the phase 1 baseline because their staged content (Rust shared
libraries, headers, pkg-config and CMake fragments) was unchanged. The
`webp` package SHA-256 changed from
`af8f1dba3a6b59c27923e003acd0d5e7e0b183fd0580635248f345e312a1f45c`
(98178 bytes) to
`11bf556e7d1ae98928823ab830660b76dc9179c4a9f1d8f99bfec1a0c883f30d`
(111160 bytes) because the upstream tool sources are now compiled with
the `WEBP_HAVE_PNG=1`, `WEBP_HAVE_JPEG=1`, `WEBP_HAVE_TIFF=1`, and
`WEBP_HAVE_GIF=1` defines so `pngdec.c`, `jpegdec.c`, `tiffdec.c`, and
`image_enc.c` PNG output compile in real implementations instead of
the stub branches.

## Commands executed

From `/home/yans/safelibs/pipeline/ports/port-libwebp/`:

```bash
# 1. Rebuild the eight safe Debian packages from the patched xtask.
cargo run -p xtask --manifest-path safe/Cargo.toml -- package-deb --output-dir safe/dist

# 2. Refresh the local override root and rewrite the port-debs lock so
#    `commit` matches the latest `safe/` commit and SHA-256 + size
#    fields exactly track the on-disk .deb files.
rm -rf validator/artifacts/debs/local/libwebp
mkdir -p validator/artifacts/debs/local/libwebp validator/artifacts/libwebp-safe/proof
find safe/dist -maxdepth 1 -type f -name '*.deb' \
  -exec cp -f -t validator/artifacts/debs/local/libwebp {} +
# (then run the inline python that wrote local-port-debs-lock.json with
#  commit=e98031dac96b1ee74e8a4b62165b12f11a12ec9c and the new hashes)
```

From `/home/yans/safelibs/pipeline/ports/port-libwebp/validator/`:

```bash
# 3. Run the full libwebp port matrix and verify the proof.
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

## Phase 3 outcome

No source/CLI failures. All five source/CLI testcases —
`cwebp-dwebp-roundtrip`, `decode-c-api-smoke`,
`malformed-webp-rejection`, `webpinfo-inspection`, and
`webpmux-metadata` — passed in the phase 2 port matrix run, with
`override_debs_installed == true` in every per-case result JSON. Phase 3
required no `safe/` source/CLI fixes, no new regression tests under
`safe/tests/c/` or `safe/crates/webp-core/tests/`, no ABI export
changes, no waivers, and no rebuild of the eight Debian packages: the
artifacts on disk
(`validator/artifacts/libwebp-safe/port/results/libwebp/*.json`,
`validator/artifacts/libwebp-safe/proof/libwebp-safe-port-proof.json`,
and `validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json`)
already cover the current `safe/` HEAD
(`e98031dac96b1ee74e8a4b62165b12f11a12ec9c`), which equals
`libraries[0].commit` in the lock and `libraries[0].port_commit` in
the proof.

The phase 3 commit (`impl_source_cli_failures`) is therefore an empty
commit recording that the source/CLI gate had nothing to fix on top of
phase 2.

## Phase 2 fix

Phase 1 surfaced one failing case,
`usage-vips-webpload-all-frames`. The first failing log line was

> `PNG support not compiled. Please install the libpng development package before building.`

emitted by `img2webp` from the safe `webp` package while encoding three
PNG frames into `anim.webp`. The string is printed by the
`#else // !WEBP_HAVE_PNG` stub of `original/imageio/pngdec.c` (and the
matching stub in `image_enc.c`). Although `safe/xtask/src/package.rs`
was already linking `libpng`/`libjpeg`/`libtiff` via pkg-config and
compiling the upstream `pngdec.c`/`jpegdec.c`/`tiffdec.c`/`image_enc.c`
sources for `cwebp`, `dwebp`, `img2webp`, `anim_diff`, and `anim_dump`,
the `WEBP_HAVE_PNG`, `WEBP_HAVE_JPEG`, and `WEBP_HAVE_TIFF` defines
were never passed to the compiler, so every one of those translation
units fell through to its stub branch.

Fix: declare the matching `WEBP_HAVE_*=1` defines in `tool_spec` for
each tool that already links those system libraries. `vwebp`,
`webpinfo`, and `webpmux` are unchanged; `gif2webp` already declared
`WEBP_HAVE_GIF=1`. Verified post-rebuild that `img2webp` links
`libpng16.so.16`, `libjpeg.so.8`, and `libtiff.so.6`, and that
`usage-vips-webpload-all-frames` now passes (status `passed`,
`exit_code: 0`, `override_debs_installed: true`).

## Failures

None. All 176 cases pass with override packages installed.

## Validator-bug waivers

None. No waiver was added for this phase, so the original-package
matrix did not need to be re-run.

## Unsafe-block snapshot (from proof)

- `rs_loc`: 92749
- Total unsafe blocks: 194 (abi-shaped 133, voluntary 61, no-enclosing 0)
- Blocks with any flagged op: 12 (3 voluntary)
- Op buckets: `from_raw`=9, `static_mut`=4, `zeroed_uninit`=1

## Notes

- The only `safe/` change made this phase is the
  `WEBP_HAVE_PNG/JPEG/TIFF/GIF` define list in
  `safe/xtask/src/package.rs`; debian/control, debian/rules,
  debian/changelog, the per-package `*.install` files, the
  pkg-config/CMake templates, headers, and ABI manifests were not
  touched.
- The lock `commit` field equals the latest commit that touched
  `safe/` (`e98031dac96b1ee74e8a4b62165b12f11a12ec9c`), as the
  validator requires for `port` mode.
- The nested `validator/` checkout remains excluded from the parent
  repo via `.git/info/exclude` and is not added to git.
