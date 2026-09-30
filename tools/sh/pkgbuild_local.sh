#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "$SCRIPT_DIR/../.." && pwd)"

BUILD_DIR="$ROOT_DIR/build"
ARTIFACTS_DIR="$BUILD_DIR/artifacts"
DIST_DIR="$BUILD_DIR/dist"
CARGO_TARGET_CACHE="$BUILD_DIR/cargo-target"

PKGBUILD_DIR="$ROOT_DIR/packaging/arch/local"
PKGBUILD="$PKGBUILD_DIR/PKGBUILD"

read -r pkgname pkgver <<<"$(bash -c 'source "$1"; printf "%s %s" "$pkgname" "$pkgver"' bash "$PKGBUILD")"

mkdir -p "$ARTIFACTS_DIR" "$DIST_DIR" "$CARGO_TARGET_CACHE"
find "$DIST_DIR" -maxdepth 1 -type f -name "${pkgname}-*.pkg.tar.*" -delete

archive="$ARTIFACTS_DIR/${pkgname}-${pkgver}.tar.gz"

I18N_ROOT="$ROOT_DIR/../argvus-i18n"
TUI_ROOT="$ROOT_DIR/../argvus-tui"

[[ -f "$I18N_ROOT/Cargo.toml" ]] || {
  printf '%s\n' "argvus-i18n checkout not found beside $ROOT_DIR" >&2
  exit 1
}
[[ -f "$TUI_ROOT/Cargo.toml" ]] || {
  printf '%s\n' "argvus-tui checkout not found beside $ROOT_DIR" >&2
  exit 1
}

echo "Creating local source archive: $archive"
staging_dir="$(mktemp -d)"
trap 'rm -rf -- "$staging_dir"' EXIT
mkdir -p "$staging_dir/${pkgname}-${pkgver}" "$staging_dir/argvus-i18n" "$staging_dir/argvus-tui"

tar -cf - \
  --exclude='./.git' \
  --exclude='./.release' \
  --exclude='./packages-repo' \
  --exclude='./build' \
  --exclude='./dist' \
  --exclude='./target' \
  --exclude='./tools' \
  --exclude='./packaging/arch/local/src' \
  --exclude='./packaging/arch/local/pkg' \
  --exclude='./packaging/arch/local/PKGBUILD.local' \
  --exclude='./packaging/arch/local/*.pkg.tar*' \
  --exclude='./packaging/arch/local/*.tar.gz' \
  --exclude='./packaging/arch/ci/src' \
  --exclude='./packaging/arch/ci/pkg' \
  -C "$ROOT_DIR" . \
  | tar -xf - -C "$staging_dir/${pkgname}-${pkgver}"

for dependency_root in "$I18N_ROOT" "$TUI_ROOT"; do
  dependency_name="${dependency_root##*/}"
  tar -cf - \
    --exclude='./.git' \
    --exclude='./build' \
    --exclude='*/build' \
    --exclude='./target' \
    --exclude='*/target' \
    -C "$dependency_root" . \
    | tar -xf - -C "$staging_dir/$dependency_name"
done

tar -czf "$archive" -C "$staging_dir" \
  "${pkgname}-${pkgver}" argvus-i18n argvus-tui

if [[ -n "${MAKEPKG_FLAGS:-}" ]]; then
  # shellcheck disable=SC2206
  flags=(${MAKEPKG_FLAGS})
else
  flags=(--nodeps --noconfirm --needed --cleanbuild --force --check)
fi

cd "$PKGBUILD_DIR"
export BUILDDIR="$ARTIFACTS_DIR"
export SRCDEST="$ARTIFACTS_DIR"
export PKGDEST="$DIST_DIR"

{
  printf 'export CARGO_TARGET_DIR=%q\n' "$CARGO_TARGET_CACHE"
  cat "$PKGBUILD"
} > "$PKGBUILD_DIR/PKGBUILD.local"
trap 'rm -f "$PKGBUILD_DIR/PKGBUILD.local"' EXIT

sha256="$(sha256sum "$archive" | awk '{print $1}')"
sed -i "s/^sha256sums=.*/sha256sums=(\"${sha256}\")/" "$PKGBUILD_DIR/PKGBUILD.local"

makepkg -p PKGBUILD.local "${flags[@]}" "$@"

printf 'Packages created in %s:\n' "$DIST_DIR"
find "$DIST_DIR" -maxdepth 1 -type f -name "${pkgname}-*.pkg.tar.zst" -printf '  %f\n' | sort

printf 'Artifacts kept in %s:\n' "$ARTIFACTS_DIR"
find "$ARTIFACTS_DIR" -mindepth 1 -maxdepth 1 -printf '  %f\n' | sort
