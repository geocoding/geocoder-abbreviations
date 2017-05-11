# geocoder-abbreviations

Geocoder Abbreviations divided into language groups. These are lossy tokens that are useful for geocoding.

They include english abbreviations like:

`street => st`
`drive => dr`

And non-english language codes like the indonesian

`jalan => jln => jl`

## Usage

index.js exposes a single function with an optional `lang` arg or if null returns a map of all the tokens seprated by language.

See the index.js JSDoc for more details
