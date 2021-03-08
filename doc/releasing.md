```shell
# First, choose `major`, `minor`, or `patch` for the level to release
# Next, run the command in --dry-run mode
$ cargo release --dry-run --no-dev-version minor

# Then do it for real
$ cargo release --no-dev-version minor
```
