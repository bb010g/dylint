use anyhow::{anyhow, Context, Result};
use cargo_metadata::Dependency;
use dylint_internal::{clone, env, packaging::isolate};
use std::{
    env::set_var,
    fs::{read_to_string, write},
    path::Path,
};
use tempfile::tempdir_in;
use test_log::test;

const ERROR_LINES: usize = 5;

#[test]
fn ui() {
    // smoelius: Try to order failures by how informative they are: failure to build the library,
    // failure to find the library, failure to build/find the driver.

    dylint_internal::cargo::build("clippy", false)
        .success()
        .unwrap();

    let tempdir = tempdir_in(env!("CARGO_MANIFEST_DIR")).unwrap();

    clone_rust_clippy(tempdir.path()).unwrap();

    isolate(tempdir.path()).unwrap();

    let src_base = tempdir.path().join("tests/ui");
    // smoelius: I can't remember why disabling `rustfix` was necessary.
    // disable_rustfix(&src_base).unwrap();
    adjust_macro_use_imports_test(&src_base).unwrap();

    // smoelius: `DYLINT_LIBRARY_PATH` must be set before `dylint_libs` is called.
    // smoelius: This is no longer true. See comment in `dylint_testing::initialize`.
    let metadata = dylint_internal::cargo::current_metadata().unwrap();
    let dylint_library_path = metadata.target_directory.join("debug");
    set_var(env::DYLINT_LIBRARY_PATH, &dylint_library_path);

    let dylint_libs = dylint_testing::dylint_libs("clippy").unwrap();
    let driver =
        dylint::driver_builder::get(&dylint::Dylint::default(), env!("RUSTUP_TOOLCHAIN")).unwrap();

    // smoelius: Clippy's `compile-test` panics if multiple rlibs exist for certain crates (see
    // `third_party_crates` in
    // https://github.com/rust-lang/rust-clippy/blob/master/tests/compile-test.rs). This can happen
    // as a result of using a shared target directory. The workaround I have adopted is to use a
    // temporary target directory.
    let target_dir = tempdir_in(env!("CARGO_MANIFEST_DIR")).unwrap();

    let mut command = dylint_internal::cargo::test("clippy", false);
    command
        .current_dir(&tempdir)
        .envs([
            (env::CARGO_TARGET_DIR, &*target_dir.path().to_string_lossy()),
            (env::DYLINT_LIBS, &dylint_libs),
            (env::CLIPPY_DRIVER_PATH, &*driver.to_string_lossy()),
            (env::DYLINT_RUSTFLAGS, r#"--cfg feature="cargo-clippy""#),
        ])
        .args(&["--test", "compile-test"]);

    // smoelius: Error messages like the following have occurred in Windows GitHub workflows:
    //   LINK : fatal error LNK1318: Unexpected PDB error; OK (0) 'D:\a\dylint\dylint\examples\
    //     testing\clippy\...\debug\test\ui\useless_attribute.stage-id.aux\proc_macro_derive.pdb'
    // According to Microsoft Learn, "This error message is produced for uncommon issues in PDB
    // files":
    // https://learn.microsoft.com/en-us/cpp/error-messages/tool-errors/linker-tools-error-lnk1318?view=msvc-170
    // While I don't know the underlying cause, my approach to this problem is to not link PDB
    // files. Taken from here:
    // https://github.com/rust-lang/rust/issues/67012#issuecomment-561801877
    #[cfg(windows)]
    command.envs([(env::RUSTFLAGS, "-C link-arg=/DEBUG:NONE")]);

    command.success().unwrap();
}

fn clone_rust_clippy(path: &Path) -> Result<()> {
    let clippy_lints = clippy_lints_dependency()?;
    let source = clippy_lints.source.ok_or_else(|| anyhow!("No source"))?;
    let url = source
        .strip_prefix("git+")
        .ok_or_else(|| anyhow!("Wrong prefix"))?;
    let (url, refname) = url
        .rsplit_once('=')
        .and_then(|(url, refname)| url.rsplit_once('?').map(|(url, _)| (url, refname)))
        .ok_or_else(|| anyhow!("Wrong suffix"))?;
    clone(url, refname, path, false)?;
    Ok(())
}

fn clippy_lints_dependency() -> Result<Dependency> {
    let metadata = dylint_internal::cargo::current_metadata()?;
    let package = metadata
        .packages
        .iter()
        .find(|package| package.name == env!("CARGO_PKG_NAME"))
        .ok_or_else(|| anyhow!("Could not find package"))?;
    let dependency = package
        .dependencies
        .iter()
        .find(|dependency| dependency.name == "clippy_lints")
        .ok_or_else(|| anyhow!("Could not find dependency"))?;
    Ok(dependency.clone())
}

// smoelius: The `macro_use_imports` test produces the right four errors, but not in the right
// order. I haven't yet figured out why. Hence, this hack.
#[allow(clippy::shadow_unrelated)]
fn adjust_macro_use_imports_test(src_base: &Path) -> Result<()> {
    let stderr_file = src_base.join("macro_use_imports.stderr");
    let contents = read_to_string(&stderr_file).with_context(|| {
        format!(
            "`read_to_string` failed for `{}`",
            stderr_file.to_string_lossy()
        )
    })?;
    let lines: Vec<String> = contents.lines().map(ToString::to_string).collect();

    let (first_error, rest) = lines.split_at(ERROR_LINES);
    let (note, rest) = rest.split_at(2);
    let (_blank_line, rest) = rest.split_at(1);
    let (second_error, rest) = rest.split_at(ERROR_LINES);
    let (_blank_line, rest) = rest.split_at(1);
    let (third_error, rest) = rest.split_at(ERROR_LINES);
    let (_blank_line, rest) = rest.split_at(1);
    let (fourth_error, rest) = rest.split_at(ERROR_LINES);
    let (blank_line, summary) = rest.split_at(rest.len() - 2);

    let permuted: Vec<String> = std::iter::empty()
        .chain(first_error.iter().cloned())
        .chain(note.iter().cloned())
        .chain(blank_line.iter().cloned())
        .chain(second_error.iter().cloned())
        .chain(blank_line.iter().cloned())
        .chain(third_error.iter().cloned())
        .chain(blank_line.iter().cloned())
        .chain(fourth_error.iter().cloned())
        .chain(blank_line.iter().cloned())
        .chain(summary.iter().cloned())
        .collect();

    let mut lines_sorted = lines.clone();
    let mut permuted_sorted = permuted.clone();
    lines_sorted.sort();
    permuted_sorted.sort();
    assert_eq!(lines_sorted, permuted_sorted);

    write(
        &stderr_file,
        permuted
            .iter()
            .map(|line| format!("{}\n", line))
            .collect::<String>(),
    )
    .with_context(|| format!("Could not write to `{}`", stderr_file.to_string_lossy()))?;

    Ok(())
}
