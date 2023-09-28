#!/bin/sh
set -Cefu

get_sys() {
	uname=$(uname -ms)
	platform="${uname%% *}"
	arch="${uname#* }"

	case $platform in
	Darwin*)
		os="$(sw_vers -n) $(sw_vers -v)"
		model=$(sysctl -n hw.model)
		;;
	FreeBSD* | OpenBSD* | NetBSD*)
		os="$(sw_vers -n) $(sw_vers -v)"
		model=$(sw_vers -n)
		;;
	Linux*)
		sysd_rel='/etc/os-release'
		vendor='/sys/devices/virtual/dmi/id/board_vendor'

		# shellcheck disable=SC1090
		[ -f "$sysd_rel" ] && . "$sysd_rel" && os="$PRETTY_NAME"
		[ -f "$vendor" ] && read -r model < "$vendor"
		;;
	esac
}
