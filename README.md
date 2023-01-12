# Unicode String Shortener

> **Try it out!** Live at [https://noguera.dev/unicode-string-shortener](https://noguera.dev/unicode-string-shortener).

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

### Online

Available online at [https://noguera.dev/unicode-string-shortener](https://noguera.dev/unicode-string-shortener). The program is compiled to webassembly and runs entirely client-side in your browser.

### Command Line Interface

The `program` member of this workspace compiles to an executable program, `unishorten`, which by default takes a single argument that is the string to shorten. Alternatively, `unishorten -i` will run in interactive mode and prompt for input.

```
Shortens ascii strings by substituting unicode characters that look like more than one ascii character

Usage: unishorten [OPTIONS] [input]

Arguments:
  [input]  string to shorten

Options:
  -i, --interactive  interactive mode
  -h, --help         Print help information
  -V, --version      Print version information
```

### Crate

You can also import the `program` folder of this repo as a library. This can be done with the command:

```terminal
cargo add --git https://github.com/michaelnoguera/unicode-string-shortener unishorten
```

After running the command, your `cargo.toml` file should contain:
```toml
[dependencies]
unishorten = { git = "https://github.com/michaelnoguera/unicode-string-shortener", version = "0.1.0" }
```

Most users will want to instantiate a `StringShortener` rather than interacting with the various utility functions in the library.

```rust
use unishorten::StringShortener;

let Shortener = StringShortener::new();
let out = Shortener.shorten_by_chars(/* reference to input string */);
```

To customize the list of mappings, clone this repository, edit `map.tsv`, then delete `map.bincode` and run the command line program again. The bincode file will be regenerated from whatever the tsv file contains.

---

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
