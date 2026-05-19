# linear-mg

A Rust CLI and library for the [Linear](https://linear.app) GraphQL API, designed for AI agent consumption.

Output is human-readable by default, with `--json` for structured JSON output with distinct exit codes per error category, making it straightforward to parse from scripts and agent toolchains.

## Install

```sh
cargo binstall linear-mg
```

Or from source:

```sh
cargo install linear-mg
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
linear-mg auth init

# Or pass the key directly
linear-mg auth init --key lin_api_xxxxx

# Check who you're authenticated as
linear-mg auth status

# Remove stored credentials
linear-mg auth revoke
```

You can also set your API key via environment variable:

```sh
export LINEAR_API_KEY=lin_api_xxxxx
```

Or override per-command:

```sh
linear-mg --api-key lin_api_xxxxx issues list
```

Resolution order: `--api-key` flag > `LINEAR_API_KEY` env var > config file.

## Usage

```
linear-mg [OPTIONS] <COMMAND>

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
linear-mg issues list --team ENG --limit 10

# Filter by state and assignee
linear-mg issues list --team ENG --state "In Progress" --assignee me

# Get a single issue by identifier
linear-mg issues get ENG-123

# Create an issue (accepts names, emails, "me" — IDs resolved automatically)
linear-mg issues create --team ENG --title "Fix login bug" --priority 1 --assignee me --state "In Progress"

# Update an issue (--state and --assignee accept names, not just IDs)
linear-mg issues update ENG-123 --state "In Progress" --assignee me

# Add/remove labels
linear-mg issues update ISSUE_ID --add-labels LABEL_ID_1,LABEL_ID_2

# Archive or delete
linear-mg issues archive ISSUE_ID
linear-mg issues delete ISSUE_ID

# Search
linear-mg issues search "login bug"
```

### Teams & Users

```sh
linear-mg teams list
linear-mg teams get TEAM_ID

linear-mg users list
linear-mg users me        # Current authenticated user
linear-mg users get USER_ID
```

### Projects

```sh
linear-mg projects list
linear-mg projects create --name "Q3 Roadmap" --teams TEAM_ID_1,TEAM_ID_2
linear-mg projects update PROJECT_ID --name "Q3 Roadmap v2"
linear-mg projects archive PROJECT_ID
```

### Comments

```sh
linear-mg comments list
linear-mg comments create --issue ENG-123 --body "This is a comment"
linear-mg comments update COMMENT_ID --body "Updated comment"
linear-mg comments delete COMMENT_ID
```

### Workflow States, Labels, Cycles

```sh
# List all workflow states (useful for finding state IDs)
linear-mg states list

# List and create labels
linear-mg labels list
linear-mg labels create --name "P0" --color "#FF0000"

# List cycles
linear-mg cycles list
linear-mg cycles get CYCLE_ID
```

### Documents, Initiatives, Milestones, Attachments

```sh
linear-mg documents list
linear-mg documents create --title "Design Doc" --content "# Overview\n..."

linear-mg initiatives list
linear-mg initiatives get INITIATIVE_ID

linear-mg milestones list
linear-mg milestones create --name "Beta launch" --project PROJECT_ID

linear-mg attachments list
linear-mg attachments create --issue ENG-123 --title "PR Link" --url "https://github.com/..."
linear-mg attachments delete ATTACHMENT_ID
```

### Pagination

All list commands support pagination:

```sh
# Limit results (default: 50, max: 250)
linear-mg issues list --limit 10

# Auto-paginate through all results
linear-mg issues list --all

# Manual cursor-based pagination
linear-mg issues list --limit 50 --after CURSOR_VALUE

# Include archived items
linear-mg issues list --include-archived

# Order by updated time instead of created time
linear-mg issues list --order-by updated-at
```

## Output Format

Output is human-readable by default. Use `--json` for structured JSON output.

```sh
# Pretty (default)
linear-mg issues list --team ENG

# JSON for scripts and agents
linear-mg issues list --team ENG --json
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
