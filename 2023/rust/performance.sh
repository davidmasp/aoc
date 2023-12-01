

hyperfine --runs 5 --export-json=performance.json \
    "cat d01p1/input | d01p1/target/debug/d01" \
    "cat d01p2/input | d01p2/target/debug/d01p2"


