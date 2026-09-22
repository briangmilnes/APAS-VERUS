BEGIN { FS="\t"; OFS=""; }
FNR==NR { d[$1]=$2; next }
{
  key=$1; fn=$2; txt=$3;
  split(key, kp, ":"); path=kp[1]; line=kp[2];
  nparts=split(path, pp, "/"); file=pp[nparts]; chap=pp[2]; sub(/^Chap/, "", chap);
  if (path ~ /standards/) chap="std";
  ab=txt;
  gsub(/Code review \(Claude Opus 4\.6\): /, "CR: ", ab);
  gsub(/APAS \([^)]*\): /, "APAS: ", ab);
  gsub(/ — .*/, "", ab); gsub(/ -- .*/, "", ab); gsub(/; .*/, "", ab); gsub(/: O\(lg n\) rounds.*/, "", ab);
  if (length(ab) > 40) ab = substr(ab, 1, 39) "…";
  v = (key in d) ? "disputed " d[key] : "confirmed";
  if (txt ~ /^APAS/) v = "quote";
  if (txt ~ /^APAS/ && key in d) v = "disputed " d[key];
  n++;
  print "| ", n, " | ", chap, " | ", file, " | ", line, " | ", fn, " | ", ab, " | ", v, " |";
}
