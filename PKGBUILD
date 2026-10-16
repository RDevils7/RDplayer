# Maintainer: you (local custom build)
# Based on AUR tsukimi 26.9.2-1 by tippfehlr <tippfehlr@archlinux.org>
#
# This PKGBUILD builds from the LOCAL working tree (your modified fork), not
# upstream. It packages the custom danmaku changes:
#   - configurable DandanPlay-compatible danmaku server (default
#     http://192.168.5.7:9321/87654321)
#   - auto-select danmaku by source priority (tencent > iqiyi > youku > 360)
#     and by episode number parsed from the filename.

pkgname=tsukimi
pkgver=26.9.2
pkgrel=1
pkgdesc='A simple third-party GTK4 Emby/Jellyfin client (local custom build with danmaku server support)'
arch=('x86_64' 'aarch64')
url='https://github.com/tsukinaha/tsukimi'
license=('GPL-3.0-or-later')
depends=(
	'mpv' 'libadwaita' 'gstreamer' 'gtk4' 'dconf' 'glib2' 'pango' 'libseccomp'
	'graphene' 'glibc' 'libgcc' 'hicolor-icon-theme' 'fontconfig' 'lcms2'
	'openssl' 'dbus' 'glycin' 'gst-plugins-base' 'gst-plugins-bad' 'libepoxy'
)
makedepends=('cargo' 'git' 'meson' 'blueprint-compiler')
# Build from the local repo working tree (this PKGBUILD lives at the repo root).
# We copy the tree into $srcdir excluding heavy build artifacts.
source=()
sha256sums=()
options=(!lto) # `ring` crate does not compile with LTO

prepare() {
	install -d "$srcdir/$pkgname-$pkgver"
	# Copy the repository into the build tree, skipping build caches and VCS data.
	tar -C "$startdir" \
		--exclude=target \
		--exclude=build \
		--exclude=.git \
		--exclude=node_modules \
		--exclude=result \
		--exclude=src \
		--exclude=pkg \
		-cf - . | tar -C "$srcdir/$pkgname-$pkgver" -xf -
}

build() {
	meson setup build "$srcdir/$pkgname-$pkgver" --prefix=/usr --buildtype=release
	meson compile -C build
}

package() {
	meson install -C build --no-rebuild --destdir "$pkgdir"
}
