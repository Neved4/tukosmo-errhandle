#!/bin/sh
set -Cefu

black='\033[0m' red='\033[31m'

err_cmd() {
	cmd=$1
	printf '%b\n' "${red}error:${black} $cmd not found" \
		"" "Please install '$cmd'."
}

check_cmd() {
	cmd="$1"
	if ! command -v "$cmd" >/dev/null
	then
		err_cmd "$cmd"
		exit 1
	fi
}

check_deps() {
	for cmd
	do
		check_cmd "$cmd"
	done
}
