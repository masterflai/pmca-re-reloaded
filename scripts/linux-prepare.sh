#!/bin/sh
# Prepare a Linux host for talking to a Sony camera over raw USB.
#
# Two things get in the way on Linux, and neither is a fault of the toolkit:
#
#   1. Permissions. Raw USB access needs more than the default 0600 root-only
#      node, so a udev rule hands the device to the logged-in user.
#   2. Drivers that already hold the interface. In mass-storage mode the
#      kernel's `usb-storage` binds interface 0, and `claim_interface` then
#      fails with EBUSY. In app-install mode nothing binds in the kernel (PTP
#      is served from user space), but the desktop's gvfs monitors open the
#      device on sight and must be stopped.
#
# Run with: sh linux-prepare.sh [rules|unbind|gvfs|all]

set -eu

VENDOR=054c
RULES=/etc/udev/rules.d/99-pmca-re-reloaded.rules

install_rules() {
	echo "==> installing $RULES"
	# `uaccess` is what gives the seat's active user access; MODE alone would
	# only widen the permissions for a group nobody is in.
	printf 'SUBSYSTEM=="usb", ATTR{idVendor}=="%s", MODE="0660", TAG+="uaccess"\n' "$VENDOR" \
		| sudo tee "$RULES" > /dev/null
	sudo udevadm control --reload-rules
	sudo udevadm trigger
	echo "    reconnect the camera for this to take effect"
}

unbind_storage() {
	echo "==> unbinding usb-storage from Sony devices"
	found=0
	for path in /sys/bus/usb/drivers/usb-storage/*:*; do
		[ -e "$path" ] || continue
		iface=$(basename "$path")
		# "1-2:1.0" -> "1-2", the parent device holding the ids.
		parent=${iface%%:*}
		vid=$(cat "/sys/bus/usb/devices/$parent/idVendor" 2>/dev/null || echo "")
		[ "$vid" = "$VENDOR" ] || continue
		echo "    $iface"
		printf '%s' "$iface" | sudo tee /sys/bus/usb/drivers/usb-storage/unbind > /dev/null
		found=1
	done
	[ "$found" = 1 ] || echo "    nothing bound (fine if the camera is not in mass-storage mode)"
}

stop_gvfs() {
	echo "==> stopping the desktop's MTP/PTP monitors"
	# Absent on a headless box; a failure here is not fatal.
	systemctl --user stop gvfs-mtp-volume-monitor.service 2>/dev/null || true
	systemctl --user stop gvfs-gphoto2-volume-monitor.service 2>/dev/null || true
	pkill -f gvfsd-mtp 2>/dev/null || true
	pkill -f gvfsd-gphoto2 2>/dev/null || true
	echo "    they restart on demand; re-run this before each session"
}

case "${1:-all}" in
	rules) install_rules ;;
	unbind) unbind_storage ;;
	gvfs) stop_gvfs ;;
	all)
		install_rules
		stop_gvfs
		unbind_storage
		;;
	*)
		echo "usage: $0 [rules|unbind|gvfs|all]" >&2
		exit 2
		;;
esac

echo
echo "current Sony devices:"
lsusb -d "$VENDOR:" || echo "  none found"
