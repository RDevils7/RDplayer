#!/bin/bash
# tsukimi-danmaku installer
# One-line install on Arch-based distros (CachyOS, Arch, Manjaro, EndeavourOS):
#   git clone https://github.com/<you>/tsukimi-danmaku.git
#   cd tsukimi-danmaku
#   ./install.sh
set -euo pipefail

echo "==> tsukimi-danmaku 安装脚本"

if [ "$(id -u)" -eq 0 ]; then
    echo "错误：请不要以 root 运行此脚本（makepkg 禁止 root）。请用普通用户执行。" >&2
    exit 1
fi

if [ -f /etc/os-release ]; then
    # shellcheck disable=SC1091
    . /etc/os-release
fi

case " ${ID:-} ${ID_LIKE:-} " in
    *"arch"*|*"Arch"*|*"cachyos"*|*"CachyOS"*|*"manjaro"*|*"endeavouros"*)
        echo "==> 检测到 Arch 系发行版，使用 pacman + makepkg 安装"
        sudo pacman -S --needed --noconfirm \
            base-devel rust cargo meson blueprint-compiler \
            gtk4 libadwaita mpv glycin gstreamer gst-plugins-base gst-plugins-bad \
            openssl dbus libepoxy glib2 pango
        echo "==> 开始构建并安装（makepkg -si）"
        makepkg -si
        ;;
    *)
        echo "未检测到 Arch 系发行版，回退到通用 meson 构建。" >&2
        echo "请先手动安装依赖：rust, cargo, meson, blueprint-compiler, gtk4, libadwaita," >&2
        echo "mpv, glycin, gstreamer (+ gst-plugins-base/bad), openssl, dbus, libepoxy。" >&2
        meson setup build --prefix=/usr
        meson compile -C build
        sudo meson install -C build
        ;;
esac

echo "==> 完成。运行 'rdplayer' 启动播放器。"
