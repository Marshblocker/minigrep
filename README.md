# rgrep
grep in rust (WIP)

## Features Checklist

- [ ] Search for all occurrences of PATTERN in a file and print the line where
      it occur along with the line number. The command signature should be
      `rgrep FILEPATH 'PATTERN'`. The PATTERN is just a substring for now 
      (not a regex pattern) and the FILEPATH is just be an absolute filepath 
      for now. The single quotes on PATTERN is needed for now.
     - [X] Extract command arguments in the CLI.
     - [ ] Read content of the file directed by FILEPATH.
     - [ ] Perform grep on the content.
     - [ ] Output matched lines.
- [ ] Add support to Linux (Currently supports Windows only).



