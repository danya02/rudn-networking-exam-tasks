# DNS resolve

This task is to manually resolve a DNS address, manually issuing queries to virtual name servers.

The world is represented by a `Session` object. This is seeded with a random string, and is used to generate the state of the world,
such as IP addresses for specific servers.
The algorithm that does this is deterministic, using the `Session`'s key, as well as a secret that's loaded from disk.

## Data sources

The TLD list is taken from the ICANN root zone file: https://www.internic.net/zones/root.zone

The lower domain names are taken from the Majestic Million, a dataset maintained by Majestic-12 Ltd: https://downloads.majestic.com/majestic_million.csv
