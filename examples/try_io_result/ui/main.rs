use anyhow::{self, Context};
use std::fs::File;

fn main() {
    foo().unwrap();
    bar().unwrap();
    baz().unwrap();
}

fn foo() -> std::io::Result<()> {
    let _ = File::open("/dev/null")?;
    Ok(())
}

fn bar() -> anyhow::Result<()> {
    let _ = File::open("/dev/null")?;
    Ok(())
}

fn baz() -> anyhow::Result<()> {
    let _ = File::open("/dev/null").with_context(|| "could not open `/dev/null`")?;
    Ok(())
}
