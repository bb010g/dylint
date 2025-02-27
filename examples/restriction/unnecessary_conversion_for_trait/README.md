# unnecessary_conversion_for_trait

**What it does:** Checks for trait-behavior-preserving calls in positions where a trait
implementation is expected.

**Why is this bad?** Such unnecessary calls make the code more verbose and could impact
performance.

**Known problems:** None.

**Example:**

```rust
let _ = Command::new("ls")
    .args(["-a", "-l"].iter())
    .status()
    .unwrap();
let _ = Path::new("/").join(Path::new("."));
```

Use instead:

```rust
let _ = Command::new("ls")
    .args(["-a", "-l"])
    .status()
    .unwrap();
let _ = Path::new("/").join(".");
```
