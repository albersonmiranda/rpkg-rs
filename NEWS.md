# rpkg

## rpkg 0.3.2

### BUG FIXES

* The personal library fix in 0.3.1 was applied only when installing new packages.
  Now it also affects updating, when `--update` flag is used.

## rpkg 0.3.1

### BREAKING CHANGES

* Removed the `--non-interactive` flag. Mirror selection now auto-selects when
  stdin is not a terminal (e.g. in CI/scripting), without needing an explicit
  flag.

### BUG FIXES

* When the default R library (e.g. `/usr/lib64/R/library`) is not writable,
  `rpkg` now automatically detects this and falls back to the user's personal
  library (`R_LIBS_USER`), creating it if necessary. This replicates the
  behavior of an interactive R session where the user is prompted to use a
  personal library. Previously, the command would fail with
  `'lib = "..." is not writable'` on systems where the user doesn't have write
  access to the system library.

## rpkg 0.3.0

### NEW FEATURES

* Added `--update` flag to update all installed packages before installing new
  ones. Can be used standalone (`rpkg --update`) or combined with package
  installation (`rpkg ggplot2 --update`).
* Packages argument is no longer required, allowing `rpkg --update` to run on
  its own.

## rpkg 0.2.0

### NEW FEATURES

* Added `--url <URL>` option to specify additional custom repository URLs
  (e.g. R-Universe). Custom URLs take precedence over the CRAN mirror
  (`repos = c(<URL>, <CRAN mirror>)`).

## rpkg 0.1.0

### NEW FEATURES

* CRAN mirror selection by country with fuzzy matching (`-c` / `--country`).
  When multiple mirrors match, an interactive numbered selection prompt is shown.
* Install packages from git sources via explicit flags: `--github`,
  `--gitlab`, `--bitbucket`, `--codeberg` (accepts `OWNER/REPO` format).
* Install to a specific library path with `-l` / `--library`.

### Other changes

* Refactored codebase into `cli` and `cran` modules.
* Renamed package from `rip` to `rpkg`.
* Added `strsim` dependency for fuzzy matching.

## rpkg 0.0.1

* Initial release. Basic CLI to install one or more R packages from CRAN via
  `Rscript`, using `https://cloud.r-project.org/` as the default mirror.
