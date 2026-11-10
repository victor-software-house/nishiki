#!/bin/bash
# aa.sh <runs-dir> <free|pin>: per language, the throughput of each run
# (total KiB over total time), the A/A gap between the medians of odd and
# even runs, and the spread (max-min over median) over all runs.
d=$1; mode=$2
for f in "$d"/$mode-*.txt; do
  r=$(basename "$f" .txt); r=${r#*-}
  awk -v r="$r" '
    NR>1 && $1 ~ /\// { split($1,a,"/"); kib[a[1]]+=$2; t[a[1]]+=$2/$3 }
    END { for (l in kib) print l, r, kib[l]/t[l] }
  ' "$f"
done | sort -k1,1 -k2,2n | awk '
function median(arr, n,   i, j, tmp) {
  for (i=1;i<=n;i++) for (j=i+1;j<=n;j++) if (arr[j]<arr[i]) {tmp=arr[i];arr[i]=arr[j];arr[j]=tmp}
  return n%2 ? arr[(n+1)/2] : (arr[n/2]+arr[n/2+1])/2
}
{
  l=$1; v=$3
  if (l!=cur && cur!="") out()
  cur=l; all[++na]=v
  if ($2%2) odd[++no]=v; else even[++ne]=v
}
function out(   mo, me, ma, mn, mx, i) {
  mo=median(odd,no); me=median(even,ne); ma=median(all,na)
  mn=all[1]; mx=all[1]
  for(i=1;i<=na;i++){if(all[i]<mn)mn=all[i]; if(all[i]>mx)mx=all[i]}
  printf "%-16s median %8.1f  aa %+6.2f%%  spread %6.2f%%\n",
    cur, ma, (me/mo-1)*100, (mx-mn)/ma*100
  delete odd; delete even; delete all; no=ne=na=0
}
END { out() }'
