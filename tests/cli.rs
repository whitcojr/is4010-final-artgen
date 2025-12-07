use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[allow(deprecated)]
#[test]
fn runs_with_valid_input() -> Result<(), Box<dyn std::error::Error>> {
    // Note: The `cargo_bin` macro automatically finds the executable based on the package name.
    let mut cmd = Command::cargo_bin("is4010-final-artgen")?;

    cmd.arg("IS4010");

    // Check for successful execution and a recognizable fragment of "IS4010"
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("____"));

    Ok(())
}

#[allow(deprecated)]
#[test]
fn fails_without_required_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("is4010-final-artgen")?;

    // Check for failure (exit code 2) when no text argument is provided
    cmd.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));

    Ok(())
}

#[allow(deprecated)]
#[test]
fn uses_starwars_font() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("is4010-final-artgen")?;

    // Test with the starwars font option (uses standard in this version)
    cmd.arg("Final").args(["--font", "starwars"]);

    cmd.assert().success().stdout(predicate::str::contains("_"));

    Ok(())
}
