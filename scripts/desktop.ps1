$wmDesktopArguments = $args
. "$PSScriptRoot\native-env.ps1"
& pnpm --filter '@wm/desktop' tauri @wmDesktopArguments
exit $LASTEXITCODE
