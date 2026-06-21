# nexora-websocket

WebSocket client and server for the Nexora runtime.

## Installation

```bash
nxm install nexora-websocket
```

## Server

### Quick Start

```nx
import { WebSocketServer } from "nexora-websocket";

const server = new WebSocketServer({ port: 8080 });

server.onConnection = (client) => {
  console.log("Client connected:", client.id);
  client.send({ type: "welcome", id: client.id });
};

server.onMessage = (client, data) => {
  console.log("Received:", data);
  server.broadcast(data, client.id);
};

await server.start();
```

### Broadcasting

```nx
// Broadcast to all clients
server.broadcast({ type: "announcement", text: "Hello everyone" });

// Broadcast to all except sender
server.broadcast(data, client.id);
```

### Channels / Rooms

```nx
// Client subscribes to a channel
client.subscribe("chat");

// Broadcast to a channel
server.to("chat").broadcast({ type: "message", text: "Hello" });

// Unsubscribe
client.unsubscribe("chat");
```

### Events

```nx
server.onConnection = (client) => {
  console.log("Connected:", client.id);
};

server.onMessage = (client, data) => {
  console.log("Message from", client.id, ":", data);
};

server.onClose = (client) => {
  console.log("Disconnected:", client.id);
};

server.onError = (client, err) => {
  console.error("Error from", client.id, ":", err);
};
```

### Stopping the Server

```nx
await server.stop();
```

## Client

### Quick Start

```nx
import { connect } from "nexora-websocket";

const ws = await connect("ws://localhost:8080");

ws.onMessage = (data) => {
  console.log("Received:", data);
};

ws.send({ type: "hello" });
```

### Events

```nx
import { WebSocket } from "nexora-websocket";

const ws = new WebSocket("ws://localhost:8080", {
  onOpen: () => console.log("Connected"),
  onMessage: (data) => console.log("Message:", data),
  onClose: () => console.log("Disconnected"),
  onError: (err) => console.error("Error:", err),
});

await ws.connect();
```

### Sending Data

```nx
// String
ws.send("hello");

// Object (auto-serialized)
ws.send({ type: "chat", text: "Hello" });
```

### Closing

```nx
ws.close();
```

## Full Example: Chat Server

```nx
import { WebSocketServer } from "nexora-websocket";

const server = new WebSocketServer({ port: 8080 });

server.onConnection = (client) => {
  console.log("User joined:", client.id);
  client.subscribe("general");
  server.to("general").broadcast({
    type: "system",
    text: `${client.id} joined`,
  });
};

server.onMessage = (client, data) => {
  if (data.type === "chat") {
    server.to("general").broadcast({
      type: "chat",
      user: client.id,
      text: data.text,
      time: Date.now(),
    });
  }
};

server.onClose = (client) => {
  server.to("general").broadcast({
    type: "system",
    text: `${client.id} left`,
  });
};

await server.start();
console.log("Chat server running on ws://localhost:8080");
```

## Full Example: Client

```nx
import { connect } from "nexora-websocket";

const ws = await connect("ws://localhost:8080");

ws.onMessage = (data) => {
  if (data.type === "chat") {
    console.log(`${data.user}: ${data.text}`);
  } else if (data.type === "system") {
    console.log(`[System] ${data.text}`);
  }
};

// Send a message
ws.send({ type: "chat", text: "Hello everyone!" });

// Close on exit
process.on("SIGINT", () => {
  ws.close();
  process.exit(0);
});
```

## License

MIT
