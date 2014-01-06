#!/usr/bin/env python
import sys, os

sz = os.path.getsize(sys.argv[1])
if sz > 510:
    print "boot block too large: %d bytes (max 510)" % sz
else:
    print "boot block is %d bytes (max 510)" % sz
    with open(sys.argv[1], 'ab+') as f:
        f.seek(0, os.SEEK_END)
        f.write('\0'*(510-sz))
        f.write("\x55\xAA")
        f.flush()
