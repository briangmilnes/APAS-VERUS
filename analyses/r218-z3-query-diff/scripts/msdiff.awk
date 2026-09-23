# Multiset difference of lines. Usage: awk -v MODE=Aonly|Bonly -f msdiff.awk A B
# Prints each line of A that has no matching occurrence in B (MODE=Aonly),
# or each line of B with no matching occurrence in A (MODE=Bonly), with
# multiplicity. Used instead of comm, which rejected sort's C-locale output
# for these long lines.
NR == FNR { a[$0]++; order[++n] = $0; next }
{ if (MODE == "Bonly") { if (a[$0] > 0) a[$0]--; else print } else b[$0]++ }
END { if (MODE != "Bonly") for (i = 1; i <= n; i++) { k = order[i]; if (b[k] > 0) b[k]--; else print k } }
