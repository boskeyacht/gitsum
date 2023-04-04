# gitsum

> **Note**
>
> `gitsum` is a WIP

`gitsum` is a tool for summarizing github repositories using gpt. Currently only works on smaller repositories, but support for repositories of any size are coming soon.

# Installation
For now, clone this repository and run `cargo build`

# Usage
Configuration is provided through CLI flags, two of which can alternatively be set as environment variables:
- `GITHUB_KEY`
- `OPEN_AI_KEY`
```shell
Usage: gitsum sum [OPTIONS] --username <USERNAME> --repo <REPO> --branch <BRANCH>

Options:
  -u, --username <USERNAME>        The username of the repository owner
  -r, --repo <REPO>                The name of the repository
  -b, --branch <BRANCH>            The branch of the repository
  -g, --git-key <GIT_KEY>          Your github api key
  -o, --open-ai-key <OPEN_AI_KEY>  Your openai api key
  -h, --help                       Print help
```

## Example