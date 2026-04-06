# Context

`/home/yans/code/safelibs/ported/libwebp/original` is upstream `libwebp` `v1.3.2` with:

- CMake and autotools build graphs in `original/CMakeLists.txt`, `original/src/*/Makefile.am`, and `original/sharpyuv/Makefile.am`
- Debian packaging metadata in `original/debian/`
- command-line tools in `original/examples/`, helper code in `original/imageio/` and `original/extras/`, and manpages in `original/man/`
- upstream regression and fuzz tests in `original/tests/public_api_test.c` and `original/tests/fuzzer/`
- a dependent-runtime Docker harness in `test-original.sh`
- compatibility/security inputs in `dependents.json` and `relevant_cves.json`

There is no Rust implementation yet; `safe/` does not exist and must become a standard Rust workspace that emits a drop-in Ubuntu 24.04 replacement for the upstream package family.

The compatibility target is broader than the public header set:

- Installed public headers come from `original/src/webp/{decode,demux,encode,mux,mux_types,types}.h` and `original/sharpyuv/{sharpyuv,sharpyuv_csp}.h`
- Shared-library SONAMEs come from libtool `-version-info` in:
  - `original/src/Makefile.am`
  - `original/src/demux/Makefile.am`
  - `original/src/mux/Makefile.am`
  - `original/sharpyuv/Makefile.am`
- Debian package splits come from `original/debian/control` and `original/debian/*.install`
- Installed tools are the C example programs in `original/examples/` plus animation utilities enabled by `original/CMakeLists.txt`

The upstream package family and install surface that the Rust port must preserve is:

- Runtime libraries:
  - `libwebp.so.7` from `8:8:1`
  - `libwebpdecoder.so.3` from `4:8:1`
  - `libwebpdemux.so.2` from `2:14:0`
  - `libwebpmux.so.3` from `3:13:0`
  - `libsharpyuv.so.0` from `0:1:0`
- Debian binary packages:
  - `libwebp7`
  - `libwebpdecoder3`
  - `libwebpdemux2`
  - `libwebpmux3`
  - `libsharpyuv0`
  - `libwebp-dev`
  - `libsharpyuv-dev`
  - `webp`
- Installed tools expected in the `webp` package on Ubuntu 24.04:
  - `cwebp`
  - `dwebp`
  - `gif2webp`
  - `img2webp`
  - `vwebp`
  - `webpinfo`
  - `webpmux`
  - `anim_diff`
  - `anim_dump`
- Installed manpages expected in the `webp` package:
  - `cwebp.1`
  - `dwebp.1`
  - `gif2webp.1`
  - `img2webp.1`
  - `vwebp.1`
  - `webpinfo.1`
  - `webpmux.1`

A temporary shared-library build of `original/` confirms the exported ABI that later checkers must treat as authoritative:

- `libsharpyuv.so.0.0.1`: 5 dynamic exports
- `libwebpdecoder.so.3.1.8`: 45 dynamic exports
- `libwebp.so.7.1.8`: 85 dynamic exports
- `libwebpdemux.so.2.0.14`: 20 dynamic exports
- `libwebpmux.so.3.0.13`: 24 dynamic exports

Notable ABI details that are not fully described by installed headers:

- `libwebp` and `libwebpdecoder` both export the global object symbol `VP8GetCPUInfo`, declared in `original/src/dsp/cpu.c:176-258`
- `libwebp` and `libwebpdecoder` export non-header helper symbols such as `VP8CheckSignature`, `VP8GetInfo`, `VP8LCheckSignature`, and `VP8LGetInfo`
- `libwebp` and `libwebpdecoder` export worker hook APIs backed by `original/src/utils/thread_utils.c:350-365`
- `libwebp` depends on `libsharpyuv`; `libwebpdemux` and `libwebpmux` depend on `libwebp`; `libwebpmux` and `libsharpyuv` both retain a `libm` dependency

The linker dependency graph is part of the compatibility contract and must be
preserved exactly, not inferred from higher-level smoke tests:

- `libsharpyuv.so.0` must keep `DT_NEEDED` entries for `libm.so.6` and `libc.so.6`
- `libwebpdecoder.so.3` must keep `DT_NEEDED` entries only for `libc.so.6`
- `libwebp.so.7` must keep `DT_NEEDED` entries for `libsharpyuv.so.0`, `libm.so.6`, and `libc.so.6`
- `libwebpdemux.so.2` must keep `DT_NEEDED` entries for `libwebp.so.7` and `libc.so.6`
- `libwebpmux.so.3` must keep `DT_NEEDED` entries for `libwebp.so.7`, `libm.so.6`, and `libc.so.6`

These edges matter because existing binaries and downstream shared objects bind
them at load time; an accidental extra edge such as `libwebpdecoder.so.3 ->
libwebp.so.7` is a drop-in compatibility regression even if API tests still
pass.

The highest-risk implementation areas are:

- checked allocation and overflow guards in `original/src/utils/utils.c:159-210` and `original/src/utils/utils.h:30-56`
- thread-worker override hooks in `original/src/utils/thread_utils.c:263-365` and `original/src/utils/thread_utils.h`
- CPU dispatch and `VP8GetCPUInfo` global state in `original/src/dsp/cpu.c:176-258` and `original/src/dsp/cpu.h`
- incremental decode and decode front-end state in `original/src/dec/webp_dec.c` and `original/src/dec/idec_dec.c`
- lossless Huffman allocation logic in `original/src/dec/vp8l_dec.c:360-520` and `original/src/utils/huffman_utils.c:217-292`
- animation decode and encode state machines in `original/src/demux/anim_decode.c` and `original/src/mux/anim_encode.c`
- picture/view ownership rules in `original/src/enc/picture_enc.c`, `original/src/enc/picture_rescale_enc.c`, and `original/src/enc/webp_enc.c`

The non-memory-safety CVE requirements from `relevant_cves.json` are:

- `CVE-2016-9085`: checked arithmetic for width, height, stride, bytes-per-pixel, and animation frame sizing
- `CVE-2020-36332`: bound and validate Huffman-table allocation derived from malformed lossless bitstreams

Existing artifacts that the plan must reuse instead of rediscovering:

- `original/` as the upstream source snapshot
- `original/tests/public_api_test.c` and `original/tests/fuzzer/`
- `original/examples/`, `original/imageio/`, `original/extras/`, and `original/man/`
- `original/debian/`, `original/src/*.pc.in`, `original/src/demux/libwebpdemux.pc.in`, `original/src/mux/libwebpmux.pc.in`, and `original/cmake/WebPConfig.cmake.in`
- `test-original.sh`
- `dependents.json`
- `relevant_cves.json`

Scratch outputs already present in the working tree, such as `build-check/` and the untracked `.o` and `.a` files under `original/`, should be treated only as incidental local build byproducts. The generated workflow should not depend on them being present in a clean checkout; phase 1 must derive checked-in baselines from `original/` and then use those checked-in baselines afterward.

Because Cargo cannot directly emit five independently versioned shared libraries from one crate, the Rust port should be a workspace rooted at `safe/Cargo.toml` with:

- `crates/webp-abi` for `repr(C)` types, constants, and layout tests
- `crates/webp-core` for shared codec/container/runtime logic
- five export crates:
  - `crates/libwebp`
  - `crates/libwebpdecoder`
  - `crates/libwebpdemux`
  - `crates/libwebpmux`
  - `crates/libsharpyuv`
- `xtask/` for baseline capture, symbol/SONAME/`DT_NEEDED` verification, upstream C-test and tool compilation, relink checks, Debian packaging, and install-tree validation

The safest implementation order is to mirror the upstream source partition first, prove ABI parity continuously, then reduce `unsafe` only after compatibility is locked down.

# Generated Workflow Contract

The generated workflow that later planners derive from this document must obey all of the following rules:

1. Execution must be strictly linear. Do not create `parallel_groups`.
2. The workflow YAML must be fully self-contained and inline-only.
3. Do not use top-level `include`.
4. Do not use phase-level `prompt_file`, `workflow_file`, `workflow_dir`, `checks`, or any other YAML indirection.
5. Every verifier must be an explicit top-level `check` phase with its own `phase_id`.
6. Every verifier must remain in the implement block it verifies and must use exactly one fixed `bounce_target`, pointing to that implement phase.
7. Do not use `bounce_targets` lists, dynamic routing, or agent-selected bounce behavior.
8. Any test, lint, build, packaging, Docker, or analysis command a verifier must run has to be written directly into that verifier’s instructions. Do not model those commands as separate non-agentic phases.
9. Every implement prompt in the generated workflow must instruct the agent to commit its work to git before yielding.
10. Consume existing checked-in artifacts in place:
    - `original/` is the source-of-truth upstream tree.
    - `dependents.json` is the source-of-truth dependent inventory.
    - `relevant_cves.json` is the source-of-truth non-memory CVE dataset.
    - `test-original.sh` is the source-of-truth dependent/runtime harness and must be modified in place.
    - `original/tests/public_api_test.c` and `original/tests/fuzzer/` are the source-of-truth upstream tests and must be compiled or wired in, not rewritten from scratch.
    - `original/examples/`, `original/imageio/`, `original/extras/`, and `original/man/` are the source-of-truth tool and helper sources and must be reused rather than reimplemented in Rust.
    - `original/debian/`, upstream `.pc.in` files, and `original/cmake/WebPConfig.cmake.in` are the packaging/install baselines and must be reused.
11. Do not rely on `build-check/` or any current untracked `.o`, `.a`, or generated binaries as workflow inputs; they are non-canonical scratch artifacts.
12. If a derived artifact is needed repeatedly, create it once in phase 1 and check it in under `safe/abi/original/` or another explicit `safe/` path. Later phases must consume that derived artifact rather than rebuilding it ad hoc.
13. The derived baseline artifacts that later phases must reuse are:
    - symbol export lists per shared library
    - SONAME and `DT_NEEDED` manifests
    - install-surface manifest listing expected packages, headers, pkg-config files, CMake files, binaries, and manpages
    - relink fixture manifest for original object files and their link lines
14. Compatibility verifiers must compare Rust outputs against the checked-in baselines from phase 1, not against informal expectations.
15. Any phase that claims a shared library is ABI-complete must run an explicit `verify-needed` check that consumes `safe/abi/original/needed.json`; application smoke tests, package-content checks, and original-object relink tests do not substitute for exact `DT_NEEDED` comparison.
16. The package/tooling contract must stay explicit: later workflow prompts must name every required binary in the `webp` package and must not collapse that requirement into a wildcard like “preserve `usr/bin/*`”.

# Implementation Phases

## 1. ABI, Header, and Install-Surface Baseline

- Phase Name: ABI, Header, and Install-Surface Baseline
- Implement Phase ID: `impl-abi-baseline`
- Verification Phases:
  - `check-abi-baseline`
    - type: `check`
    - fixed `bounce_target`: `impl-abi-baseline`
    - purpose: confirm the Rust workspace skeleton exists, only the installed public headers were copied, packaging metadata baselines were imported, and authoritative ABI/install manifests were captured from `original/`.
    - commands it should run:
      - `cargo check --workspace --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- capture-baseline --original-dir /home/yans/code/safelibs/ported/libwebp/original --out-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original`
      - `diff -u /home/yans/code/safelibs/ported/libwebp/original/src/webp/decode.h /home/yans/code/safelibs/ported/libwebp/safe/include/webp/decode.h`
      - `diff -u /home/yans/code/safelibs/ported/libwebp/original/src/webp/demux.h /home/yans/code/safelibs/ported/libwebp/safe/include/webp/demux.h`
      - `diff -u /home/yans/code/safelibs/ported/libwebp/original/src/webp/encode.h /home/yans/code/safelibs/ported/libwebp/safe/include/webp/encode.h`
      - `diff -u /home/yans/code/safelibs/ported/libwebp/original/src/webp/mux.h /home/yans/code/safelibs/ported/libwebp/safe/include/webp/mux.h`
      - `diff -u /home/yans/code/safelibs/ported/libwebp/original/src/webp/mux_types.h /home/yans/code/safelibs/ported/libwebp/safe/include/webp/mux_types.h`
      - `diff -u /home/yans/code/safelibs/ported/libwebp/original/src/webp/types.h /home/yans/code/safelibs/ported/libwebp/safe/include/webp/types.h`
      - `diff -u /home/yans/code/safelibs/ported/libwebp/original/sharpyuv/sharpyuv.h /home/yans/code/safelibs/ported/libwebp/safe/include/webp/sharpyuv/sharpyuv.h`
      - `diff -u /home/yans/code/safelibs/ported/libwebp/original/sharpyuv/sharpyuv_csp.h /home/yans/code/safelibs/ported/libwebp/safe/include/webp/sharpyuv/sharpyuv_csp.h`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-baseline-manifests --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original`
- Preexisting Inputs:
  - `original/CMakeLists.txt`
  - `original/src/Makefile.am`
  - `original/src/demux/Makefile.am`
  - `original/src/mux/Makefile.am`
  - `original/sharpyuv/Makefile.am`
  - `original/src/webp/{decode,demux,encode,mux,mux_types,types}.h`
  - `original/sharpyuv/{sharpyuv,sharpyuv_csp}.h`
  - `original/src/*.pc.in`
  - `original/src/demux/libwebpdemux.pc.in`
  - `original/src/mux/libwebpmux.pc.in`
  - `original/sharpyuv/libsharpyuv.pc.in`
  - `original/cmake/WebPConfig.cmake.in`
  - `original/debian/*`
  - `original/man/*.1`
  - `original/examples/`
  - `original/imageio/`
  - `original/extras/`
  - `original/tests/public_api_test.c`
  - `test-original.sh`
  - `dependents.json`
  - `relevant_cves.json`
- New Outputs:
  - `safe/Cargo.toml`
  - `safe/rust-toolchain.toml`
  - `safe/crates/*/Cargo.toml`
  - `safe/xtask/`
  - `safe/include/webp/{decode,demux,encode,mux,mux_types,types}.h`
  - `safe/include/webp/sharpyuv/{sharpyuv,sharpyuv_csp}.h`
  - `safe/pkgconfig/{libwebp,libwebpdecoder,libwebpdemux,libwebpmux,libsharpyuv}.pc.in`
  - `safe/cmake/WebPConfig.cmake.in`
  - `safe/debian/*`
  - `safe/man/{cwebp,dwebp,gif2webp,img2webp,vwebp,webpinfo,webpmux}.1`
  - `safe/abi/original/{libsharpyuv,libwebp,libwebpdecoder,libwebpdemux,libwebpmux}.exports`
  - `safe/abi/original/sonames.json`
  - `safe/abi/original/needed.json`
  - `safe/abi/original/install-surface.json`
  - `safe/abi/original/relink-manifest.json`
- File Changes:
  - Create `safe/Cargo.toml` as a virtual workspace manifest
  - Create `safe/rust-toolchain.toml`
  - Create `safe/xtask/Cargo.toml` and `safe/xtask/src/{main.rs,baseline.rs,verify.rs,link.rs,tools.rs,package.rs}`
  - Create crate manifests for `safe/crates/webp-abi`, `safe/crates/webp-core`, `safe/crates/libwebp`, `safe/crates/libwebpdecoder`, `safe/crates/libwebpdemux`, `safe/crates/libwebpmux`, and `safe/crates/libsharpyuv`
  - Copy only the installed public headers listed above into `safe/include/`
  - Copy upstream `.pc.in`, CMake config template, Debian metadata, and manpages into `safe/`
  - Add baseline files under `safe/abi/original/`
- Implementation Details:
  - Make `safe/Cargo.toml` a virtual workspace with shared edition, lint, and release-profile settings.
  - Keep the public installed headers as literal copies of upstream. Do not generate headers from Rust types and do not copy non-installed headers such as `config.h.in` or `format_constants.h` into the install tree.
  - Implement `xtask capture-baseline` so it:
    - builds `original/` as shared libraries in a temp directory
    - records dynamic exports into `*.exports`
    - records SONAMEs and `DT_NEEDED` edges into `sonames.json` and `needed.json`
    - compiles `original/tests/public_api_test.c` and the original example objects that are available from the upstream build (`cwebp`, `dwebp`, `img2webp`, `webpinfo`, `webpmux`) and stores their link lines in `relink-manifest.json`
    - parses `original/debian/control`, `original/debian/*.install`, `original/debian/webp.manpages`, `original/man/Makefile.am`, and `original/CMakeLists.txt` to create `install-surface.json` with explicit package names, headers, pkg-config files, CMake files, binaries, and manpages
  - Record the explicit `webp` tool list in `install-surface.json` as:
    - `cwebp`, `dwebp`, `gif2webp`, `img2webp`, `vwebp`, `webpinfo`, `webpmux`, `anim_diff`, `anim_dump`
  - Record the explicit manpage list in `install-surface.json` as:
    - `cwebp.1`, `dwebp.1`, `gif2webp.1`, `img2webp.1`, `vwebp.1`, `webpinfo.1`, `webpmux.1`
  - Implement `xtask verify-needed` so later phases compare built Rust libraries against `safe/abi/original/needed.json` exactly, including the absence of extra dependency edges.
  - Make later phases consume these checked-in baselines instead of rebuilding them.
- Verification:
  - Commands listed above

## 2. Public ABI Types, Layout Tests, Common Runtime, and `libsharpyuv`

- Phase Name: Public ABI Types, Layout Tests, Common Runtime, and `libsharpyuv`
- Implement Phase ID: `impl-abi-runtime-sharpyuv`
- Verification Phases:
  - `check-abi-runtime-sharpyuv`
    - type: `check`
    - fixed `bounce_target`: `impl-abi-runtime-sharpyuv`
    - purpose: verify `repr(C)` layouts, shared runtime hooks, the exported `VP8GetCPUInfo` object symbol, and full `libsharpyuv` parity.
    - commands it should run:
      - `cargo test --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p webp-abi --test ffi_layout`
      - `cargo build --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p libsharpyuv -p libwebp -p libwebpdecoder --release`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-symbols --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libsharpyuv`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-symbol-subset --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libwebp libwebpdecoder --subset common_runtime`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-sonames --libs libsharpyuv`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-needed --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libsharpyuv`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- c-smoke --name sharpyuv_runtime`
- Preexisting Inputs:
  - `safe/abi/original/*`
  - `safe/include/webp/*`
  - `safe/include/webp/sharpyuv/*`
  - `relevant_cves.json`
  - `original/src/utils/utils.h`
  - `original/src/utils/thread_utils.h`
  - `original/src/dsp/cpu.h`
  - `original/sharpyuv/*`
- New Outputs:
  - `safe/crates/webp-abi/src/{lib.rs,types.rs,decode.rs,encode.rs,demux.rs,mux.rs,sharpyuv.rs}`
  - `safe/crates/webp-abi/build.rs`
  - `safe/crates/webp-abi/tests/ffi_layout.rs`
  - `safe/crates/webp-core/src/{lib.rs,alloc.rs,checked.rs,threading.rs,cpu.rs}`
  - `safe/crates/webp-core/src/sharpyuv/{mod.rs,csp.rs,dsp.rs,gamma.rs,convert.rs}`
  - `safe/crates/libsharpyuv/src/lib.rs`
  - `safe/crates/libsharpyuv/build.rs`
  - `safe/crates/libwebp/src/lib.rs`
  - `safe/crates/libwebp/build.rs`
  - `safe/crates/libwebpdecoder/src/lib.rs`
  - `safe/crates/libwebpdecoder/build.rs`
- File Changes:
  - Create the `webp-abi` type modules and layout-test scaffolding
  - Create `webp-core` runtime modules for allocation, overflow checks, threading hooks, and CPU dispatch
  - Create the `libsharpyuv` export crate and build script
  - Add build scripts for `libwebp` and `libwebpdecoder` that already apply SONAME and version-script baselines even before those libraries are complete
- Implementation Details:
  - Define all public structs and enums from the installed headers with field-exact `#[repr(C)]` layout, including padding fields where the C ABI relies on them.
  - Put the ABI layout test at `safe/crates/webp-abi/tests/ffi_layout.rs` so the checker command `cargo test -p webp-abi --test ffi_layout` is valid.
  - Use a `build.rs` in `webp-abi` to compile small C snippets against the copied headers and compare `sizeof`, `alignof`, and `offsetof` with Rust definitions.
  - Centralize arithmetic hardening in `checked.rs`, reproducing `WEBP_MAX_ALLOCABLE_MEMORY` and `CheckSizeOverflow` semantics from `original/src/utils/utils.h`.
  - Export the shared runtime functions and object symbols needed by both `libwebp` and `libwebpdecoder`:
    - `WebPMalloc`
    - `WebPFree`
    - `WebPSafeMalloc`
    - `WebPSafeCalloc`
    - `WebPSafeFree`
    - `WebPSetWorkerInterface`
    - `WebPGetWorkerInterface`
    - `VP8GetCPUInfo` as a mutable exported global object symbol
  - Implement `SharpYuvGetVersion`, `SharpYuvComputeConversionMatrix`, `SharpYuvGetConversionMatrix`, `SharpYuvConvert`, and `SharpYuvInit` in safe Rust where possible, limiting `unsafe` to FFI boundaries and CPU intrinsics.
  - Make every export crate `build.rs` consume the phase-1 `*.exports` baseline to drive linker version scripts and set exact SONAMEs.
- Verification:
  - Commands listed above

## 3. Decoder Core and `libwebpdecoder` Parity

- Phase Name: Decoder Core and `libwebpdecoder` Parity
- Implement Phase ID: `impl-decode-core`
- Verification Phases:
  - `check-decode-core`
    - type: `check`
    - fixed `bounce_target`: `impl-decode-core`
    - purpose: prove the Rust decoder path matches upstream symbols and behavior for info queries, decode entry points, external-memory decode, and incremental decode.
    - commands it should run:
      - `cargo build --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p libwebpdecoder -p libwebp --release`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-symbols --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libwebpdecoder`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-symbol-subset --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libwebp --subset decode`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-sonames --libs libwebpdecoder`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-needed --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libwebpdecoder`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- build-c-tests --suite decode_api`
      - `ctest --test-dir /home/yans/code/safelibs/ported/libwebp/safe/build/tests -R decode_api --output-on-failure`
  - `check-decode-security`
    - type: `check`
    - fixed `bounce_target`: `impl-decode-core`
    - purpose: validate denial-of-service and malformed-input hardening for the lossless Huffman path.
    - commands it should run:
      - `cargo test --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p webp-core cve_2020_36332 -- --exact`
      - `cargo test --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p webp-core malformed_huffman_tables -- --exact`
- Preexisting Inputs:
  - `original/src/dec/*`
  - decode-side `original/src/dsp/*`
  - decode-side `original/src/utils/*`
  - `safe/abi/original/*`
  - `safe/crates/webp-abi/*`
  - `safe/crates/webp-core/src/{alloc.rs,checked.rs,threading.rs,cpu.rs}`
  - `original/examples/{test.webp,test_ref.ppm}`
  - `relevant_cves.json`
- New Outputs:
  - `safe/crates/webp-core/src/decode/{mod.rs,alpha_dec.rs,buffer_dec.rs,frame_dec.rs,idec_dec.rs,io_dec.rs,quant_dec.rs,tree_dec.rs,vp8_dec.rs,vp8l_dec.rs,webp_dec.rs}`
  - `safe/crates/webp-core/src/dsp/{mod.rs,alpha_processing.rs,dec.rs,filters.rs,lossless.rs,rescaler.rs,upsampling.rs,yuv.rs}`
  - `safe/crates/webp-core/src/utils/{mod.rs,bit_reader.rs,color_cache.rs,filters.rs,huffman.rs,quant_levels_dec.rs,rescaler.rs,utils.rs}`
  - decoder exports added to `safe/crates/libwebpdecoder/src/lib.rs`
  - decode-side exports added to `safe/crates/libwebp/src/lib.rs`
  - `safe/tests/c/decode_api_test.c`
- File Changes:
  - Add decoder modules mirroring the upstream decoder C files
  - Expand `libwebpdecoder` and `libwebp` export crates to expose the decode-side ABI
  - Add a dedicated C test suite for decode APIs under `safe/tests/c/`
- Implementation Details:
  - Port `WebPGetInfo`, `WebPGetFeaturesInternal`, `WebPInitDecBufferInternal`, `WebPInitDecoderConfigInternal`, and the `WebPDecode*` family.
  - Port incremental decode APIs:
    - `WebPINewDecoder`
    - `WebPINewRGB`
    - `WebPINewYUV`
    - `WebPINewYUVA`
    - `WebPIAppend`
    - `WebPIUpdate`
    - `WebPIDecGetRGB`
    - `WebPIDecGetYUVA`
    - `WebPIDecodedArea`
    - `WebPIDelete`
  - Preserve decode status-code behavior for null pointers, truncated inputs, bad parameters, and invalid bitstreams.
  - Mirror upstream buffer-ownership semantics with safe owned storage internally and raw pointers only at the FFI boundary.
  - Rework the `ReadHuffmanCodes` and `VP8LHuffmanTablesAllocate` logic so group counts and table sizes are derived with checked arithmetic and geometry-based caps rather than trusting malformed metadata.
- Verification:
  - Commands listed above

## 4. Demux and Animation Decode

- Phase Name: Demux and Animation Decode
- Implement Phase ID: `impl-demux-animdecode`
- Verification Phases:
  - `check-demux-animdecode`
    - type: `check`
    - fixed `bounce_target`: `impl-demux-animdecode`
    - purpose: validate `libwebpdemux` symbols and animated decode behavior against upstream semantics.
    - commands it should run:
      - `cargo build --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p libwebpdemux --release`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-symbols --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libwebpdemux`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-sonames --libs libwebpdemux`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-needed --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libwebpdemux`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- build-c-tests --suite demux_animdecode`
      - `ctest --test-dir /home/yans/code/safelibs/ported/libwebp/safe/build/tests -R demux_animdecode --output-on-failure`
- Preexisting Inputs:
  - `original/src/demux/*`
  - `original/src/webp/demux.h`
  - `original/examples/test.webp`
  - decoder outputs from phase 3
  - `safe/abi/original/*`
- New Outputs:
  - `safe/crates/webp-core/src/demux/{mod.rs,demux.rs,anim_decode.rs}`
  - `safe/crates/libwebpdemux/src/lib.rs`
  - `safe/crates/libwebpdemux/build.rs`
  - `safe/tests/c/demux_animdecode_test.c`
- File Changes:
  - Add demux and animation-decode modules
  - Create the `libwebpdemux` export crate
  - Add C tests for chunk iteration, frame iteration, and animation decode
- Implementation Details:
  - Port `WebPDemuxInternal`, `WebPDemuxGetI`, `WebPDemuxGetFrame`, `WebPDemuxGetChunk`, iterator stepping, and iterator-release APIs.
  - Port `WebPAnimDecoderOptionsInitInternal`, `WebPAnimDecoderNewInternal`, `WebPAnimDecoderGetInfo`, `WebPAnimDecoderGetNext`, `WebPAnimDecoderHasMoreFrames`, `WebPAnimDecoderReset`, `WebPAnimDecoderGetDemuxer`, and `WebPAnimDecoderDelete`.
  - Preserve canvas disposal, blending, timestamp, and background-color semantics from `original/src/demux/anim_decode.c`.
  - Reuse the phase-3 decoder core for fragment decode rather than forking separate image paths.
- Verification:
  - Commands listed above

## 5. Encoder Core and Still-Image `libwebp` Write Path

- Phase Name: Encoder Core and Still-Image `libwebp` Write Path
- Implement Phase ID: `impl-encode-core`
- Verification Phases:
  - `check-encode-core`
    - type: `check`
    - fixed `bounce_target`: `impl-encode-core`
    - purpose: validate config initialization, picture ownership utilities, memory-writer behavior, and still-image encode/decode roundtrips.
    - commands it should run:
      - `cargo build --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p libwebp --release`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-symbol-subset --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libwebp --subset encode`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-sonames --libs libwebp`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-needed --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libwebp`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- build-c-tests --suite encode_api`
      - `ctest --test-dir /home/yans/code/safelibs/ported/libwebp/safe/build/tests -R encode_api --output-on-failure`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- build-upstream-tools --tools cwebp dwebp --safe-prefix /home/yans/code/safelibs/ported/libwebp/safe/dist`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- tool-smoke --prefix /home/yans/code/safelibs/ported/libwebp/safe/dist --tools cwebp dwebp`
  - `check-encode-security`
    - type: `check`
    - fixed `bounce_target`: `impl-encode-core`
    - purpose: validate arithmetic-hardening regressions required by `CVE-2016-9085`.
    - commands it should run:
      - `cargo test --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p webp-core cve_2016_9085 -- --exact`
      - `cargo test --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p webp-core oversized_picture_alloc -- --exact`
- Preexisting Inputs:
  - `original/src/enc/*`
  - encode-side `original/src/utils/*`
  - encode-side `original/src/dsp/*`
  - `original/sharpyuv/*`
  - `original/examples/{cwebp.c,dwebp.c,example_util.c,stopwatch.h}`
  - `original/imageio/*`
  - `safe/abi/original/*`
  - previous phase outputs
  - `relevant_cves.json`
- New Outputs:
  - `safe/crates/webp-core/src/encode/{mod.rs,alpha_enc.rs,analysis_enc.rs,backward_references_cost_enc.rs,backward_references_enc.rs,config_enc.rs,cost_enc.rs,filter_enc.rs,frame_enc.rs,histogram_enc.rs,iterator_enc.rs,near_lossless_enc.rs,picture_csp_enc.rs,picture_enc.rs,picture_rescale_enc.rs,picture_tools_enc.rs,predictor_enc.rs,quant_enc.rs,syntax_enc.rs,token_enc.rs,tree_enc.rs,vp8l_enc.rs,webp_enc.rs}`
  - encode-side updates to `safe/crates/libwebp/src/lib.rs`
  - `safe/tests/c/encode_api_test.c`
- File Changes:
  - Add encoder modules mirroring the upstream encoder families
  - Extend `libwebp` exports to cover the still-image write path
  - Add encode-side C compatibility tests
- Implementation Details:
  - Port:
    - `WebPConfigInitInternal`
    - `WebPConfigLosslessPreset`
    - `WebPValidateConfig`
    - `WebPPictureInitInternal`
    - `WebPPictureAlloc`
    - `WebPPictureCopy`
    - `WebPPictureCrop`
    - `WebPPictureView`
    - `WebPPictureRescale`
    - `WebPPictureImport*`
    - `WebPPictureARGBToYUVA`
    - `WebPPictureYUVAToARGB`
    - `WebPMemoryWriterInit`
    - `WebPMemoryWriterClear`
    - `WebPMemoryWrite`
    - `WebPEncode`
    - the `WebPEncode{RGB,RGBA,BGR,BGRA,Lossless*}` helpers
  - Preserve exact default config values and validation bounds from `original/src/enc/config_enc.c`.
  - Model `WebPPicture` ownership and “view” state explicitly so `WebPPictureIsView` and `WebPPictureFree` remain bug-compatible from the caller’s perspective.
  - Route all size computations through the checked-arithmetic helper to satisfy `CVE-2016-9085`.
  - Build upstream `cwebp` and `dwebp` from `original/examples/` and `original/imageio/` against the Rust-installed headers and libraries rather than reimplementing those tools.
- Verification:
  - Commands listed above

## 6. Mux and Animation Encode

- Phase Name: Mux and Animation Encode
- Implement Phase ID: `impl-mux-animencode`
- Verification Phases:
  - `check-mux-animencode`
    - type: `check`
    - fixed `bounce_target`: `impl-mux-animencode`
    - purpose: validate mux APIs, animation encode APIs, and the unchanged upstream public API regression test.
    - commands it should run:
      - `cargo build --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p libwebpmux -p libwebpdemux -p libwebp --release`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-symbols --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libwebpmux`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-sonames --libs libwebpmux`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-needed --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libwebpmux`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- build-upstream-public-api-test --source /home/yans/code/safelibs/ported/libwebp/original/tests/public_api_test.c`
      - `ctest --test-dir /home/yans/code/safelibs/ported/libwebp/safe/build/tests -R webp_public_api_test --output-on-failure`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- build-upstream-tools --tools img2webp webpmux webpinfo --safe-prefix /home/yans/code/safelibs/ported/libwebp/safe/dist`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- tool-smoke --prefix /home/yans/code/safelibs/ported/libwebp/safe/dist --tools img2webp webpmux webpinfo`
- Preexisting Inputs:
  - `original/src/mux/*`
  - `original/tests/public_api_test.c`
  - `original/examples/{img2webp.c,webpmux.c,webpinfo.c,anim_util.c,example_util.c}`
  - `original/imageio/*`
  - previous phase outputs
  - `safe/abi/original/*`
- New Outputs:
  - `safe/crates/webp-core/src/mux/{mod.rs,anim_encode.rs,muxedit.rs,muxinternal.rs,muxread.rs}`
  - `safe/crates/libwebpmux/src/lib.rs`
  - `safe/crates/libwebpmux/build.rs`
  - build glue for the unchanged upstream `public_api_test.c`
- File Changes:
  - Add mux and animation-encode modules
  - Create the `libwebpmux` export crate
  - Wire the unchanged upstream public API test into the safe test build
- Implementation Details:
  - Port:
    - `WebPGetMuxVersion`
    - `WebPNewInternal`
    - `WebPMuxCreateInternal`
    - `WebPMuxSetImage`
    - `WebPMuxSetChunk`
    - `WebPMuxGetChunk`
    - `WebPMuxGetFrame`
    - `WebPMuxGetFeatures`
    - `WebPMuxGetCanvasSize`
    - `WebPMuxNumChunks`
    - `WebPMuxAssemble`
    - `WebPAnimEncoderOptionsInitInternal`
    - `WebPAnimEncoderNewInternal`
    - `WebPAnimEncoderAdd`
    - `WebPAnimEncoderAssemble`
    - `WebPAnimEncoderGetError`
    - `WebPAnimEncoderDelete`
  - Preserve `WebPData` ownership semantics, especially `WebPDataClear()` and `copy_data` behavior in mux APIs.
  - Keep animation timestamp, loop-count, key-frame, and background-color semantics aligned with the upstream implementation so `original/tests/public_api_test.c` passes unchanged.
- Verification:
  - Commands listed above

## 7. Debian Packaging, Tool Surface, and Harness Migration

- Phase Name: Debian Packaging, Tool Surface, and Harness Migration
- Implement Phase ID: `impl-packaging-harness`
- Verification Phases:
  - `check-package-surface`
    - type: `check`
    - fixed `bounce_target`: `impl-packaging-harness`
    - purpose: verify the Rust workspace builds installable Ubuntu 24.04 packages whose contents match the explicit install-surface baseline and whose shared-library dependency edges still match `needed.json`.
    - commands it should run:
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- package-deb --output-dir /tmp/libwebp-safe-debs`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-needed --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libsharpyuv libwebpdecoder libwebp libwebpdemux libwebpmux`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-install-tree --baseline /home/yans/code/safelibs/ported/libwebp/safe/abi/original/install-surface.json --package-dir /tmp/libwebp-safe-debs`
      - `dpkg-deb -c /tmp/libwebp-safe-debs/webp_*.deb | rg 'usr/bin/(cwebp|dwebp|gif2webp|img2webp|vwebp|webpinfo|webpmux|anim_diff|anim_dump)$'`
      - `dpkg-deb -c /tmp/libwebp-safe-debs/webp_*.deb | rg 'usr/share/man/man1/(cwebp|dwebp|gif2webp|img2webp|vwebp|webpinfo|webpmux)\\.1(\\.gz)?$'`
      - `dpkg-deb -c /tmp/libwebp-safe-debs/libwebp-dev_*.deb | rg 'usr/include/webp/(decode|demux|encode|mux|mux_types|types)\\.h$'`
      - `dpkg-deb -c /tmp/libwebp-safe-debs/libsharpyuv-dev_*.deb | rg 'usr/include/webp/sharpyuv/(sharpyuv|sharpyuv_csp)\\.h$'`
  - `check-safe-tool-harness`
    - type: `check`
    - fixed `bounce_target`: `impl-packaging-harness`
    - purpose: verify the modified Docker harness installs the generated packages and explicitly exercises every shipped tool.
    - commands it should run:
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- package-deb --output-dir /tmp/libwebp-safe-debs`
      - `LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs /home/yans/code/safelibs/ported/libwebp/test-original.sh --variant safe --only tools`
      - `LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs /home/yans/code/safelibs/ported/libwebp/test-original.sh --variant safe --only tool:gif2webp`
      - `LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs /home/yans/code/safelibs/ported/libwebp/test-original.sh --variant safe --only tool:vwebp`
  - `check-dependent-harness`
    - type: `check`
    - fixed `bounce_target`: `impl-packaging-harness`
    - purpose: verify the modified root harness installs the Rust packages and still passes the full dependent matrix.
    - commands it should run:
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- package-deb --output-dir /tmp/libwebp-safe-debs`
      - `LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs /home/yans/code/safelibs/ported/libwebp/test-original.sh --variant safe`
      - `LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs /home/yans/code/safelibs/ported/libwebp/test-original.sh --variant safe --only pillow`
      - `LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs /home/yans/code/safelibs/ported/libwebp/test-original.sh --variant safe --only ffmpeg`
- Preexisting Inputs:
  - `safe/abi/original/install-surface.json`
  - `safe/abi/original/needed.json`
  - `safe/debian/*`
  - `safe/man/*.1`
  - `test-original.sh`
  - `dependents.json`
  - `original/examples/*`
  - `original/imageio/*`
  - `original/extras/*`
  - `original/man/*.1`
- New Outputs:
  - finalized Debian packaging in `safe/debian/`
  - install/package logic in `xtask`
  - modified `test-original.sh` with `--variant` and tool-smoke support
  - package-ready install tree under `safe/dist/`
- File Changes:
  - Update `safe/debian/{control,rules,clean,changelog,libwebp-dev.install,libwebp7.install,libwebpdecoder3.install,libwebpdemux2.install,libwebpmux3.install,libsharpyuv-dev.install,libsharpyuv0.install,webp.install,webp.manpages,libwebp-dev.docs}`
  - Update `safe/pkgconfig/*.pc.in`
  - Update `safe/cmake/WebPConfig.cmake.in`
  - Update `safe/xtask/src/{main.rs,tools.rs,package.rs,verify.rs}`
  - Modify `test-original.sh`
- Implementation Details:
  - Preserve the upstream package names and package split exactly.
  - Replace the wildcard semantics in `safe/debian/webp.install` with an explicit binary list:
    - `usr/bin/cwebp`
    - `usr/bin/dwebp`
    - `usr/bin/gif2webp`
    - `usr/bin/img2webp`
    - `usr/bin/vwebp`
    - `usr/bin/webpinfo`
    - `usr/bin/webpmux`
    - `usr/bin/anim_diff`
    - `usr/bin/anim_dump`
  - Replace the wildcard semantics in `safe/debian/webp.manpages` with an explicit manpage list:
    - `usr/share/man/man1/cwebp.1`
    - `usr/share/man/man1/dwebp.1`
    - `usr/share/man/man1/gif2webp.1`
    - `usr/share/man/man1/img2webp.1`
    - `usr/share/man/man1/vwebp.1`
    - `usr/share/man/man1/webpinfo.1`
    - `usr/share/man/man1/webpmux.1`
  - Reuse the upstream example, imageio, extras, and manpage sources rather than rewriting the tools in Rust.
  - Implement `xtask package-deb` as a wrapper over the `safe/debian/` package build, not as an unrelated packaging path.
  - Modify `test-original.sh` in place so it:
    - accepts `--variant original|safe`
    - in `safe` mode installs the `.deb` files from `LIBWEBP_SAFE_DEB_DIR`
    - still supports `--only <dependent-slug>`
    - adds `--only tools` and `--only tool:<name>` modes for explicit tool verification
    - adds `libglut-dev` to the Docker image so `vwebp` can be built or run in the container
    - runs a `run_tool_smokes` step before dependent checks
  - `run_tool_smokes` must explicitly exercise:
    - `cwebp` by encoding `original/examples/test_ref.ppm`
    - `dwebp` by decoding `original/examples/test.webp`
    - `gif2webp` by converting a small animated GIF generated with Pillow from existing fixtures
    - `img2webp` by assembling `frame1.ppm` and `frame2.ppm`
    - `webpinfo` by inspecting still and animated fixtures
    - `webpmux` by setting and reading EXIF metadata on `input.webp`
    - `anim_dump` by dumping frames from `animated.webp`
    - `anim_diff` by comparing two known-equivalent animations
    - `vwebp` by running `-version` or `-info` under `xvfb-run`
- Verification:
  - Commands listed above

## 8. Final Safety Reduction and Full Compatibility Sweep

- Phase Name: Final Safety Reduction and Full Compatibility Sweep
- Implement Phase ID: `impl-final-hardening`
- Verification Phases:
  - `check-final-compat`
    - type: `check`
    - fixed `bounce_target`: `impl-final-hardening`
    - purpose: validate the final symbol surface, SONAMEs, `DT_NEEDED` edges, original-object relink compatibility, and the unchanged upstream public API test.
    - commands it should run:
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-symbols --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libsharpyuv libwebpdecoder libwebp libwebpdemux libwebpmux`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-sonames --libs libsharpyuv libwebpdecoder libwebp libwebpdemux libwebpmux`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-needed --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libsharpyuv libwebpdecoder libwebp libwebpdemux libwebpmux`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- relink-original-objects --manifest /home/yans/code/safelibs/ported/libwebp/safe/abi/original/relink-manifest.json --fixtures webp_public_api_test cwebp dwebp img2webp webpinfo webpmux`
      - `ctest --test-dir /home/yans/code/safelibs/ported/libwebp/safe/build/tests --output-on-failure`
  - `check-final-runtime`
    - type: `check`
    - fixed `bounce_target`: `impl-final-hardening`
    - purpose: rerun the full packaged tool and dependent matrix against the final Rust packages.
    - commands it should run:
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- package-deb --output-dir /tmp/libwebp-safe-debs`
      - `LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs /home/yans/code/safelibs/ported/libwebp/test-original.sh --variant safe --only tools`
      - `LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs /home/yans/code/safelibs/ported/libwebp/test-original.sh --variant safe`
  - `check-final-security`
    - type: `check`
    - fixed `bounce_target`: `impl-final-hardening`
    - purpose: validate the remaining `unsafe` surface is minimal and deliberate and that upstream fuzz targets still build against the Rust static libraries.
    - commands it should run:
      - `cargo test --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml --workspace --release`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- unsafe-audit`
      - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- build-upstream-fuzzers --source-dir /home/yans/code/safelibs/ported/libwebp/original/tests/fuzzer --safe-prefix /home/yans/code/safelibs/ported/libwebp/safe/dist`
      - `cargo test --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p webp-core cve_2016_9085 -- --exact`
      - `cargo test --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -p webp-core cve_2020_36332 -- --exact`
- Preexisting Inputs:
  - all prior phase outputs
  - `relevant_cves.json`
  - `original/tests/fuzzer/*`
  - `safe/abi/original/*`
- New Outputs:
  - final `safe/docs/unsafe-audit.md`
  - any remaining compatibility or safety fixes across the workspace
  - final static-library and fuzzer build glue
- File Changes:
  - Update export crates and `webp-core` modules to eliminate avoidable internal `unsafe`
  - Add `safe/docs/unsafe-audit.md`
  - Add any final regression tests under `safe/tests/`
- Implementation Details:
  - Restrict remaining `unsafe` to:
    - FFI entry points
    - exported global-object symbols
    - unavoidable pointer conversions at the C ABI boundary
    - architecture intrinsics and linker/section attributes
  - Replace any remaining internal raw-pointer traversal with slices, iterators, and typed state where behavior permits.
  - Ensure worker-interface mutation and `VP8GetCPUInfo` remain ABI-correct and synchronized after the refactor.
  - Preserve the checked-arithmetic and Huffman-cap tests introduced for the relevant CVEs.
- Verification:
  - Commands listed above

# Critical Files

- `test-original.sh`
  - Add `--variant original|safe`, `--only tools`, and `--only tool:<name>` support; install safe `.deb` packages when requested; add the explicit tool-smoke matrix; extend the Docker package list with `libglut-dev`.
- `safe/Cargo.toml`
  - Virtual workspace manifest, shared dependency declarations, and common build profiles.
- `safe/rust-toolchain.toml`
  - Rust toolchain pin for reproducible builds.
- `safe/abi/original/{libsharpyuv,libwebp,libwebpdecoder,libwebpdemux,libwebpmux}.exports`
  - Checked-in authoritative export lists used by later symbol verifiers and linker version scripts.
- `safe/abi/original/sonames.json`
  - Checked-in SONAME baseline.
- `safe/abi/original/needed.json`
  - Checked-in `DT_NEEDED` baseline that later `verify-needed` checks must consume exactly.
- `safe/abi/original/install-surface.json`
  - Checked-in package/install baseline, including explicit binary and manpage lists.
- `safe/abi/original/relink-manifest.json`
  - Checked-in relink fixtures and link lines for original object files.
- `safe/include/webp/{decode,demux,encode,mux,mux_types,types}.h`
  - Canonical installed public headers copied from upstream.
- `safe/include/webp/sharpyuv/{sharpyuv,sharpyuv_csp}.h`
  - Canonical installed `libsharpyuv` headers copied from upstream.
- `safe/pkgconfig/{libwebp,libwebpdecoder,libwebpdemux,libwebpmux,libsharpyuv}.pc.in`
  - pkg-config metadata preserving upstream names and link semantics.
- `safe/cmake/WebPConfig.cmake.in`
  - CMake package metadata preserving upstream `WebP::` package behavior.
- `safe/man/{cwebp,dwebp,gif2webp,img2webp,vwebp,webpinfo,webpmux}.1`
  - Packaged manpages reused from upstream.
- `safe/debian/control`
  - Debian package split and dependencies.
- `safe/debian/rules`
  - Debian build/install entry point invoking Cargo and upstream tool builds.
- `safe/debian/{libwebp-dev.install,libwebp7.install,libwebpdecoder3.install,libwebpdemux2.install,libwebpmux3.install,libsharpyuv-dev.install,libsharpyuv0.install}`
  - Runtime/dev install manifests for the library packages.
- `safe/debian/webp.install`
  - Explicit binary list for the `webp` package; no wildcard.
- `safe/debian/webp.manpages`
  - Explicit manpage list for the `webp` package; no wildcard.
- `safe/crates/webp-abi/src/{lib.rs,types.rs,decode.rs,encode.rs,demux.rs,mux.rs,sharpyuv.rs}`
  - Public C ABI types and version constants.
- `safe/crates/webp-abi/build.rs`
  - Layout-test code generation/compilation against copied headers.
- `safe/crates/webp-abi/tests/ffi_layout.rs`
  - ABI layout assertions; this is the correct path for the `cargo test -p webp-abi --test ffi_layout` command.
- `safe/crates/webp-core/src/{alloc.rs,checked.rs,threading.rs,cpu.rs}`
  - Shared runtime services for safe allocation, overflow checking, worker hooks, and CPU dispatch.
- `safe/crates/webp-core/src/decode/*.rs`
  - Decoder modules mirroring `original/src/dec/*.c`.
- `safe/crates/webp-core/src/encode/*.rs`
  - Encoder modules mirroring `original/src/enc/*.c`.
- `safe/crates/webp-core/src/demux/*.rs`
  - Demux and animation-decode modules mirroring `original/src/demux/*.c`.
- `safe/crates/webp-core/src/mux/*.rs`
  - Mux and animation-encode modules mirroring `original/src/mux/*.c`.
- `safe/crates/webp-core/src/dsp/*.rs`
  - DSP dispatch and scalar/SIMD helpers mirroring `original/src/dsp/*.c`.
- `safe/crates/webp-core/src/utils/*.rs`
  - Bitreader, Huffman, rescaler, filter, and helper logic mirroring `original/src/utils/*.c`.
- `safe/crates/webp-core/src/sharpyuv/*.rs`
  - `libsharpyuv` implementation modules.
- `safe/crates/libwebp/{Cargo.toml,build.rs,src/lib.rs}`
  - Export crate for `libwebp.so.7` and `libwebp.a`.
- `safe/crates/libwebpdecoder/{Cargo.toml,build.rs,src/lib.rs}`
  - Export crate for `libwebpdecoder.so.3` and `libwebpdecoder.a`.
- `safe/crates/libwebpdemux/{Cargo.toml,build.rs,src/lib.rs}`
  - Export crate for `libwebpdemux.so.2` and `libwebpdemux.a`.
- `safe/crates/libwebpmux/{Cargo.toml,build.rs,src/lib.rs}`
  - Export crate for `libwebpmux.so.3` and `libwebpmux.a`.
- `safe/crates/libsharpyuv/{Cargo.toml,build.rs,src/lib.rs}`
  - Export crate for `libsharpyuv.so.0` and `libsharpyuv.a`.
- `safe/xtask/src/{main.rs,baseline.rs,verify.rs,link.rs,tools.rs,package.rs}`
  - Orchestration for baseline capture, exact `verify-needed`/symbol/SONAME checks, C-test/tool builds, object relinking, packaging, and verification.
- `safe/tests/c/{decode_api_test.c,demux_animdecode_test.c,encode_api_test.c}`
  - C compatibility tests that cover staged subsets of the upstream API.
- `safe/docs/unsafe-audit.md`
  - Final inventory and justification for every remaining `unsafe` block.

# Final Verification

After all phases complete, the final checker should verify the finished port from a clean checkout with Docker available:

1. Build Debian packages from `safe/`.
   - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- package-deb --output-dir /tmp/libwebp-safe-debs`
2. Verify symbols, SONAMEs, and `DT_NEEDED` dependency edges against the phase-1 baselines.
   - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-symbols --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libsharpyuv libwebpdecoder libwebp libwebpdemux libwebpmux`
   - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-sonames --libs libsharpyuv libwebpdecoder libwebp libwebpdemux libwebpmux`
   - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-needed --baseline-dir /home/yans/code/safelibs/ported/libwebp/safe/abi/original --libs libsharpyuv libwebpdecoder libwebp libwebpdemux libwebpmux`
   - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- verify-install-tree --baseline /home/yans/code/safelibs/ported/libwebp/safe/abi/original/install-surface.json --package-dir /tmp/libwebp-safe-debs`
3. Run all Rust-native tests, including ABI layout tests and CVE regressions.
   - `cargo test --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml --workspace --release`
4. Compile and run the unchanged upstream public API regression test against the Rust libraries.
   - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- build-upstream-public-api-test --source /home/yans/code/safelibs/ported/libwebp/original/tests/public_api_test.c`
   - `ctest --test-dir /home/yans/code/safelibs/ported/libwebp/safe/build/tests --output-on-failure`
5. Verify link compatibility by relinking original object files against the Rust libraries and running the resulting binaries.
   - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- relink-original-objects --manifest /home/yans/code/safelibs/ported/libwebp/safe/abi/original/relink-manifest.json --fixtures webp_public_api_test cwebp dwebp img2webp webpinfo webpmux`
6. Verify the full packaged tool surface explicitly.
   - `LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs /home/yans/code/safelibs/ported/libwebp/test-original.sh --variant safe --only tools`
7. Verify the full dependent runtime matrix.
   - `LIBWEBP_SAFE_DEB_DIR=/tmp/libwebp-safe-debs /home/yans/code/safelibs/ported/libwebp/test-original.sh --variant safe`
8. Verify the upstream fuzz targets still build against the Rust static libraries and the remaining `unsafe` is audited.
   - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- build-upstream-fuzzers --source-dir /home/yans/code/safelibs/ported/libwebp/original/tests/fuzzer --safe-prefix /home/yans/code/safelibs/ported/libwebp/safe/dist`
   - `cargo run -p xtask --manifest-path /home/yans/code/safelibs/ported/libwebp/safe/Cargo.toml -- unsafe-audit`

The port is only complete when all eight steps pass, the `webp` package really contains the full explicit tool list, the relink fixtures execute successfully, the Docker-dependent runtime matrix passes with the safe packages installed, and the remaining `unsafe` surface is documented and confined to unavoidable ABI edges.
