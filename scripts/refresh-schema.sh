#!/usr/bin/env bash
set -euo pipefail

SCHEMA_URL="https://raw.githubusercontent.com/linear/linear/master/packages/sdk/src/schema.graphql"
SCHEMA_PATH="$(dirname "$0")/../schema/linear.graphql"

echo "Downloading Linear GraphQL schema..."
curl -sL "$SCHEMA_URL" -o "$SCHEMA_PATH"
echo "Schema downloaded to $SCHEMA_PATH ($(wc -l < "$SCHEMA_PATH" | tr -d ' ') lines)"
