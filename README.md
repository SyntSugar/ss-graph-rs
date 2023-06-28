# ss-graph-rs

## Description

This is a graph algorithm library written in Rust, primarily featuring a function for finding all paths. This library takes full advantage of Rust's efficient concurrency and memory management to provide a quick, precise, and user-friendly way to handle graph data.

## Features

Finding all paths: Given a graph and start and end points, our algorithm can find all possible paths.

## Quick Start

Installing this library is straightforward. First, make sure you have Rust installed. Then, add the following line to the dependencies in your Cargo.toml:

```toml
[dependencies]
ss-graph-rs = "0.1.8"
```

Then, you can import and use it in your Rust files:

```rust
use ss_graph_rs::graph;
```

## Example

Here's a simple example of using this library to find all paths:

```rust
use ss_graph_rs::graph::Graph;

let mut graph = Graph::new(Some(false));
graph.add_edge(1, 3);
graph.add_edge(1, 2);

// Find all paths
let paths = graph.find_all_paths(1, 4);

for path in paths {
    println!("{:?}", path);
}
```

## Documentation

Detailed API documentation can be found in the Rust Doc.

## Contributing

We welcome and appreciate all contributors. You can contribute by submitting issues or pull requests. Before submitting a pull request, make sure your code has passed all tests.

## License

This library is under the MIT license. For more information, please see the [LICENSE](./LICENSE) file.

## Contact

If you have any questions or suggestions, please feel free to raise an issue on GitHub.
