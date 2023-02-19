update: clean
	wget https://www.internic.net/zones/root.zone
	wget https://downloads.majestic.com/majestic_million.csv
	python3 clean_root_zone.py

secret:
	dd if=/dev/urandom bs=1 count=512 of=secret.bin