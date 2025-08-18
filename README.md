# uml-rs

## What is it

uml-rs is simply a Rust learning opportunity. I've been trying to learn Rust for a while now but never actually stick with a project until the end.
This is my attempt to remedy that.

The idea is to handle uml diagrams with code. Initially I aim to support PlantUML with ASCII rendering.
The project will be considered stable once it is able to render all of the projects's PlantUML diagrams.

## Goals

The current main goal is enough development to support the project's own diagrams.

### PlantUML Support

Currently, the focus is on a subset of component diagrams. Support for more diagram types and syntax is planned for the future.

### ASCII Rendering

As I've never dwelt into graphics programming, I believe an ASCII representation of the diagram is a good start

### CLI

The main way to interact with the project will be through a CLI for now

### Neovim Integration

Once the tool is mature enough I aim to add Neovim integration for rendering diagrams directly inside Neovim without needing a heavyweight plugin.

Once done, a simple :PlantUMLAscii command in any buffer containing PlantUML code should render a live ASCII preview.

## Architecture

As well as a Rust learning opportunity this project is also a place for me to exercise my Software Architecture Design skills

The project follows a Clean Architecture design packaged by components.
I devide the code into 4 main layers:

![uml-rs architectural layers](docs/diagrams/rendered/uml_rs_architectural_layers.png "uml-rs Architectural Layers")

As an example usage we could have:

![uml-rs architectural example](docs/diagrams/rendered/uml_rs_architectural_example.png "uml-rs Architectural Example")

Layer names and configuration may vary.

Currently I aim to have the following components:

![uml-rs components](docs/diagrams/rendered/uml_rs_components.png "uml-rs Components")

By using Cargo Workspaces we can separate each component into its own 'module'.
Most of the components are libraries with the executable binary crates being the ones that actually instantiate all objects.

The full intented architecture is as follows:

![uml-rs full architectural layout](docs/diagrams/rendered/uml_rs_full_architectural_layout.png "uml-rs Full Architectural Layout")

(Some helper classes are hidden for simplicity)
