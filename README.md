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

#### Finding the emoji list
The easiest way of finding the emoji list in the discord client is to open `https://discord.com/channels/@me`
in Chrome (Firefox search does not seem to work), open dev-tools, reload the page, open the global search with
`ctrl+shift+f` and search for an emoji (either the actual Unicode surrogate, or a unique name like `muscle_tone`).
The json object may be embedded in a js file, in which case it should just be manually extracted and formatted with jq.