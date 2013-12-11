#!/bin/bash

rustc soundex_tdd/soundex.rs --test -o soundex_tdd/soundex
./soundex_tdd/soundex

rustc tcp-client-server/server.rs
rustc tcp-client-server/client.rs
