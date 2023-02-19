# This script cleans the root.zone file, removing all records that are not NS.
data = open('root.zone').readlines()
with open('root.zone', 'w') as o:
    for row in data:
        if 'IN$$NS$$' not in '$$'.join(row.split()):
            continue
        o.write(row)
