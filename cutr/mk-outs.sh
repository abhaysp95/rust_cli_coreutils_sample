#!/bin/bash

set -u  # fail unset expansion

CSV="tests/inputs/movies1.csv"
TSV="tests/inputs/movies1.tsv"
BOOKS="tests/inputs/books.tsv"
OUT_DIR="tests/expected"

[ ! -d "${OUT_DIR}" ] && mkdir -p "${OUT_DIR}"
rm -f $OUT_DIR/*

for FIELD in 1 2 3 1-2 2-3 1-3; do
	cut -f $FIELD $TSV > "${OUT_DIR}/$(basename $TSV).f${FIELD}.out"
	cut -f $FIELD -d , $CSV > "${OUT_DIR}/$(basename $CSV).f${FIELD}.dcomma.out"
done

for POS in 1 2 8 1-2 2-3 1-8; do
	cut -b $POS $TSV > "$OUT_DIR/$(basename $TSV).b${POS}.out"
	cut -b $POS $CSV > "$OUT_DIR/$(basename $CSV).b${POS}.out"
done

for POS in 1 2 8 1-2 2-3 1-8; do
    cut -c $POS $TSV > "$OUT_DIR/$(basename $TSV).c${POS}.out"
    cut -c $POS $CSV > "$OUT_DIR/$(basename $CSV).c${POS}.out"
done

echo -e "AA\nÉÉ\nSS\nJJ" > "$OUT_DIR/books.c1,1.out"
