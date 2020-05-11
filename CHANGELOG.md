# CHANGELOG

## Emoji Cheatsheet
- :pencil2: doc updates
- :bug: when fixing a bug
- :rocket: when making general improvements
- :white_check_mark: when adding tests
- :arrow_up: when upgrading dependencies
- :tada: when adding new features

# Version History

## v4.2.0

- :rocket: Add German variations of `on the`, `at the`, and `in the`, along with variations of popular street names ending in `str` or `straße`. Add in regex to convert street names ending in `strasse` or `str` to end in `straße` to match OSM network street names.

## v4.1.8

- :rocket: Add add `type: determiner` to signify meaning 'of' & update tokens for Spanish, French, and Italian.

## v4.1.7

- :tada: Add Czech tokens

## v4.1.6

- :rocket: Add numbers, ordinals, `type: way` property to Russian tokens

## v4.1.5

- :rocket: Add optional reduceRelevance property

## v4.1.4

- :rocket: Add `Farm-To-Market` & `Ranch-To-Market`

## v4.1.3

- :rocket: Add `type: way` property to `place` token in English

## v4.1.2

- :rocket: Add `type: way` property to `road` token in English

## v4.1.1

- :rocket: Add `Clone`, `Serialize`, and `Deserialize` traits to `TokenType` enum

## v4.1.0

- :rocket: Make fields in `Token` struct public, add `TokenType` enum

## v4.0.0

- :tada: Add support for importing abbreviations as Rust structs

## v3.1.0

- Add several new replacements focused on and scoped to US addresses, designed to strip unit numbers from address strings.

## v3.0.0

- Change format of geocoder-abbrevations contents, and add mechanisms for querying data in new format; see current README.md for details

## v2.4.0

- :tada: Add Russian tokens

## v2.3.2

- Add CHANGELOG.md
