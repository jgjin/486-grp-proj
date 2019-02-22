extern crate reqwest;

mod token;
mod track;
mod track_types;

use reqwest::{
    Client,
};

fn main(
) {
    let client = Client::new();

    let token = token::retrieve_access_token(&client)
        .expect("Error in access token")
        .access_token;
    
    let track_info = track::get_track(&client, &token[..], "3JIxjvbbDrA9ztYlNcp3yL")
        .expect("Error in getting track");

    println!("{:?}", track_info);
}
