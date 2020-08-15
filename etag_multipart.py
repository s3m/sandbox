import os
import sys
from hashlib import md5

def calc_etag(inputfile, partsize):
    md5_digests = []
    with open(inputfile, 'rb') as f:
        for chunk in iter(lambda: f.read(partsize), b''):
            md5_digests.append(md5(chunk).digest())
    return md5(b''.join(md5_digests)).hexdigest() + '-' + str(len(md5_digests))

def main():
    etag = calc_etag("wine.json",  10485760);
    print(etag);

if __name__ == "__main__":
    main()
