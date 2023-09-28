#!/bin/sh
# cargo, deno, hyperfine, rustc
set -Cefu

cleanup() {
	rm -r target/
} 2>/dev/null

trap cleanup EXIT

filter() {
	awk '/^\| Command | Mean /{p=1} p && !/^$/{print} p && /^$/{exit}'
}

compile_time() {
	cargo build
	hyperfine -N -w16 -r64 --export-markdown - \
		-p "find . ! -mindepth 1 -name 'deps' -exec rm -r {} \;" \
		-L bin thiserror,custom_error,build_domain_error \
		'cargo build --bin {bin}' | filter
}

run_time() {
	cargo build
	hyperfine -N -w32 -r256 --export-markdown - \
		'./target/debug/thiserror' \
		'./target/debug/custom_error' \
		'./target/debug/build_domain_error' | filter
}

code_size() {
	{
		printf '%s\n' \
			"| Command | Lines | Bytes" \
			"| ------- | ----: | ----:"
	}

	for file in 'custom_error' 'thiserror' 'build_domain_error'
	do
		{ wc ./src/bin/"$file".rs; } | {
			read -r _ lines bytes _
			printf "| %s | %s | %s |\n" "\`$file\`" \
				"$lines" "$bytes"
		}
	done
}

setup_info() {
	uname=$(uname -ms)
	platform="${uname%% *}"
	arch="${uname#* }"

	case $platform in
	Darwin)
		os="$(sw_vers -n) $(sw_vers -v)"
		model=$(sysctl -n hw.model)
	esac

	rs_ver=$(rustc -V | cut -d' ' -f2)

	cat <<- EOF
		- Hardware: \`$model\`
		- OS: \`$os\`
		- CPU arch: \`$arch\`
		- Rust: \`$rs_ver\`
	EOF
}

{
	printf '%s\n' '## Benchmarks'            \
		'### Compile Time' "$(compile_time)" \
		'### Runtime'      "$(run_time)"     \
		'### Code Size'    "$(code_size)"    \
		'### Setup Info'   "$(setup_info)"
} | sed -e 's:\./target/debug/::g' \
		-e 's/cargo build --bin//g' |
		deno fmt --ext md - >| bench.md
