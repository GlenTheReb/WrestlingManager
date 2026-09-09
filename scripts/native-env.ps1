$ErrorActionPreference = 'Stop'
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"

$wmVsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path -LiteralPath $wmVsWhere) {
    $wmVsRoots = & $wmVsWhere -all -products '*' -property installationPath
    foreach ($wmVsRoot in $wmVsRoots) {
        $wmDevShell = Join-Path $wmVsRoot 'Common7\Tools\Launch-VsDevShell.ps1'
        if (Test-Path -LiteralPath $wmDevShell) {
            & $wmDevShell -SkipAutomaticLocation -Arch amd64 -HostArch amd64 | Out-Null
            break
        }
    }
}
