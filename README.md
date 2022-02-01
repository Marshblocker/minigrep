# rgrep
grep in rust (WIP)

## Features Checklist

- [ ] Search for all occurrences of 'STRING' in a file and print the line where
     it occur along with the line number. The command signature should be
     `rgrep FILEPATH STRING`. The 'STRING' is not a regex pattern for now and
     the 'FILEPATH' is just be an absolute filepath for now.
     - [ ] Extract command arguments in the CLI.
     - [ ] Read content of the file directed by 'FILEPATH'.
     - [ ] Perform grep on the content.
     - [ ] Output matched lines.
- [ ] Add support to Linux (Currently supports Windows only).



