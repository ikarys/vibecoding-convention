# The image digest is pinned: it runs with the repository mounted.
jekyll := "jekyll/jekyll:pages@sha256:c9f932710cbe866385ed8fe1ba88a6cf0c85fd7e766c1f75e2b9001f83c77020"

# Serve the website at http://127.0.0.1:4000 with GitHub Pages' Jekyll.
dev:
    docker run --rm -it -p 127.0.0.1:4000:4000 -v "$PWD":/srv/jekyll {{jekyll}} jekyll serve --host 0.0.0.0 --baseurl ''

# Everything that must pass before a change is done.
check:
    cargo test --manifest-path vibecheck/Cargo.toml
    cargo clippy --manifest-path vibecheck/Cargo.toml --all-targets -- -D warnings
    cargo fmt --manifest-path vibecheck/Cargo.toml --check
    cargo run --quiet --manifest-path vibecheck/Cargo.toml -- .
