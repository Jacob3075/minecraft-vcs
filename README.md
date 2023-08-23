# minecraft-vcs

Creates and manages saves of minecraft world on google drive. Can be used to manage backups or share worlds with others.

## Usage

```
Usage: minecraft-vcs <COMMAND>

Commands:
  init  initialize folder structure on remote
  pull  get latest version of world from remote
  push  push latest version of world to remote
  logs  get details for saved worlds on remote
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Sample config file

Place config file (`config.toml`) in the same directory as the executable. Path values in config file are relative
to the executable.

```toml
world_name = ""
local_location = ""
remote_root_id = ""

[credentials]
secrets_location = "some/path"
cache_location = "some/path"
```

## File structure

```
- minecraft-vcs
- configs.toml
- data/
   - credentials.json
   - tokencache.json
```
