# Changelog

## 0.1.2

### Fixed

- Multi-file input (`svgm *.svg`) in non-terminal contexts (scripts, CI, piped commands) now correctly optimizes files in place instead of silently writing concatenated output to stdout.
- `--stdout` with multiple input files now returns an explicit error instead of producing malformed output.

## 0.1.1

### Fixed

- README images now use absolute GitHub URLs so they render correctly on crates.io.

## 0.1.0

Initial release.

- 25 optimization passes (24 default + 1 opt-in)
- Fixed-point convergence — one invocation is always enough
- Arena-based AST with O(1) parent access and mark-and-sweep removal
- Path data optimization (absolute/relative, curve shorthands, implicit repeats)
- Transform merging, application, and push-down
- CSS `<style>` inlining and minification
- Conservative defaults — preserves `<desc>`, `<title>`, animations, `<foreignObject>`
- CLI with in-place editing, `--stdout`, `--dry-run`, `--quiet`
