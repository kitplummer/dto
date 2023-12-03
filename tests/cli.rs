use assert_cmd::prelude::*; // Add methods on commands
                            //use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("configure");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("Configuration file exists"));
    Ok(())
}

#[test]
fn test_configuration_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("configure");
    cmd.arg("--help");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("Create or show"));
    cmd.assert().stdout(predicates::str::contains("--create"));
    Ok(())
}

#[test]
fn test_collections() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("collections");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("stuff"));
    Ok(())
}

#[test]
fn test_execute_valid_query() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("execute");
    cmd.arg("-q");
    cmd.arg("SELECT * FROM stuff");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("hello"));
    Ok(())
}

#[test]
fn test_execute_valid_query_where_clause() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("execute");
    cmd.arg("-q");
    cmd.arg("SELECT * FROM stuff WHERE blah='world'");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("blah"));
    Ok(())
}

// Projections don't exist yet:
// thread \'main\' panicked at \'called `Result::unwrap()` on an `Err` value: FFILegacyError { code: 8, message: \"<dql> Invalid query: `Projections other than wildcard (*) is not supported`. For more information on Ditto\'s query language see: https://docs.ditto.live/dql-guide\", kind: FfiLegacy }\', src/query.rs:6:65
// Because Ditto returns a Document set, not a partial one.
//
#[test]
fn test_execute_valid_query_with_field() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("execute");
    cmd.arg("-q");
    cmd.arg("SELECT hello FROM stuff");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("unsupported query"));
    Ok(())
}

#[test]
fn test_execute_invalid_query() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("execute");
    cmd.arg("-q");
    cmd.arg("SEL * FROM stuff");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("not a valid SQL query"));
    Ok(())
}

// #[test]
// fn test_observe_valid_query() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("dto")?;
//     cmd.arg("observe");
//     cmd.arg("-q");
//     cmd.arg("SELECT * FROM stuff");
//     cmd.assert()
//         //.failure()
//         .stdout(predicates::str::contains("a valid SQL"));
//     Ok(())
// }

#[test]
fn test_observe_invalid_query() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("observe");
    cmd.arg("-q");
    cmd.arg("SEL * FROM stuff");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("not a valid SQL query"));
    Ok(())
}

#[test]
fn test_default_configuration_write() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("configure");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("Configuration file exists at"));
    Ok(())
}

#[test]
fn test_presence_local() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("presence");
    cmd.arg("-s");
    cmd.arg("local");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("isConnectedToDittoCloud"));
    Ok(())
}

#[test]
fn test_utils_storage() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("utils");
    cmd.arg("-s");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("size_in_bytes"));
    Ok(())
}

#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("help");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("Interact with a Ditto database."));
    Ok(())
}

#[test]
fn test_invalid_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dto")?;
    cmd.arg("fred");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("unrecognized subcommand"));
    Ok(())
}
