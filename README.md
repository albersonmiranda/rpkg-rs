# rpkg

`rpkg` (R Package) is a lightweight CLI wrapper around `Rscript` written in [Rust](https://www.rust-lang.org/) that simplifies installing R packages through terminal commands.

## Features

- Install single or multiple packages in one command
- Default CRAN mirror set to `https://cloud.r-project.org/`
- Optionally select CRAN mirrors by country with fuzzy matching
- Optionally set custom URLs that takes precedence over CRAN url (e.g., R-Universe)
- Interactive numbered repository selection when there are multiple country matches
- Auto-selection of the best match when stdin is not a terminal (e.g. CI/scripts)
- Install from explicit git source flags (Github, Gitlab, Codeberg, Bitbucket)
- Optionally install to a library path with `-l` / `--library`
- Automatic fallback to personal library when the system library is not writable

## Installation

```bash
# windows
winget install AlbersonMiranda.rpkg

# macOS
brew install albersonmiranda/rpkg-rs/rpkg

# from source (requires Rust toolchain)
cargo install rpkg
```

## Usage

```text
rpkg [OPTIONS] <PACKAGE> [<PACKAGE> ...]
```

Examples:

```bash
# Install a single package from CRAN
rpkg ggplot2

# Install multiple CRAN packages at once
rpkg ggplot2 fio mlr3

# Select a CRAN mirror by country (fuzzy matched), then select one between matches
rpkg ggplot2 --country brazil

# Set custom URL (e.g., R-Universe)
rpkg fio --url https://albersonmiranda.r-universe.dev -c brazil

# Install into a specific library path
rpkg ggplot2 -l /path/to/R/library

# Update all installed packages before installing new ones
# (this updates all installed packages THEN installs ggplot)
rpkg ggplot2 --update

# Update + install with specific country mirror
rpkg ggplot2 --country brazil --update

# Update + install using custom library path
rpkg ggplot2 --library /path/to/R/library --update

# Only update all installed packages
rpkg --update
```

> [!IMPORTANT]
>  **Mirror resolution precedence**
>
> 1. If `--country` is set, `rpkg` retrieves CRAN mirrors through R and asks for selection when needed.
> 2. Else `rpkg` falls back to `https://cloud.r-project.org/`.
> 3. If one or many `--url <URL>` is set, then they are appended before CRAN's (`repos = c(<URL>, <URL>, ..., <CRAN mirror>)`)

```bash
# Install from GitHub source flag
rpkg --github=albersonmiranda/fio

# Mix multiple explicit git sources in one command (no flag means CRAN)
# Here, ggplot2 and dplyr are installing from CRAN (using a brazilian mirror),
# while emo is installing from Github and raven from Gitlab
rpkg ggplot2 dplyr -c brazil --github=hadley/emo --gitlab=r-packages/raven
```

> [!IMPORTANT]
> **Git source behavior**
> 1. Positional packages are always installed from CRAN.
> 2. Git installs happen only through explicit source flags (`--github`, `--gitlab`, `--bitbucket`, `--codeberg`).
> 3. Source flag values must be in `OWNER/REPO` format.
> 4. Each source flag can be repeated to install multiple repositories.

Options:

```text
-c, --country <COUNTRY>         Country query used for CRAN mirror fuzzy matching
-l, --library <LIBRARY>         Path to install package library (optional)
--url <URL>                     Additional custom repository URL (repeatable)
--update                        Update all installed packages before installing
--github <OWNER/REPO>           Install from GitHub (repeatable)
--gitlab <OWNER/REPO>           Install from GitLab (repeatable)
--bitbucket <OWNER/REPO>        Install from Bitbucket (repeatable)
--codeberg <OWNER/REPO>         Install from Codeberg (repeatable)
-h, --help                      Print help information
-V, --version                   Print version information
```

## Library path behavior

When no `--library` is specified, `rpkg` checks if the default R library (`.libPaths()[1]`) is writable. If it isn't — which is common on Linux/macOS where the system library is owned by root — `rpkg` automatically:

1. Reads the `R_LIBS_USER` environment variable (set by R, typically `~/R/<arch>/<version>/`)
2. Creates the directory if it doesn't exist
3. Prepends it to `.libPaths()` so packages install there and passes it to `update.packages()` when `--update` is used

This mirrors what R does in an interactive session when it prompts _"Would you like to use a personal library instead?"_.

## Uninstall

Follow your package manager or, if installed via Cargo,

```bash
cargo uninstall rpkg
```

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to open a PR or issue.

## Changelog

See [NEWS.md](NEWS.md) for a detailed changelog.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
