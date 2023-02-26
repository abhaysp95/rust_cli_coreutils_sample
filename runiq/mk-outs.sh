#!/bin/bash

OUT_DIR="tests/expected"
ROOT="tests/inputs"

[ ! -d "${OUT_DIR}" ] && mkdir -p "${OUT_DIR}"

# Cf https://github.com/coreutils/coreutils/blob/master/tests/misc/uniq.pl
echo -ne "a\na\n"    > $ROOT/t1.txt
echo -ne "a\na"      > $ROOT/t2.txt
echo -ne "a\nb"      > $ROOT/t3.txt
echo -ne "a\na\nb"   > $ROOT/t4.txt
echo -ne "b\na\na\n" > $ROOT/t5.txt
echo -ne "a\nb\nc\n" > $ROOT/t6.txt

for fl in $ROOT/*.txt; do
	bfl="$(basename ${fl})"
	uniq $fl > "${OUT_DIR}/${bfl}.out"
	uniq -c $fl > "${OUT_DIR}/${bfl}.c.out"
	uniq < $fl > "${OUT_DIR}/${bfl}.stdin.out"
	uniq -c $fl > "${OUT_DIR}/${bfl}.stdin.c.out"
done
