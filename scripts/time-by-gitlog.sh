#!/usr/bin/env bash
# Analyze git commit times: per-day first/last commit, duration, commit count.
# Usage:
#   scripts/time-by-gitlog.sh [repo_path]
# Defaults to the current repository if no path is given.
# Writes:
#   <repo>/logs/git.<TS>.log                     — raw `git log` (iso dates)
#   <repo>/analyses/time-by-gitlog-<TS>.log      — analysis report

set -euo pipefail

REPO="${1:-$(pwd)}"
if [ ! -d "$REPO/.git" ]; then
  echo "error: $REPO is not a git repository" >&2
  exit 1
fi

TS="$(date +%Y%m%d-%H%M%S)"
REPO_NAME="$(basename "$REPO")"
mkdir -p "$REPO/logs" "$REPO/analyses"
GIT_LOG="$REPO/logs/git.$REPO_NAME.$TS.log"
REPORT="$REPO/analyses/time-by-gitlog-$REPO_NAME-$TS.log"

{
  echo "# repo: $REPO"
  echo "# generated: $(date -Iseconds)"
  git -C "$REPO" log --reverse --pretty=format:'%ad %H %s' --date=iso
  echo ""
} > "$GIT_LOG"

awk '
/^#/ || NF == 0 { next }
{
  date = $1
  time = $2
  split(time, t, ":")
  secs = t[1]*3600 + t[2]*60 + t[3]
  if (!(date in first_s) || secs < first_s[date]) { first_s[date] = secs; first_t[date] = time }
  if (!(date in last_s)  || secs > last_s[date])  { last_s[date]  = secs; last_t[date]  = time }
  count[date]++
  total++
  if (first_date == "") first_date = date
  last_date = date
}
END {
  n = 0
  for (d in count) dates[++n] = d
  # Sort dates ascending (ISO dates are lexicographic-friendly).
  for (i = 1; i <= n; i++) for (j = i+1; j <= n; j++) if (dates[i] > dates[j]) { tmp = dates[i]; dates[i] = dates[j]; dates[j] = tmp }

  printf("| #   | Date       | First    | Last     | Duration | Commits |\n")
  printf("|-----|------------|----------|----------|----------|---------|\n")
  total_dur = 0
  for (i = 1; i <= n; i++) {
    d = dates[i]
    dur = last_s[d] - first_s[d]
    total_dur += dur
    h = int(dur/3600); m = int((dur%3600)/60); s = dur%60
    printf("| %-3d | %s | %s | %s | %02d:%02d:%02d | %7d |\n", i, d, first_t[d], last_t[d], h, m, s, count[d])
  }

  # Calendar span: days between first and last commit dates, inclusive.
  # Use mktime for accuracy.
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
  printf("Days with commits:  %d\n", n)
  printf("Total commits:      %d\n", total)
  printf("Sum of durations:   %02d:%02d:%02d  (%d seconds)\n", th, tm, ts, total_dur)
  printf("Average per day:    %02d:%02d:%02d  (over %d committed days)\n", ah, am, as_, n)
}
' "$GIT_LOG" | tee "$REPORT"

echo ""
echo "Raw git log:  $GIT_LOG"
echo "Report:       $REPORT"
