# discord-emoji

A crate for parsing emojis from Discord.

## Examples
Find the unicode surrogates for a given emoji name
```rust
fn main() {
    let emoji = discord_emoji::lookup("tm").unwrap();
    assert_eq!(emoji, "™️");
}
```

### Emoji list
The `discord_emojis.json` file is extracted from the Discord client source code.
Compared to other emoji lists it contains a number of different entries that map to a
single emoji as well as a number of recent Unicode additions that may not be widely included in fonts.