# About subcommand `list_and_set`:
It prints all list of clipboard history entries (with removed spaces, for easy to view or use in fuzzy finders, like `fzf`)
List format:
[entry_index] [entry_formatted_text]

... and then wait for input.
Input should be a number, index of entry that we want to get.
After program got input, it will print (this time to stderr (this was done for easy to use in scripts)) original text of entry, so we can pipe it to `xclip`

Also see `fzf.sh`, example of usage `rclip` with `fzf`
