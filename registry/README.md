# Nexora Registry Server

Registry server for Nexora packages, similar to npm registry.

## Setup

```bash
# Install dependencies
npm install

# Configure environment
cp .env.example .env

# Start services
docker-compose up -d

# Run migrations
npm run migrate

# Start server
npm run start
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/packages | List all packages |
| GET | /api/packages/:name | Get package info |
| GET | /api/packages/:name/:version | Get specific version |
| POST | /api/packages | Create package (auth) |
| PUT | /api/packages/:name | Update package (auth) |
| DELETE | /api/packages/:name | Delete package (auth) |
| GET | /api/search?q=query | Search packages |
| GET | /api/users/:user | Get user packages |
| POST | /api/auth/register | Register user |
| POST | /api/auth/login | Login |
| GET | /api/health | Health check |

## Configuration

Environment variables:
- `DB_PASSWORD` - PostgreSQL password
- `REDIS_PASSWORD` - Redis password
- `JWT_SECRET` - JWT signing secret
- `GITHUB_TOKEN` - GitHub API token
- `AWS_ACCESS_KEY_ID` - S3 access key
- `AWS_SECRET_ACCESS_KEY` - S3 secret key