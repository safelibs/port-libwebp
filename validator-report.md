# libwebp Validator Report - Phase 1: ABI Baseline Refresh

## Run summary

Validator URL: https://github.com/safelibs/validator
Validator commit: 87b321fe728340d6fc6dd2f638583cca82c667c3
Safe source commit tested: 90e0c784097445cc9083ac5a8e1669b2702aea25
Inline `/home/yans/code/...` safe source commit tested: 6c7ba877dfe4c4e0aaf5697af4f51cc3a906a9e8
Final verification date: 2026-05-05 (America/Phoenix).

## Validator setup

`/home/yans/safelibs/pipeline/ports/port-libwebp/validator` existed and
`git -C validator pull --ff-only` reported `Already up to date.` The checkout
was clean before use. I read `validator/README.md`; for later full libwebp
port validation after package artifacts are refreshed, the selected invocation
is the README's selected-library/local-override flow:

```bash
cd /home/yans/safelibs/pipeline/ports/port-libwebp/validator
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
  --min-source-cases "$SOURCE_COUNT" \
  --min-usage-cases "$USAGE_COUNT" \
  --min-cases "$TOTAL_COUNT" \
  --ports-root /home/yans/safelibs/pipeline/ports
```

This phase did not edit validator tests, shared scripts, manifests, or tools.
The full libwebp validator matrix was not rerun in Phase 1 because this phase's
validator responsibility is setup, README review, and invocation selection; the
full matrix is required after package artifacts are available and again in the
final hardening phase.

## Commands and checks executed

From `/home/yans/safelibs/pipeline/ports/port-libwebp/`:

```bash
git -C validator pull --ff-only
git -C validator rev-parse HEAD
git -C validator remote get-url origin
cargo fmt --manifest-path safe/Cargo.toml --all --check
cargo build -p webp-core --manifest-path safe/Cargo.toml \
  --no-default-features --features decode,encode
cargo check --workspace --manifest-path safe/Cargo.toml
cargo run -p xtask --manifest-path safe/Cargo.toml -- \
  capture-baseline --original-dir original --out-dir safe/abi/original
diff -u original/src/webp/decode.h safe/include/webp/decode.h
diff -u original/src/webp/demux.h safe/include/webp/demux.h
diff -u original/src/webp/encode.h safe/include/webp/encode.h
diff -u original/src/webp/mux.h safe/include/webp/mux.h
diff -u original/src/webp/mux_types.h safe/include/webp/mux_types.h
diff -u original/src/webp/types.h safe/include/webp/types.h
diff -u original/sharpyuv/sharpyuv.h safe/include/webp/sharpyuv/sharpyuv.h
diff -u original/sharpyuv/sharpyuv_csp.h \
  safe/include/webp/sharpyuv/sharpyuv_csp.h
cargo run -p xtask --manifest-path safe/Cargo.toml -- \
  verify-baseline-manifests --baseline-dir safe/abi/original
git diff --exit-code -- safe/abi/original
```

The same `cargo fmt`, `cargo build -p webp-core --no-default-features
--features decode,encode`, `cargo check`, `capture-baseline`, public header
diffs, `verify-baseline-manifests`, and
`git diff --exit-code -- safe/abi/original` checks were also run against the
inline phase source paths under `/home/yans/code/safelibs/ported/libwebp/`.

## Failures found

The previous implementation attempt failed verification because it committed an
out-of-scope change to `safe/crates/libwebp/build.rs`. That source change has
been reverted in both workspace copies. No ABI, SONAME, NEEDED,
install-surface, relink-manifest, pkg-config, CMake, Debian metadata, manpage,
or copied-header drift was found. Recapturing the baseline left
`safe/abi/original/` unchanged in both workspace copies.

## Fixes applied

The out-of-scope `safe/crates/libwebp/build.rs` change was removed. No
phase-scoped safe source, ABI, header, package metadata, pkg-config, CMake,
Debian, or manpage file required a content update.

No validator testcase failure required a safe-side regression test in this
phase. No validator waivers were added.

Waived testcase ids:

## Final status

Phase 1 is clean. The net implementation diff from the pre-phase base is
limited to this report update; `safe/crates/libwebp/build.rs` has no net diff.
The checked-in ABI/install-surface manifests and copied public headers remain
authoritative and match the current upstream snapshot. The validator checkout is
present, up to date, and recorded at
`87b321fe728340d6fc6dd2f638583cca82c667c3`; the selected full libwebp
validator invocation for later phases is recorded above.

---

# libwebp Validator Report — Final (Phase 5: Final Validator Hardening And Report Closure)

## Run summary

Validator commit: 87b321fe728340d6fc6dd2f638583cca82c667c3
Safe source commit tested: 90e0c784097445cc9083ac5a8e1669b2702aea25
Checks executed: cargo run -p xtask -- unsafe-audit; cargo run -p xtask -- package-deb (eight Debian binary packages); regenerate validator/artifacts/debs/local/libwebp/ from safe/dist; regenerate validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json from on-disk .deb hashes/sizes; bash validator/test.sh --mode port --override-deb-root artifacts/debs/local --port-deb-lock artifacts/libwebp-safe/proof/local-port-debs-lock.json --library libwebp --record-casts; python3 validator/tools/verify_proof_artifacts.py --mode port --library libwebp --require-casts --min-source-cases 5 --min-usage-cases 171 --min-cases 176; per-result invariant scan over validator/artifacts/libwebp-safe/port/results/libwebp/*.json (status, override_debs_installed, port_commit, override_installed_packages).
Failures found: 0 (no source/CLI failures, no usage/runtime failures, no package/install failures).
Waived testcase ids:

Final status: clean — full libwebp validator matrix passes with zero unwaived failures and Waived testcase ids is empty, so summary["failed"] == 0 satisfies the final closure rule.
Final verification date: 2026-05-03 (UTC-7).

Mode: port
Library: libwebp
Validator URL: https://github.com/safelibs/validator
Local release tag: local-90e0c7840974
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
Per-case logs recorded: 177 (176 testcase logs + 1 dependent-package install log produced by `validator/tests/_shared/install_override_debs.sh`).

The live validator inventory at commit
`87b321fe728340d6fc6dd2f638583cca82c667c3` contains
`SOURCE_COUNT=5` (`find validator/tests/libwebp/tests/cases/source
-maxdepth 1 -name '*.sh' | wc -l`), `USAGE_COUNT=171` (`find
validator/tests/libwebp/tests/cases/usage -maxdepth 1 -name '*.sh' |
wc -l`), and `TOTAL_COUNT=176`. These are the exact `--min-*-cases`
thresholds passed to `verify_proof_artifacts.py`, and they match
`source_cases`, `usage_cases`, and `cases` in
`validator/artifacts/libwebp-safe/port/results/libwebp/summary.json`,
which reads:

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

`summary["cases"] == summary["source_cases"] + summary["usage_cases"]`
holds (176 == 5 + 171). `summary["failed"] == 0` and
`Waived testcase ids:` is empty, so the final-verifier closure rule
"if `Waived testcase ids:` is empty, `summary["failed"] == 0`" is
satisfied without any waiver.

## Per-case override install status

Every per-case result JSON under
`validator/artifacts/libwebp-safe/port/results/libwebp/` reports
`override_debs_installed == true` and lists the eight safe override
packages in `override_installed_packages[]`
(`{libwebp7, libwebpdemux2, libwebpmux3, libwebpdecoder3,
libsharpyuv0, libwebp-dev, libsharpyuv-dev, webp}`). Every per-case
result JSON also reports `port_commit ==
90e0c784097445cc9083ac5a8e1669b2702aea25`, confirming each Pillow,
FFmpeg/FFprobe, libvips, SDL2_image, and webp-pixbuf-loader case is
exercising the safe libwebp/libsharpyuv builds rather than the stock
Ubuntu noble packages.

## Safe Debian packages (built via `cargo run -p xtask -- package-deb`)

All eight packages were produced from `safe/` at commit
`90e0c784097445cc9083ac5a8e1669b2702aea25` using
`cargo run -p xtask --manifest-path safe/Cargo.toml -- package-deb
--output-dir safe/dist`, then copied into
`validator/artifacts/debs/local/libwebp/`. Package source version is
`1.3.2-0.4build3+safelibs1`.

| Package | Filename | Version | Architecture | SHA-256 | Size (bytes) |
|---|---|---|---|---|---|
| libwebp7 | libwebp7_1.3.2-0.4build3+safelibs1_amd64.deb | 1.3.2-0.4build3+safelibs1 | amd64 | `fe95290b0d8fece7d9daa6e105d7bc93fcbf4f502b0286d4ffb4c26a570dff93` | 271926 |
| libwebpdemux2 | libwebpdemux2_1.3.2-0.4build3+safelibs1_amd64.deb | 1.3.2-0.4build3+safelibs1 | amd64 | `ee831697509f1594e7ff489d3271dbf8ba2180db7799760ef738d3aa3ec5b571` | 139234 |
| libwebpmux3 | libwebpmux3_1.3.2-0.4build3+safelibs1_amd64.deb | 1.3.2-0.4build3+safelibs1 | amd64 | `d7e0d3cd735ae2b3688cc6ad5a6f4537d15d8bcb97e3e6b145416f33308cea6b` | 292306 |
| libwebpdecoder3 | libwebpdecoder3_1.3.2-0.4build3+safelibs1_amd64.deb | 1.3.2-0.4build3+safelibs1 | amd64 | `3050ba41234d5eb92e85236d98498edbee544ceb6c7011f0f6c306e525a1a35d` | 131772 |
| libsharpyuv0 | libsharpyuv0_1.3.2-0.4build3+safelibs1_amd64.deb | 1.3.2-0.4build3+safelibs1 | amd64 | `0d7997f9c888e3bc69d8b9e1b64e52e0ae0c4a676a6cc4f8ed65d4d3148e5e61` | 91776 |
| libwebp-dev | libwebp-dev_1.3.2-0.4build3+safelibs1_amd64.deb | 1.3.2-0.4build3+safelibs1 | amd64 | `13475fbb7edf0a2766f3f13681de83c1ee7e64ad22ee38ccc5fda27ba16afc9c` | 444842 |
| libsharpyuv-dev | libsharpyuv-dev_1.3.2-0.4build3+safelibs1_amd64.deb | 1.3.2-0.4build3+safelibs1 | amd64 | `db118f662844c623f4d7ca9ffa15e7161564a5e2e152e46fc3926aa0b05c7b19` | 94194 |
| webp | webp_1.3.2-0.4build3+safelibs1_amd64.deb | 1.3.2-0.4build3+safelibs1 | amd64 | `11bf556e7d1ae98928823ab830660b76dc9179c4a9f1d8f99bfec1a0c883f30d` | 111160 |

These eight `.deb` files exist on disk under
`validator/artifacts/debs/local/libwebp/` and the SHA-256 and size
columns above are exactly the values recorded in
`validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json`
under `libraries[0].debs[]`. `libraries[0].commit` and
`libraries[0].port_commit` (in the proof) both equal
`90e0c784097445cc9083ac5a8e1669b2702aea25`, which equals the output
of `git log -1 --format=%H -- safe`.

## Commands executed

From `/home/yans/safelibs/pipeline/ports/port-libwebp/`:

```bash
# Phase 5 safe-side refresh (only safe/ change in this phase: regenerated
# unsafe-audit.md so its header path matches the current working directory).
cargo run -p xtask --manifest-path safe/Cargo.toml -- unsafe-audit
git add safe/docs/unsafe-audit.md
git commit -m 'impl_final_validator_clean_run: refresh unsafe-audit path header'
# -> commit 90e0c784097445cc9083ac5a8e1669b2702aea25 (this is the
#    Safe source commit tested in this report).

# Rebuild the eight safe Debian binary packages from the new safe HEAD.
mkdir -p safe/dist
cargo run -p xtask --manifest-path safe/Cargo.toml -- package-deb \
  --output-dir safe/dist

# Refresh the validator local override root and port lock.
rm -rf validator/artifacts/debs/local/libwebp
mkdir -p validator/artifacts/debs/local/libwebp \
         validator/artifacts/libwebp-safe/proof
find safe/dist -maxdepth 1 -type f -name '*.deb' \
  -exec cp -f -t validator/artifacts/debs/local/libwebp {} +
python3 - <<'PY'
# Recomputes lock from on-disk .deb hashes/sizes and the latest commit
# that touched safe/. This is the same generator as in the workflow plan.
import hashlib, json, subprocess
from pathlib import Path
packages = ["libwebp7", "libwebpdemux2", "libwebpmux3", "libwebpdecoder3",
            "libsharpyuv0", "libwebp-dev", "libsharpyuv-dev", "webp"]
leaf = Path("validator/artifacts/debs/local/libwebp")
debs = []
for package in packages:
    path = sorted(leaf.glob(f"{package}_*.deb"))[0]
    arch = subprocess.check_output(
        ["dpkg-deb", "-f", str(path), "Architecture"], text=True).strip()
    data = path.read_bytes()
    debs.append({"package": package, "filename": path.name,
                 "architecture": arch,
                 "sha256": hashlib.sha256(data).hexdigest(),
                 "size": len(data),
                 "asset_api_url": None, "browser_download_url": None})
commit = subprocess.check_output(
    ["git", "log", "-1", "--format=%H", "--", "safe"], text=True).strip()
release = f"local-{commit[:12]}"
lock = {"schema_version": 1, "mode": "port",
        "generated_at": "1970-01-01T00:00:00Z",
        "source_config": "repositories.yml",
        "source_inventory": "local-overrides",
        "libraries": [{"library": "libwebp",
                       "repository": "safelibs/port-libwebp",
                       "url": "https://github.com/safelibs/port-libwebp",
                       "tag_ref": f"refs/tags/{release}",
                       "commit": commit, "release_tag": release,
                       "debs": debs,
                       "unported_original_packages": []}]}
Path("validator/artifacts/libwebp-safe/proof/local-port-debs-lock.json"
     ).write_text(json.dumps(lock, indent=2, sort_keys=True) + "\n")
PY
```

From `/home/yans/safelibs/pipeline/ports/port-libwebp/validator/`:

```bash
# Final libwebp validator port matrix run plus proof generation.
SOURCE_COUNT=5
USAGE_COUNT=171
TOTAL_COUNT=176
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
  --min-source-cases "$SOURCE_COUNT" \
  --min-usage-cases "$USAGE_COUNT" \
  --min-cases "$TOTAL_COUNT" \
  --ports-root /home/yans/safelibs/pipeline/ports
```

## Failures and disposition

No validator testcases failed.

- All 5 source/CLI cases pass: `cwebp-dwebp-roundtrip`,
  `decode-c-api-smoke`, `malformed-webp-rejection`,
  `webpinfo-inspection`, `webpmux-metadata`. Every per-case JSON has
  `status == "passed"` and `exit_code == 0`. No source/CLI fixes are
  required in this phase.
- All 171 usage cases pass across `usage-ffmpeg-*` (41),
  `usage-ffprobe-*` (7), `usage-libsdl2-image-*` (7),
  `usage-python3-pil-*` (90), `usage-vips-*` (17), and
  `usage-webp-pixbuf-loader-*` (9). Every per-case JSON has
  `status == "passed"`. No usage/runtime fixes are required in this
  phase.
- All 176 per-case JSON files report `override_debs_installed == true`
  and the eight safe override packages in
  `override_installed_packages[]`. The
  `dpkg --install --force-downgrade` step in
  `validator/tests/_shared/install_override_debs.sh` unpacks all
  eight local override packages over the stock noble apt versions
  before each testcase runs. No package/install fixes are required in
  this phase.

There is therefore no triage list and no fix list for phase 5; the
catch-all bucket is empty by construction.

## Safe fixes applied in phase 5

Only one tracked change was needed for the final closure:

- `safe/docs/unsafe-audit.md` (commit
  `90e0c784097445cc9083ac5a8e1669b2702aea25`,
  `impl_final_validator_clean_run: refresh unsafe-audit path header`):
  regenerated by `cargo run -p xtask --manifest-path safe/Cargo.toml --
  unsafe-audit` so the recorded generator path matches the current
  working directory (`/home/yans/safelibs/pipeline/ports/port-libwebp`)
  rather than the stale `/home/yans/code/safelibs/ported/libwebp` path
  baked into a prior commit. The audit inventory body is unchanged
  (still `194` total unsafe blocks; `133` ABI-shaped, `61` voluntary,
  `0` no-enclosing; `12` blocks with any flagged op of which `3` are
  voluntary; op buckets `from_raw=9`, `static_mut=4`,
  `zeroed_uninit=1`; `92749` Rust LOC). No source, ABI, packaging,
  test, or include change accompanies this refresh.

No other `safe/` source/test/package/script change was required;
phases 2 (`e98031d ci: enable PNG/JPEG/TIFF in webp tools`) and the
prior translation/ABI/packaging phases already produced an all-pass
matrix.

## Validator-bug waivers

None. `Waived testcase ids:` (the machine-readable line above) is
empty. No original-package validator matrix re-run was required in
phase 5 because no failure exists to adjudicate against the original
packages.

## Per-phase status (cumulative)

- Phase 1 (`impl_validator_baseline`): clean baseline. 5/5 source
  cases, 171/171 usage cases, 176/176 total passed against the safe
  override packages built from
  `e98031dac96b1ee74e8a4b62165b12f11a12ec9c`. Validator commit
  `87b321fe728340d6fc6dd2f638583cca82c667c3`.
- Phase 2 (`impl_package_provenance_waiver_gate`): no package,
  install, symbol, dependency, or validator-bug waiver work required;
  every per-case JSON already had `override_debs_installed == true`.
  Empty phase commit.
- Phase 3 (`impl_source_cli_failures`): no source/CLI failures; no
  fix or regression added. Empty phase commit.
- Phase 4 (`impl_usage_runtime_failures`): no usage/runtime failures;
  no fix or regression added. Empty phase commit.
- Phase 5 (`impl_final_validator_clean_run`): one tracked safe/
  refresh (regenerated `safe/docs/unsafe-audit.md` to match the local
  generator path), then full safe Debian package rebuild, validator
  override refresh, port lock regeneration, full libwebp validator
  port matrix re-run, and proof regeneration. All 176 cases pass
  against safe commit
  `90e0c784097445cc9083ac5a8e1669b2702aea25`.

## Unsafe-block snapshot (from proof)

- `rs_loc`: 92749
- Total unsafe blocks: 194 (abi-shaped 133, voluntary 61, no-enclosing 0)
- Blocks with any flagged op: 12 (3 voluntary)
- Op buckets: `from_raw=9`, `static_mut=4`, `zeroed_uninit=1`

These figures come from
`validator/artifacts/libwebp-safe/proof/libwebp-safe-port-proof.json`
`unsafe_blocks` and `libraries[0].unsafe_blocks` (identical because
this is a single-library run).

## Notes

- The nested `validator/` checkout remains excluded from the parent
  repo via `.git/info/exclude` and is not added to git. `git -C
  validator status --short` is clean and `git -C validator rev-parse
  HEAD` is `87b321fe728340d6fc6dd2f638583cca82c667c3`. No validator
  test, shared script, `repositories.yml`, `test.sh`, or `tools/`
  file was modified to make any failure pass; no failures existed to
  begin with.
- The `.deb` files in the parent worktree's top level (e.g.
  `libwebp7_1.3.2-0.4build3+safelibs1_amd64.deb`) are CI/local-build
  byproducts and are not tracked by git. The validator override
  packages live under `validator/artifacts/debs/local/libwebp/` and
  are the only copies consumed by `bash test.sh`.
- The untracked `original/__pycache__/` and `original/swig/__pycache__/`
  directories are preexisting Python bytecode caches; per the plan
  they are intentionally not deleted or committed.
