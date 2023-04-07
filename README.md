# gitsum

`gitsum` is a tool for summarizing github repositories using gpt.

# Installation
For now, clone this repository and run `cargo build`

# Usage
`gitsum` allows you to summarize an entire repository (useful in cases where there is no README), folders, or files. 

Configuration is provided through CLI flags, two of which can alternatively be set as environment variables:
- `GITHUB_KEY`
- `OPEN_AI_KEY`

> **Note**
>
> You should only change the chat configuration if you know what you're doing. I'm not liable
> for any wonky responses if you change the default chat config.

```shell
Usage: gitsum sum [OPTIONS] --username <USERNAME> --repo <REPO> --branch <BRANCH>

Options:
  -u, --username <USERNAME>
          The username of the repository owner

  -r, --repo <REPO>
          The name of the repository

  -b, --branch <BRANCH>
          The branch of the repository

  -g, --git-key <GIT_KEY>
          Your github api key

  -o, --open-ai-key <OPEN_AI_KEY>
          Your openai api key

  -f, --folder <FOLDER>
          The folder to sumamrize

  -s, --file <FILE>
          The file to save the summaries to

  -m, --max-tokens <MAX_TOKENS>
          The maximum number of tokens to generate in the chat completion
          
          [default: 4096]

  -x, --temperature <TEMPERATURE>
          What sampling temperature to use, between 0 and 2
          
          [default: 0.7]

  -t, --top-p <TOP_P>
          An alternative to sampling with temperature, called nucleus sampling,
          
          The model considers the results of the tokens with top_p probability mass. It it recommended to alter this or temperature but not both.
          
          [default: 1.0]

  -p, --presence-penalty <PRESENCE_PENALTY>
          Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far
          
          [default: 0.0]

  -q, --frequency-penalty <FREQUENCY_PENALTY>
          Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far
          
          [default: 0.0]

  -h, --help
          Print help (see a summary with '-h')
```

# How it works
When summarizing...
- Files
  - `gitsum` will simply make a summarize the file, returning an error message if the file is too large
- Folders
  - `gitsum` will traverse the directory, summarizing each file in the specified folder, while additionally providing an overarching summary of the entire folder.
- Respositories
  - `gitsum` will traverse the entire repository summarizing each folder as explained above, while additionally providing an overarching summary of the entire repository. 

## Example

# To-Do
- [ ] Handle large files 