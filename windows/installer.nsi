; RDplayer Windows installer (NSIS)
;
; Packages the self-contained bundle produced by windows/bundle.sh
; (a `dist/` directory with bin/, share/ and lib/) into a single .exe installer.
;
; Build (on Windows, from the repository root where `dist/` lives):
;   makensis windows/installer.nsi
;
; The resulting file is RDplayer-Setup-<VERSION>-x86_64.exe.

!define APPNAME "RDplayer"
!define APPNAME_LC "rdplayer"
!define VERSION "1.0.0"
!define PUBLISHER "rdevils7"
!define WEBSITE "https://github.com/rdevils7/RDplayer"

; Keep VERSION in sync with the Cargo / meson project version.

SetCompressor /SOLID lzma

Name "${APPNAME} ${VERSION}"
OutFile "RDplayer-Setup-${VERSION}-x86_64.exe"
InstallDir "$PROGRAMFILES64\${APPNAME}"
RequestExecutionLevel admin

VIProductVersion "${VERSION}.0"
VIAddVersionKey "ProductName" "${APPNAME}"
VIAddVersionKey "ProductVersion" "${VERSION}"
VIAddVersionKey "CompanyName" "${PUBLISHER}"
VIAddVersionKey "LegalCopyright" "(c) ${PUBLISHER}"
VIAddVersionKey "FileVersion" "${VERSION}.0"
VIAddVersionKey "FileDescription" "${APPNAME} installer"

!include "MUI2.nsh"

; --- MUI configuration ---
!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"
!define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\modern-uninstall.ico"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "SimpChinese"
!insertmacro MUI_LANGUAGE "English"

; --- Install section ---
Section "Main" SecMain
    SetOutPath "$INSTDIR\bin"
    File /r "dist\bin"

    SetOutPath "$INSTDIR\share"
    File /r "dist\share"

    SetOutPath "$INSTDIR\lib"
    File /r "dist\lib"

    ; gdk-pixbuf caches loader paths as absolutes, so regenerate it against the
    ; real install location (the bundle's cache is only valid at build time).
    ExecWait 'cmd /c ""$INSTDIR\bin\gdk-pixbuf-query-loaders.exe" "$INSTDIR\lib\gdk-pixbuf-2.0\2.10.0\loaders\*.dll" > "$INSTDIR\lib\gdk-pixbuf-2.0\2.10.0\loaders\loaders.cache""'

    ; Start menu shortcut
    CreateDirectory "$SMPROGRAMS\${APPNAME}"
    CreateShortcut "$SMPROGRAMS\${APPNAME}\${APPNAME}.lnk" \
        "$INSTDIR\bin\${APPNAME_LC}.exe" "" "$INSTDIR\bin\${APPNAME_LC}.exe"
    CreateShortcut "$SMPROGRAMS\${APPNAME}\Uninstall.lnk" \
        "$INSTDIR\Uninstall.exe"

    ; Uninstaller + registry entries
    WriteUninstaller "$INSTDIR\Uninstall.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" \
        "DisplayName" "${APPNAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" \
        "UninstallString" "$\"$INSTDIR\Uninstall.exe$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" \
        "DisplayIcon" "$\"$INSTDIR\bin\${APPNAME_LC}.exe$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" \
        "Publisher" "${PUBLISHER}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" \
        "URLInfoAbout" "${WEBSITE}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" \
        "DisplayVersion" "${VERSION}"
SectionEnd

; --- Uninstall section ---
Section "Uninstall"
    Delete "$SMPROGRAMS\${APPNAME}\${APPNAME}.lnk"
    Delete "$SMPROGRAMS\${APPNAME}\Uninstall.lnk"
    RMDir "$SMPROGRAMS\${APPNAME}"

    RMDir /r "$INSTDIR\bin"
    RMDir /r "$INSTDIR\share"
    RMDir /r "$INSTDIR\lib"
    Delete "$INSTDIR\Uninstall.exe"
    RMDir "$INSTDIR"

    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}"
SectionEnd
