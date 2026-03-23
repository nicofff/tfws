# tfws — Terraform Workspace Selector

Tired of typing `terraform workspace select <long-workspace-name>`? `tfws` runs `terraform workspace list`, lets you pick with arrow keys, and switches instantly.

## Usage

Run `tfws` in any directory with an initialized Terraform project:

```
$ tfws
  default
> staging
  production
```

- `↑` / `↓` or `k` / `j` — move selection
- `Enter` — switch to workspace
- `Esc` / `q` — cancel

The current workspace is pre-selected.

## Install

```sh
cargo install tfws
```

## How it works

`tfws` writes the selected workspace name to `.terraform/environment`, which is the same file Terraform uses internally when you run `terraform workspace select`. No Terraform state is modified.

## Requirements

- Terraform must be installed and on your `$PATH`
- Must be run from a directory where `terraform init` has been run
