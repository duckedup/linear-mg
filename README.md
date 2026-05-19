# linear-wp

A Rust CLI and library for the [Linear](https://linear.app) GraphQL API, designed for AI agent consumption.

Output is human-readable by default, with `--json` for structured JSON output with distinct exit codes per error category, making it straightforward to parse from scripts and agent toolchains.

## Install

```sh
cargo binstall linear-wp
```

Or from source:

```sh
cargo install linear-wp
```

## Development

[just](https://github.com/casey/just) is used as the task runner. Install it with:

```sh
# macOS
brew install just

# cargo
cargo install just
```

Available commands:

```sh
just fmt          # Format code
just fmt-check    # Check formatting
just lint         # Run clippy lints
just clippy       # Alias for lint
just build        # Build the project
just test         # Run tests
just check        # Run all checks (fmt, lint, test)
just publish-dry  # Dry run publish to crates.io
just refresh-schema  # Download latest Linear GraphQL schema
```

CI runs `just check` on every pull request and requires a version bump in `Cargo.toml` before merging.

## Authentication

```sh
# Interactive setup — prompts for your API key and validates it
linear-wp auth init

# Or pass the key directly
linear-wp auth init --key lin_api_xxxxx

# Check who you're authenticated as
linear-wp auth status

# Remove stored credentials
linear-wp auth revoke
```

You can also set your API key via environment variable:

```sh
export LINEAR_API_KEY=lin_api_xxxxx
```

Or override per-command:

```sh
linear-wp --api-key lin_api_xxxxx issues list
```

Resolution order: `--api-key` flag > `LINEAR_API_KEY` env var > config file.

## Usage

```
linear-wp [OPTIONS] <COMMAND>

Commands:
  auth         Manage authentication
  issues       Manage issues
  teams        Manage teams
  projects     Manage projects
  users        Manage users
  comments     Manage comments
  labels       Manage issue labels
  cycles       Manage cycles
  states       Manage workflow states
  documents    Manage documents
  initiatives  Manage initiatives
  milestones   Manage project milestones
  attachments  Manage attachments

Options:
      --json               Output as JSON
      --api-key <API_KEY>  Linear API key
  -v, --verbose            Enable debug logging
```

### Issues

```sh
# List issues for a team
linear-wp issues list --team ENG --limit 10

# Filter by state and assignee
linear-wp issues list --team ENG --state "In Progress" --assignee me

# Get a single issue by identifier
linear-wp issues get ENG-123

# Create an issue (accepts names, emails, "me" — IDs resolved automatically)
linear-wp issues create --team ENG --title "Fix login bug" --priority 1 --assignee me --state "In Progress"

# Update an issue (--state and --assignee accept names, not just IDs)
linear-wp issues update ENG-123 --state "In Progress" --assignee me

# Add/remove labels
linear-wp issues update ISSUE_ID --add-labels LABEL_ID_1,LABEL_ID_2

# Archive or delete
linear-wp issues archive ISSUE_ID
linear-wp issues delete ISSUE_ID

# Search
linear-wp issues search "login bug"
```

### Teams & Users

```sh
linear-wp teams list
linear-wp teams get TEAM_ID

linear-wp users list
linear-wp users me        # Current authenticated user
linear-wp users get USER_ID
```

### Projects

```sh
linear-wp projects list
linear-wp projects create --name "Q3 Roadmap" --teams TEAM_ID_1,TEAM_ID_2
linear-wp projects update PROJECT_ID --name "Q3 Roadmap v2"
linear-wp projects archive PROJECT_ID
```

### Comments

```sh
linear-wp comments list
linear-wp comments create --issue ENG-123 --body "This is a comment"
linear-wp comments update COMMENT_ID --body "Updated comment"
linear-wp comments delete COMMENT_ID
```

### Workflow States, Labels, Cycles

```sh
# List all workflow states (useful for finding state IDs)
linear-wp states list

# List and create labels
linear-wp labels list
linear-wp labels create --name "P0" --color "#FF0000"

# List cycles
linear-wp cycles list
linear-wp cycles get CYCLE_ID
```

### Documents, Initiatives, Milestones, Attachments

```sh
linear-wp documents list
linear-wp documents create --title "Design Doc" --content "# Overview\n..."

linear-wp initiatives list
linear-wp initiatives get INITIATIVE_ID

linear-wp milestones list
linear-wp milestones create --name "Beta launch" --project PROJECT_ID

linear-wp attachments list
linear-wp attachments create --issue ENG-123 --title "PR Link" --url "https://github.com/..."
linear-wp attachments delete ATTACHMENT_ID
```

### Pagination

All list commands support pagination:

```sh
# Limit results (default: 50, max: 250)
linear-wp issues list --limit 10

# Auto-paginate through all results
linear-wp issues list --all

# Manual cursor-based pagination
linear-wp issues list --limit 50 --after CURSOR_VALUE

# Include archived items
linear-wp issues list --include-archived

# Order by updated time instead of created time
linear-wp issues list --order-by updated-at
```

## Output Format

Output is human-readable by default. Use `--json` for structured JSON output.

```sh
# Pretty (default)
linear-wp issues list --team ENG

# JSON for scripts and agents
linear-wp issues list --team ENG --json
```

**List responses** (JSON) include pagination info:

```json
{
  "nodes": [ ... ],
  "pageInfo": {
    "hasNextPage": false,
    "endCursor": "abc123"
  }
}
```

**Mutation responses** (JSON) include a success flag:

```json
{
  "success": true,
  "data": { ... }
}
```

**Errors** are written to stderr with distinct exit codes:

| Exit Code | Meaning |
|-----------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Authentication error / no API key |
| 3 | Rate limited |
| 4 | Not found |
| 5 | Invalid input |
| 6 | API / GraphQL error |

## License

MIT
