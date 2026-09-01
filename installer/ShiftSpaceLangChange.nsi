Unicode true
RequestExecutionLevel user

!ifndef VERSION
  !define VERSION "0.1.2"
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
!define STOP_RETRIES 20
!define STOP_INTERVAL_MS 250

Function .onInit
  ; Stop and unlock the old executable before an install or uninstall changes
  ; any metadata. Abort leaves the existing installation otherwise untouched.
  Call StopExistingApp
FunctionEnd

Function StopExistingApp
  IfFileExists "${APP_EXE}" 0 stop_done
  ExecWait '"${APP_EXE}" --quit-existing'
  StrCpy $0 0

stop_try:
  IfFileExists "${APP_EXE}" 0 stop_done
  ClearErrors
  Delete "${APP_EXE}"
  IfErrors stop_wait stop_done

stop_wait:
  IntOp $0 $0 + 1
  IntCmp $0 ${STOP_RETRIES} stop_timeout stop_sleep stop_timeout

stop_sleep:
  Sleep ${STOP_INTERVAL_MS}
  Goto stop_try

stop_timeout:
  MessageBox MB_ICONSTOP|MB_OK "기존 실행 중인 앱을 종료하거나 파일 잠금을 해제하지 못했습니다.$\r$\n설치 또는 제거를 중단합니다. 앱을 직접 종료한 뒤 다시 시도해 주세요."
  Abort

stop_done:
FunctionEnd

Function un.onInit
  ; For an uninstaller, NSIS initializes $INSTDIR to the directory containing
  ; this uninstaller. Stop the installed app before the uninstall section runs.
  Call un.StopExistingApp
FunctionEnd

Function un.StopExistingApp
  IfFileExists "${APP_EXE}" 0 un_stop_done
  ExecWait '"${APP_EXE}" --quit-existing'
  StrCpy $0 0

un_stop_try:
  IfFileExists "${APP_EXE}" 0 un_stop_done
  ClearErrors
  Delete "${APP_EXE}"
  IfErrors un_stop_wait un_stop_done

un_stop_wait:
  IntOp $0 $0 + 1
  IntCmp $0 ${STOP_RETRIES} un_stop_timeout un_stop_sleep un_stop_timeout

un_stop_sleep:
  Sleep ${STOP_INTERVAL_MS}
  Goto un_stop_try

un_stop_timeout:
  MessageBox MB_ICONSTOP|MB_OK "기존 실행 중인 앱을 종료하거나 파일 잠금을 해제하지 못했습니다.$\r$\n설치 또는 제거를 중단합니다. 앱을 직접 종료한 뒤 다시 시도해 주세요."
  Abort

un_stop_done:
FunctionEnd

Section "Install"
  SetShellVarContext current
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
  SetShellVarContext current
  ; un.onInit has already stopped and deleted the executable. Only metadata and
  ; the now-unlocked remaining files are removed here.
  DeleteRegValue HKCU "${RUN_KEY}" "${RUN_VALUE}"
  DeleteRegKey HKCU "Software\ShiftSpaceLangChange"
  DeleteRegKey HKCU "${UNINSTALL_KEY}"

  Delete "${START_MENU_DIR}\한영 전환 도우미.lnk"
  RMDir "${START_MENU_DIR}"
  Delete "${APP_EXE}"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"
SectionEnd
