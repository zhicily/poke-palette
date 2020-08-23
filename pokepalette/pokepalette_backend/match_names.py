import sys
import difflib as dl

TARGET_NAME = sys.argv[1]
POKEMON_NAMES = sys.argv[2]

def get_matches(name, pokemon):
    if len(pokemon) == 0:
        print('NO_MATCHES')

    pokemon_list = pokemon.split(',')

    matches = dl.get_close_matches(name, pokemon_list, n=4, cutoff=0.7)

    if len(matches) == 0:
        print('NO_MATCHES')
    else:
        sep = ', '
        matches_str = sep.join(matches)

        print(matches_str)

get_matches(TARGET_NAME, POKEMON_NAMES)