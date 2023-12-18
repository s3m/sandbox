#!/bin/bash

# Set the destination IP and port, read from environment variables if available
DEST_IP="${DEST_IP:-195.65.233.41}"
DEST_PORT="${DEST_PORT:-443}"

# Function to disable the firewall rule
disable_firewall_rule() {
	# Check if the rule already exists
	if sudo iptables -C OUTPUT -p tcp --dport "$DEST_PORT" -d "$DEST_IP" -j REJECT; then
		# If the rule exists, delete it
		sudo iptables -D OUTPUT -p tcp --dport "$DEST_PORT" -d "$DEST_IP" -j REJECT
		echo "Firewall rule disabled for $DEST_IP:$DEST_PORT"
	else
		echo "Firewall rule does not exist for $DEST_IP:$DEST_PORT"
	fi
}

# Function to enable the firewall rule
enable_firewall_rule() {
	# Check if the rule already exists
	if ! sudo iptables -C OUTPUT -p tcp --dport "$DEST_PORT" -d "$DEST_IP" -j REJECT; then
		# If the rule does not exist, add it
		sudo iptables -A OUTPUT -p tcp --dport "$DEST_PORT" -d "$DEST_IP" -j REJECT
		echo "Firewall rule enabled for $DEST_IP:$DEST_PORT"
	else
		echo "Firewall rule is already enabled for $DEST_IP:$DEST_PORT"
	fi
}

# Check command-line argument
case "$1" in
"disable")
	disable_firewall_rule
	;;
"enable")
	enable_firewall_rule
	;;
*)
	echo "Usage: $0 [disable|enable]"
	exit 1
	;;
esac

exit 0
