#!/bin/sh
# cargo, deno, hyperfine, rustc
set -Cefu

cleanup() {
	rm -r compile_time.md run_time.md code_size.md setup_info.md tmp target/
} 2>/dev/null

trap cleanup EXIT

compile_time() {
	cargo build
	hyperfine -N -w16 -r64 --export-markdown compile_time.md \
		-p "find . ! -mindepth 1 -name 'deps' -exec rm -r {} \;" \
		-L bin thiserror,custom_error,build_domain_error 'cargo build --bin {bin}'
}

run_time() {
	cargo build
	hyperfine -N -w32 -r256 --export-markdown run_time.md \
		'./target/debug/thiserror' \
		'./target/debug/custom_error' \
		'./target/debug/build_domain_error'
}

code_size() {
	{
		printf '%s\n' \
			"| Command | Lines | Bytes" \
			"| ------- | ----: | ----:"
	} > code_size.md

	for file in 'custom_error' 'thiserror' 'build_domain_error'
	do
		{ wc ./src/bin/"$file".rs; } | {
			read -r _ lines bytes _
			printf "| %s | %s | %s |\n" "\`$file\`" \
				"$lines" "$bytes"
		}
	done >> code_size.md
}

setup_info() {
	uname=$(uname -ms)
	platform="${uname%% *}"
	arch="${uname#* }"

	case $platform in
	Darwin)
		os="$(sw_vers -n) $(sw_vers -v)"
		model=$(system_profiler SPHardwareDataType |
			awk '/Model Identifier/ {print $3}')
	esac

	rs_ver=$(rustc -V | cut -d' ' -f2)

	cat <<- EOF
		- Hardware: \`$model\`
		- OS: \`$os\`
		- CPU arch: \`$arch\`
		- Rust: \`$rs_ver\`
	EOF
}

compile_time
run_time
code_size
setup_info >| setup_info.md

{
	printf '%s\n' '## Benchmarks'
	printf '%s\n' '## Compile Time' && cat compile_time.md
	printf '%s\n' '## Runtime'      && cat run_time.md
	printf '%s\n' '## Code Size'    && cat code_size.md
	printf '%s\n' '## Setup Info'   && cat setup_info.md
} | sed -e 's:\./target/debug/::g' \
		-e 's/cargo build --bin//g' |
		deno fmt --ext md - >| bench.md
