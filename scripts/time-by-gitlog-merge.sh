#!/usr/bin/env bash
# Merge git-log timings across multiple repos.
#
# For each calendar date that appears in ANY of the given repos, take:
#   - first commit time = min across all repos
#   - last  commit time = max across all repos
#   - duration          = last - first
#   - commits           = sum across all repos
#   - sources           = which repos contributed commits that day
#
# This extends work-day boundaries across co-worked repos without
# double-counting concurrent minutes within a single day.
#
# Usage:
#   scripts/time-by-gitlog-merge.sh <repo1> <repo2> [repo3 ...]
# Writes:
#   APAS-VERUS/logs/git-merge.<tag>.<TS>.log          — raw combined stream
#   APAS-VERUS/analyses/time-by-gitlog-merge-<tag>-<TS>.log — report

set -euo pipefail

if [ "$#" -lt 2 ]; then
  echo "usage: $0 <repo1> <repo2> [repo3 ...]" >&2
  exit 1
fi

for REPO in "$@"; do
  if [ ! -d "$REPO/.git" ]; then
    echo "error: $REPO is not a git repository" >&2
    exit 1
  fi
done

APAS_VERUS="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TS="$(date +%Y%m%d-%H%M%S)"
TAG=""
for REPO in "$@"; do
  NAME="$(basename "$REPO")"
  TAG="${TAG:+$TAG+}$NAME"
done

mkdir -p "$APAS_VERUS/logs" "$APAS_VERUS/analyses"
COMBINED="$APAS_VERUS/logs/git-merge.$TAG.$TS.log"
REPORT="$APAS_VERUS/analyses/time-by-gitlog-merge-$TAG-$TS.log"

{
  echo "# repos: $*"
  echo "# generated: $(date -Iseconds)"
  for REPO in "$@"; do
    NAME="$(basename "$REPO")"
    git -C "$REPO" log --pretty=format:"%ad $NAME" --date=iso
    echo ""
  done | grep -v '^$' | sort
} > "$COMBINED"

awk -v tag="$TAG" '
/^#/ || NF == 0 { next }
{
  date = $1
  time = $2
  repo = $4
  split(time, t, ":")
  secs = t[1]*3600 + t[2]*60 + t[3]
  if (!(date in first_s) || secs < first_s[date]) { first_s[date] = secs; first_t[date] = time; first_r[date] = repo }
  if (!(date in last_s)  || secs > last_s[date])  { last_s[date]  = secs; last_t[date]  = time; last_r[date]  = repo  }
  count[date]++
  per_repo[date SUBSEP repo]++
  total++
  if (first_date == "") first_date = date
  last_date = date
}
END {
  n = 0
  for (d in count) dates[++n] = d
  for (i = 1; i <= n; i++) for (j = i+1; j <= n; j++) if (dates[i] > dates[j]) { tmp = dates[i]; dates[i] = dates[j]; dates[j] = tmp }

  # Collect unique repo names in stable order of first appearance.
  nr = 0
  for (key in per_repo) {
    split(key, kk, SUBSEP)
    r = kk[2]
    if (!(r in seen_r)) { seen_r[r] = 1; repos[++nr] = r }
  }

  printf("Merged time-by-gitlog for: %s\n\n", tag)
  printf("| #   | Date       | First    | Last     | Duration | Commits | Sources                                  |\n")
  printf("|-----|------------|----------|----------|----------|---------|------------------------------------------|\n")

  total_dur = 0
  for (i = 1; i <= n; i++) {
    d = dates[i]
    dur = last_s[d] - first_s[d]
    total_dur += dur
    h = int(dur/3600); m = int((dur%3600)/60); s = dur%60

    # Build sources string like "APAS-AI(3)+rusticate(5)".
    src = ""
    for (k = 1; k <= nr; k++) {
      r = repos[k]
      key = d SUBSEP r
      if (key in per_repo) {
        src = src (src == "" ? "" : "+") r "(" per_repo[key] ")"
      }
    }
    printf("| %-3d | %s | %s | %s | %02d:%02d:%02d | %7d | %-40s |\n", i, d, first_t[d], last_t[d], h, m, s, count[d], src)
  }

  split(first_date, fd, "-")
  split(last_date,  ld, "-")
  ft = mktime(fd[1] " " fd[2] " " fd[3] " 00 00 00")
  lt = mktime(ld[1] " " ld[2] " " ld[3] " 00 00 00")
  span_days = int((lt - ft) / 86400) + 1

  avg = (n > 0) ? total_dur / n : 0
  th = int(total_dur/3600); tm = int((total_dur%3600)/60); ts = total_dur%60
  ah = int(avg/3600);       am = int((avg%3600)/60);       as_ = int(avg%60)

  printf("\n")
  printf("Start date:         %s\n", first_date)
  printf("End date:           %s\n", last_date)
  printf("Calendar span:      %d days\n", span_days)
  printf("Days with commits:  %d (union across all repos)\n", n)
  printf("Total commits:      %d\n", total)
  printf("Sum of durations:   %02d:%02d:%02d  (%d seconds)\n", th, tm, ts, total_dur)
  printf("Average per day:    %02d:%02d:%02d  (over %d committed days)\n", ah, am, as_, n)
}
' "$COMBINED" | tee "$REPORT"

echo ""
echo "Raw combined log:  $COMBINED"
echo "Report:            $REPORT"
