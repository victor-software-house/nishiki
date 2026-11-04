#!/bin/bash
# Written for nishiki: what the shellscript grammar's other inputs leave out.
alias -p -x name[key]+=value; alias other[3]-=1 && alias third[@]=x || alias fourth=y &
declare -a list[2]+=(one two) more[$key]-=(three)
declare -A map=([key]=v name=value [x]=y)
declare -i n=0x1F m=017 b=16#ff f=1.5 v=1.2.3
count[1]=2 total[@]+=3 sum-=1
true||false
alias nums[3]=1
alias a=b&&alias c=d||alias e=f&
arr[2]=(x)
declare -i idx[2]-=1 idx[3]+=1
count[1]-=2
cat <<-END && while 0x1F 017 16#ff
	END
cat <<< "a\$b\"c"
[[ . == : ]]
[[ ~ == x ]]
x=1 $x$"y" arg
~/bin/run
$"cmd" arg
return 0x1F
cd ~
ls -l ~/src
ls -l
case "$x" in
  (a) echo a ;;
  @(b|c)) echo b ;;
  [\]x]) echo c ;;
esac
"my\"cmd" arg
'single' arg
echo "a \$ b \" c"
cat <<-"EOF" | tr a b
	text
	EOF
cat <<"EOF" > out
text
EOF
cat <<-EOF | sort
	text
	EOF
cat <<< 'here'
cat <<< word\ more
for item in a b c; do echo "$item"; done
echo $(( 0x1F + 017 + 16#ff ))
(( n = 0x1F + 017 + 2#101 ))
PATH=~/bin:~/lib
time ls
echo x >>file.log
printf $'\101"\x41"\cA"'
. ./lib.sh
: noop
echo `\$HOME`
x=`# note`
