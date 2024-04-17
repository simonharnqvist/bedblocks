# bedblocks
### A tool for making genomic blocks from a BED file

> :warning: This is experimental software in alpha testing.


A common(ish) problem in genomics is the need to split a genome into blocks, preferably with blocks some distance apart. Bedblocks allows users to efficiently split a BED file into blocks of a given length a set distance apart.

### Installation
In due course, bedblocks will be available through the appropriate channels. For now, the binary made on MacOS is available in this Git repo.


### Usage

Given a BED file such as:
```
chr2	0	5000
chr3	4	4000
chr4	10	255
chr5	1	899
```

We can run:
```shell
bedblocks --input example.bed --blocklength 100 --min-dist 1000
```

Which results in:
```
chr2    0       99
chr2    1100    1199
chr2    2200    2299
chr2    3300    3399
chr3    4       103
chr3    1104    1203
chr3    2204    2303
```

### Speed
On an Apple M2 chip, bedblocks split a BED file of 17.4 million entries into blocks of length 100 with a minimum separation of 100 nt in about 7.5 seconds. 