# libwebp Validator Report — Phase 1 Baseline

## Run summary

- Mode: `port`
- Library: `libwebp`
- Validator URL: https://github.com/safelibs/validator
- Validator commit: `87b321fe728340d6fc6dd2f638583cca82c667c3`
- Safe source commit tested: `6c7ba877dfe4c4e0aaf5697af4f51cc3a906a9e8`
- Local release tag: `local-6c7ba877dfe4`
- Override deb root: `validator/artifacts/debs/local`
- Override leaf: `validator/artifacts/debs/local/libwebp/`
- Artifact root: `validator/artifacts/libwebp-safe/`
- Proof JSON: `validator/artifacts/libwebp-safe/proof/libwebp-safe-port-proof.json`
- Lock JSON: `validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json`
- Per-case results dir: `validator/artifacts/libwebp-safe/port/results/libwebp/`
- Per-case logs dir: `validator/artifacts/libwebp-safe/port/logs/libwebp/`
- Per-case casts dir: `validator/artifacts/libwebp-safe/port/casts/libwebp/`

The `Safe source commit tested` value above equals
`libraries[0].commit` in `local-port-debs-lock.json` and
`libraries[0].port_commit` in `libwebp-safe-port-proof.json`.

## Testcase totals

- Source cases: 5
- Usage cases: 171
- Total cases: 176
- Passed: 175
- Failed: 1
- Casts recorded: 176

Checks executed: validator unit suite (`make -C validator unit`, 110 tests),
testcase manifest check (`make -C validator check-testcases`),
libwebp-only manifest check
(`python3 validator/tools/testcases.py … --library libwebp --check`),
full port matrix run (`bash test.sh --mode port --library libwebp
--record-casts`), proof verification
(`python3 tools/verify_proof_artifacts.py …`).

Failures found: 1.

Waived testcase ids:

## Safe Debian packages (built via `cargo run -p xtask -- package-deb`)

All eight packages were produced from `safe/` at commit
`6c7ba877dfe4c4e0aaf5697af4f51cc3a906a9e8` using
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
| webp | webp_1.3.2-0.4build3+safelibs1_amd64.deb | amd64 | `af8f1dba3a6b59c27923e003acd0d5e7e0b183fd0580635248f345e312a1f45c` | 98178 |

Package version (Debian field): `1.3.2-0.4build3+safelibs1`.

## Commands executed

From `/home/yans/safelibs/pipeline/ports/port-libwebp/`:

```bash
# 1. Track validator under the parent repo's local exclude.
grep -qxF '/validator/' .git/info/exclude || printf '\n/validator/\n' >> .git/info/exclude

# 2. Clone validator (no prior checkout existed).
git clone https://github.com/safelibs/validator validator
git -C validator rev-parse HEAD
# -> 87b321fe728340d6fc6dd2f638583cca82c667c3

# 3. Validator tooling smoke tests.
make -C validator unit
make -C validator check-testcases

SOURCE_COUNT=$(find validator/tests/libwebp/tests/cases/source -maxdepth 1 -name '*.sh' | wc -l)  # 5
USAGE_COUNT=$(find validator/tests/libwebp/tests/cases/usage  -maxdepth 1 -name '*.sh' | wc -l)  # 171
TOTAL_COUNT=$((SOURCE_COUNT + USAGE_COUNT))                                                       # 176
python3 validator/tools/testcases.py \
  --config validator/repositories.yml \
  --tests-root validator/tests \
  --library libwebp \
  --check \
  --min-source-cases "$SOURCE_COUNT" \
  --min-usage-cases "$USAGE_COUNT" \
  --min-cases "$TOTAL_COUNT"

# 4. Build the eight safe Debian packages.
mkdir -p safe/dist
cargo run -p xtask --manifest-path safe/Cargo.toml -- package-deb --output-dir safe/dist

# 5. Stage the local override root and write the port-debs lock.
rm -rf validator/artifacts/debs/local/libwebp
mkdir -p validator/artifacts/debs/local/libwebp validator/artifacts/libwebp-safe/proof
find safe/dist -maxdepth 1 -type f -name '*.deb' \
  -exec cp -f -t validator/artifacts/debs/local/libwebp {} +
# (then run the inline python that wrote local-port-debs-lock.json)
```

From `/home/yans/safelibs/pipeline/ports/port-libwebp/validator/`:

```bash
# 6. Run the full libwebp port matrix and verify the proof.
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

## Failures

| Testcase id | Kind | Symptom (first failing line from log) | Triage bucket |
|---|---|---|---|
| `usage-vips-webpload-all-frames` | usage | `img2webp` from the safe `webp` package aborts with `PNG support not compiled. Please install the libpng development package before building.` while encoding three PNG frames into `anim.webp`; the testcase script then fails its `validator_require_file "$tmpdir/anim.webp"` check. | package/install — the safe-built `webp` binary tools were linked without libpng, so `img2webp` cannot read PNG inputs. The override `webp_1.3.2-0.4build3+safelibs1_amd64.deb` needs to gain a PNG-enabled `img2webp` (and presumably the other CLI tools that consume PNG) to match upstream behavior on a stock noble image. |

Result paths for the failing case:

- `validator/artifacts/libwebp-safe/port/results/libwebp/usage-vips-webpload-all-frames.json`
- `validator/artifacts/libwebp-safe/port/logs/libwebp/usage-vips-webpload-all-frames.log`
- `validator/artifacts/libwebp-safe/port/casts/libwebp/usage-vips-webpload-all-frames.cast`

All 175 other testcases passed. Per the phase 1 brief, baseline failures
are recorded for triage in later phases; no waivers are added at this
stage.

## Unsafe-block snapshot (from proof)

- `rs_loc`: 92739
- Total unsafe blocks: 194 (abi-shaped 133, voluntary 61, no-enclosing 0)
- Blocks with any flagged op: 12 (3 voluntary)
- Op buckets: `from_raw`=9, `static_mut`=4, `zeroed_uninit`=1

## Notes

- `safe/` was not modified during this phase; the package build and
  port matrix run exercise the existing tree at
  `6c7ba877dfe4c4e0aaf5697af4f51cc3a906a9e8`.
- The nested `validator/` checkout is excluded from the parent repo via
  `.git/info/exclude` and is not added to git.
