#!/usr/bin/env bash
#
# Bundle a Windows build of RDplayer into a self-contained directory.
#
# This is intended to run INSIDE an MSYS2 UCRT64 shell on the Windows CI runner
# (where `ldd`, `glib-compile-schemas`, `gdk-pixbuf-query-loaders`, etc. live).
#
# Usage:
#   windows/bundle.sh <meson-destdir> <output-dir>
#
# Layout produced (mirrors a typical $prefix install):
#   <output>/bin/rdplayer.exe   + all dependency DLLs
#   <output>/share/...          gresource, gschema, gettext catalogs, icons, GTK data
#   <output>/lib/...            gdk-pixbuf loaders, gstreamer plugins, gtk modules
#
# The NSIS installer (windows/installer.nsi) packages this directory verbatim.

set -euo pipefail

DESTDIR="${1:-$PWD/artifact}"
DIST="${2:-$PWD/dist}"
APP="rdplayer"

echo "==> Bundling RDplayer Windows build"
echo "    destdir : $DESTDIR"
echo "    dist    : $DIST"

rm -rf "$DIST"
mkdir -p "$DIST/bin" "$DIST/share" "$DIST/lib"

# 1. Locate and copy the built executable.
EXE=$(find "$DESTDIR" -name "${APP}.exe" | head -n1)
if [ -z "$EXE" ]; then
    echo "ERROR: ${APP}.exe not found under $DESTDIR" >&2
    exit 1
fi
cp "$EXE" "$DIST/bin/"
echo "    copied $(basename "$EXE")"

# 2. Copy runtime DLLs that ldd resolves from the MSYS2 prefix.
echo "==> Collecting DLL dependencies"
while IFS= read -r line; do
    dll=$(echo "$line" | sed -n 's/.*=> \([^ ]*\) .*/\1/p')
    [ -n "$dll" ] || continue
    case "$dll" in
        /ucrt64/bin/*|/mingw64/bin/*)
            cp -n "$dll" "$DIST/bin/" 2>/dev/null || true
            ;;
    esac
done < <(ldd "$EXE")

# 2b. gdk-pixbuf ships a helper used to (re)generate the loader cache at install
#     time, since the cache embeds absolute paths that change with $INSTDIR.
cp "/ucrt64/bin/gdk-pixbuf-query-loaders.exe" "$DIST/bin/" 2>/dev/null || true

# 3. Application resources installed by meson (gschema, locale, gresource, icons).
echo "==> Copying application share/ resources"
if [ -d "$DESTDIR/usr/share" ]; then
    cp -a "$DESTDIR/usr/share/." "$DIST/share/" 2>/dev/null || true
fi

# 4. GTK4 / Adwaita runtime data (schemas, icon themes, gtk config, fonts).
echo "==> Copying GTK runtime data"
for d in glib-2.0 gtk-4.0 icons fonts; do
    if [ -d "/ucrt64/share/$d" ]; then
        mkdir -p "$DIST/share/$d"
        cp -a "/ucrt64/share/$d/." "$DIST/share/$d/" 2>/dev/null || true
    fi
done

# 5. gdk-pixbuf loaders.
echo "==> Copying gdk-pixbuf loaders"
if [ -d "/ucrt64/lib/gdk-pixbuf-2.0" ]; then
    cp -a "/ucrt64/lib/gdk-pixbuf-2.0" "$DIST/lib/" 2>/dev/null || true
fi

# 6. GTK modules (e.g. print backends) if present.
if [ -d "/ucrt64/lib/gtk-4.0" ]; then
    cp -a "/ucrt64/lib/gtk-4.0" "$DIST/lib/" 2>/dev/null || true
fi

# 7. GStreamer plugins (music playback).
echo "==> Copying GStreamer plugins"
if [ -d "/ucrt64/lib/gstreamer-1.0" ]; then
    cp -a "/ucrt64/lib/gstreamer-1.0" "$DIST/lib/" 2>/dev/null || true
fi

# 8. Recompile GSettings schemas so both app + GTK schemas are indexed.
echo "==> Recompiling GSettings schemas"
SCHEMA_DIR="$DIST/share/glib-2.0/schemas"
if [ -d "$SCHEMA_DIR" ] && command -v glib-compile-schemas >/dev/null; then
    glib-compile-schemas "$SCHEMA_DIR"
fi

# 9. Regenerate the gdk-pixbuf loader cache against the bundled tree.
echo "==> Regenerating gdk-pixbuf loader cache"
LOADERS_DIR="$DIST/lib/gdk-pixbuf-2.0/2.10.0/loaders"
if [ -d "$LOADERS_DIR" ] && command -v gdk-pixbuf-query-loaders >/dev/null; then
    PATH="$DIST/bin:$PATH" gdk-pixbuf-query-loaders "$LOADERS_DIR"/*.dll \
        > "$LOADERS_DIR/loaders.cache" 2>/dev/null || true
fi

echo "==> Done. Bundle ready at $DIST"
