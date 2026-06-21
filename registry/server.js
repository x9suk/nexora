const http = require('http');
const fs = require('fs');
const path = require('path');

const PORT = 3000;
const PACKAGES_DIR = path.join(__dirname, '..', 'packages');

const packages = {
  // === GAMING ===
  'minecraft-nx': { name: 'minecraft-nx', version: '1.0.0', description: 'Minecraft bot framework', author: 'Nexora Community', keywords: ['minecraft','bot','game'], downloads: 12500, stars: 340 },
  'steam-nx': { name: 'steam-nx', version: '1.0.0', description: 'Steam API integration', author: 'Nexora Community', keywords: ['steam','gaming','api'], downloads: 8200, stars: 210 },
  'roblox-nx': { name: 'roblox-nx', version: '1.0.0', description: 'Roblox API integration', author: 'Nexora Community', keywords: ['roblox','gaming','api'], downloads: 6800, stars: 180 },
  'discord-nx': { name: 'discord-nx', version: '1.0.0', description: 'Discord bot framework', author: 'Nexora Community', keywords: ['discord','bot','chat'], downloads: 28400, stars: 890 },
  'unity-nx': { name: 'unity-nx', version: '1.0.0', description: 'Unity game engine bridge', author: 'Nexora Gaming', keywords: ['unity','game','3d'], downloads: 5200, stars: 150 },
  'godot-nx': { name: 'godot-nx', version: '1.0.0', description: 'Godot engine integration', author: 'Nexora Gaming', keywords: ['godot','game','2d','3d'], downloads: 3100, stars: 95 },
  'unreal-nx': { name: 'unreal-nx', version: '1.0.0', description: 'Unreal Engine bindings', author: 'Nexora Gaming', keywords: ['unreal','game','3d'], downloads: 4500, stars: 130 },
  'pygame-nx': { name: 'pygame-nx', version: '1.0.0', description: 'Python-like game library', author: 'Nexora Gaming', keywords: ['pygame','game','2d'], downloads: 7200, stars: 210 },
  'phaser-nx': { name: 'phaser-nx', version: '1.0.0', description: 'Phaser game framework port', author: 'Nexora Gaming', keywords: ['phaser','game','web'], downloads: 4800, stars: 140 },
  'pixi-nx': { name: 'pixi-nx', version: '1.0.0', description: 'PixiJS 2D renderer', author: 'Nexora Gaming', keywords: ['pixi','2d','render'], downloads: 3900, stars: 115 },
  'three-nx': { name: 'three-nx', version: '1.0.0', description: 'Three.js 3D library', author: 'Nexora Gaming', keywords: ['three','3d','webgl'], downloads: 6100, stars: 185 },
  'canvas-nx': { name: 'canvas-nx', version: '1.0.0', description: 'HTML5 Canvas API', author: 'Nexora Graphics', keywords: ['canvas','2d','draw'], downloads: 5500, stars: 165 },

  // === WEB ===
  'express-nx': { name: 'express-nx', version: '1.8.3', description: 'Express.js inspired framework', author: 'Nexora Contributors', keywords: ['express','web','middleware'], downloads: 32100, stars: 890 },
  'nexora-http': { name: 'nexora-http', version: '2.4.0', description: 'HTTP framework for web apps', author: 'Nexora Team', keywords: ['http','web','api'], downloads: 45200, stars: 1200 },
  'nexora-websocket': { name: 'nexora-websocket', version: '1.2.0', description: 'WebSocket library', author: 'Nexora Team', keywords: ['websocket','realtime','chat'], downloads: 18500, stars: 456 },
  'fastify-nx': { name: 'fastify-nx', version: '1.0.0', description: 'Fast HTTP framework', author: 'Nexora Web', keywords: ['fastify','fast','web'], downloads: 14200, stars: 420 },
  'koa-nx': { name: 'koa-nx', version: '1.0.0', description: 'Koa-inspired middleware framework', author: 'Nexora Web', keywords: ['koa','middleware','web'], downloads: 8900, stars: 265 },
  'hapi-nx': { name: 'hapi-nx', version: '1.0.0', description: 'Enterprise HTTP framework', author: 'Nexora Web', keywords: ['hapi','enterprise','web'], downloads: 6700, stars: 195 },
  'nest-nx': { name: 'nest-nx', version: '1.0.0', description: 'NestJS-style framework', author: 'Nexora Web', keywords: ['nestjs','modular','web'], downloads: 11300, stars: 340 },
  'graphql-nx': { name: 'graphql-nx', version: '1.0.0', description: 'GraphQL server implementation', author: 'Nexora Web', keywords: ['graphql','api','query'], downloads: 9800, stars: 290 },
  'rest-nx': { name: 'rest-nx', version: '1.0.0', description: 'REST API toolkit', author: 'Nexora Web', keywords: ['rest','api','crud'], downloads: 12400, stars: 370 },
  'rpc-nx': { name: 'rpc-nx', version: '1.0.0', description: 'JSON-RPC implementation', author: 'Nexora Web', keywords: ['rpc','json','remote'], downloads: 4500, stars: 135 },
  'cors-nx': { name: 'cors-nx', version: '1.0.0', description: 'CORS middleware', author: 'Nexora Web', keywords: ['cors','middleware','security'], downloads: 21000, stars: 620 },
  'helmet-nx': { name: 'helmet-nx', version: '1.0.0', description: 'Security headers middleware', author: 'Nexora Web', keywords: ['helmet','security','headers'], downloads: 16800, stars: 500 },
  'rate-limit-nx': { name: 'rate-limit-nx', version: '1.0.0', description: 'Rate limiting middleware', author: 'Nexora Web', keywords: ['ratelimit','security','throttle'], downloads: 13500, stars: 400 },
  'session-nx': { name: 'session-nx', version: '1.0.0', description: 'Session management', author: 'Nexora Web', keywords: ['session','cookie','state'], downloads: 10200, stars: 300 },
  'cookie-nx': { name: 'cookie-nx', version: '1.0.0', description: 'Cookie parser and handler', author: 'Nexora Web', keywords: ['cookie','parser','http'], downloads: 8700, stars: 260 },
  'body-parser-nx': { name: 'body-parser-nx', version: '1.0.0', description: 'Request body parser', author: 'Nexora Web', keywords: ['body','parser','json'], downloads: 19500, stars: 580 },
  'multer-nx': { name: 'multer-nx', version: '1.0.0', description: 'File upload middleware', author: 'Nexora Web', keywords: ['upload','file','multipart'], downloads: 11800, stars: 350 },
  'static-nx': { name: 'static-nx', version: '1.0.0', description: 'Static file server', author: 'Nexora Web', keywords: ['static','files','serve'], downloads: 15600, stars: 460 },
  'proxy-nx': { name: 'proxy-nx', version: '1.0.0', description: 'HTTP proxy middleware', author: 'Nexora Web', keywords: ['proxy','reverse','forward'], downloads: 7400, stars: 220 },
  'template-nx': { name: 'template-nx', version: '1.0.0', description: 'Template engine', author: 'Nexora Web', keywords: ['template','html','render'], downloads: 9100, stars: 270 },

  // === DATABASE ===
  'nexora-db': { name: 'nexora-db', version: '1.8.0', description: 'Database abstraction layer', author: 'Nexora Team', keywords: ['database','sql','nosql'], downloads: 31200, stars: 780 },
  'mongo-nx': { name: 'mongo-nx', version: '1.0.0', description: 'MongoDB driver', author: 'Nexora DB', keywords: ['mongodb','nosql','database'], downloads: 14500, stars: 430 },
  'postgres-nx': { name: 'postgres-nx', version: '1.0.0', description: 'PostgreSQL client', author: 'Nexora DB', keywords: ['postgres','sql','database'], downloads: 12800, stars: 380 },
  'mysql-nx': { name: 'mysql-nx', version: '1.0.0', description: 'MySQL client', author: 'Nexora DB', keywords: ['mysql','sql','database'], downloads: 11200, stars: 330 },
  'sqlite-nx': { name: 'sqlite-nx', version: '1.0.0', description: 'SQLite database', author: 'Nexora DB', keywords: ['sqlite','embedded','database'], downloads: 16700, stars: 500 },
  'redis-nx': { name: 'redis-nx', version: '1.0.0', description: 'Redis client', author: 'Nexora DB', keywords: ['redis','cache','memory'], downloads: 13900, stars: 415 },
  'elastic-nx': { name: 'elastic-nx', version: '1.0.0', description: 'Elasticsearch client', author: 'Nexora DB', keywords: ['elasticsearch','search','index'], downloads: 6300, stars: 190 },
  'cassandra-nx': { name: 'cassandra-nx', version: '1.0.0', description: 'Cassandra database client', author: 'Nexora DB', keywords: ['cassandra','nosql','distributed'], downloads: 3800, stars: 115 },
  'dynamo-nx': { name: 'dynamo-nx', version: '1.0.0', description: 'DynamoDB client', author: 'Nexora DB', keywords: ['dynamodb','aws','nosql'], downloads: 5100, stars: 155 },
  'neo4j-nx': { name: 'neo4j-nx', version: '1.0.0', description: 'Neo4j graph database', author: 'Nexora DB', keywords: ['neo4j','graph','database'], downloads: 4200, stars: 125 },
  'orm-nx': { name: 'orm-nx', version: '1.0.0', description: 'Object-relational mapper', author: 'Nexora DB', keywords: ['orm','database','model'], downloads: 18400, stars: 550 },
  'migrate-nx': { name: 'migrate-nx', version: '1.0.0', description: 'Database migrations', author: 'Nexora DB', keywords: ['migrate','schema','version'], downloads: 9600, stars: 285 },
  'seed-nx': { name: 'seed-nx', version: '1.0.0', description: 'Database seeding', author: 'Nexora DB', keywords: ['seed','test','data'], downloads: 5400, stars: 160 },
  'pool-nx': { name: 'pool-nx', version: '1.0.0', description: 'Connection pooling', author: 'Nexora DB', keywords: ['pool','connection','performance'], downloads: 7800, stars: 235 },

  // === AUTH ===
  'nexora-auth': { name: 'nexora-auth', version: '1.5.0', description: 'Authentication library', author: 'Nexora Team', keywords: ['auth','jwt','oauth'], downloads: 24300, stars: 670 },
  'oauth-nx': { name: 'oauth-nx', version: '1.0.0', description: 'OAuth 2.0 implementation', author: 'Nexora Auth', keywords: ['oauth','login','social'], downloads: 11500, stars: 340 },
  'passport-nx': { name: 'passport-nx', version: '1.0.0', description: 'Authentication middleware', author: 'Nexora Auth', keywords: ['passport','strategy','login'], downloads: 9800, stars: 290 },
  'jwt-nx': { name: 'jwt-nx', version: '1.0.0', description: 'JSON Web Token library', author: 'Nexora Auth', keywords: ['jwt','token','auth'], downloads: 15200, stars: 450 },
  'bcrypt-nx': { name: 'bcrypt-nx', version: '1.0.0', description: 'Password hashing', author: 'Nexora Auth', keywords: ['bcrypt','hash','password'], downloads: 13100, stars: 390 },
  'crypto-nx': { name: 'crypto-nx', version: '1.0.0', description: 'Cryptography utilities', author: 'Nexora Auth', keywords: ['crypto','encrypt','security'], downloads: 16800, stars: 500 },
  '2fa-nx': { name: '2fa-nx', version: '1.0.0', description: 'Two-factor authentication', author: 'Nexora Auth', keywords: ['2fa','totp','otp'], downloads: 7600, stars: 225 },
  'oauth-github-nx': { name: 'oauth-github-nx', version: '1.0.0', description: 'GitHub OAuth provider', author: 'Nexora Auth', keywords: ['github','oauth','login'], downloads: 5800, stars: 175 },
  'oauth-google-nx': { name: 'oauth-google-nx', version: '1.0.0', description: 'Google OAuth provider', author: 'Nexora Auth', keywords: ['google','oauth','login'], downloads: 6200, stars: 185 },
  'oauth-discord-nx': { name: 'oauth-discord-nx', version: '1.0.0', description: 'Discord OAuth provider', author: 'Nexora Auth', keywords: ['discord','oauth','login'], downloads: 4100, stars: 125 },
  'session-auth-nx': { name: 'session-auth-nx', version: '1.0.0', description: 'Session-based authentication', author: 'Nexora Auth', keywords: ['session','auth','cookie'], downloads: 8400, stars: 250 },

  // === UTILITY ===
  'lodash-nx': { name: 'lodash-nx', version: '4.17.21', description: 'Utility functions', author: 'Nexora Contributors', keywords: ['lodash','utility','functional'], downloads: 89200, stars: 2100 },
  'nexora-logger': { name: 'nexora-logger', version: '2.0.1', description: 'Logging library', author: 'Nexora Team', keywords: ['logging','debug','console'], downloads: 56700, stars: 1400 },
  'dayjs-nx': { name: 'dayjs-nx', version: '1.0.0', description: 'Date/time library', author: 'Nexora Utils', keywords: ['date','time','moment'], downloads: 42300, stars: 1050 },
  'uuid-nx': { name: 'uuid-nx', version: '1.0.0', description: 'UUID generator', author: 'Nexora Utils', keywords: ['uuid','guid','id'], downloads: 35600, stars: 880 },
  'validator-nx': { name: 'validator-nx', version: '1.0.0', description: 'String validation', author: 'Nexora Utils', keywords: ['validate','string','check'], downloads: 28900, stars: 720 },
  'sanitize-nx': { name: 'sanitize-nx', version: '1.0.0', description: 'HTML sanitization', author: 'Nexora Utils', keywords: ['sanitize','html','security'], downloads: 19400, stars: 580 },
  'marked-nx': { name: 'marked-nx', version: '1.0.0', description: 'Markdown parser', author: 'Nexora Utils', keywords: ['markdown','parser','html'], downloads: 15800, stars: 470 },
  'highlight-nx': { name: 'highlight-nx', version: '1.0.0', description: 'Syntax highlighting', author: 'Nexora Utils', keywords: ['highlight','syntax','code'], downloads: 12300, stars: 370 },
  'cron-nx': { name: 'cron-nx', version: '1.0.0', description: 'Cron job scheduler', author: 'Nexora Utils', keywords: ['cron','scheduler','timer'], downloads: 14700, stars: 440 },
  'queue-nx': { name: 'queue-nx', version: '1.0.0', description: 'Job queue system', author: 'Nexora Utils', keywords: ['queue','job','worker'], downloads: 11200, stars: 335 },
  'pool-utils-nx': { name: 'pool-utils-nx', version: '1.0.0', description: 'Object pooling', author: 'Nexora Utils', keywords: ['pool','memory','performance'], downloads: 6800, stars: 205 },
  'retry-nx': { name: 'retry-nx', version: '1.0.0', description: 'Retry with backoff', author: 'Nexora Utils', keywords: ['retry','resilience','error'], downloads: 9500, stars: 285 },
  'debounce-nx': { name: 'debounce-nx', version: '1.0.0', description: 'Debounce/throttle utilities', author: 'Nexora Utils', keywords: ['debounce','throttle','performance'], downloads: 13200, stars: 395 },
  'event-nx': { name: 'event-nx', version: '1.0.0', description: 'Event emitter', author: 'Nexora Utils', keywords: ['event','emitter','listener'], downloads: 16500, stars: 495 },
  'emitter-nx': { name: 'emitter-nx', version: '1.0.0', description: 'Event emitter enhanced', author: 'Nexora Utils', keywords: ['emitter','events','pubsub'], downloads: 8100, stars: 245 },
  'memoize-nx': { name: 'memoize-nx', version: '1.0.0', description: 'Function memoization', author: 'Nexora Utils', keywords: ['memoize','cache','performance'], downloads: 7300, stars: 220 },
  'lock-nx': { name: 'lock-nx', version: '1.0.0', description: 'Mutex and locks', author: 'Nexora Utils', keywords: ['lock','mutex','concurrency'], downloads: 5900, stars: 180 },
  'semver-nx': { name: 'semver-nx', version: '1.0.0', description: 'Semantic versioning', author: 'Nexora Utils', keywords: ['semver','version','compare'], downloads: 10800, stars: 325 },
  'glob-nx': { name: 'glob-nx', version: '1.0.0', description: 'File globbing', author: 'Nexora Utils', keywords: ['glob','files','pattern'], downloads: 14200, stars: 425 },
  'chokidar-nx': { name: 'chokidar-nx', version: '1.0.0', description: 'File watcher', author: 'Nexora Utils', keywords: ['watch','files','change'], downloads: 11800, stars: 355 },
  'dotenv-nx': { name: 'dotenv-nx', version: '1.0.0', description: 'Environment variables', author: 'Nexora Utils', keywords: ['dotenv','env','config'], downloads: 22400, stars: 665 },
  'config-nx': { name: 'config-nx', version: '1.0.0', description: 'Configuration management', author: 'Nexora Utils', keywords: ['config','settings','manage'], downloads: 13500, stars: 405 },
  'cache-nx': { name: 'cache-nx', version: '1.0.0', description: 'Caching utilities', author: 'Nexora Utils', keywords: ['cache','memory','performance'], downloads: 15900, stars: 475 },
  'hash-nx': { name: 'hash-nx', version: '1.0.0', description: 'Hashing utilities', author: 'Nexora Utils', keywords: ['hash','checksum','digest'], downloads: 10200, stars: 305 },

  // === TESTING ===
  'nexora-test': { name: 'nexora-test', version: '1.2.0', description: 'Testing framework', author: 'Nexora Team', keywords: ['testing','assertions','mock'], downloads: 42800, stars: 1100 },
  'jest-nx': { name: 'jest-nx', version: '1.0.0', description: 'Jest-style testing', author: 'Nexora Test', keywords: ['jest','test','unit'], downloads: 31500, stars: 780 },
  'mocha-nx': { name: 'mocha-nx', version: '1.0.0', description: 'Mocha-style testing', author: 'Nexora Test', keywords: ['mocha','test','bdd'], downloads: 18700, stars: 560 },
  'chai-nx': { name: 'chai-nx', version: '1.0.0', description: 'Assertion library', author: 'Nexora Test', keywords: ['chai','assert','expect'], downloads: 24300, stars: 720 },
  'sinon-nx': { name: 'sinon-nx', version: '1.0.0', description: 'Test doubles (mocks/spies)', author: 'Nexora Test', keywords: ['sinon','mock','spy'], downloads: 15600, stars: 465 },
  'supertest-nx': { name: 'supertest-nx', version: '1.0.0', description: 'HTTP testing', author: 'Nexora Test', keywords: ['supertest','http','api'], downloads: 12400, stars: 370 },
  'cypress-nx': { name: 'cypress-nx', version: '1.0.0', description: 'E2E testing framework', author: 'Nexora Test', keywords: ['cypress','e2e','browser'], downloads: 9800, stars: 295 },
  'playwright-nx': { name: 'playwright-nx', version: '1.0.0', description: 'Browser automation testing', author: 'Nexora Test', keywords: ['playwright','browser','automation'], downloads: 8200, stars: 250 },
  'coverage-nx': { name: 'coverage-nx', version: '1.0.0', description: 'Code coverage', author: 'Nexora Test', keywords: ['coverage','report','quality'], downloads: 11300, stars: 340 },
  'snapshot-nx': { name: 'snapshot-nx', version: '1.0.0', description: 'Snapshot testing', author: 'Nexora Test', keywords: ['snapshot','compare','regression'], downloads: 7600, stars: 230 },

  // === AI/ML ===
  'tensorflow-nx': { name: 'tensorflow-nx', version: '1.0.0', description: 'TensorFlow bindings', author: 'Nexora AI', keywords: ['tensorflow','ml','ai'], downloads: 8900, stars: 270 },
  'openai-nx': { name: 'openai-nx', version: '1.0.0', description: 'OpenAI API client', author: 'Nexora AI', keywords: ['openai','gpt','ai'], downloads: 15600, stars: 470 },
  'anthropic-nx': { name: 'anthropic-nx', version: '1.0.0', description: 'Anthropic Claude API', author: 'Nexora AI', keywords: ['anthropic','claude','ai'], downloads: 7200, stars: 220 },
  'ml-nx': { name: 'ml-nx', version: '1.0.0', description: 'Machine learning utilities', author: 'Nexora AI', keywords: ['ml','learn','model'], downloads: 5400, stars: 165 },
  'nlp-nx': { name: 'nlp-nx', version: '1.0.0', description: 'Natural language processing', author: 'Nexora AI', keywords: ['nlp','text','language'], downloads: 4100, stars: 125 },
  'vision-nx': { name: 'vision-nx', version: '1.0.0', description: 'Computer vision', author: 'Nexora AI', keywords: ['vision','image','detect'], downloads: 3800, stars: 115 },
  'speech-nx': { name: 'speech-nx', version: '1.0.0', description: 'Speech recognition', author: 'Nexora AI', keywords: ['speech','voice','stt'], downloads: 3200, stars: 98 },
  'tts-nx': { name: 'tts-nx', version: '1.0.0', description: 'Text-to-speech', author: 'Nexora AI', keywords: ['tts','voice','speech'], downloads: 2900, stars: 88 },

  // === CRYPTO ===
  'bitcoin-nx': { name: 'bitcoin-nx', version: '1.0.0', description: 'Bitcoin API client', author: 'Nexora Crypto', keywords: ['bitcoin','btc','blockchain'], downloads: 6700, stars: 200 },
  'ethereum-nx': { name: 'ethereum-nx', version: '1.0.0', description: 'Ethereum/web3 library', author: 'Nexora Crypto', keywords: ['ethereum','web3','blockchain'], downloads: 8400, stars: 255 },
  'solana-nx': { name: 'solana-nx', version: '1.0.0', description: 'Solana blockchain client', author: 'Nexora Crypto', keywords: ['solana','blockchain','defi'], downloads: 5100, stars: 155 },
  'wallet-nx': { name: 'wallet-nx', version: '1.0.0', description: 'Crypto wallet utilities', author: 'Nexora Crypto', keywords: ['wallet','key','address'], downloads: 4800, stars: 145 },
  'nft-nx': { name: 'nft-nx', version: '1.0.0', description: 'NFT utilities', author: 'Nexora Crypto', keywords: ['nft','erc721','collect'], downloads: 3500, stars: 108 },
  'defi-nx': { name: 'defi-nx', version: '1.0.0', description: 'DeFi protocol helpers', author: 'Nexora Crypto', keywords: ['defi','swap','yield'], downloads: 4200, stars: 128 },

  // === MOBILE ===
  'react-native-nx': { name: 'react-native-nx', version: '1.0.0', description: 'React Native bridge', author: 'Nexora Mobile', keywords: ['react-native','mobile','ios','android'], downloads: 11200, stars: 335 },
  'flutter-nx': { name: 'flutter-nx', version: '1.0.0', description: 'Flutter integration', author: 'Nexora Mobile', keywords: ['flutter','dart','mobile'], downloads: 7800, stars: 235 },
  'capacitor-nx': { name: 'capacitor-nx', version: '1.0.0', description: 'Capacitor native bridge', author: 'Nexora Mobile', keywords: ['capacitor','native','hybrid'], downloads: 5600, stars: 170 },
  'push-nx': { name: 'push-nx', version: '1.0.0', description: 'Push notifications', author: 'Nexora Mobile', keywords: ['push','notification','fcm','apns'], downloads: 8900, stars: 268 },
  'camera-nx': { name: 'camera-nx', version: '1.0.0', description: 'Camera access', author: 'Nexora Mobile', keywords: ['camera','photo','video'], downloads: 6200, stars: 188 },
  'geolocation-nx': { name: 'geolocation-nx', version: '1.0.0', description: 'GPS and location', author: 'Nexora Mobile', keywords: ['gps','location','map'], downloads: 9400, stars: 282 },

  // === DEVOPS ===
  'docker-nx': { name: 'docker-nx', version: '1.0.0', description: 'Docker API client', author: 'Nexora DevOps', keywords: ['docker','container','deploy'], downloads: 12300, stars: 370 },
  'kubernetes-nx': { name: 'kubernetes-nx', version: '1.0.0', description: 'Kubernetes client', author: 'Nexora DevOps', keywords: ['kubernetes','k8s','orchestration'], downloads: 7800, stars: 235 },
  'aws-nx': { name: 'aws-nx', version: '1.0.0', description: 'AWS SDK wrapper', author: 'Nexora DevOps', keywords: ['aws','lambda','s3'], downloads: 14500, stars: 435 },
  'gcp-nx': { name: 'gcp-nx', version: '1.0.0', description: 'Google Cloud client', author: 'Nexora DevOps', keywords: ['gcp','cloud','firebase'], downloads: 6800, stars: 205 },
  'azure-nx': { name: 'azure-nx', version: '1.0.0', description: 'Azure SDK wrapper', author: 'Nexora DevOps', keywords: ['azure','cloud','microsoft'], downloads: 5400, stars: 165 },
  'ci-nx': { name: 'ci-nx', version: '1.0.0', description: 'CI/CD utilities', author: 'Nexora DevOps', keywords: ['ci','cd','pipeline'], downloads: 8200, stars: 250 },
  'terraform-nx': { name: 'terraform-nx', version: '1.0.0', description: 'Terraform wrapper', author: 'Nexora DevOps', keywords: ['terraform','iac','infra'], downloads: 4900, stars: 150 },

  // === SECURITY ===
  'helmet-sec-nx': { name: 'helmet-sec-nx', version: '1.0.0', description: 'Security headers', author: 'Nexora Security', keywords: ['helmet','headers','csp'], downloads: 18200, stars: 545 },
  'sanitize-xss-nx': { name: 'sanitize-xss-nx', version: '1.0.0', description: 'XSS prevention', author: 'Nexora Security', keywords: ['xss','sanitize','security'], downloads: 12800, stars: 385 },
  'csrf-nx': { name: 'csrf-nx', version: '1.0.0', description: 'CSRF protection', author: 'Nexora Security', keywords: ['csrf','token','form'], downloads: 10500, stars: 315 },
  'ratelimit-sec-nx': { name: 'ratelimit-sec-nx', version: '1.0.0', description: 'Rate limiting', author: 'Nexora Security', keywords: ['ratelimit','throttle','abuse'], downloads: 14300, stars: 430 },
  'cors-sec-nx': { name: 'cors-sec-nx', version: '1.0.0', description: 'CORS configuration', author: 'Nexora Security', keywords: ['cors','origin','header'], downloads: 16700, stars: 500 },
  'audit-nx': { name: 'audit-nx', version: '1.0.0', description: 'Security auditing', author: 'Nexora Security', keywords: ['audit','vulnerability','scan'], downloads: 7100, stars: 215 },

  // === DATA ===
  'csv-nx': { name: 'csv-nx', version: '1.0.0', description: 'CSV parser/writer', author: 'Nexora Data', keywords: ['csv','parse','export'], downloads: 15400, stars: 460 },
  'excel-nx': { name: 'excel-nx', version: '1.0.0', description: 'Excel file handling', author: 'Nexora Data', keywords: ['excel','xlsx','spreadsheet'], downloads: 11200, stars: 335 },
  'pdf-nx': { name: 'pdf-nx', version: '1.0.0', description: 'PDF generation', author: 'Nexora Data', keywords: ['pdf','document','generate'], downloads: 9800, stars: 295 },
  'image-nx': { name: 'image-nx', version: '1.0.0', description: 'Image processing', author: 'Nexora Data', keywords: ['image','resize','convert'], downloads: 8400, stars: 255 },
  'chart-nx': { name: 'chart-nx', version: '1.0.0', description: 'Chart generation', author: 'Nexora Data', keywords: ['chart','graph','visualize'], downloads: 7200, stars: 218 },
  'table-nx': { name: 'table-nx', version: '1.0.0', description: 'Table formatting', author: 'Nexora Data', keywords: ['table','format','console'], downloads: 10600, stars: 320 },
  'xml-nx': { name: 'xml-nx', version: '1.0.0', description: 'XML parser', author: 'Nexora Data', keywords: ['xml','parse','dom'], downloads: 8900, stars: 268 },
  'yaml-nx': { name: 'yaml-nx', version: '1.0.0', description: 'YAML parser', author: 'Nexora Data', keywords: ['yaml','parse','config'], downloads: 12100, stars: 365 },
  'toml-nx': { name: 'toml-nx', version: '1.0.0', description: 'TOML parser', author: 'Nexora Data', keywords: ['toml','config','parse'], downloads: 6500, stars: 198 },

  // === NETWORK ===
  'fetch-nx': { name: 'fetch-nx', version: '1.0.0', description: 'HTTP fetch API', author: 'Nexora Net', keywords: ['fetch','http','request'], downloads: 28500, stars: 710 },
  'axios-nx': { name: 'axios-nx', version: '1.0.0', description: 'HTTP client', author: 'Nexora Net', keywords: ['axios','http','promise'], downloads: 34200, stars: 850 },
  'grpc-nx': { name: 'grpc-nx', version: '1.0.0', description: 'gRPC client/server', author: 'Nexora Net', keywords: ['grpc','rpc','proto'], downloads: 7800, stars: 235 },
  'mqtt-nx': { name: 'mqtt-nx', version: '1.0.0', description: 'MQTT client', author: 'Nexora Net', keywords: ['mqtt','iot','message'], downloads: 6200, stars: 188 },
  'amqp-nx': { name: 'amqp-nx', version: '1.0.0', description: 'RabbitMQ client', author: 'Nexora Net', keywords: ['amqp','rabbitmq','queue'], downloads: 4800, stars: 145 },
  'dns-nx': { name: 'dns-nx', version: '1.0.0', description: 'DNS utilities', author: 'Nexora Net', keywords: ['dns','resolve','lookup'], downloads: 5100, stars: 155 },
  'ping-nx': { name: 'ping-nx', version: '1.0.0', description: 'Network ping', author: 'Nexora Net', keywords: ['ping','icmp','network'], downloads: 7400, stars: 225 },
  'ssh-nx': { name: 'ssh-nx', version: '1.0.0', description: 'SSH client', author: 'Nexora Net', keywords: ['ssh','remote','shell'], downloads: 8900, stars: 268 },
  'ftp-nx': { name: 'ftp-nx', version: '1.0.0', description: 'FTP client', author: 'Nexora Net', keywords: ['ftp','transfer','upload'], downloads: 5600, stars: 170 },
  'smtp-nx': { name: 'smtp-nx', version: '1.0.0', description: 'Email sending', author: 'Nexora Net', keywords: ['smtp','email','mail'], downloads: 10200, stars: 308 },

  // === CMS ===
  'wordpress-nx': { name: 'wordpress-nx', version: '1.0.0', description: 'WordPress API client', author: 'Nexora CMS', keywords: ['wordpress','cms','blog'], downloads: 7800, stars: 235 },
  'strapi-nx': { name: 'strapi-nx', version: '1.0.0', description: 'Strapi CMS client', author: 'Nexora CMS', keywords: ['strapi','cms','headless'], downloads: 5400, stars: 165 },
  'sanity-nx': { name: 'sanity-nx', version: '1.0.0', description: 'Sanity CMS client', author: 'Nexora CMS', keywords: ['sanity','cms','content'], downloads: 4200, stars: 128 },
  'contentful-nx': { name: 'contentful-nx', version: '1.0.0', description: 'Contentful client', author: 'Nexora CMS', keywords: ['contentful','cms','api'], downloads: 6100, stars: 185 },

  // === PAYMENTS ===
  'stripe-nx': { name: 'stripe-nx', version: '1.0.0', description: 'Stripe API client', author: 'Nexora Pay', keywords: ['stripe','payment','checkout'], downloads: 16800, stars: 505 },
  'paypal-nx': { name: 'paypal-nx', version: '1.0.0', description: 'PayPal integration', author: 'Nexora Pay', keywords: ['paypal','payment','checkout'], downloads: 9400, stars: 282 },
  'square-nx': { name: 'square-nx', version: '1.0.0', description: 'Square payments', author: 'Nexora Pay', keywords: ['square','payment','pos'], downloads: 5200, stars: 158 },
  'invoice-nx': { name: 'invoice-nx', version: '1.0.0', description: 'Invoice generation', author: 'Nexora Pay', keywords: ['invoice','billing','pdf'], downloads: 7800, stars: 235 },

  // === SOCIAL ===
  'twitter-nx': { name: 'twitter-nx', version: '1.0.0', description: 'Twitter/X API client', author: 'Nexora Social', keywords: ['twitter','x','social'], downloads: 11200, stars: 335 },
  'github-nx': { name: 'github-nx', version: '1.0.0', description: 'GitHub API client', author: 'Nexora Social', keywords: ['github','git','repos'], downloads: 18500, stars: 555 },
  'youtube-nx': { name: 'youtube-nx', version: '1.0.0', description: 'YouTube API client', author: 'Nexora Social', keywords: ['youtube','video','stream'], downloads: 8900, stars: 268 },
  'twitch-nx': { name: 'twitch-nx', version: '1.0.0', description: 'Twitch API client', author: 'Nexora Social', keywords: ['twitch','stream','chat'], downloads: 6400, stars: 195 },
  'reddit-nx': { name: 'reddit-nx', version: '1.0.0', description: 'Reddit API client', author: 'Nexora Social', keywords: ['reddit','posts','community'], downloads: 7200, stars: 218 },
  'telegram-nx': { name: 'telegram-nx', version: '1.0.0', description: 'Telegram bot API', author: 'Nexora Social', keywords: ['telegram','bot','chat'], downloads: 12800, stars: 385 },
  'whatsapp-nx': { name: 'whatsapp-nx', version: '1.0.0', description: 'WhatsApp Business API', author: 'Nexora Social', keywords: ['whatsapp','messaging','business'], downloads: 9100, stars: 275 },
  'slack-nx': { name: 'slack-nx', version: '1.0.0', description: 'Slack API client', author: 'Nexora Social', keywords: ['slack','workspace','bot'], downloads: 10500, stars: 315 },
  'linkedin-nx': { name: 'linkedin-nx', version: '1.0.0', description: 'LinkedIn API client', author: 'Nexora Social', keywords: ['linkedin','professional','network'], downloads: 5800, stars: 175 },

  // === PRODUCTIVITY ===
  'notion-nx': { name: 'notion-nx', version: '1.0.0', description: 'Notion API client', author: 'Nexora Productivity', keywords: ['notion','docs','wiki'], downloads: 8400, stars: 255 },
  'trello-nx': { name: 'trello-nx', version: '1.0.0', description: 'Trello API client', author: 'Nexora Productivity', keywords: ['trello','kanban','board'], downloads: 6200, stars: 188 },
  'jira-nx': { name: 'jira-nx', version: '1.0.0', description: 'Jira API client', author: 'Nexora Productivity', keywords: ['jira','agile','issues'], downloads: 7800, stars: 235 },
  'calendar-nx': { name: 'calendar-nx', version: '1.0.0', description: 'Calendar utilities', author: 'Nexora Productivity', keywords: ['calendar','events','schedule'], downloads: 9200, stars: 278 },
  'todo-nx': { name: 'todo-nx', version: '1.0.0', description: 'Todo list utilities', author: 'Nexora Productivity', keywords: ['todo','task','list'], downloads: 11400, stars: 345 },

  // === IOT ===
  'mqtt-iot-nx': { name: 'mqtt-iot-nx', version: '1.0.0', description: 'MQTT for IoT', author: 'Nexora IoT', keywords: ['mqtt','iot','sensor'], downloads: 5400, stars: 165 },
  'bluetooth-nx': { name: 'bluetooth-nx', version: '1.0.0', description: 'Bluetooth Low Energy', author: 'Nexora IoT', keywords: ['bluetooth','ble','device'], downloads: 4100, stars: 125 },
  'serial-nx': { name: 'serial-nx', version: '1.0.0', description: 'Serial port communication', author: 'Nexora IoT', keywords: ['serial','uart','hardware'], downloads: 3800, stars: 115 },
  'gpio-nx': { name: 'gpio-nx', version: '1.0.0', description: 'GPIO control', author: 'Nexora IoT', keywords: ['gpio','raspberry','pi'], downloads: 4500, stars: 138 },

  // === SCIENTIFIC ===
  'math-nx': { name: 'math-nx', version: '1.0.0', description: 'Advanced math library', author: 'Nexora Sci', keywords: ['math','linear','algebra'], downloads: 12300, stars: 370 },
  'stats-nx': { name: 'stats-nx', version: '1.0.0', description: 'Statistics library', author: 'Nexora Sci', keywords: ['statistics','data','analysis'], downloads: 8900, stars: 268 },
  'chart-science-nx': { name: 'chart-science-nx', version: '1.0.0', description: 'Scientific plotting', author: 'Nexora Sci', keywords: ['plot','graph','science'], downloads: 6200, stars: 188 },
  'matrix-nx': { name: 'matrix-nx', version: '1.0.0', description: 'Matrix operations', author: 'Nexora Sci', keywords: ['matrix','linear','algebra'], downloads: 5400, stars: 165 },
  'fft-nx': { name: 'fft-nx', version: '1.0.0', description: 'Fast Fourier Transform', author: 'Nexora Sci', keywords: ['fft','signal','frequency'], downloads: 3200, stars: 98 },

  // === GAMES EXTRA ===
  'chess-nx': { name: 'chess-nx', version: '1.0.0', description: 'Chess engine', author: 'Nexora Games', keywords: ['chess','engine','ai'], downloads: 4800, stars: 148 },
  'poker-nx': { name: 'poker-nx', version: '1.0.0', description: 'Poker utilities', author: 'Nexora Games', keywords: ['poker','cards','game'], downloads: 3500, stars: 108 },
  'dice-nx': { name: 'dice-nx', version: '1.0.0', description: 'Dice rolling', author: 'Nexora Games', keywords: ['dice','random','rpg'], downloads: 5200, stars: 158 },
  'rng-nx': { name: 'rng-nx', version: '1.0.0', description: 'Random number generators', author: 'Nexora Games', keywords: ['rng','random','generate'], downloads: 8400, stars: 255 },
  'noise-nx': { name: 'noise-nx', version: '1.0.0', description: 'Perlin/Simplex noise', author: 'Nexora Games', keywords: ['noise','perlin','terrain'], downloads: 4100, stars: 125 },
  'tilemap-nx': { name: 'tilemap-nx', version: '1.0.0', description: 'Tilemap utilities', author: 'Nexora Games', keywords: ['tilemap','2d','grid'], downloads: 3800, stars: 115 },
  'physics-nx': { name: 'physics-nx', version: '1.0.0', description: '2D physics engine', author: 'Nexora Games', keywords: ['physics','2d','collision'], downloads: 5600, stars: 172 },
  'pathfinding-nx': { name: 'pathfinding-nx', version: '1.0.0', description: 'Pathfinding algorithms', author: 'Nexora Games', keywords: ['pathfinding','astar','ai'], downloads: 6200, stars: 188 },
  'ecs-nx': { name: 'ecs-nx', version: '1.0.0', description: 'Entity Component System', author: 'Nexora Games', keywords: ['ecs','architecture','game'], downloads: 4500, stars: 138 },
  'sprite-nx': { name: 'sprite-nx', version: '1.0.0', description: 'Sprite animation', author: 'Nexora Games', keywords: ['sprite','animation','2d'], downloads: 5100, stars: 155 },
  'audio-nx': { name: 'audio-nx', version: '1.0.0', description: 'Audio playback', author: 'Nexora Games', keywords: ['audio','sound','music'], downloads: 7200, stars: 218 },
  'input-nx': { name: 'input-nx', version: '1.0.0', description: 'Input handling', author: 'Nexora Games', keywords: ['input','keyboard','mouse'], downloads: 6800, stars: 208 },
  'collision-nx': { name: 'collision-nx', version: '1.0.0', description: 'Collision detection', author: 'Nexora Games', keywords: ['collision','aabb','detection'], downloads: 4900, stars: 150 },
  'particle-nx': { name: 'particle-nx', version: '1.0.0', description: 'Particle system', author: 'Nexora Games', keywords: ['particle','effect','visual'], downloads: 5400, stars: 165 },
  'tween-nx': { name: 'tween-nx', version: '1.0.0', description: 'Tween animations', author: 'Nexora Games', keywords: ['tween','animation','ease'], downloads: 6100, stars: 185 },
};

const server = http.createServer((req, res) => {
  res.setHeader('Content-Type', 'application/json');
  res.setHeader('Access-Control-Allow-Origin', '*');
  if (req.method === 'OPTIONS') { res.writeHead(200); res.end(); return; }

  const url = new URL(req.url, `http://localhost:${PORT}`);

  if (url.pathname === '/api/health') {
    res.writeHead(200);
    res.end(JSON.stringify({ status: 'ok', version: '1.0.0', packages: Object.keys(packages).length }));
    return;
  }

  if (url.pathname === '/api/packages/search') {
    const query = url.searchParams.get('q') || '';
    const results = Object.values(packages).filter(p =>
      p.name.includes(query) || p.description.includes(query) || p.keywords.some(k => k.includes(query))
    );
    res.writeHead(200);
    res.end(JSON.stringify({ packages: results, total: results.length }));
    return;
  }

  const pkgMatch = url.pathname.match(/^\/api\/packages\/([^/]+)$/);
  if (pkgMatch && req.method === 'GET') {
    const name = pkgMatch[1];
    const pkg = packages[name];
    if (pkg) {
      const dir = getPackageDir(name);
      if (dir) {
        const configPath = path.join(dir, 'nexora.json');
        if (fs.existsSync(configPath)) {
          const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
          res.writeHead(200);
          res.end(JSON.stringify({ ...pkg, ...config }));
          return;
        }
      }
      res.writeHead(200);
      res.end(JSON.stringify(pkg));
    } else {
      res.writeHead(404);
      res.end(JSON.stringify({ error: 'Package not found' }));
    }
    return;
  }

  const dlMatch = url.pathname.match(/^\/api\/packages\/([^/]+)\/download$/);
  if (dlMatch && req.method === 'GET') {
    const name = dlMatch[1];
    const dir = getPackageDir(name);
    if (dir) {
      const files = [];
      const walk = (d, rel = '') => {
        const entries = fs.readdirSync(d, { withFileTypes: true });
        for (const e of entries) {
          const fp = path.join(d, e.name);
          const rp = rel ? `${rel}/${e.name}` : e.name;
          if (e.isDirectory()) walk(fp, rp);
          else if (e.name !== 'nexora.json') files.push({ path: rp, content: fs.readFileSync(fp, 'utf8') });
        }
      };
      walk(dir);
      res.writeHead(200);
      res.end(JSON.stringify({ files }));
    } else {
      // Generate stub package
      res.writeHead(200);
      res.end(JSON.stringify({ files: [
        { path: 'index.nx', content: `// ${name}\n// Auto-generated package\n\nexport default {\n  name: "${name}",\n  version: "1.0.0"\n}` },
        { path: 'README.md', content: `# ${name}\n\n${packages[name]?.description || 'Nexora package'}` }
      ]}));
    }
    return;
  }

  if (url.pathname === '/api/packages') {
    res.writeHead(200);
    res.end(JSON.stringify({ packages: Object.values(packages), total: Object.keys(packages).length }));
    return;
  }

  res.writeHead(404);
  res.end(JSON.stringify({ error: 'Not found' }));
});

function getPackageDir(name) {
  try {
    const dirs = fs.readdirSync(PACKAGES_DIR);
    for (const dir of dirs) {
      const pkgPath = path.join(PACKAGES_DIR, dir, 'nexora.json');
      if (fs.existsSync(pkgPath)) {
        const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
        if (pkg.name === name) return path.join(PACKAGES_DIR, dir);
      }
    }
  } catch (e) {}
  return null;
}

server.listen(PORT, () => {
  console.log(`Nexora Registry running on http://localhost:${PORT}`);
  console.log(`Packages available: ${Object.keys(packages).length}`);
});
