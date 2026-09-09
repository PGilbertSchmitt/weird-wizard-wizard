# Weird Wizard Wizard

A "Wizard" for creating characters for the Shadow of the Weird Wizard TTRPG

## Design Choices

This application was designed to contain as little information about the Shadow of the Weird Wizard game as possible. While 100% separation would probably be possible in an ideastic sort of way, it would also be 100% unfeasible. The fact is, a tiny bit of the game does have to seep in, such as:

- The fact that there are Novice, Expert, and Master paths, and you choose them at levels 1, 3, and 7 respectively.
- The fact that ancestry paths force you to take their respective ancestry.
- A level can alter your health, natural/armored defence, bonuse damage, speed, and traditions and spells.
- The specific nature of how player health and player damage affect each other.

However, these are all implemented in the application generically. The actual information about what paths, talents, and professions _are_ is excluded and must be manually imported by the player.

## Design Choices (Literally)

I am a web developer (hence why I used ReactJS to create a desktop application). What I am _not_ is a designer. When someone gives me a wireframe, I can implement it flawlessly. I cannot, however, create my own good-looking wireframes. I have a symbiotic relationship with the product teams that I've worked with, and without them, I make things that look _okay_. To simplify the burden of design, I used a styling library called [neobrutalism](https://www.neobrutalism.dev). Is it good? Is it bad? That's for individual people to decide for themselves. I chose it because it was minimalistic and dead simple.

And yet I still feel like it could be done better. If you feel the same way, please reach out to me with wireframes or mockups and I'll happily consider them.

NOTE: If you find anything that looks really out of place, there's a possibility that this is caused by a difference in webview. I'm developing this on Linux using whatever default webview comes shipped with Ubuntu. If you're on Windows, Mac, or some esoteric Linux distro, there's a chance that your bundled webview might implement (or not implement) some CSS magic differently from mine. If you discover anything that looks super funky and buggy, please open an issue on the Github repo and I'll try to address it.

## How to develop

To run the app, run:

```bash
$ pnpm tauri dev
```

SQLX can check queries at compile time, but it requires having an up-to-date database that it can access. Since I'm using Sqlite, it's trivial to set up. It's been configured to look for `./src-tauri/test.db`. All that needs to be done is to run this from the `./src-tauri/` directory:

```bash
$ sqlite3 test.db < db/migrations/*.sql
```

The DB doesn't need to contain any actual data, it just needs the schemas to be up-to-date. If you ever update the schema by creating new migrations, run them against the `test.db` as well (also from the `./src-tauri/)` directory:

### TS Types

This app used a library called [ts-rs](https://crates.io/crates/ts-rs) which converts types in Rust into types in TypeScript. This is amazing when I have data passing the boundary (and this is a Tauri app, so that's kinda the point). When a struct or enum is added or updated, if it has `#[derive(TS)]` and `#[ts(export, ...)]` macros, then it can be automatically converted by running:

```bash
$ pnpm build-types
```

The `#[ts(export, ...)]` macro defines the path to the type file in the TypeScript code where the types will be ported.
