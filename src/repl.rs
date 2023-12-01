use anyhow::{Context, Result};
use dittolive_ditto::ditto::Ditto;
use regex::Regex;

use crate::presence;
use crate::query;
use crate::subscription;

pub fn parse(line_input: String, ditto: &Ditto) -> Result<()> {
    match parse_command(line_input) {
        Ok(response) => match response.0.as_str() {
            "subscribe" => {
                subscription::subscribe(response.1, ditto);
            }
            "query" => {
                query::query(response.1, ditto);
            }
            "presence" => {
                presence::presence(response.1, ditto);
            }
            "help" => {
                print_help();
            }
            "?" => {
                print_help();
            }
            &_ => {
                print_help();
            }
        },
        Err(e) => {
            println!("err: {}", e);
            print_help();
            return Err(e);
        }
    };
    Ok(())
}

pub fn print_help() {
    println!("Available commands: subscribe, query, presence - all require an argument");
}

pub fn parse_command(input: String) -> Result<(String, String)> {
    let command = input.split_once(" ").context("Invalid command.")?;
    let command_regex = Regex::new(r#"'(?<words>[^']*)'"#).unwrap();
    let query = command_regex
        .captures(command.1)
        .context("invalid arg - need single quotes?")?;

    Ok((command.0.to_string(), query["words"].to_string()))
}

#[cfg(test)]
mod tests {
    use crate::repl::parse_command;
    #[test]
    fn parse_valid_input() {
        let input = "command 'SELECT * FROM cars'".to_string();
        let command_query = parse_command(input).unwrap();
        assert_eq!(command_query.0, "command");
        assert_eq!(command_query.1, "SELECT * FROM cars")
    }
}
