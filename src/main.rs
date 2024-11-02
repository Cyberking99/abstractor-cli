use inquire::Select;
use std::fs;
use std::process::{Command, exit};

fn main() {
    println!("Welcome to the Account Abstraction Project Bootstrapper!");

    let options = vec!["Foundry Non-Diamond", "Foundry Diamond"];
    let framework = Select::new("Choose your framework:", options)
        .prompt()
        .expect("Failed to read input");

    match framework {
        "Foundry Non-Diamond" => setup_foundry(),
        "Foundry Diamond" => setup_foundry_diamond(),
        _ => {
            eprintln!("Invalid selection.");
            exit(1);
        }
    }
}

fn setup_foundry() {
    println!("Setting up an Account Abstraction project using Foundry (Non Diamond Implementation)...");

    fs::create_dir_all("account-abstraction-foundry/src").expect("Failed to create project directory");

    fs::write("account-abstraction-foundry/src/AccountAbstraction.sol", get_sample_contract())
        .expect("Failed to write sample contract file");

    Command::new("forge")
        .args(&["init", "account-abstraction-foundry", "--force"])
        .status()
        .expect("Failed to initialize Foundry project");

    println!("Foundry setup completed!");
}

fn setup_foundry_diamond() {
    println!("Setting up an Account Abstraction project using Foundry (Diamond Implementation)...");

    fs::create_dir_all("account-abstraction-hardhat/contracts").expect("Failed to create project directory");

    fs::write("account-abstraction-hardhat/contracts/AccountAbstraction.sol", get_sample_contract())
        .expect("Failed to write sample contract file");

    Command::new("npx")
        .args(&["hardhat", "init", "account-abstraction-hardhat"])
        .status()
        .expect("Failed to initialize Hardhat project");

    println!("Hardhat setup completed!");
}

fn get_sample_contract() -> &'static str {
    r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract AccountAbstraction {
    address public owner;

    constructor() {
        owner = msg.sender;
    }

    function executeTransaction(address to, uint256 value, bytes calldata data) external {
        require(msg.sender == owner, "Only owner can execute transactions");
        (bool success, ) = to.call{value: value}(data);
        require(success, "Transaction failed");
    }
}
"#
}
