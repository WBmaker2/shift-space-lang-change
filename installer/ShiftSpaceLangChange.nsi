Unicode true
RequestExecutionLevel user
SetShellVarContext current

!ifndef VERSION
  !define VERSION "0.1.0"
!endif

Name "한/영 전환 도우미"
OutFile "..\dist\ShiftSpaceLangChange-Setup-${VERSION}-x64.exe"
InstallDir "$LOCALAPPDATA\Programs\ShiftSpaceLangChange"
InstallDirRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\ShiftSpaceLangChange" "InstallLocation"
ShowInstDetails show
ShowUninstDetails show

!define APP_EXE "$INSTDIR\shift-space-lang-change.exe"
!define START_MENU_DIR "$SMPROGRAMS\한영 전환 도우미"
!define RUN_KEY "Software\Microsoft\Windows\CurrentVersion\Run"
!define RUN_VALUE "ShiftSpaceLangChange"
!define UNINSTALL_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\ShiftSpaceLangChange"

Function .onInit
  ; Ask an existing version to leave before NSIS replaces its executable.
  IfFileExists "${APP_EXE}" 0 +2
    ExecWait '"${APP_EXE}" --quit-existing'
  Sleep 500
FunctionEnd

Section "Install"
  SetOutPath "$INSTDIR"
  File "..\target\release\shift-space-lang-change.exe"

  WriteUninstaller "$INSTDIR\Uninstall.exe"

  CreateDirectory "${START_MENU_DIR}"
  CreateShortCut "${START_MENU_DIR}\한영 전환 도우미.lnk" \
    "${APP_EXE}" "" "${APP_EXE}" 0 SW_SHOWNORMAL \
    "한/영 입력 전환 단축키를 설정합니다."

  ; Auto-start is a real HKCU Run value, not a duplicated app setting.
  WriteRegStr HKCU "${RUN_KEY}" "${RUN_VALUE}" '"${APP_EXE}" --background'

  WriteRegStr HKCU "${UNINSTALL_KEY}" "DisplayName" "한/영 전환 도우미"
  WriteRegStr HKCU "${UNINSTALL_KEY}" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "${UNINSTALL_KEY}" "Publisher" "ShiftSpaceLangChange"
  WriteRegStr HKCU "${UNINSTALL_KEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "${UNINSTALL_KEY}" "UninstallString" '"$INSTDIR\Uninstall.exe"'
  WriteRegDWORD HKCU "${UNINSTALL_KEY}" "NoModify" 1
  WriteRegDWORD HKCU "${UNINSTALL_KEY}" "NoRepair" 1

  Exec '"${APP_EXE}"'
SectionEnd

Section "Uninstall"
  ; The helper process sends the private quit message and exits. A short grace
  ; period prevents normal upgrades/removals from racing the old process.
  IfFileExists "${APP_EXE}" 0 +2
    ExecWait '"${APP_EXE}" --quit-existing'
  Sleep 500

  DeleteRegValue HKCU "${RUN_KEY}" "${RUN_VALUE}"
  DeleteRegKey HKCU "Software\ShiftSpaceLangChange"
  DeleteRegKey HKCU "${UNINSTALL_KEY}"

  Delete "${START_MENU_DIR}\한영 전환 도우미.lnk"
  RMDir "${START_MENU_DIR}"
  Delete "${APP_EXE}"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"
SectionEnd
