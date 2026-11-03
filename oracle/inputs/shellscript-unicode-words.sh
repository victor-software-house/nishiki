#!/bin/sh
# Names whose letters carry a combining mark (U+0301 or U+0308).
café=1
echo "$café ${café} ${café:-x}"
echo $näive and $HOMÉ
for é in a b; do echo "$é"; done
doné foŕ if́ theń whilé
resumé() { echo ok; }
function résume { :; }
case x́ in x́) echo 1 ;; esac
local näme=1 export ét́é
test -n "$a" && echo café || echo -é x
echo ${#café} $((café + 1)) [[ $x́ -eq́ 1 ]]
select é in a b; do break; done
