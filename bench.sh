#!/bin/sh
set -Cefu

cleanup() {
	rm -r bench_build.md bench_exec.md bench_size.md bench_setup.md \
		tmp target/ 2>/dev/null
}

trap cleanup EXIT

cargo build
hyperfine -N -w32 -r256 --export-markdown bench_exec.md \
	'./target/debug/thiserror' \
	'./target/debug/custom_error' \
	'./target/debug/build_domain_error'

cargo build
hyperfine -N -w16 -r64 --export-markdown bench_build.md \
	-p "find . ! -mindepth 1 -name 'deps' -exec rm -r {} \;" \
	-L bin thiserror,custom_error,build_domain_error 'cargo build --bin {bin}'

{ printf '%s\n' \
	"| Command | Lines | Bytes" \
	"| ------- | ----: | ----:"
} > bench_size.md

for file in 'custom_error' 'thiserror' 'build_domain_error'
do
	{ wc ./src/bin/"$file".rs; } | {
		read -r _ lines bytes _
		printf "| %s | %s | %s |\n" "\`$file\`" \
			"$lines" "$bytes"
	}
done >> bench_size.md

info() {
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

	printf '\n'
	cat <<- EOF
		- Hardware: \`$model\`
		- OS: \`$os\`
		- CPU arch: \`$arch\`
		- Rust: \`$rs_ver\`
	EOF
}

info >| bench_setup.md

{
	cat <<- EOF
	## Benchmarks

	#### Compile time
	$(cat bench_build.md)

	#### Runtime
	$(cat bench_exec.md)

	#### Size
	$(cat bench_size.md)

	#### Setup
	$(cat bench_setup.md)
	EOF
} | sed -e 's:\./target/debug/::g' \
		-e 's/cargo build --bin//g' |
		deno fmt --ext md - >| bench.md
