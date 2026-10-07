#!/bin/jq -nrRf
def group_by(f): reduce .[] as $item ({}; .[$item|f]+=[$item]) | map(.);
[
  foreach inputs as $line ([""];[last, $line];
    select(last|test("time:"))
  ) |
  if last | test("^ *time:") | not then
    last|match(" ").offset as $i | [.[:$i], .[$i:]]
  end | map(trim) |
  last |= capture("\\[\\S+ \\S+ (?<n>\\S+) (?<t>\\S+) \\S+ \\S+\\]$") |
  {name:first}+last
] |
group_by(.name|sub("/.*";"")) |
map(select(length==2)) |
map(sort_by(.name)) |
map(map(.n |= tonumber | if .t == "ms" then
  .t = "µs" | .n *= 1000
end)) |
map(
  {name:(first.name|sub("/.*";"")),scale:last.n/first.n} |
  .scale |= (tostring|sub("(?<=\\.\\d{2}).*";"x")) |
  .name |= sub("^deque_";"") |
  .name |= "`\(.)`"
) |
(map(.name|length)|max) as $maxname |
"| bench of VecDeque\(" "*($maxname-17)) | scale |",
"| ---\(" "*($maxname-3)) | ---   |",
(
  .[] | "| \(.name|.+" "*($maxname-length)) | \(.scale|.+" "*(5-length)) |"
)
