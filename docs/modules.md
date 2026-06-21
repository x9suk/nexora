# Modules

## Table of Contents

- [Importing Modules](#importing-modules)
- [Import with Alias](#import-with-alias)
- [Named Imports](#named-imports)
- [Exporting](#exporting)
- [Module Declaration](#module-declaration)
- [Standard Library Modules](#standard-library-modules)
- [Package Packages](#package-packages)
- [Game Packages](#game-packages)
- [How Modules Work](#how-modules-work)

## Importing Modules

Use `import` to bring in external modules:

```nexora
import "math"
import "string"
import "http"
```

Use imported functions:

```nexora
let pi = math.PI
print("Pi: " + str(pi))

let text = "Hello World"
let upper = string.upper(text)
print("Upper: " + upper)
```

## Import with Alias

Give a module a custom name:

```nexora
import "os" as operatingSystem

let platform = operatingSystem.platform()
print("Platform: " + platform)
```

## Named Imports

Import specific items from a module:

```nexora
import { sqrt, pow, abs } from "math"

let result = sqrt(16)
print("Square root: " + str(result))
```

## Exporting

Use `export` to make functions and classes available to other modules:

```nexora
export func publicFunction() {
    print("This function is exported")
}

export class PublicClass {
    init() {
        print("This class is exported")
    }
}

export const VERSION = "1.0.0"
```

## Module Declaration

Define a module with multiple exports:

```nexora
module MathUtils {
    export func square(x) {
        return x * x
    }
    
    export func cube(x) {
        return x ** 3
    }
    
    export const PI = 3.14159
}

// Use the module
print(MathUtils.square(5))  // 25
print(MathUtils.PI)         // 3.14159
```

## Standard Library Modules

### math — Mathematical Functions

```nexora
import { sqrt, pow, abs, min, max, floor, ceil, round } from "math"

print(sqrt(16))      // 4
print(pow(2, 10))    // 1024
print(abs(-42))      // 42
print(min(1, 2, 3))  // 1
print(max(1, 2, 3))  // 3
print(floor(3.7))    // 3
print(ceil(3.2))     // 4
print(round(3.5))    // 4
```

### string — String Operations

```nexora
import { split, join, contains, upper, lower, trim, replace, length } from "string"

let text = "Hello World"
print(upper(text))           // HELLO WORLD
print(lower(text))           // hello world
print(length(text))          // 11
print(contains(text, "World")) // true
print(trim("  hello  "))     // hello
print(replace(text, "World", "Nexora")) // Hello Nexora

let words = split("a,b,c", ",")
print(join(words, "-"))      // a-b-c
```

### collection — Collection Operations

```nexora
import { map, filter, reduce, sort, find, forEach } from "collection"

let nums = [1, 2, 3, 4, 5]

let doubled = map(nums, x => x * 2)
print(doubled)  // [2, 4, 6, 8, 10]

let evens = filter(nums, x => x % 2 == 0)
print(evens)    // [2, 4]

let sum = reduce(nums, (acc, x) => acc + x, 0)
print(sum)      // 15

let sorted = sort([3, 1, 4, 1, 5, 9])
print(sorted)   // [1, 1, 3, 4, 5, 9]

let found = find(nums, x => x > 3)
print(found)    // 4
```

### http — HTTP Client

```nexora
import { get, post, put, del } from "http"

// GET request
let response = await get("https://api.example.com/data")
print(response.body)

// POST request
let data = await post("https://api.example.com/users", {
  name: "John",
  email: "john@example.com"
})
print(data.body)
```

### fs — File System

```nexora
import { read, write, append, exists, mkdir, remove } from "fs"

// Read file
let content = read("hello.txt")
print(content)

// Write file
write("output.txt", "Hello Nexora!")

// Append to file
append("log.txt", "New log entry\n")

// Check if file exists
if (exists("config.json")) {
  print("Config file found")
}

// Create directory
mkdir("new-folder")

// Remove file
remove("temp.txt")
```

### json — JSON Operations

```nexora
import { parse, stringify } from "json"

// Parse JSON
let data = parse('{"name": "Nexora", "version": "1.0"}')
print(data.name)  // Nexora

// Stringify to JSON
let obj = { name: "Nexora", version: "1.0" }
let json = stringify(obj)
print(json)
```

### os — Operating System

```nexora
import { env, args, platform, sleep, exec } from "os"

// Get environment variable
let home = env("HOME")
print("Home: " + home)

// Get command line arguments
let arguments = args()
print("Args: " + str(arguments))

// Get platform
let os = platform()
print("Platform: " + os)

// Sleep for 1 second
await sleep(1000)

// Execute command
let result = exec("echo Hello")
print(result.stdout)
```

### time — Time Operations

```nexora
import { now, sleep, timestamp } from "time"

// Get current timestamp
let ts = timestamp()
print("Timestamp: " + str(ts))

// Get current time
let current = now()
print("Current time: " + current)
```

### test — Testing Framework

```nexora
import { describe, it, expect } from "test"

describe("Math Operations", () => {
  it("adds numbers", () => {
    expect(1 + 1).toEqual(2)
  })

  it("multiplies numbers", () => {
    expect(2 * 3).toEqual(6)
  })
})
```

## Package Packages

Install packages with `nxm`:

```bash
nxm install lodash-nx
nxm install express-nx
nxm install nexora-auth
```

### Popular Packages

| Package | Description |
|---------|-------------|
| `lodash-nx` | Utility functions (89k downloads) |
| `express-nx` | Web framework (32k downloads) |
| `nexora-http` | HTTP client (45k downloads) |
| `nexora-auth` | Authentication (24k downloads) |
| `nexora-db` | Database layer (31k downloads) |
| `nexora-logger` | Logging (56k downloads) |
| `nexora-test` | Testing framework (42k downloads) |
| `nexora-websocket` | WebSockets (18k downloads) |

## Game Packages

Build games and game bots with Nexora:

### minecraft-nx

Minecraft bot framework:

```nexora
import { Bot, Position, Blocks, Mobs } from "minecraft-nx"

let bot = new Bot({
  username: "MyBot",
  host: "localhost",
  port: 25565,
})

bot.on("spawn", async () => {
  await bot.chat("Hello world!")
  await bot.moveTo(100, 64, 100)
})

await bot.connect()
```

### steam-nx

Steam API integration:

```nexora
import { SteamClient } from "steam-nx"

let client = new SteamClient(process.env.STEAM_API_KEY)

let games = await client.getOwnedGames(steamId)
print("Games: " + str(games.length))
```

### roblox-nx

Roblox API integration:

```nexora
import { RobloxClient } from "roblox-nx"

let client = new RobloxClient()

let user = await client.getUser(userId)
print("User: " + user.name)
```

### discord-nx

Discord bot framework:

```nexora
import { Client, Events, Intents, Embed } from "discord-nx"

let client = new Client({
  token: process.env.DISCORD_TOKEN,
  intents: Intents.GUILDS | Intents.GUILD_MESSAGES,
})

client.on(Events.MESSAGE_CREATE, async (message) => {
  if (message.content === "!ping") {
    await client.sendMessage(message.channel_id, "Pong!")
  }
})

await client.login()
```

## How Modules Work

1. **Standard Library**: Built-in modules available everywhere
2. **Local Modules**: Files in your project can be imported
3. **Package Modules**: Installed via `nxm install`
4. **Global Modules**: Available system-wide

### Module Resolution

When you import a module, Nexora searches in this order:

1. Standard library (`math`, `string`, `http`, etc.)
2. Local `nexora_modules/` directory
3. Global `~/.nexora/packages/` directory
4. Current directory

### Creating Your Own Package

```bash
nxm init --name my-package
```

This creates:

```
my-package/
├── nexora.json    # Package metadata
├── index.nx       # Main module file
├── test.nx        # Tests
└── README.md      # Documentation
```

Publish to the registry:

```bash
nxm publish
```
