#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
IMAGE_TAG="${LIBWEBP_ORIGINAL_TEST_IMAGE:-libwebp-original-test:ubuntu24.04}"
ONLY=""

usage() {
  cat <<'EOF'
usage: test-original.sh [--only <name|runtime-package|source-package|slug>]

Builds the checked-in original libwebp inside an Ubuntu 24.04 Docker container,
installs it into /usr/local inside that container, and then exercises the
dependent software listed in dependents.json with focused smoke tests.

--only runs a single dependent check.
EOF
}

while (($#)); do
  case "$1" in
    --only)
      ONLY="${2:?missing value for --only}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf 'unknown option: %s\n' "$1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

command -v docker >/dev/null 2>&1 || {
  printf 'docker is required to run %s\n' "$0" >&2
  exit 1
}

[[ -d "$ROOT/original" ]] || {
  printf 'missing original source tree\n' >&2
  exit 1
}

[[ -f "$ROOT/dependents.json" ]] || {
  printf 'missing dependents.json\n' >&2
  exit 1
}

docker build -t "$IMAGE_TAG" - <<'DOCKERFILE'
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      cmake \
      dbus-x11 \
      emacs-gtk \
      ffmpeg \
      file \
      gimp \
      libgdk-pixbuf2.0-bin \
      libgif-dev \
      libjpeg-dev \
      libpng-dev \
      libsail-common-dev \
      libsail-dev \
      libsdl2-image-dev \
      libtiff-dev \
      libvips-dev \
      libvips-tools \
      libwebkit2gtk-4.1-dev \
      libreoffice-core \
      libreoffice-draw \
      pkg-config \
      python3 \
      python3-pil \
      qt6-base-dev \
      qt6-image-formats-plugins \
      sail-codecs \
      shotwell \
      webp-pixbuf-loader \
      xauth \
      xvfb \
 && rm -rf /var/lib/apt/lists/*
DOCKERFILE

docker run --rm -i \
  -e "LIBWEBP_TEST_ONLY=$ONLY" \
  -v "$ROOT":/work:ro \
  "$IMAGE_TAG" \
  bash -s <<'CONTAINER_SCRIPT'
set -euo pipefail

export LANG=C.UTF-8
export LC_ALL=C.UTF-8

ROOT=/work
ONLY_FILTER="${LIBWEBP_TEST_ONLY:-}"
HOME=/tmp/libwebp-home
XDG_RUNTIME_DIR=/tmp/libwebp-runtime
SRC_COPY=/tmp/libwebp-src
BUILD_DIR=/tmp/libwebp-build
FIXTURE_DIR=/tmp/libwebp-fixtures
TEST_ROOT=/tmp/libwebp-dependent-tests
MULTIARCH="$(gcc -print-multiarch)"
LOCAL_LIBDIR=""
MATCHED_ONLY=0

mkdir -p "$HOME" "$XDG_RUNTIME_DIR" "$FIXTURE_DIR" "$TEST_ROOT"
chmod 700 "$XDG_RUNTIME_DIR"

log_step() {
  printf '\n==> %s\n' "$1"
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

require_nonempty_file() {
  local path="$1"

  [[ -s "$path" ]] || die "expected non-empty file: $path"
}

require_contains() {
  local path="$1"
  local needle="$2"

  grep -F -- "$needle" "$path" >/dev/null 2>&1 || {
    printf 'missing expected text in %s: %s\n' "$path" "$needle" >&2
    printf -- '--- %s ---\n' "$path" >&2
    cat "$path" >&2
    exit 1
  }
}

find_file_or_die() {
  local root="$1"
  local pattern="$2"
  local path

  path="$(find "$root" -path "$pattern" -print -quit 2>/dev/null || true)"
  [[ -n "$path" ]] || die "unable to find file matching $pattern under $root"
  printf '%s\n' "$path"
}

assert_uses_local_soname() {
  local target="$1"
  local soname="$2"
  local label="${3:-$1}"
  local resolved

  resolved="$(ldd "$target" 2>/dev/null | awk -v lib="$soname" '$1 == lib { print $3; exit }')"
  [[ -n "$resolved" ]] || die "$label does not link against $soname"
  [[ "$resolved" == /usr/local/* ]] || die "$label resolved $soname to $resolved instead of /usr/local"
}

assert_any_file_under_uses_local_soname() {
  local root="$1"
  local pattern="$2"
  local soname="$3"
  local label="$4"
  local candidate resolved

  while IFS= read -r -d '' candidate; do
    resolved="$(ldd "$candidate" 2>/dev/null | awk -v lib="$soname" '$1 == lib { print $3; exit }')"
    if [[ "$resolved" == /usr/local/* ]]; then
      return 0
    fi
  done < <(find "$root" -type f -name "$pattern" -print0 2>/dev/null)

  die "$label did not resolve $soname from /usr/local in any file under $root matching $pattern"
}

reset_test_dir() {
  local slug="$1"
  local dir="$TEST_ROOT/$slug"

  rm -rf "$dir"
  mkdir -p "$dir"
  printf '%s\n' "$dir"
}

should_run() {
  local slug="$1"
  local name="$2"
  local runtime_package="$3"
  local source_package="$4"

  if [[ -z "$ONLY_FILTER" ]]; then
    return 0
  fi

  [[ "$ONLY_FILTER" == "$slug" \
    || "$ONLY_FILTER" == "$name" \
    || "$ONLY_FILTER" == "$runtime_package" \
    || "$ONLY_FILTER" == "$source_package" ]]
}

run_check() {
  local slug="$1"
  local name="$2"
  local runtime_package="$3"
  local source_package="$4"
  local func="$5"

  if ! should_run "$slug" "$name" "$runtime_package" "$source_package"; then
    return 0
  fi

  MATCHED_ONLY=1
  log_step "$name"
  "$func"
}

validate_dependents_inventory() {
  python3 <<'PY'
import json
from pathlib import Path

expected = [
    {"name": "GIMP", "source_package": "gimp", "runtime_package": "gimp"},
    {"name": "Pillow", "source_package": "pillow", "runtime_package": "python3-pil"},
    {"name": "WebKitGTK", "source_package": "webkit2gtk", "runtime_package": "libwebkit2gtk-4.1-0"},
    {"name": "Qt 6 Image Formats Plugins", "source_package": "qt6-imageformats", "runtime_package": "qt6-image-formats-plugins"},
    {"name": "SDL2_image", "source_package": "libsdl2-image", "runtime_package": "libsdl2-image-2.0-0"},
    {"name": "libvips", "source_package": "vips", "runtime_package": "libvips42t64"},
    {"name": "GNU Emacs (GTK build)", "source_package": "emacs", "runtime_package": "emacs-gtk"},
    {"name": "Shotwell", "source_package": "shotwell", "runtime_package": "shotwell"},
    {"name": "LibreOffice", "source_package": "libreoffice", "runtime_package": "libreoffice-core"},
    {"name": "FFmpeg libavcodec", "source_package": "ffmpeg", "runtime_package": "libavcodec60"},
    {"name": "webp-pixbuf-loader", "source_package": "webp-pixbuf-loader", "runtime_package": "webp-pixbuf-loader"},
    {"name": "SAIL codecs", "source_package": "sail", "runtime_package": "sail-codecs"},
]

data = json.loads(Path("/work/dependents.json").read_text(encoding="utf-8"))
actual = [
    {
        "name": entry["name"],
        "source_package": entry["source_package"],
        "runtime_package": entry["runtime_package"],
    }
    for entry in data["dependents"]
]

if actual != expected:
    raise SystemExit(
        f"unexpected dependents.json contents: expected {expected}, found {actual}"
    )
PY
}

build_original_libwebp() {
  local libwebp_so

  rm -rf "$SRC_COPY" "$BUILD_DIR"
  mkdir -p "$SRC_COPY"
  cp -a "$ROOT/original/." "$SRC_COPY"

  cmake -S "$SRC_COPY" -B "$BUILD_DIR" \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX=/usr/local \
    -DBUILD_SHARED_LIBS=ON \
    -DWEBP_LINK_STATIC=OFF \
    -DWEBP_BUILD_VWEBP=OFF \
    >/tmp/libwebp-configure.log 2>&1
  cmake --build "$BUILD_DIR" -j"$(nproc)" >/tmp/libwebp-build.log 2>&1
  cmake --install "$BUILD_DIR" >/tmp/libwebp-install.log 2>&1
  ldconfig

  libwebp_so="$(find /usr/local -name 'libwebp.so.7' -print -quit)"
  [[ -n "$libwebp_so" ]] || die "failed to locate installed libwebp.so.7 under /usr/local"
  LOCAL_LIBDIR="$(dirname "$libwebp_so")"
  export LD_LIBRARY_PATH="$LOCAL_LIBDIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
}

prepare_fixtures() {
  cat >"$FIXTURE_DIR/shotwell-exif.py" <<'PY'
from pathlib import Path

model = b"SafeLibs WebP\x00"
payload = bytearray()
payload += b"Exif\x00\x00"
payload += b"II*\x00"
payload += (8).to_bytes(4, "little")
payload += (1).to_bytes(2, "little")
payload += (0x0110).to_bytes(2, "little")
payload += (2).to_bytes(2, "little")
payload += (len(model)).to_bytes(4, "little")
payload += (26).to_bytes(4, "little")
payload += (0).to_bytes(4, "little")
payload += model
Path("shotwell.exif").write_bytes(payload)
PY

  cat >"$FIXTURE_DIR/make-animated-frames.py" <<'PY'
from pathlib import Path

source = Path("frame1.ppm").read_bytes()
parts = source.split(b"\n", 3)
if len(parts) != 4 or parts[0] != b"P6":
    raise SystemExit("unexpected PPM fixture format")

pixels = bytearray(parts[3])
if len(pixels) < 6:
    raise SystemExit("fixture is too small")

pixels[0:3] = bytes([255 - pixels[0], 255 - pixels[1], 255 - pixels[2]])
pixels[3:6] = bytes([pixels[4], pixels[5], pixels[3]])
Path("frame2.ppm").write_bytes(parts[0] + b"\n" + parts[1] + b"\n" + parts[2] + b"\n" + pixels)
PY

  cp "$ROOT/original/examples/test.webp" "$FIXTURE_DIR/input.webp"
  cp "$ROOT/original/examples/test_ref.ppm" "$FIXTURE_DIR/input.ppm"
  cp "$ROOT/original/examples/test_ref.ppm" "$FIXTURE_DIR/frame1.ppm"

  dwebp "$FIXTURE_DIR/input.webp" -o "$FIXTURE_DIR/input.png" >/tmp/dwebp.log 2>&1
  (
    cd "$FIXTURE_DIR"
    python3 make-animated-frames.py
  )
  img2webp -loop 0 -lossless \
    -o "$FIXTURE_DIR/animated.webp" \
    "$FIXTURE_DIR/frame1.ppm" \
    "$FIXTURE_DIR/frame2.ppm" \
    >/tmp/img2webp.log 2>&1
  (
    cd "$FIXTURE_DIR"
    python3 shotwell-exif.py
    webpmux -set exif shotwell.exif input.webp -o metadata.webp >/tmp/webpmux.log 2>&1
  )

  require_nonempty_file "$FIXTURE_DIR/input.png"
  require_nonempty_file "$FIXTURE_DIR/animated.webp"
  require_nonempty_file "$FIXTURE_DIR/metadata.webp"

  webpinfo "$FIXTURE_DIR/input.webp" >/tmp/webpinfo-input.log 2>&1 || die "webpinfo rejected input.webp"
  webpinfo "$FIXTURE_DIR/animated.webp" >/tmp/webpinfo-animated.log 2>&1 || die "webpinfo rejected animated.webp"
  webpinfo "$FIXTURE_DIR/metadata.webp" >/tmp/webpinfo-metadata.log 2>&1 || die "webpinfo rejected metadata.webp"
  require_contains /tmp/webpinfo-metadata.log "EXIF"
}

test_gimp() {
  local dir plugin

  dir="$(reset_test_dir gimp)"
  plugin="$(find_file_or_die /usr/lib '*/gimp/2.0/plug-ins/file-webp/file-webp')"
  assert_uses_local_soname "$plugin" libwebp.so.7 "GIMP WebP plug-in"

  timeout 180 gimp-console-2.10 -i -d -f \
    -b "(let* ((image (car (gimp-file-load RUN-NONINTERACTIVE \"$FIXTURE_DIR/input.webp\" \"$FIXTURE_DIR/input.webp\"))) (drawable (car (gimp-image-get-active-layer image)))) (gimp-file-save RUN-NONINTERACTIVE image drawable \"$dir/gimp-out.webp\" \"$dir/gimp-out.webp\") (gimp-image-delete image))" \
    -b "(gimp-quit 0)" \
    >"$dir/run.log" 2>&1 || {
      cat "$dir/run.log" >&2
      exit 1
    }

  require_nonempty_file "$dir/gimp-out.webp"
  webpinfo "$dir/gimp-out.webp" >"$dir/webpinfo.log" 2>&1 || {
    cat "$dir/webpinfo.log" >&2
    exit 1
  }
}

test_pillow() {
  local dir

  dir="$(reset_test_dir pillow)"
  assert_any_file_under_uses_local_soname /usr/lib/python3/dist-packages/PIL '*_webp*.so' libwebp.so.7 'Pillow WebP extension'

  FIXTURE_DIR="$FIXTURE_DIR" OUTPUT_WEBP="$dir/pillow-anim.webp" python3 - <<'PY'
import os
from PIL import Image, ImageSequence

source = os.path.join(os.environ["FIXTURE_DIR"], "animated.webp")
target = os.environ["OUTPUT_WEBP"]

image = Image.open(source)
assert getattr(image, "n_frames", 1) >= 2
frames = [frame.copy().convert("RGBA") for frame in ImageSequence.Iterator(image)]
frames[0].save(
    target,
    format="WEBP",
    save_all=True,
    append_images=frames[1:],
    duration=[80] * len(frames),
    loop=0,
)
roundtrip = Image.open(target)
assert getattr(roundtrip, "n_frames", 1) >= 2
print(roundtrip.n_frames)
PY

  require_nonempty_file "$dir/pillow-anim.webp"
  webpinfo "$dir/pillow-anim.webp" >"$dir/webpinfo.log" 2>&1 || {
    cat "$dir/webpinfo.log" >&2
    exit 1
  }
}

test_webkitgtk() {
  local dir

  dir="$(reset_test_dir webkitgtk)"
  cp "$FIXTURE_DIR/input.webp" "$dir/input.webp"

  cat >"$dir/index.html" <<'HTML'
<!doctype html>
<html>
  <body>
    <img
      id="webp"
      src="file:///tmp/libwebp-dependent-tests/webkitgtk/input.webp"
      onload="document.title='loaded-' + this.naturalWidth + 'x' + this.naturalHeight;"
      onerror="document.title='error';">
  </body>
</html>
HTML

  cat >"$dir/webkit_smoke.c" <<'C'
#include <gtk/gtk.h>
#include <webkit2/webkit2.h>

static gboolean check_title(gpointer data) {
  WebKitWebView *view = WEBKIT_WEB_VIEW(data);
  const gchar *title = webkit_web_view_get_title(view);

  if (!title) {
    return G_SOURCE_CONTINUE;
  }

  if (g_str_has_prefix(title, "loaded-")) {
    g_print("title=%s\n", title);
    gtk_main_quit();
    return G_SOURCE_REMOVE;
  }

  if (g_strcmp0(title, "error") == 0) {
    g_printerr("image load failed\n");
    gtk_main_quit();
    return G_SOURCE_REMOVE;
  }

  return G_SOURCE_CONTINUE;
}

int main(void) {
  gtk_init(NULL, NULL);

  GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  GtkWidget *view = webkit_web_view_new();
  gtk_container_add(GTK_CONTAINER(window), view);
  gtk_widget_show_all(window);

  webkit_web_view_load_uri(
      WEBKIT_WEB_VIEW(view),
      "file:///tmp/libwebp-dependent-tests/webkitgtk/index.html");
  g_timeout_add(100, check_title, view);
  g_timeout_add_seconds(20, (GSourceFunc)gtk_main_quit, NULL);
  gtk_main();

  const gchar *title = webkit_web_view_get_title(WEBKIT_WEB_VIEW(view));
  return (title && g_str_has_prefix(title, "loaded-")) ? 0 : 1;
}
C

  cc "$dir/webkit_smoke.c" -o "$dir/webkit-smoke" $(pkg-config --cflags --libs webkit2gtk-4.1) >/tmp/webkit-compile.log 2>&1 || {
    cat /tmp/webkit-compile.log >&2
    exit 1
  }

  timeout 120 xvfb-run -a --server-args="-screen 0 1024x768x24" \
    "$dir/webkit-smoke" >"$dir/run.log" 2>&1 || {
      cat "$dir/run.log" >&2
      exit 1
    }

  require_contains "$dir/run.log" 'title=loaded-'
}

test_qt6_image_formats() {
  local dir plugin

  dir="$(reset_test_dir qt6-image-formats-plugins)"
  plugin="$(find_file_or_die /usr/lib '*/qt6/plugins/imageformats/libqwebp.so')"
  assert_uses_local_soname "$plugin" libwebp.so.7 "Qt WebP image plug-in"

  cat >"$dir/qt_webp_probe.cpp" <<'CPP'
#include <QCoreApplication>
#include <QImage>
#include <QImageReader>
#include <QImageWriter>
#include <QTextStream>

int main(int argc, char **argv) {
  QCoreApplication app(argc, argv);
  if (argc != 3) {
    return 2;
  }

  bool hasWebp = false;
  for (const QByteArray &format : QImageReader::supportedImageFormats()) {
    if (format.toLower() == "webp") {
      hasWebp = true;
      break;
    }
  }

  if (!hasWebp) {
    QTextStream(stderr) << "missing WebP support\n";
    return 1;
  }

  QImage image(argv[1]);
  if (image.isNull()) {
    QTextStream(stderr) << "failed to load input WebP\n";
    return 1;
  }

  image = image.mirrored(true, false);
  QImageWriter writer(argv[2], "webp");
  if (!writer.write(image)) {
    QTextStream(stderr) << writer.errorString() << '\n';
    return 1;
  }

  QTextStream(stdout) << image.width() << "x" << image.height() << '\n';
  return 0;
}
CPP

  g++ -std=c++17 "$dir/qt_webp_probe.cpp" -o "$dir/qt-webp-probe" \
    $(pkg-config --cflags --libs Qt6Gui) >/tmp/qt-compile.log 2>&1 || {
      cat /tmp/qt-compile.log >&2
      exit 1
    }

  QT_QPA_PLATFORM=offscreen "$dir/qt-webp-probe" \
    "$FIXTURE_DIR/input.webp" "$dir/qt-out.webp" >"$dir/run.log" 2>&1 || {
      cat "$dir/run.log" >&2
      exit 1
    }

  require_nonempty_file "$dir/qt-out.webp"
  webpinfo "$dir/qt-out.webp" >"$dir/webpinfo.log" 2>&1 || {
    cat "$dir/webpinfo.log" >&2
    exit 1
  }
}

test_sdl2_image() {
  local dir

  dir="$(reset_test_dir sdl2-image)"
  assert_uses_local_soname "/usr/lib/${MULTIARCH}/libSDL2_image-2.0.so.0" libwebp.so.7 "SDL2_image runtime library"

  cat >"$dir/sdl_webp.c" <<'C'
#include <SDL.h>
#include <SDL_image.h>

int main(int argc, char **argv) {
  if (argc != 2) {
    return 2;
  }

  if (SDL_Init(0) != 0) {
    return 1;
  }

  int flags = IMG_Init(IMG_INIT_WEBP);
  if ((flags & IMG_INIT_WEBP) == 0) {
    SDL_Quit();
    return 1;
  }

  SDL_Surface *surface = IMG_Load(argv[1]);
  if (surface == NULL) {
    IMG_Quit();
    SDL_Quit();
    return 1;
  }

  int ok = surface->w > 0 && surface->h > 0;
  SDL_FreeSurface(surface);
  IMG_Quit();
  SDL_Quit();
  return ok ? 0 : 1;
}
C

  cc "$dir/sdl_webp.c" -o "$dir/sdl-webp" $(pkg-config --cflags --libs SDL2_image) >/tmp/sdl-compile.log 2>&1 || {
    cat /tmp/sdl-compile.log >&2
    exit 1
  }

  "$dir/sdl-webp" "$FIXTURE_DIR/input.webp" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }
}

test_libvips() {
  local dir

  dir="$(reset_test_dir libvips)"
  assert_uses_local_soname "/usr/lib/${MULTIARCH}/libvips.so.42" libwebp.so.7 "libvips runtime library"

  vips copy "$FIXTURE_DIR/input.webp" "$dir/roundtrip.png" >/tmp/vips-cli.log 2>&1 || {
    cat /tmp/vips-cli.log >&2
    exit 1
  }
  vips copy "$dir/roundtrip.png" "$dir/roundtrip.webp" >/tmp/vips-cli2.log 2>&1 || {
    cat /tmp/vips-cli2.log >&2
    exit 1
  }

  cat >"$dir/vips_webp.c" <<'C'
#include <stdio.h>
#include <vips/vips.h>

int main(int argc, char **argv) {
  if (argc != 3) {
    return 2;
  }

  if (VIPS_INIT("vips-webp-smoke")) {
    return 1;
  }

  VipsImage *image = vips_image_new_from_file(argv[1], NULL);
  if (!image) {
    vips_error_exit(NULL);
  }

  if (vips_image_write_to_file(image, argv[2], NULL)) {
    g_object_unref(image);
    vips_error_exit(NULL);
  }

  printf("%dx%d\n", vips_image_get_width(image), vips_image_get_height(image));
  g_object_unref(image);
  vips_shutdown();
  return 0;
}
C

  cc "$dir/vips_webp.c" -o "$dir/vips-webp" $(pkg-config --cflags --libs vips) >/tmp/vips-compile.log 2>&1 || {
    cat /tmp/vips-compile.log >&2
    exit 1
  }

  "$dir/vips-webp" "$FIXTURE_DIR/input.webp" "$dir/vips-out.webp" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  require_nonempty_file "$dir/roundtrip.webp"
  require_nonempty_file "$dir/vips-out.webp"
  webpinfo "$dir/roundtrip.webp" >"$dir/webpinfo-roundtrip.log" 2>&1 || {
    cat "$dir/webpinfo-roundtrip.log" >&2
    exit 1
  }
  webpinfo "$dir/vips-out.webp" >"$dir/webpinfo-vips.log" 2>&1 || {
    cat "$dir/webpinfo-vips.log" >&2
    exit 1
  }
}

test_emacs() {
  local dir emacs_bin

  dir="$(reset_test_dir emacs)"
  emacs_bin="$(readlink -f "$(command -v emacs)")"
  assert_uses_local_soname "$emacs_bin" libwebpdecoder.so.3 "Emacs binary"

  cat >"$dir/emacs-webp.el" <<'EL'
(setq debug-on-error t)

(unless (image-type-available-p 'webp)
  (error "webp image support is unavailable"))

(let ((frame (make-frame '((visibility . nil) (minibuffer . t)))))
  (unwind-protect
      (let* ((image (create-image "INPUT_WEBP" 'webp nil))
             (size (image-size image t frame))
             (width (ceiling (car size)))
             (height (ceiling (cdr size))))
        (unless (and (> width 0) (> height 0))
          (error "invalid image size"))
        (with-temp-file "OUTPUT_PATH"
          (insert (format "%dx%d\n" width height))))
    (delete-frame frame t)))

(kill-emacs 0)
EL

  sed -i "s|INPUT_WEBP|$FIXTURE_DIR/input.webp|g; s|OUTPUT_PATH|$dir/size.txt|g" "$dir/emacs-webp.el"

  timeout 120 xvfb-run -a emacs -Q -l "$dir/emacs-webp.el" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  require_nonempty_file "$dir/size.txt"
  require_contains "$dir/size.txt" 'x'
}

test_shotwell() {
  local dir

  dir="$(reset_test_dir shotwell)"
  timeout 120 xvfb-run -a shotwell --show-metadata "$FIXTURE_DIR/metadata.webp" \
    >"$dir/shotwell.out" 2>"$dir/shotwell.err" || {
      cat "$dir/shotwell.out" >&2 || true
      cat "$dir/shotwell.err" >&2 || true
      exit 1
    }

  require_contains "$dir/shotwell.out" 'Exif.Image.Model'
  require_contains "$dir/shotwell.out" 'SafeLibs WebP'
}

test_libreoffice() {
  local dir

  dir="$(reset_test_dir libreoffice)"
  mkdir -p "$dir/profile" "$dir/out"
  assert_any_file_under_uses_local_soname /usr/lib/libreoffice/program '*.so*' libwebp.so.7 'LibreOffice program libraries'

  timeout 240 libreoffice --headless \
    "-env:UserInstallation=file://$dir/profile" \
    --convert-to pdf \
    --outdir "$dir/out" \
    "$FIXTURE_DIR/input.webp" \
    >"$dir/run.log" 2>&1 || {
      cat "$dir/run.log" >&2
      exit 1
    }

  require_nonempty_file "$dir/out/input.pdf"
  file "$dir/out/input.pdf" | grep -F 'PDF document' >/dev/null
}

test_ffmpeg() {
  local dir
  local -a decode_cmd encode_cmd

  dir="$(reset_test_dir ffmpeg)"
  assert_uses_local_soname "/usr/lib/${MULTIARCH}/libavcodec.so.60" libwebp.so.7 "FFmpeg libavcodec"

  decode_cmd=(
    ffmpeg
    -hide_banner
    -loglevel error
    -nostdin
    -y
    -i "$FIXTURE_DIR/input.webp"
    "$dir/decoded.png"
  )
  "${decode_cmd[@]}" >/tmp/ffmpeg-decode.log 2>&1 || {
    cat /tmp/ffmpeg-decode.log >&2
    exit 1
  }

  encode_cmd=(
    ffmpeg
    -hide_banner
    -loglevel error
    -nostdin
    -y
    -loop 1
    -i "$FIXTURE_DIR/input.png"
    -frames:v 1
    -c:v libwebp
    "$dir/encoded.webp"
  )
  "${encode_cmd[@]}" >/tmp/ffmpeg-encode.log 2>&1 || {
    cat /tmp/ffmpeg-encode.log >&2
    exit 1
  }

  require_nonempty_file "$dir/decoded.png"
  require_nonempty_file "$dir/encoded.webp"
  file "$dir/decoded.png" | grep -F 'PNG image data' >/dev/null
  webpinfo "$dir/encoded.webp" >"$dir/webpinfo.log" 2>&1 || {
    cat "$dir/webpinfo.log" >&2
    exit 1
  }
}

test_webp_pixbuf_loader() {
  local dir loader query_loaders thumbnailer

  dir="$(reset_test_dir webp-pixbuf-loader)"
  loader="$(find_file_or_die /usr/lib '*/gdk-pixbuf-2.0/*/loaders/libpixbufloader-webp.so')"
  query_loaders="$(find_file_or_die /usr '*/gdk-pixbuf-query-loaders')"
  thumbnailer="$(find_file_or_die /usr '*/gdk-pixbuf-thumbnailer')"
  assert_uses_local_soname "$loader" libwebp.so.7 "GDK Pixbuf WebP loader"

  "$query_loaders" >"$dir/loaders.cache"
  require_contains "$dir/loaders.cache" 'libpixbufloader-webp'

  GDK_PIXBUF_MODULE_FILE="$dir/loaders.cache" \
    "$thumbnailer" -s 96 "$FIXTURE_DIR/input.webp" "$dir/thumbnail.png" >"$dir/run.log" 2>&1 || {
      cat "$dir/run.log" >&2
      exit 1
    }

  require_nonempty_file "$dir/thumbnail.png"
  file "$dir/thumbnail.png" | grep -F 'PNG image data' >/dev/null
}

test_sail_codecs() {
  local dir codec

  dir="$(reset_test_dir sail)"
  codec="$(find /usr/lib -path '*sail*webp*.so*' -type f -print -quit 2>/dev/null || true)"
  if [[ -n "$codec" ]]; then
    assert_uses_local_soname "$codec" libwebpdecoder.so.3 "SAIL WebP codec"
  fi

  cat >"$dir/sail_webp.c" <<'C'
#include <stdio.h>
#include <sail/sail.h>

int main(int argc, char **argv) {
  struct sail_image *image = NULL;

  if (argc != 2) {
    return 2;
  }

  if (sail_load_from_file(argv[1], &image) != SAIL_OK || image == NULL) {
    return 1;
  }

  printf("%d x %d\n", image->width, image->height);
  sail_destroy_image(image);
  return 0;
}
C

  cc "$dir/sail_webp.c" -o "$dir/sail-webp" $(pkg-config --cflags --libs sail) >/tmp/sail-compile.log 2>&1 || {
    cat /tmp/sail-compile.log >&2
    exit 1
  }

  "$dir/sail-webp" "$FIXTURE_DIR/input.webp" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  require_contains "$dir/run.log" ' x '
}

validate_dependents_inventory

log_step "Building original libwebp"
build_original_libwebp

log_step "Preparing shared fixtures"
prepare_fixtures

run_check gimp "GIMP" gimp gimp test_gimp
run_check pillow "Pillow" python3-pil pillow test_pillow
run_check webkitgtk "WebKitGTK" libwebkit2gtk-4.1-0 webkit2gtk test_webkitgtk
run_check qt6-image-formats-plugins "Qt 6 Image Formats Plugins" qt6-image-formats-plugins qt6-imageformats test_qt6_image_formats
run_check sdl2-image "SDL2_image" libsdl2-image-2.0-0 libsdl2-image test_sdl2_image
run_check libvips "libvips" libvips42t64 vips test_libvips
run_check emacs "GNU Emacs (GTK build)" emacs-gtk emacs test_emacs
run_check shotwell "Shotwell" shotwell shotwell test_shotwell
run_check libreoffice "LibreOffice" libreoffice-core libreoffice test_libreoffice
run_check webp-pixbuf-loader "webp-pixbuf-loader" webp-pixbuf-loader webp-pixbuf-loader test_webp_pixbuf_loader
run_check sail "SAIL codecs" sail-codecs sail test_sail_codecs
run_check ffmpeg "FFmpeg libavcodec" libavcodec60 ffmpeg test_ffmpeg

if [[ -n "$ONLY_FILTER" && "$MATCHED_ONLY" -eq 0 ]]; then
  die "no dependent matched --only=$ONLY_FILTER"
fi

log_step "All dependent smoke tests passed"
CONTAINER_SCRIPT
