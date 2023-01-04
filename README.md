# Unicode String Shortener

The idea of this project is to compress text in terms of bytes or characters used while maintaining its human readability. Using this program you can enter more text than intended into a limited-size form field.

For the intents of this project, anything that looks close enough to a latin letter to be readable is considered acceptable, even if it may have an entirely different meaning.

Here is an example of program functionality.
```
Enter string to shorten: aether
Input:                         aether               (6)
Shortest in bytes:             æther                (6)
Shortest in characters used:   æᵺer                 (4)
```

### Structure

A human-readable list of shortenings used is available in `map.tsv`. The columns in that file are: unicode codepoint, unicode character, ascii equivalent strings. There can be more than one ascii equivalent string for a single unicode character, columns are added for each additional one. However, no two unicode characters can be translated to the same string (the program checks for this and will error).

To update the computer-readable list in `map.bincode`, delete the `map.bincode` file and run the program. A new bincode will be produced from `map.tsv`.

## Usage

This program is still a work in progress, but core functionality works and it is at a stage where you can try it out by following the development instructions.

I envision a final product available on a website, but it is not yet certain if I will get that far.

## Installation/Development

### Option 1: Compile locally
This assumes you have Rust installed.

1. Clone this repo. `git clone https://github.com/michaelnoguera/unicode-string-shortener`
2. Run `cargo build` to install dependencies and compile.  This will take a long time the first time you run it on this project.
3. Run `cargo run` to run the program.

### Option 2: Clone into a local Docker devcontainer with VS Code
If you use VS Code and have Docker installed, you can clone this repo into a devcontainer that reproduces my build.

1. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers), if not already installed.

2. Then paste this address into your browser's address bar (Github disables vscode:// links)

   `vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/michaelnoguera/unicode-string-shortener`

   If you don't feel like clicking my strange-looking links, you can alternatively Open VS Code, open the Command Pallete (Ctrl/Cmd + Shift + P), select "Dev Containers: Clone Repository in Container Volume" and specify this repo `https://github.com/michaelnoguera/unicode-string-shortener`.

3. Wait for the container to set itself up. `cargo build` will run automatically to configure dependencies.

4. Run the program with `cargo run`.

### Option 3: Clone into a cloud devcontainer with Github Codespaces
To open this repository in Codespaces, use the button below.

> **Warning**
> 
> Bad news: This could cost money.
> 
> Good news: You get 60 _hours_ free _per month_ (on the least powerful option), so if you are just poking around and want to check out this project for a minute or two, it will almost certainly be free.

1. Click this link

    [![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&ref=main&repo=583980246)

2. Wait for the container to set itself up (less than 5 min). `cargo build` runs automatically to configure dependencies.

3. Run the program with `cargo run`.
