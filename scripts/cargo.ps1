$wmCargoArguments = $args
. "$PSScriptRoot\native-env.ps1"

& cargo @wmCargoArguments
exit $LASTEXITCODE
