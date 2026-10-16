; Included by the Tauri NSIS installer (bundle.windows.nsis.installerHooks).

; Tauri's "Delete the application data" box only removes the folders named after
; the bundle identifier; keys, plugins, translations and rollback copies live in
; %APPDATA%\PMCA-RE-RELOADED (pmca_i18n::catalog::user_data_dir).
!macro NSIS_HOOK_POSTUNINSTALL
  ${If} $DeleteAppDataCheckboxState = 1
  ${AndIf} $UpdateMode <> 1
    SetShellVarContext current
    RmDir /r "$APPDATA\PMCA-RE-RELOADED"
  ${EndIf}
!macroend
