use serde::{
    Deserialize,
    Serialize,
};
use std::{
    collections::{
        HashMap,
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeInterval {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
    pub loudness: f32,
    pub tempo: f32,
    pub tempo_confidence: f32,
    pub key: i32,
    pub key_confidence: f32,
    pub mode: i32,
    pub mode_confidence: f32,
    pub time_signature: i32,
    pub time_signature_confidence: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Segment {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
    pub loudness_start: f32,
    pub loudness_max: f32,
    pub loudness_max_time: f32,
    pub loudness_end: Option<f32>,
    pub pitches: Vec<f32>,
    pub timbre: Vec<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioAnalysis {
    pub bars: Vec<TimeInterval>,
    pub beats: Vec<TimeInterval>,
    pub sections: Vec<Section>,
    pub segments: Vec<Segment>,
    pub tatums: Vec<TimeInterval>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioFeatures {
    pub acousticness: f32,
    pub analysis_url: String,
    pub danceability: f32,
    pub duration_ms: i32,
    pub energy: f32,
    pub id: String,
    pub instrumentalness: f32,
    pub key: i32,
    pub liveness: f32,
    pub loudness: f32,
    pub mode: i32,
    pub speechiness: f32,
    pub tempo: f32,
    pub time_signature: i32,
    pub track_href: String,
    pub uri: String,
    pub valence: f32,
    #[serde(rename = "type")] 
    pub object_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistSimple {
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub name: String,
    pub uri: String,
    #[serde(rename = "type")] 
    pub object_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub height: i32,
    pub url: String,
    pub width: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumSimple {
    pub album_group: Option<String>,
    pub album_type: String,
    pub artists: Vec<ArtistSimple>,
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub restrictions: Option<HashMap<String, String>>,
    pub uri: String,
    #[serde(rename = "type")] 
    pub object_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackLink {
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    uri: String,
    #[serde(rename = "type")]
    pub object_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    pub album: AlbumSimple,
    pub artists: Vec<ArtistSimple>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i32,
    pub explicit: bool,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub linked_from: Option<TrackLink>,
    pub name: String,
    pub popularity: i32,
    pub preview_url: String,
    pub restrictions: Option<HashMap<String, String>>,
    pub track_number: i32,
    pub uri: String,
    #[serde(rename = "type")]
    pub object_type: String,
}
