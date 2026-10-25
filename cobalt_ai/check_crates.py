import urllib.request
import json
crates = ["tokio", "axum", "serde", "serde_json", "linfa-clustering", "anyhow", "tracing", "tracing-subscriber", "burn", "tokenizers", "rand", "rand_xoshiro", "ndarray", "ndarray-rand", "linfa", "linfa-trees"]
for c in crates:
    req = urllib.request.Request(f"https://crates.io/api/v1/crates/{c}", headers={'User-Agent': 'cobalt-checker (example@example.com)'})
    try:
        res = urllib.request.urlopen(req)
        data = json.loads(res.read())
        print(f"{c}: {data['crate']['max_version']}")
    except Exception as e:
        print(f"Error checking {c}: {e}")
