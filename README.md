# Poke-Palette

## Build and Run
```
$ vagrant up
$ vagrant ssh

vagrant $ cd project/pokepalette
vagrant $ ./buildrun.sh
```
To access web app visit (on your local browser):
- http://localhost:3000/

NOTE: `vagrant up` will take ~ 10 minutes to run, and the bash script ~ 8 minutes. To reduce set-up times, increase the number of allocated VM cores in Vagrantfile.

The bash script `buildrun.sh` compiles the web-app into web assembly, and starts the server.

The console will log `Server has started!` when the server is ready to be accessed in the link above.


## About
Given user input of a Pokemon name (up until Gen 8), return a colour palette of 5 colours, as well as a corresponding Complementary colour palette, and Monochromatic (a darker, saturated variation of the original) palette.


## Languages
### JavaScript
Used `Express` (Node.js web application framework) to host the server handling HTTP (GET and POST) requests; also used to make API to retrieve Pokemon information from PokeAPI.

### Rust
Used `Yew` (Rust framework) to create front-end portion of the application. Yew's `html!` macro is heavily used for generating (and dynamically updating) the UI from within the same file, and Rust's pattern matching was used to define web app behaviour depending on user inputs.

### Python
Used `numpy` and `scikit-learn` libraries and MiniKMeans machine learning model to cluster the colours from the target image to get five designated Hex Codes. Also used `difflib` module to fuzzy-match on user inputs. 


## Features
- Hover over colours to display Hex Codes
- UI colour scheme dynamically updates to match successful searches
- List of fuzzy-matched (valid) name suggestions generated for misspelt inputs
- Mobile friendly


## Screenshots
<img src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/66a41c1e-090e-4d47-8425-248b3a68e4b3/de3h5ew-11cb5f68-6803-458e-a087-d078c6bc16e9.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3sicGF0aCI6IlwvZlwvNjZhNDFjMWUtMDkwZS00ZDQ3LTg0MjUtMjQ4YjNhNjhlNGIzXC9kZTNoNWV3LTExY2I1ZjY4LTY4MDMtNDU4ZS1hMDg3LWQwNzhjNmJjMTZlOS5wbmcifV1dLCJhdWQiOlsidXJuOnNlcnZpY2U6ZmlsZS5kb3dubG9hZCJdfQ.ZkdOKQ4sJlfAwcbEtauoEU9hfV_aEgCu-0Xtt9IHj5Y" width="400">

<img src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/66a41c1e-090e-4d47-8425-248b3a68e4b3/de3h5b0-332cb52b-169d-4edd-b6fd-a198e0e465b5.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3sicGF0aCI6IlwvZlwvNjZhNDFjMWUtMDkwZS00ZDQ3LTg0MjUtMjQ4YjNhNjhlNGIzXC9kZTNoNWIwLTMzMmNiNTJiLTE2OWQtNGVkZC1iNmZkLWExOThlMGU0NjViNS5wbmcifV1dLCJhdWQiOlsidXJuOnNlcnZpY2U6ZmlsZS5kb3dubG9hZCJdfQ.Mhxc8IK5zYfeBxvLAXFynYLprQtouEhdXdhQerCl_oI" width="396">