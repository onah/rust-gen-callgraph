# Call Graph Generator Architecture

- The call graph is represented as pairs of caller/callee.
- Data entries use full names as `String`, separated by `::`.
- The analyzer module parses source code (currently using `syn`, but designed to be switchable).
- The filter module provides flexible filtering of call graph data.
- The output module exports call graph data (currently DOT format, but designed for future extensibility).
