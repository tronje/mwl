#!/bin/bash

rm -f mwlDB.db
sqlite3 mwlDB.db < schema.sql
echo "empty db created"
