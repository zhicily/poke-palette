#!/bin/bash
cd ./pokepalette_frontend
wasm-pack build --target web
cd ..
node index.js