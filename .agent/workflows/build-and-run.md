---
description: Build and run the merge-and-split simulation
---

# Build and Run Workflow

## Steps

// turbo-all

1. **Check the project builds**
```bash
cargo check
```

2. **Build the project in debug mode**
```bash
cargo build
```

3. **Run the project**
```bash
cargo run
```

## For Performance Testing

4. **Build in release mode**
```bash
cargo build --release
```

5. **Run in release mode**
```bash
cargo run --release
```

## Notes

- Release mode is recommended for physics performance testing
- Debug mode is useful for development and debugging
- The simulation uses wgpu for rendering, ensure graphics drivers are up to date
