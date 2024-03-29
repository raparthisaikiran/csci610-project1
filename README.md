# Rust Project Setup
This document outlines the steps to set up and run a Rust project on your local machine.

Prerequisites
Before you start, make sure you have the following installed on your machine:

Rust programming language (version 1.55 or higher)
Cargo package manager (should come with Rust)
Setup

Follow these steps to set up your Rust project:
* Clone the project repository: https://github.com/raparthisaikiran/csci610-project1.git

* Navigate to the project directory: cd your-project

* Build the project using Cargo: cargo build

* Run the project: cargo run

To Run shell script

Open terminala and execute following command in the parent directory: 
* chmod +X run_program.sh
* ./run_program
* It promts for user input and enter the time in seconds or 'w' to run weekly

Dependencies
If you need to add a dependency to your Rust project, update the Cargo.toml file with the dependency name and version number. Then, run cargo build to download and install the dependency.
