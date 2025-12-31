# Elvis

**Elvis** is a CLI tool that lets you preview file-system commands like `touch`, `mv`, and `rm` by performing **dry-run** of your intended action, showing you exactly what will happen to your files without actually touching them.

## Installation

```bash
brew install iamtanong/elvis/elvis
```

Other installation options see: [Release](https://github.com/iamtanong/elvis/releases)

## Usage

| Command | Action                  | Example                           |
| ------- | ----------------------- | --------------------------------- |
| `touch` | Preview file creation   | `elvis touch new_file.txt`        |
| `mv`    | Preview moving/renaming | `elvis mv ./old_dir/* ./new_dir/` |
| `rm`    | Preview deletions       | `elvis rm *.log`                  |

### Flags

- `-y, --yes`: Skip the preview and **execute** the command immediately.
- `--summary-only`: Don't list every file; just show a high-level summary of changes.
- `-m, --max-entries <N>`: Limit the preview to N number of files (useful for massive directories).
- `--no-color`: Disable syntax highlighting in the output.

## Development

Elvis is built using [Clap](https://github.com/clap-rs/clap) framework for a robust and fast CLI experience.

**To run locally:**

```bash
cargo run -- rm ./test_folder/*.txt
```

## License

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.
