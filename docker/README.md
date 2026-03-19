# LeanSpec Docker

Run the LeanSpec UI in a Docker container — useful for CI/CD, team self-hosting, and cloud deployment.

## Quick Start

### Using Docker Compose (recommended)

```sh
docker compose up
```

Open http://localhost:3000 in your browser. Projects are managed through the UI — use the project discovery or add projects manually.

### Using Docker directly

```sh
docker pull ghcr.io/codervisor/leanspec:latest

docker run -p 3000:3000 \
  -v leanspec-data:/home/leanspec/.lean-spec \
  ghcr.io/codervisor/leanspec:latest
```

## Mounting Project Directories

Bind-mount host directories to make them visible inside the container:

```sh
docker run -p 3000:3000 \
  -v leanspec-data:/home/leanspec/.lean-spec \
  -v /path/to/project-a:/projects/project-a:ro \
  -v /path/to/project-b:/projects/project-b:ro \
  ghcr.io/codervisor/leanspec:latest
```

To auto-register a project on startup, pass `--project`:

```sh
docker run -p 3000:3000 \
  -v leanspec-data:/home/leanspec/.lean-spec \
  -v /path/to/my-project:/projects/my-project:ro \
  ghcr.io/codervisor/leanspec:latest \
  --project /projects/my-project
```

Or in `docker-compose.yml`:

```yaml
services:
  leanspec:
    image: ghcr.io/codervisor/leanspec:latest
    ports:
      - "3000:3000"
    volumes:
      - leanspec-data:/home/leanspec/.lean-spec
      - /path/to/project-a:/projects/project-a:ro
      - /path/to/project-b:/projects/project-b:ro
```

Once mounted, projects can be discovered and registered through the UI.

## Data Persistence

LeanSpec stores its data in `~/.lean-spec/` inside the container (`/home/leanspec/.lean-spec/`):

| File | Description |
|------|-------------|
| `config.json` | Server and UI configuration |
| `projects.json` | Registered project registry |
| `leanspec.db` | SQLite database (sessions, chat) |

Mount a volume at `/home/leanspec/.lean-spec` to persist data across container restarts.

## Configuration

| Option | Description |
|--------|-------------|
| `--project <path>` | Auto-register a mounted directory as a project on startup |
| `--host 0.0.0.0` | Bind all network interfaces (included by default) |
| `--no-open` | Skip browser launch (included by default) |
| `PORT` env var | Override the port (default: `3000`) |

### Custom port example

```sh
docker run -p 8080:8080 \
  -e PORT=8080 \
  -v leanspec-data:/home/leanspec/.lean-spec \
  ghcr.io/codervisor/leanspec:latest
```

## Security

The container runs as a non-root user (`leanspec`). Project directories can be mounted read-only (`:ro`) if the server only needs to read specs.

## Building Locally

```sh
docker build -t leanspec docker/
docker run -p 3000:3000 -v leanspec-data:/home/leanspec/.lean-spec leanspec
```

## Image

The image is published to GitHub Container Registry:

```
ghcr.io/codervisor/leanspec:latest
ghcr.io/codervisor/leanspec:<version>   # e.g. 0.2.27
```

The image uses a two-stage build:
- **Builder stage** (`node:20-slim`): installs `@leanspec/http-linux-x64` and `@leanspec/ui` from npm
- **Runtime stage** (`debian:12-slim`): copies only the Rust binary and pre-built UI static files — no Node at runtime

No Rust compilation happens at build time.
