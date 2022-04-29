#!/usr/bin/env sh

command="$1"
shift
near "$command" "$(<./neardev/dev-account)" "$@"
