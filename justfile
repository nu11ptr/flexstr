std_features := "bytes,cstr,osstr,path,serde"
nostd_features := "bytes,cstr,serde,str"
safe_features := if os() == "windows" { "safe,win_min_unsafe" } else { "safe" }

test:
    cargo nextest run -F {{std_features}} --workspace

test_nostd:
    cargo nextest run --no-default-features -F {{nostd_features}} --workspace

test_safe:
    cargo nextest run -F {{std_features}},{{safe_features}} --workspace

test_nostd_safe:
    cargo nextest run --no-default-features -F {{nostd_features}},{{safe_features}} --workspace

open_docs $RUSTDOCFLAGS="--cfg docsrs --cap-lints allow":
    cargo +nightly doc -F {{std_features}} --workspace --open

cover:
    cargo llvm-cov nextest -F {{std_features}} --workspace

cover_report:
    cargo llvm-cov nextest --output-path codecov.json --codecov -F {{std_features}} --workspace

miri $MIRIFLAGS="-Zmiri-ignore-leaks":
    cargo +nightly miri nextest run -F {{std_features}} --workspace

clippy:
    cargo clippy -F {{std_features}} --workspace --all-targets
