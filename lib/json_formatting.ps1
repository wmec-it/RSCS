Get-ChildItem -Recurse -Filter *.json -File | ForEach-Object {
    $tmp = [System.IO.Path]::GetTempFileName()
    try {
        jq --indent 2 '.' $_.FullName | Out-File -FilePath $tmp -Encoding utf8
        Move-Item -Force $tmp $_.FullName
    } catch {
        Write-Warning "Skipped (invalid JSON?): $($_.FullName)"
        Remove-Item -Force $tmp -ErrorAction SilentlyContinue
    }
}