# For each Alg Analysis line, print file:line, the next fn name, and the annotation text.
FNR==1 { file=FILENAME; n=0 }
/Alg Analysis/ {
  txt=$0; sub(/.*Alg Analysis: */, "", txt);
  pend[++n]=FNR; ptxt[n]=txt; next
}
/^[ \t]*(pub )?fn [A-Za-z_0-9]+/ && n>0 {
  name=$0; sub(/^[ \t]*(pub )?fn /, "", name); sub(/[<(].*/, "", name);
  for (i=1;i<=n;i++) printf "%s:%d\t%s\t%s\n", file, pend[i], name, ptxt[i];
  n=0
}
END { for (i=1;i<=n;i++) printf "%s:%d\t?\t%s\n", file, pend[i], ptxt[i] }
