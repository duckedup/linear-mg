# linear-mg

A Rust CLI and library for the [Linear](https://linear.app) GraphQL API, designed for AI agent consumption.

All output is structured JSON by default, with distinct exit codes per error category, making it straightforward to parse from scripts and agent toolchains.

## Install

```sh
cargo install linear-mg
```

Or build from source:

```sh
git clone https://github.com/duckedup/linear-mg.git
cd linear-mg
cargo build --release
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

Set your Linear API key via environment variable (recommended) or config file.

**Environment variable:**

```sh
export LINEAR_API_KEY=lin_api_xxxxx
```

**Config file:**

```sh
linear-mg auth login --key lin_api_xxxxx
# Stores to ~/.config/linear-mg/config.toml
```

**Per-command override:**

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
  -o, --format <FORMAT>    Output format: json (default), json-pretty
      --api-key <API_KEY>  Linear API key
  -v, --verbose            Enable debug logging
```

### Issues

```sh
# List issues for a team
linear-mg issues list --team ENG --limit 10

# Filter by state and assignee
linear-mg issues list --team ENG --state "In Progress" --assignee USER_ID

# Get a single issue by identifier
linear-mg issues get ENG-123

# Create an issue
linear-mg issues create --team ENG --title "Fix login bug" --priority 1 --assignee USER_ID

# Update an issue (change status, reassign, etc.)
linear-mg issues update ISSUE_ID --state STATE_ID --assignee NEW_USER_ID

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

All output is JSON by default. Use `--format json-pretty` for indented output.

**List responses** include pagination info:

```json
{
  "nodes": [ ... ],
  "page_info": {
    "has_next_page": false,
    "end_cursor": "abc123"
  }
}
```

**Mutation responses** include a success flag:

```json
{
  "success": true,
  "data": { ... }
}
```

**Errors** are written to stderr as JSON with distinct exit codes:

```json
{"error": {"type": "not_found", "message": "Not found: ..."}}
```

| Exit Code | Meaning |
|-----------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Authentication error / no API key |
| 3 | Rate limited |
| 4 | Not found |
| 5 | Invalid input |
| 6 | API / GraphQL error |

## Use as a Library

Add `linear-mg` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
linear-mg = "0.1"
```

```rust
use linear_mg::client::LinearClient;
use linear_mg::graphql::issues::queries::*;
use linear_mg::graphql::issues::types::*;
use cynic::QueryBuilder;

#[tokio::main]
async fn main() {
    let client = LinearClient::new("lin_api_xxxxx".to_string());

    // Get an issue
    let op = IssueByIdQuery::build(IssueByIdVariables {
        id: "ENG-123".to_string(),
    });
    let data = client.run_query(op).await.unwrap();
    println!("{}", data.issue.title);
}
```

## Schema Updates

The Linear GraphQL schema is checked in at `schema/linear.graphql`. To update it:

```sh
just refresh-schema
cargo build  # Recompiles with the new schema
```

## License

MIT
