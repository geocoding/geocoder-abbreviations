# Token Files

Language specific token abbreviation files used in conjunction with the `--tokens` flag in `map` mode.

Files are separated by langauge code.

Did I miss a token? Please submit a PR! It will be gladly accepted.

## Current Language Files

| Code | Language |
| ---- | -------- |
| EN   | English  |
| FR   | French   |

## Creating a new token file

Each token file is an array with a sub array of equivalent tokens.

For example:

```
[
    [ 'street', 'str', 'st' ]
]
```

Would be a potential entry for an english language file.
