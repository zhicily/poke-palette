const express = require('express')
const fetch = require("node-fetch");
const server = express()
const PORT = 8080

// ******************************************************************************************
// ========================================= SET-UP =========================================
// ******************************************************************************************

let hex_list = "";
let pokemon_limit = 885;
let pokemon_map = {};
let pokemon_names = [];

// REFERENCE: https://medium.com/@sergio13prez/fetching-them-all-poke-api-62ca580981a2

function fetchPokemon() {
    fetch(`https://pokeapi.co/api/v2/pokemon?limit=${pokemon_limit}`)
    .then(response => response.json())
    .then(function({ results: pokemon }) {

        let index = 0;
        let shift = 1;

        while (index < pokemon_limit) {
            // To account for indexing discrepancies in API call
            if (index == 807) {
                shift = 3;
            }

            pokemon[index]["name"] = pokemon[index]["name"].charAt(0).toUpperCase() + pokemon[index]["name"].slice(1);
            pokemon[index]["id"] = index + shift;

            index ++;
        }

        for (const poke of pokemon) {
            pokemon_map[poke.name.toLowerCase().replace('-', ' ').trim()] = poke.id;
            pokemon_names.push(poke.name.toLowerCase().replace('-', ' ').trim());
        }
    });
}


function python_calc(img_link) {
    return new Promise((resolve, reject) => {
        const { spawn } = require('child_process');
        
        const palette = spawn('python3', ['pokepalette_backend/palette_colours.py', img_link]);
        
        palette.stdout.on('data', function(data) {
            hex_list = String(data);        

            resolve(hex_list);
        });
    });
}


function python_match(name, names) {
    return new Promise((resolve, reject) => {
        const { spawn } = require('child_process');
        
        const matches = spawn('python3', ['pokepalette_backend/match_names.py', name, names]);
        
        matches.stdout.on('data', function(data) {
            hex_list = String(data);        

            resolve(hex_list);
        });
    });
}

// ******************************************************************************************
// ========================================= SERVER =========================================
// ******************************************************************************************

server.use(express.json());


server.post('/get_colours', async (req, res) => {
    console.log("Search input body: ", req.body);

    const pokemon = req.body.data;
    let pokeId;

    if (pokeId = pokemon_map[pokemon]) {
        let img_link = `https://pokeres.bastionbot.org/images/pokemon/${pokeId}.png`;

        console.log("Getting image from: ", img_link);

        result = await python_calc(img_link);

        if (result.toString().trim() === 'ERROR') {
            res.json({ flag: "PALETTE", data: 'NO_PALETTE'});
        } else {
            res.json({ flag: "PALETTE", data: result });
        }
    } else {
        result = await python_match(pokemon, pokemon_names.toString());

        if (result.toString().trim() === 'NO_MATCHES') {
            res.json({ flag: "NO_POKEMON", data: 'NO_POKEMON'});
        } else {
            res.json({ flag: "MATCHED", data: result});
        }
    }
});


server.get('/styles.css', (req, res) => {
    console.log("Get for styles.css");
    
    res.sendFile('pokepalette_frontend/styles.css', { root: __dirname });
});


// Handler for serving the index.html thingy for the root route
server.get('/', (req, res) => {
    res.sendFile('pokepalette_frontend/index.html', { root: __dirname });
});


// Handler for serving the frontend wasm-pack js
server.get('/frontend.js', (req, res) => {
    res.sendFile('pokepalette_frontend/pkg/pokepalette_frontend.js', { root: __dirname });
});


// Handler for serving the frontend_bg wasm-pack file
server.get('/frontend_bg.wasm', (req, res) => {
    res.sendFile('pokepalette_frontend/pkg/pokepalette_frontend_bg.wasm', { root: __dirname });
});


server.listen(PORT, '0.0.0.0', () => {
    console.log("Server has started!");
    fetchPokemon();
});