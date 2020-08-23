use yew::prelude::*;
use yew::format::Json;
use yew::events::KeyboardEvent;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use anyhow::Error;

// ******************************************************************************************
// ========================================= SET-UP =========================================
// ******************************************************************************************

pub struct App {
    pokemon: String,
    display_pokemon: String,
    curr_input: String,
    suggestion_caption: String,
    default_suggestions: [String; 4],
    link: ComponentLink<Self>,
    colours: [String; 15],
    active_request: Option<FetchTask>,
}


pub enum Msg {
    SearchPokemonInput(String),
    SearchPokemon,
    FetchColourDataSuccess(String),
    FetchColourDataFailed,
    FetchPokemonMatches(String),
    FetchPokemonFailed,
    DoNothing,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Data {
    // Flag to indicate whether response contains palette or potential keyword matches
    pub flag: String,
    pub data: String,
}


impl App {
    
    // Handling when user presses search button >> sends request to server, which calls python to do calculation
    fn search_handler(&mut self) {
        let hostname = yew::utils::host()
            .unwrap_or(String::from("localhost:8080"));
        
        let send_data = Data {
            flag: String::from(""),
            data: self.pokemon.clone(),
        };

        let body = Json(&send_data);

        let route: String = format!("http://{}/get_colours", hostname);
        

        // POST request to web-server sending over user input string onclick
        let post_request = Request::post(route)
            .header("Content-Type", "application/json")
            .body(body)
            .expect("Failed to build request.");
        

        // Fetch response from post request which should be a JSON wrapped Data string of hex codes
        let task = FetchService::fetch(
            post_request,
            self.link.callback(|response: Response<Json<Result<Data, Error>>>| {
                if let (meta, Json(Ok(body))) = response.into_parts() {
                    if meta.status.is_success() {
                        // Returning actual palette colours
                        if body.flag == "PALETTE" {
                            // Some pokemon have invalid image URLs, although they are valid names 
                            if body.data == "NO_PALETTE" {
                                return Msg::FetchColourDataFailed;
                            } else {
                                return Msg::FetchColourDataSuccess(body.data);
                            }
                        } else if body.flag == "MATCHED" {
                            return Msg::FetchPokemonMatches(body.data);
                        }
                    } 
                } 
            
                // No palette colours retrieved, nor any potential matches
                Msg::FetchPokemonFailed
            }), 
        ).expect("Fetch request failed to be a task.");


        self.active_request = Some(task);
    }
}

// ******************************************************************************************
// =================================== DEFINE BEHAVIOUR =====================================
// ******************************************************************************************

impl Component for App {
    type Message = Msg;
    type Properties = ();


    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Set default palette of blues
        let default_col = String::from("#74BCCB");

        App {
            link,
            curr_input: String::from(""),
            pokemon: String::from(""),
            display_pokemon: String::from(""),
            suggestion_caption: String::from("Try It Out:"),
            default_suggestions: [String::from("Mewtwo"), 
                                  String::from("Slowpoke"), 
                                  String::from("Reshiram"), 
                                  String::from("Charizard")],
            colours: [default_col.clone(), default_col.clone(), default_col.clone(), default_col.clone(), default_col.clone(),
                      default_col.clone(), default_col.clone(), default_col.clone(), default_col.clone(), default_col.clone(),
                      default_col.clone(), default_col.clone(), default_col.clone(), default_col.clone(), default_col.clone()],
            active_request: None,
        }
    }
    
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Search for palette; prevents another active request is one is in progress
            Msg::SearchPokemon  => {
                match &self.active_request {
                    None => {
                        self.pokemon = self.curr_input.clone();
                        self.display_pokemon = String::from("Searching ...");
                        self.search_handler();
                    }

                    Some(_) => { () }
                }
            },

            // Keep track of current input anytime user types anything into search bar
            Msg::SearchPokemonInput(p) => {
                self.curr_input = p.trim().to_lowercase()
            },

            Msg::FetchColourDataSuccess(response) => {  
                self.display_pokemon = self.pokemon.clone();

                let hex_list: Vec<&str> = response.split(", ").collect();

                // Update palette with newly retrieved colours
                for i in 0 .. 15 {
                    self.colours[i] = hex_list[i].to_owned();
                }

                self.suggestion_caption = String::from("Try It Out:");
                self.default_suggestions = [String::from("Mewtwo"), 
                                            String::from("Slowpoke"), 
                                            String::from("Reshiram"), 
                                            String::from("Charizard")];

                self.active_request = None;
            },

            // Could not retrieve pokemon image from API call
            Msg::FetchColourDataFailed => { 
                self.pokemon = String::from("No Palette Exists :(");

                self.display_pokemon = self.pokemon.clone();

                self.suggestion_caption = String::from("Try It Out:");
                self.default_suggestions = [String::from("Mewtwo"), 
                                            String::from("Slowpoke"), 
                                            String::from("Reshiram"), 
                                            String::from("Charizard")];

                self.active_request = None;
            }

            // Get a list of possible matches (if user mispelled input)
            Msg::FetchPokemonMatches(response) => { 
                self.pokemon = String::from("Invalid Pokemon Name :(");

                self.display_pokemon = self.pokemon.clone();
                
                let match_list: Vec<&str> = response.split(", ").collect();
                let list_len = match_list.len();

                for i in 0 .. list_len {     
                    self.default_suggestions[i] = match_list[i].trim().to_owned();
                }

                for i in list_len .. 4 {
                    self.default_suggestions[i] = String::from("");
                }

                self.suggestion_caption = String::from("Did You Mean:");

                self.active_request = None;
            }

            // Pokemon does not exist
            Msg::FetchPokemonFailed => { 
                self.pokemon = String::from("Invalid Pokemon Name :(");

                self.display_pokemon = self.pokemon.clone();

                self.suggestion_caption = String::from("Try It Out:");
                self.default_suggestions = [String::from("Mewtwo"), String::from("Slowpoke"), String::from("Reshiram"), String::from("Charizard")];
                
                self.active_request = None;
            }

            Msg::DoNothing => {()}
        }

        true
    }


    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }


    fn view(&self) -> Html {

        // Capitalize first letter of user input string for cleaner display
        fn upper_first_letter(s: String) -> String {
            let mut str_iter = s.chars();

            let new_str: String;
            
            match str_iter.next() {
                Some(c) => new_str = c.to_uppercase().collect::<String>() + str_iter.as_str(),
                None => new_str = String::from(""),
            }

            new_str
        }

        
        fn join_str(s1: &str, s2: String) -> String {
            let joined = [s1, &s2].join("");

            joined
        }


        // For updating webpage elements to have same colour palette as what was retrieved
        let colour_tag = "color: ";
        let background_colour_tag = "background-color: ";
        let border_tag = "border: 3px solid";
        let border_tag_search_button = "border: 1px solid";

        let display = upper_first_letter(self.display_pokemon.clone());


        html! {
            <div>
            <center>
            <p><h1>{"Pokemon: "} { display }</h1></p>

                <div class="search">
                    <input type="text" class="searchTerm" 
                                    style={ [join_str(colour_tag, self.colours[1].clone()), join_str(border_tag, self.colours[0].clone())].join("; ") } 
                                    placeholder="Search Pokemon ..." 
                                    oninput=self.link.callback(|p: InputData| Msg::SearchPokemonInput(p.value))
                                    onkeypress=self.link.callback(|k: KeyboardEvent| {
                                        if k.key() == "Enter" { Msg::SearchPokemon } else { Msg::DoNothing } 
                                    }) />

                    <button type="submit" class="searchButton"  
                                    style={ [join_str(background_colour_tag, self.colours[0].clone()), join_str(border_tag_search_button, self.colours[0].clone())].join("; ") }
                                    onclick=self.link.callback(|_| Msg::SearchPokemon)>
                                    { "Search" }
                    </button>

                </div>

            <h2>{ "Colour Palette" }</h2>

            <div class="hueContainer">
                <div class="hue" style={ join_str(background_colour_tag, self.colours[0].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[0].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[1].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[1].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[2].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[2].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[3].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[3].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[4].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[4].clone() }</div>
                    </div>
                </div>
            </div>

            <h2>{ "Complementary Palette" }</h2>

            <div class="hueContainer">
                <div class="hue" style={ join_str(background_colour_tag, self.colours[5].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[5].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[6].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[6].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[7].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[7].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[8].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[8].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[9].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[9].clone() }</div>
                    </div>
                </div>
            </div>

            <h2>{ "Monochromatic Palette" }</h2>
            
            <div class="hueContainer">
                <div class="hue" style={ join_str(background_colour_tag, self.colours[10].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[10].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[11].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[11].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[12].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[12].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[13].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[13].clone() }</div>
                    </div>
                </div>
                <div class="hue" style={ join_str(background_colour_tag, self.colours[14].clone()) }>
                    <div class="overlay">
                        <div class="hueText">{ self.colours[14].clone() }</div>
                    </div>
                </div>
            </div>

            <div class="helpContainer">
                <div class="infoBox" 
                    style={ join_str(border_tag, self.colours[0].clone()) }>

                    <h3>{ String::from("About: ") }</h3>
                    <div class="helpText">

                        { String::from("This web app allows you to search for the colour palettes of Pokemon up to Gen. 8! 
                                        No idea where to start? Try some of the suggestions on the right.") }

                    </div>
                </div>

                <div class="suggestionsBox"
                    style={ join_str(border_tag, self.colours[0].clone()) }>
                    
                    <h3>{ self.suggestion_caption.clone() }</h3>

                    <div class="helpText">
                        { upper_first_letter(self.default_suggestions[0].clone()) }<br/>
                        { upper_first_letter(self.default_suggestions[1].clone()) }<br/>
                        { upper_first_letter(self.default_suggestions[2].clone()) }<br/>
                        { upper_first_letter(self.default_suggestions[3].clone()) }
                    </div>

                </div>
            </div>

            </center>
            </div>
        }
    }
}