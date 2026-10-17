# RDplayer

> 基于 GTK4 / libadwaita 的第三方 Jellyfin / Emby 客户端（由 tsukimi 改造），视频使用 MPV、音乐使用 GStreamer。

![RDplayer](docs/tsukimi1.png)

## 特性

- 🎬 **Jellyfin / Emby 客户端**：浏览与播放媒体库，支持继续观看、剧集管理。
- 📺 **MPV 视频播放**：硬件解码、字幕、音轨、播放速度等完整控制。
- 🎵 **GStreamer 音乐播放**：后台播放与 Mpris 媒体控制。
- 💬 **自定义弹幕（Danmaku）服务器**：支持兼容 DandanPlay API 的自建 danmu_api 服务器。
  在设置中填写服务器地址（默认 `http://192.168.5.7:9321/87654321`），并按文件名自动选择番剧来源与集数
  （来源优先级 `tencent > iqiyi > youku > 306`，集数从如 `我不是大师S01E17` 的文件名解析）。
- 🌐 多语言界面（gettext）、自适应浅色 / 深色主题、强调色自定义。

## 安装

### Arch / CachyOS（推荐）

```bash
git clone https://github.com/rdevils7/RDplayer.git
cd RDplayer
./install.sh          # 自动用 pacman 安装依赖并以 makepkg -si 构建安装
```

或手动构建打包：

```bash
makepkg -si
```

### 从源码构建

```bash
meson setup build --prefix=/usr
meson compile -C build
sudo meson install -C build
```

> 依赖：rust、cargo、meson、blueprint-compiler、gtk4、libadwaita、mpv、glycin、
> gstreamer（含 gst-plugins-base / gst-plugins-bad）、openssl、dbus、libepoxy。

### Flatpak

见 `flatpak/moe.tsuna.tsukimi.yml`（需将源码地址改为你自己的 fork 后构建）。

### Windows（安装版）

提供 `.exe` 安装程序，由 GitHub Actions 自动构建（基于 MSYS2 UCRT64 + GTK4）。

- **下载安装**：在 [Releases](https://github.com/rdevils7/RDplayer/releases) 获取 `RDplayer-Setup-*-x86_64.exe`，双击即可安装（写入 `C:\Program Files\RDplayer`，创建开始菜单快捷方式并附带卸载程序）。GTK 运行库、gresource、GSettings schema、翻译、图标与 GStreamer 插件均随包自带，无需额外配置。
- **自行构建（需 Windows + MSYS2 UCRT64 终端）**：

  ```bash
  pacman -S mingw-w64-ucrt-x86_64-{toolchain,rust,gtk4,libadwaita,mpv,gstreamer,gstreamer-plugins-base,gstreamer-plugins-bad,gstreamer-libav,glycin,glib2,openssl,dbus,gettext,gdk-pixbuf2,libxml2,blueprint-compiler,meson,ninja,pkgconf}
  meson setup build --prefix=/usr
  meson compile -C build
  meson install -C build --destdir artifact
  bash windows/bundle.sh artifact dist
  makensis windows/installer.nsi   # 另需安装 NSIS
  ```

  `windows/bundle.sh` 会从 `meson install` 的目录收集 `rdplayer.exe`、依赖 DLL 与 `share/`、`lib/` 资源；`windows/installer.nsi` 将其打包为安装 exe。


## 配置弹幕服务器

启动后在弹幕设置中填入你的 danmu_api 地址（默认 `http://192.168.5.7:9321/87654321`）。
支持自动按来源优先级与文件名集数自动选集。

## 许可证

GPL-3.0-or-later。本项目基于 [tsukimi](https://github.com/tsukinaha/tsukimi)（上游作者 Inaha）改造。
