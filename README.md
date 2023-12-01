# Rust CRUD Project with Actix

This is a project aimed at creating a simple CRUD for users in Rust using the Actix framework.

## Project Structure

- `main.rs`: This is the main file to start the Actix server.

- `test/`: In this folder, you'll find the test script using Curl to verify the API's functionality.

- `modules/`: This directory contains route/operations folders for the CRUD.

## How to Use

1. Make sure you have Rust installed on your machine.

2. Clone this repository.

3. Navigate to the project folder.

Then proceed to one of the 3 environments.

_If your API still won't work, changing the Host bind address from `'0.0.0.0'` to `'127.0.0.1'` may resolve the problem._

### Devel

1. Run `cargo run` to start the Actix server.

2. Use the test script in `test/` to check the API's functionality.

### Build

1. Run `cargo build --release`

2. Run `./target/release/actix-basic-api` to initialize the API.

### Docker image

To use the Docker Image you must change the Host API address to work properly.

2. Build the docker image with `docker build -t actix-basic-api .`

3. Run the container with `docker run -p 8080:8080 actix-basic-api`

And it's done!

##

<div id="header" align="center">
  <a href="https://github.com/HeinzDev/">
    <img src="https://i.imgur.com/RtsYtRt.png" width="100"/>
  </a>
  <a href="https://github.com/HeinzDev/">
    <h3>HeinzDev</h3>  
  </a>
</div>
