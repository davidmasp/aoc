

hyperfine --warmup 2 --runs 5 --export-json=performance.json \
    "cat d01p1/input | d01p1/target/debug/d01" \
    "cat d01p1/input | d01p2/target/debug/d01p2" \
    "cat d02p1/input | d02p1/target/debug/d02p1" \
    "cat d02p1/input | d02p2/target/debug/d02p2" \
    "cat d03p1/input | d03p1/target/debug/d03p1" \
    "cat d03p1/input | d03p2/target/debug/d03p2" \
    "cat d04p1/input | d04p1/target/debug/d04p1" \
    "cat d04p1/input | d04p2/target/debug/d04p2" 




