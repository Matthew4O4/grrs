use std::process::Command;
use assert_cmd::cargo::*;
use assert_cmd::prelude::OutputAssertExt;
use predicates::prelude::*;
use tempfile::NamedTempFile;
use std::io::{self, Write};


#[test]
fn file_doest_exist() -> Result<(), Box<dyn std::error::Error>> {

    let mut  cmd = Command::main_binary()?;

    cmd.arg("foobar")
        .arg("test/file/doesnt/exist");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())

}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::main_binary()?;
    cmd.arg("test")
        .arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

