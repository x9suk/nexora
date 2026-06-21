# nexora-logger

Logging library for the Nexora runtime. Structured logging with levels, transports, and child loggers.

## Installation

```bash
nxm install nexora-logger
```

## Quick Start

```nx
import { createLogger } from "nexora-logger";

const logger = createLogger({ name: "app" });

logger.info("Server started", { port: 3000 });
logger.error("Something failed", { code: 500 });
```

## Log Levels

| Level  | Value |
|--------|-------|
| debug  | 0     |
| info   | 1     |
| warn   | 2     |
| error  | 3     |
| silent | 4     |

```nx
const logger = createLogger({ name: "app", level: "warn" });

logger.debug("hidden");
logger.info("hidden");
logger.warn("visible");
logger.error("visible");
```

## Creating Loggers

```nx
import { createLogger, consoleTransport, jsonTransport, fileTransport } from "nexora-logger";

// Console output (default)
const logger = createLogger({ name: "app" });

// JSON output
const logger = createLogger({
  name: "app",
  transports: [jsonTransport()],
});

// File output
const logger = createLogger({
  name: "app",
  transports: [fileTransport("./logs/app.log")],
});

// Multiple transports
const logger = createLogger({
  name: "app",
  transports: [
    consoleTransport(),
    fileTransport("./logs/app.log"),
    jsonTransport(),
  ],
});
```

## Logging Messages

```nx
logger.debug("Variable value", { x: 42 });
logger.info("User created", { id: 123, name: "Alice" });
logger.warn("Disk space low", { free: "100MB" });
logger.error("Connection failed", { host: "db.local", error: "timeout" });
```

## Child Loggers

```nx
const logger = createLogger({ name: "app" });

const dbLog = logger.child("db");
dbLog.info("Connected");
// Output: ... INFO [app:db] Connected

const httpLog = logger.child("http");
httpLog.info("Request received");
// Output: ... INFO [app:http] Request received
```

## Context

```nx
const logger = createLogger({
  name: "app",
  meta: { requestId: "abc-123" },
});

logger.info("Processing");
// Output: ... INFO [app] Processing {"requestId":"abc-123"}

const reqLog = logger.withContext({ userId: 42 });
reqLog.info("User action");
// Output: ... INFO [app] User action {"requestId":"abc-123","userId":42}
```

## Changing Level

```nx
logger.setLevel("debug");
```

## Transports

### Console Transport

```nx
import { consoleTransport } from "nexora-logger";

const logger = createLogger({
  transports: [consoleTransport()],
});
```

### File Transport

```nx
import { fileTransport } from "nexora-logger";

const logger = createLogger({
  transports: [fileTransport("./logs/app.log")],
});
```

### JSON Transport

```nx
import { jsonTransport } from "nexora-logger";

const logger = createLogger({
  transports: [jsonTransport()],
});
```

### Custom Transport

```nx
function myTransport(options) {
  return (entry) => {
    // entry = { timestamp, level, message, data, meta }
    sendToService(entry);
  };
}

const logger = createLogger({
  transports: [myTransport({ url: "https://logs.example.com" })],
});
```

## Timing

```nx
const timer = logger.startTimer();
// ... do work
timer.end("Request processed");
// Output: ... INFO [app] Request processed {"duration":"42ms"}
```

## Full Example

```nx
import { createLogger, consoleTransport, fileTransport } from "nexora-logger";

const logger = createLogger({
  name: "server",
  level: "debug",
  transports: [
    consoleTransport(),
    fileTransport("./logs/server.log"),
  ],
});

function handleRequest(req) {
  const reqLog = logger.child("http").withContext({
    method: req.method,
    path: req.path,
  });

  reqLog.info("Request received");

  try {
    const result = processRequest(req);
    reqLog.info("Request completed", { status: 200 });
    return result;
  } catch (err) {
    reqLog.error("Request failed", { error: err.message });
    throw err;
  }
}

logger.info("Server started", { port: 3000 });
```

## License

MIT
