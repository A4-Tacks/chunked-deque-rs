#!/bin/jq -nrRf
def trim: gsub("^\\s+|\\s+$";"");
def group_by(f): reduce .[] as $item ({}; .[$item|f|tojson]+=[$item]) | map(.);
[
  foreach inputs as $line ([""];[last, $line];
    select(last|test("time:"))
  ) |
  if last | test("^ *time:") | not then
    last|match(" ").offset as $i | [.[:$i], .[$i:]]
  end | map(trim) |
  last |= capture("\\[\\S+ \\S+ (?<n>\\S+) (?<t>\\S+) \\S+ \\S+\\]$") |
  {name:first}+last |
  .data = (.name|match("/\\d+$").string[1:]) // . |
  .name |= sub("/\\d+$";"")
] |
group_by([.data,.name|sub("/.*";"")]) |
map(map(select(.name|test("linked_list$")| not))|select(length==2)) |
map(sort_by(.name)) |
map(map(.n |= tonumber | if .t == "ms" then
  .t = "µs" | .n *= 1000
elif .t == "ns" then
  .t = "µs" | .n /= 1000
end)) |
map(
  {name:(first.name|sub("/.*";"")),scale:(last.n/first.n),data:(first.data)} |
  .scale |= (tostring+".00"|sub("(?<=\\.\\d{2}).*|$";"x")) |
  .name |= sub("^deque_";"") |
  .name |= "`\(.)`"
) |
group_by(.name) |
map({name:(first.name), datas:map(del(.name))}) |
def fi($to): .+" "*($to-length);
(reduce .[].datas as $data (first.datas|map(del(.scale)); if . != ($data|map(del(.scale))) then halt_error end)) as $_ |
[
  ["bench of VecDeque", first.datas[].data],
  [0, first.datas[]|"---"],
  map([.name, .datas[].scale])[]
] |
[range(first|length) as $i | map(.[$i]|length)|max] as $maxes |
.[] | "| \([to_entries[] as {$value,$key} | $value|fi($maxes[$key])] | join(" | ")) |"
