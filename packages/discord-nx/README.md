# discord-nx

Discord bot framework for Nexora. Build powerful Discord bots with a clean, expressive API.

## Features

- **REST API Client** - Full Discord REST API support
- **Gateway Events** - Listen to Discord gateway events
- **Embed Builder** - Rich embed creation with chaining
- **Components** - Buttons, select menus, action rows
- **Slash Commands** - Application command support
- **Permissions** - Permission checking utilities
- **Interactions** - Handle interactions and modals

## Installation

```bash
nxm install discord-nx
```

## Quick Start

```nx
import { Client, Events, Intents } from "discord-nx"

let client = new Client({
  token: process.env.DISCORD_TOKEN,
  intents: Intents.GUILDS | Intents.GUILD_MESSAGES,
})

client.on(Events.READY, (data) => {
  console.log("Logged in as " + data.user.username)
})

client.on(Events.MESSAGE_CREATE, async (message) => {
  if (message.content === "!ping") {
    await client.sendMessage(message.channel_id, "Pong!")
  }
})

await client.login()
```

## Examples

### Embed

```nx
import { Embed } from "discord-nx"

let embed = new Embed()
  .setTitle("Hello!")
  .setDescription("This is an embed")
  .setColor(0x00ff00)
  .addField("Field 1", "Value 1", true)
  .setTimestamp()

await client.sendMessage(channelId, "", { embed: embed })
```

### Buttons

```nx
import { ActionRow, ButtonStyles } from "discord-nx"

let row = new ActionRow()
  .addButton({
    style: ButtonStyles.PRIMARY,
    label: "Click Me",
    customId: "my_button",
  })

await client.sendMessage(channelId, "Click the button!", { components: [row] })
```

### Slash Commands

```nx
import { SlashCommand, OptionTypes } from "discord-nx"

let cmd = new SlashCommand("greet", "Greet a user")
  .addOption({
    name: "user",
    description: "User to greet",
    type: OptionTypes.USER,
    required: true,
  })
  .build()
```

### Permissions

```nx
import { Permissions, hasPermission } from "discord-nx"

let member = await client.getGuildMember(guildId, userId)
if (hasPermission(member.permissions, Permissions.KICK_MEMBERS)) {
  await client.removeGuildMember(guildId, targetUserId)
}
```

## Intents

```nx
import { Intents } from "discord-nx"

// Combine intents with |
let intents = Intents.GUILDS | Intents.GUILD_MESSAGES | Intents.MESSAGE_CONTENT

let client = new Client({ token: "...", intents: intents })
```

## Events

| Event | Description |
|-------|-------------|
| `READY` | Bot is connected |
| `MESSAGE_CREATE` | New message sent |
| `MESSAGE_UPDATE` | Message edited |
| `MESSAGE_DELETE` | Message deleted |
| `GUILD_CREATE` | Bot joined a guild |
| `GUILD_MEMBER_ADD` | Member joined |
| `GUILD_MEMBER_REMOVE` | Member left |
| `INTERACTION_CREATE` | Slash command or button |

## License

MIT - Nexora Community
