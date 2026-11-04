@echo off
rem Written for nishiki: what the bat grammar's other inputs leave out.
setlocal EnableDelayedExpansion
set /a total=(1+2)*3, count+=1, mask^=0x0F, shift<<=2, flag=!flag, neg=~5
set /a "quoted=4/2, other%%=3"
set /p answer=Enter a value:
set /p "name=Your name? "
echo %path:~1,-2% %str:~5% %var:a=b%
echo !path:~0,3! !var:old=new! !var:x! !value!
echo %%literal ^^! ^& ^| caret^
echo "a %%b c" "%x%"
x & : comment after an ampersand
x & :: two colons
rem. dotted remark
REM.with text | more
rem a | b < c
dir | find "a" || echo none
(echo a, b; c)
for %%f in (*.txt) do echo %%~nxf %%~dp$PATH:f
echo NUL > out.txt
echo "unclosed
