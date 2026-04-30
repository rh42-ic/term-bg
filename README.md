# term-bg

A fast, zero-dependency (only `libc`) CLI tool to detect the terminal's background color (dark or light) or extract its RGB/Luma values. 

The background detection logic is extracted from [Yazi](https://github.com/sxyazi/yazi), fully optimized for speed and binary size to be seamlessly integrated into scripts.

## Features

- **Extreme Speed**: Bypasses heavy TUI libraries or async runtimes. Uses direct `/dev/tty` syscalls via `libc` and raw terminal mode `termios`.
- **Zero-Cost Math**: Computes the Luma value using the BT.709 standard formula (`Y ≈ 0.2126 R + 0.7152 G + 0.0722 B`) exclusively through integer bit-shifting for maximum performance.
- **Fail-Safe**: Includes a strict configurable timeout (default 50ms). In environments where OSC 11 queries are unsupported or hanging, `term-bg` will safely exit with a default response and a `1` exit code, never hanging your scripts.

## Installation

You can compile it directly with Cargo. The `Cargo.toml` is already pre-configured to optimize for binary size (`opt-level = "z"`, `lto = true`, `strip = true`).

```bash
cargo build --release
cp target/release/term-bg ~/.local/bin/
```

## Usage

```bash
term-bg [-d|-r|-l] [-t <ms>]
```

### Options

| Flag | Description | Success Output | Timeout / Failure Output | Exit Code |
|------|-------------|----------------|--------------------------|-----------|
| `-d` | **[Default]** Dark/Light mode | `dark` or `light` | `dark` | 0 (Success) / 1 (Failure) |
| `-r` | RGB Hex format | e.g., `#1E1E2E` | `0` | 0 (Success) / 1 (Failure) |
| `-l` | Luma value | Integer `0-255` | `0` | 0 (Success) / 1 (Failure) |
| `-t` | Timeout in ms | (No output) | (No output) | N/A (Default: 500ms) |

### Examples

**1. Basic Theme Detection**
Most common use case. Easily assign a variable based on the output.

```bash
THEME=$(term-bg)
if [ "$THEME" = "light" ]; then
    echo "Terminal is light!"
else
    echo "Terminal is dark!"
fi
```

**2. Getting Raw RGB**
```bash
$ term-bg -r
#1E1E2E
```

**3. Adjusting the Timeout**
If you are over a slow SSH connection, you might want to increase the wait time to 200ms.
```bash
$ term-bg -d -t 200
dark
```

## How It Works

1. Saves current `termios` state.
2. Enters raw mode (`ECHO` and `ICANON` disabled).
3. Sends the standard OSC 11 query: `\x1b]11;?\x07`
4. Uses `select` to poll for the response (usually formatted as `\x1b]11;rgb:RRRR/GGGG/BBBB\x07`) within the timeout window.
5. Parses the RGB components.
6. Calculates the Luma using the integer-optimized BT.709 formula: `(R*218 + G*732 + B*74 + 512) >> 10`.
7. Checks if the Luma crosses the `153` threshold to determine `light` or `dark`.
8. Restores the exact `termios` state and exits.