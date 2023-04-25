# A Very Poorly Written Rust Project

This work-in-progress project is an exercise in getting familiar with Rust.  
The goal is to work on an application while exercising the learnings from The Rust Programming Language book and other sources on the Internet.  

Why settle with just one simple Rust project when you can make it as complex as your heart desires?  

## Overview

This project contains: 
1. a Web application backend API written in Rust (using Axum)
2. a Web application frontend written in Rust (using Yew)
3. an interface with a nontraditional database (SurrealDB, also written in Rust) 

## Goals of the exercise

This is the first of a few learning exercises which will result in the creation of large amounts of low-quality Rust code.  
Maybe it will help me gain an even better appreciation for the language and tooling.  
Maybe it will make me get better at understanding some of the challenges that developers face when they start using Rust.  
Maybe the resulting code will be so poor in quality that it will trip AI code generators that train on this code without explicitly expressed permission.  

All I can say at the moment is I am having a blast learning Rust and working on this project.  
Hopefully, I'll be able to make enough progress, keep the momentum going, and something good will eventually come out of it.  

Also, if you are new to Rust, here is one piece of advice.  
Don't fear the Borrow Checker. Embrace it. 
Let the compiler be your guide.

## Getting Started

### Prerequisites

1. Ensure you have rust 1.60.0 installed.  
2. Install Yew prerequisites, trunk cargo crate, as well as wasm target.  
```
cargo install trunk
rustup add target wasm32-unknown-unknown
```
3. Install the `cargo-watch` crate. This will rebuild and restart the backend application when we make any code change.  
```
cargo install cargo-watch
```
4. Docker/Podman is needed with ability to run docker-compose. This is needed to run SurrealDB in docker container. Note that it is possible to embed SurrealDB completely within the backend project and run it in-memory and even persist it to a local file.  
You may try it out. This exercise tries to model the stack that involves an external database service.  
5. Make a symbolic link from `frontend/dist` folder to `backend/dist` folder. If you are unable to do so, you'll need to copy the files manually.  
6. (Optional) Install the `cargo-run-script` crate. This will allow you to run included run-scripts for convenience.  
```
cargo install cargo-run-script
```

### Launching the project

1. Copy the included `.env.example` file to `.env`.
2. Launch the SurrealDB container by running the docker-comose.yml as follows:
```
docker-compose up
```
3. Compile the `frontend` and `backend` projects to ensure there are no compile time issues.  
```
cd backend
cargo build
cd ../frontend
cargo build
```
4. Start the `frontend` application build with watch using `trunk`.
```
trunk serve
```
5. Start the `backend` application with `cargo watch`
```
cargo watch -c -q -x "run"
```
6. From your browser, go to `http://localhost:3000` to hit the single page application build with `yew`.  
Verify backend is working by going to `http://localhost:3000/api/persons`.  
Neither of these should work since the database is empty.
7. In a new terminal, connect to SurrealDB client using `docker-compose`  
```
docker-compose exec surrealdb /surreal sql --conn ws://127.0.0.1:3000
```
8. Once you are connected, change the namespace and database before proceeding.  
```
use ns test
use db test
```
9. Now create new persons records in SurrealDB. Note that only `Name` field is required for the project.  
```
CREATE persons SET name = 'Karim'
```
10. Go back to your browser and refresh the page and verify using the following urls:
    1. `https://localhost:3000/api/persons` to verify that backend API is working.
    2. `https://localhost:3000/` to verify frontend app is working 

### Run Scripts

#### Backend  
1. Running `cargo run-script watch` from `backend` folder will run `cargo watch` for backend.
2. Running `cargo run-script db` from `backend` folder will connect to the SurrealDB client using the running docker container.

#### Frontend
No `cargo run-script` command for frontend.

### Inspirations

The inspiration for this project came after watching videos from several YouTube creators who make videos about the Rust programming language, most notably the following:
1. [Code to the Moon](https://www.youtube.com/@codetothemoon)
2. [Let's Get Rusty](https://www.youtube.com/@letsgetrusty)
3. [No Boilerplate](https://www.youtube.com/@NoBoilerplate)
4. [Jeremy Chone](https://www.youtube.com/@JeremyChone)
