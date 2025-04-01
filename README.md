Script to parse the output of the Wireguard `wg show` command and assign friendly names to each peer.

The command requires a list of peer public keys and friendly names in the form...

`<base64 hash of public key>:<friendly name>`

one per line saved to `/etc/wireguard/peers`

The script will parse the output of the `wg show` command, identifying lines starting with `peer` then match the associated public key to the corresponding public key/friendly name pair in the `/etc/wireguard/peers` file, inserting the friendly name into the original output.

The command now has an update option, when specifying the update flag `-u` the script will parse the `/etc/wireguard/wg0.conf` config file and identify any line that starts `#name = `, the subsequent name will be added to the peers file along with the PublicKey specified on the following line.

The following is an example of how the wireguard peer entry should look in the config file in order for the script to properly parse the information and produce friendly names.

```

[Peer]
#name = <users name or email address etc.>
PublicKey = <users public key>
AllowedIPs = <assigned IP address>

```
It must be noted that the `name = ` parameter identifier is not a compatible wireguard variable, hence the preceeding `#` commenting it out. This information is added purely for ease of administration/the script output.

It should also be noted that this script is hardcoded to parse the default `/etc/wireguard/wg0.conf` wireguard configuration file, if you use alternative configuration files, update line 80 of `src/main.rs` .
 
