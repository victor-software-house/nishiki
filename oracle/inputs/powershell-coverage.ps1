#requires -Version 7.0 -Modules Az
using namespace System.Text

function script:Get-Coverage {
    param([string]$Name)
    $quote = 'it''s'
    $mail = "write to a.b@example.com, say ""hi"" `
        and go on"
    $here = @"
Hello $Name
"@
    $raw = @'
No $expansion here
'@
    $escapes = "`u{1F600} `u{10FFFF} `u{zz"
    $vars = "$true $Error $ConfirmPreference $global:x ${global:y} ${any text} $_"
    $numbers = 0xFFl, 0b1010, 1e5, 1.e5, 10d, 1.5kb
    $sizes = 0xFFkb, 0b1mb, 1e5gb, 1.e5tb, 10dpb
    $VerbosePreference.Value
    ${global:z}.Length
    ${some name}
    ${env:temp}.Length
    $total = "{0}" -f 1..5
    cmd.exe /c dir
    icacls.exe --% /grant Users:F
    Get-Process | Where-Object { $_.Id } | Tee-Object -Variable t
}

class Coverage {
    hidden [string] $Secret
    static [int] $Count
}

# A key quoted after a word, which the grammar's hashtable key reads.
$table = @{ a"b" = 1 }
