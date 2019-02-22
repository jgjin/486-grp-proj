use reqwest::{
    Client,
};

use crate::track_types::{
    AudioAnalysis,
    AudioFeatures,
    Track,
};

pub fn get_track_analysis(
    client: &Client,
    token: &str,
    track_id: &str,
) -> Result<AudioAnalysis, reqwest::Error> {
    Ok(
        client.get(&format!("https://api.spotify.com/v1/audio-analysis/{}/", track_id)[..])
            .bearer_auth(token)
            .send()?
            .json()?
    )
}

pub fn get_track_features(
    client: &Client,
    token: &str,
    track_id: &str,
) -> Result<AudioFeatures, reqwest::Error> {
    Ok(
        client.get(&format!("https://api.spotify.com/v1/audio-features/{}/", track_id)[..])
            .bearer_auth(token)
            .send()?
            .json()?
    )
}

pub fn get_track(
    client: &Client,
    token: &str,
    track_id: &str,
) -> Result<Track, reqwest::Error> {
    Ok(
        client.get(&format!("https://api.spotify.com/v1/tracks/{}/", track_id)[..])
            .bearer_auth(token)
            .send()?
            .json()?
    )
}
