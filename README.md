# sonora-rs

![GitHub top language](https://img.shields.io/github/languages/top/d-mckee/sonora-rs?color=orange&style=flat&logo=rust)![GitHub last commit](https://img.shields.io/github/last-commit/d-mckee/sonora-rs?style=flat)![GitHub repo size](https://img.shields.io/github/repo-size/d-mckee/sonora-rs?style=flat) 

A custom Discord bot, written in Rust, using serenity-rs as a wrapper for the Discord API. 

## Description

Discord bot currently being written for the SonoraBuild Discord server, with plans to release more publicly as features are polished.

## Docs

- [Setup](docs/setup/README.md)
- [Commands](docs/commands/README.md) 

## Planned Features

- Robust "reminder" command (`!remindme, !remind, !rm`)
  - Complex combinations of times (`4h2m34s, 2y4mo5d, etc.`)
  - Options to remind multiple users, multiple user groups, or `@everyone/@here` (privileged to specific usergroups based on the reminder)
- Dice rolling command (`!diceroll, !roll, !dice, !r, !d`)
  - Rolling complex combinations of dice of various sizes, intended to make interpretation of user input as seamless as possible
  - Save specific dice roll equations as "commands" (i.e., `!r save 4d6 + 10 charstats`
- Twitter embed command (`!twitter, !tt`)
  - Subcommand to expand a twitter thread (`!twitter thread #`), variable number of posts expanded, max expansions (to disallow spamming the server with a thread)
- Google embed command (`!google, !search, !go`)
- Music player command (`!music, !play, !p`)
  - Allow music streaming to playback with audio effects (variable bitrate, bitcrushing, reverb, etc.)
- Twitch integration commands (`!twitch, !tw`)
  - Enabling notifications when specific channels go live
- Pterodactyl control panel server status tracking
  - Server status command (`!server, !status, !st`)
  - Server status logging (`!status on`) to a specific channel
- Key giveaway command (`!giveaway, !enter #`)
  - User can DM bot with key, giving the key a specific name, then `!giveaway <keyname>` in the server (if enabled) to start a giveaway
  - When a user wins the giveaway, the bot automatically sends them the key
- Bot configuration
  - Configuration of messages by the bot (help commands, style of responses, etc.)
  - Configuration of default bot options (prefixes, user groups allowed to use specific commands, etc.)
- Various smaller utilities
  - Timezone conversion (`!timezone, !tz`)
- Various fun commands
  - Command to "microwave" a user by making their avatar a custom emoji temporarily, then placing them in the middle of the 3x3 microwave icons 
  - ASCII art!
    - Pulling ASCII art from a database (either internal or an external one, depending on availability)
    - Generating ASCII from given image [either by](https://rapidapi.com/orthosie/api/ascii-art/endpoints) [API](http://artii.herokuapp.com/) or [using some of the](https://github.com/edelsonc/asciify) [various projects in Rust](https://github.com/lnenad/image-to-ascii) 
  - Rude responses mode
- Messages/responses in the style of a specific user based off their past chat logs using the GPT-2 language model by OpenAI. (Tentative, may be released as an addon module)

