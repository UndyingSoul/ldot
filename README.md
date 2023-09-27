# LDOT - Local Development Orchestration Tool

![LDOT Logo](media/ldot-logo.png)

LDOT is a command-line tool designed to help DevOps Engineers manage local development environments efficiently. It provides a set of commands to streamline the setup, configuration, and execution of development stacks, making local development workflows smoother and more productive.

## Table of Contents

- [LDOT - Local Development Orchestration Tool](#ldot---local-development-orchestration-tool)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
    - [Option 1: Use Pre-built Binaries](#option-1-use-pre-built-binaries)
    - [Option 1: Build from Source](#option-1-build-from-source)
  - [Usage](#usage)
    - [Validating Configuration](#validating-configuration)
    - [Generating Configuration](#generating-configuration)
    - [Loading Configuration](#loading-configuration)
    - [Unloading Configuration](#unloading-configuration)
    - [Configuring LDOT](#configuring-ldot)
      - [List Configurations](#list-configurations)
      - [Set Default Stack](#set-default-stack)
      - [Regenerate Configuration](#regenerate-configuration)
    - [Executing Stack Commands](#executing-stack-commands)
    - [LDOT Build Details](#ldot-build-details)
    - [Executing Scripts](#executing-scripts)
  - [License](#license)

## Installation

To install LDOT, you have two options:

### Option 1: Use Pre-built Binaries

Pre-built binaries for Windows, Linux, and macOS are available in the [Releases section](https://github.com/your-username/ldot/releases) of the LDOT GitHub repository. Follow these steps to install LDOT using pre-built binaries:

1. Visit the [Releases section](https://github.com/your-username/ldot/releases) of the LDOT GitHub repository.

2. Download the zipped binary for your operating system (e.g., `ldot-x86_64-pc-windows-msvc.zip` for Windows, `ldot-x86_64-unknown-linux-gnu.tar.gz` for Linux, or `ldot-x86_64-apple-darwin.tar.gz` for macOS).

3. Extract the contents of the zipped archive file using your favorite tool (mine is [7-zip](https://www.7-zip.org/)).

3. Place the unzipped binary in a directory that is included in your system's PATH.

4. Open a terminal or command prompt and run LDOT by entering `ldot` followed by the desired command.

This option provides a quick way to get started with LDOT without the need to build it from source.

### Option 1: Build from Source

Be sure you have first followed the instructions to get [Rust installed on your machine](https://www.rust-lang.org/tools/install).

1. Clone the LDOT repository to your local machine:

   ```shell
   git clone https://github.com/UndyingSoul/ldot.git
   ```

2. Change into the LDOT directory:

   ```shell
   cd ldot
   ```

3. Build the LDOT binary using Cargo:

   ```shell
   cargo build --release
   ```

4. Once the build process is complete, you can find the LDOT binary in the `target/release` directory.

5. You can either add the binary to your system's PATH or run it directly from the `target/release` directory.

## Usage

LDOT provides a range of commands and options to manage your local development environment. Below are the available commands:

### Validating Configuration

```shell
ldot validate [filename]
```

- Use this command to validate a configuration file.
- Replace `[filename]` with the path to your configuration file, default is `ldot_stack.json`.
- Every command in the stack scripts or project stage is executed using the default shell.

### Generating Configuration

```shell
ldot generate
```

- Use this command to generate a new LDOT stack configuration file using the configuration wizard.
- Projects, Scripts, Stages, and commands are not configurable using this utility. Edit the file directly to configure the stack.
- For examples of this file, check [the example](data/examples/ldot_stack.json).

### Loading Configuration

```shell
ldot load [filename]
```

- Use this command to load a specific configuration file.
- Replace `[filename]` with the path to your configuration file, default is `ldot_stack.json`.

### Unloading Configuration

```shell
ldot unload [filename]
```

- Use this command to unload a specific configuration file.
- Replace `[filename]` with the path to your configuration file, default is `ldot_stack.json`.

### Configuring LDOT

LDOT allows you to configure various aspects of the tool. Below are the available configuration commands:

#### List Configurations

```shell
ldot config list
```

- Use this command to list all current LDOT configurations.

#### Set Default Stack

```shell
ldot config default <stack_name>
```

- Use this command to set the default development stack.
- Replace `<stack_name>` with the name of the stack you want to set as default.

#### Regenerate Configuration

```shell
ldot config regenerate
```

- Use this command to regenerate the LDOT configuration file in case of broken configs.

### Executing Stack Commands

```shell
ldot execute [stack_name] <project_name> <stage_name>
```

- Use this command to execute commands for a specific stack, project, and stage.
- If no arguments are provided, LDOT will use the default stack and prompt you for project and stage names.

### LDOT Build Details

```shell
ldot version
```

- Use this command to display LDOT build details, including program name, description, author, homepage, repository, and version.

### Executing Scripts

```shell
ldot script [stack_name] <script_name>
```

- Use this command to execute a script associated with a specific stack.
- If no stack name is provided, LDOT will use the default stack.
- Replace `<script_name>` with the name of the script you want to execute.
<!-- Still working on
## Contributing

We welcome contributions from the community. If you'd like to contribute to LDOT, please follow our [Contributing Guidelines](CONTRIBUTING.md). -->

## License

LDOT is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute it in accordance with the terms of this license.