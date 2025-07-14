#!/bin/bash

set -e

if [ ! -f .env ]; then
    echo "Error: .env file not found"
    exit 1
fi

source .env

if [ -z "$DATABASE_URL" ]; then
    echo "Error: DATABASE_URL not set in .env file"
    exit 1
fi

case "$1" in
    "run")
        echo "Running migrations..."
        sqlx migrate run --database-url "$DATABASE_URL"
        ;;
    "revert")
        echo "Reverting last migration..."
        sqlx migrate revert --database-url "$DATABASE_URL"
        ;;
    "info")
        echo "Migration status:"
        sqlx migrate info --database-url "$DATABASE_URL"
        ;;
    "add")
        if [ -z "$2" ]; then
            echo "Usage: $0 add <migration_name>"
            exit 1
        fi
        echo "Creating new migration: $2"
        sqlx migrate add "$2" --database-url "$DATABASE_URL"
        ;;
    *)
        echo "Usage: $0 {run|revert|info|add <name>}"
        echo ""
        echo "Commands:"
        echo "  run     - Run all pending migrations"
        echo "  revert  - Revert the last applied migration"
        echo "  info    - Show migration status"
        echo "  add     - Create a new migration file"
        exit 1
        ;;
esac