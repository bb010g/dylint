# redundant_reference

**What it does:** Checks for fields that are references used only to read one copyable
subfield, and whose lifetimes are not used elsewhere.

**Why is this bad?** Storing the reference instead of a copy of the subfield adds an
unnecessary lifetime parameter to the struct. It also creates an unnecessary pointer
dereference at runtime.

**Known problems:** None.

**Example:**

```rust
struct V<'cx, 'tcx> {
    cx: &'cx LateContext<'tcx>,
}

impl<'cx, 'tcx> Visitor<'tcx> for V<'cx, 'tcx>
{
    type Map = rustc_middle::hir::map::Map<'tcx>;
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.cx.tcx.hir()
    }
}
```

Use instead:

```rust
struct V<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> Visitor<'tcx> for V<'tcx>
{
    type Map = rustc_middle::hir::map::Map<'tcx>;
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.tcx.hir()
    }
}
```

**Options:**
`REDUNDANT_REFERENCE_NO_LIFETIME_CHECK`: Setting this environment variable to anything other
than `0` disables the check that the lifetime use is unique. That is, the lint becomes a
check for: fields that are references used only to read one copyable subfield.
