use std::fmt;
use std::sync::{Arc, RwLock};

use chrono::{DateTime, Utc};
use rayon::prelude::*;
use rspotify::model::album::FullAlbum;
use rspotify::model::track::{FullTrack, SavedTrack, SimplifiedTrack};

use crate::album::Album;
use crate::artist::Artist;
use crate::library::Library;
use crate::playable::Playable;
use crate::queue::Queue;
use crate::traits::{IntoBoxedViewExt, ListItem, ViewExt};
use crate::ui::recommendations::RecommendationsView;

#[derive(Clone, Deserialize, Serialize)]
pub struct Track {
    pub id: Option<String>,
    pub uri: String,
    pub title: String,
    pub track_number: u32,
    pub disc_number: i32,
    pub duration: u32,
    pub artists: Vec<String>,
    pub artist_ids: Vec<String>,
    pub album: Option<String>,
    pub album_id: Option<String>,
    pub album_artists: Vec<String>,
    pub cover_url: Option<String>,
    pub url: String,
    pub added_at: Option<DateTime<Utc>>,
}

impl Track {
    pub fn from_simplified_track(track: &SimplifiedTrack, album: &FullAlbum) -> Track {
        let artists = track
            .artists
            .iter()
            .map(|ref artist| artist.name.clone())
            .collect::<Vec<String>>();
        let artist_ids = track
            .artists
            .iter()
            .filter(|a| a.id.is_some())
            .map(|ref artist| artist.id.clone().unwrap())
            .collect::<Vec<String>>();
        let album_artists = album
            .artists
            .iter()
            .map(|ref artist| artist.name.clone())
            .collect::<Vec<String>>();

        Self {
            id: track.id.clone(),
            uri: track.uri.clone(),
            title: track.name.clone(),
            track_number: track.track_number,
            disc_number: track.disc_number,
            duration: track.duration_ms,
            artists,
            artist_ids,
            album: Some(album.name.clone()),
            album_id: Some(album.id.clone()),
            album_artists,
            cover_url: album.images.get(0).map(|img| img.url.clone()),
            url: track.uri.clone(),
            added_at: None,
        }
    }

    pub fn duration_str(&self) -> String {
        let minutes = self.duration / 60_000;
        let seconds = (self.duration / 1000) % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

impl From<&SimplifiedTrack> for Track {
    fn from(track: &SimplifiedTrack) -> Self {
        let artists = track
            .artists
            .iter()
            .map(|ref artist| artist.name.clone())
            .collect::<Vec<String>>();
        let artist_ids = track
            .artists
            .iter()
            .filter(|a| a.id.is_some())
            .map(|ref artist| artist.id.clone().unwrap())
            .collect::<Vec<String>>();

        Self {
            id: track.id.clone(),
            uri: track.uri.clone(),
            title: track.name.clone(),
            track_number: track.track_number,
            disc_number: track.disc_number,
            duration: track.duration_ms,
            artists,
            artist_ids,
            album: None,
            album_id: None,
            album_artists: Vec::new(),
            cover_url: None,
            url: track.uri.clone(),
            added_at: None,
        }
    }
}

impl From<&FullTrack> for Track {
    fn from(track: &FullTrack) -> Self {
        let artists = track
            .artists
            .iter()
            .map(|ref artist| artist.name.clone())
            .collect::<Vec<String>>();
        let artist_ids = track
            .artists
            .iter()
            .filter(|a| a.id.is_some())
            .map(|ref artist| artist.id.clone().unwrap())
            .collect::<Vec<String>>();
        let album_artists = track
            .album
            .artists
            .iter()
            .map(|ref artist| artist.name.clone())
            .collect::<Vec<String>>();

        Self {
            id: track.id.clone(),
            uri: track.uri.clone(),
            title: track.name.clone(),
            track_number: track.track_number,
            disc_number: track.disc_number,
            duration: track.duration_ms,
            artists,
            artist_ids,
            album: Some(track.album.name.clone()),
            album_id: track.album.id.clone(),
            album_artists,
            cover_url: track.album.images.get(0).map(|img| img.url.clone()),
            url: track.uri.clone(),
            added_at: None,
        }
    }
}

impl From<&SavedTrack> for Track {
    fn from(st: &SavedTrack) -> Self {
        let mut track: Self = (&st.track).into();
        track.added_at = Some(st.added_at);
        track
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.artists.join(", "), self.title)
    }
}

impl fmt::Debug for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({} - {} ({:?}))",
            self.artists.join(", "),
            self.title,
            self.id
        )
    }
}

impl ListItem for Track {
    fn is_playing(&self, queue: Arc<Queue>) -> bool {
        let current = queue.get_current();
        current.map(|t| t.id() == self.id).unwrap_or(false)
    }

    fn as_listitem(&self) -> Box<dyn ListItem> {
        Box::new(self.clone())
    }

    fn display_left(&self) -> String {
        format!("{}", self)
    }

    fn display_center(&self, library: Arc<Library>) -> String {
        if library.cfg.values().album_column.unwrap_or(true) {
            self.album.clone().unwrap_or_default()
        } else {
            "".to_string()
        }
    }

    fn display_right(&self, library: Arc<Library>) -> String {
        let saved = if library.is_saved_track(&Playable::Track(self.clone())) {
            if library.cfg.values().use_nerdfont.unwrap_or(false) {
                "\u{f62b} "
            } else {
                "✓ "
            }
        } else {
            ""
        };
        format!("{}{}", saved, self.duration_str())
    }

    fn play(&mut self, queue: Arc<Queue>) {
        let index = queue.append_next(vec![Playable::Track(self.clone())]);
        queue.play(index, true, false);
    }

    fn play_next(&mut self, queue: Arc<Queue>) {
        queue.insert_after_current(Playable::Track(self.clone()));
    }

    fn queue(&mut self, queue: Arc<Queue>) {
        queue.append(Playable::Track(self.clone()));
    }

    fn save(&mut self, library: Arc<Library>) {
        library.save_tracks(vec![self], true);
    }

    fn unsave(&mut self, library: Arc<Library>) {
        library.unsave_tracks(vec![self], true);
    }

    fn toggle_saved(&mut self, library: Arc<Library>) {
        if library.is_saved_track(&Playable::Track(self.clone())) {
            library.unsave_tracks(vec![self], true);
        } else {
            library.save_tracks(vec![self], true);
        }
    }

    fn open(&self, _queue: Arc<Queue>, _library: Arc<Library>) -> Option<Box<dyn ViewExt>> {
        None
    }

    fn open_recommentations(
        &self,
        queue: Arc<Queue>,
        library: Arc<Library>,
    ) -> Option<Box<dyn ViewExt>> {
        let spotify = queue.get_spotify();

        let recommendations: Option<Vec<Track>> = if let Some(id) = &self.id {
            spotify
                .recommentations(None, None, Some(vec![id.clone()]))
                .map(|r| r.tracks)
                .map(|tracks| {
                    tracks
                        .par_iter()
                        .filter_map(|track| match track.id.as_ref() {
                            Some(id) => spotify.track(id),
                            None => None,
                        })
                        .collect()
                })
                .map(|tracks: Vec<FullTrack>| tracks.iter().map(Track::from).collect())
        } else {
            None
        };

        recommendations.map(|tracks| {
            RecommendationsView::new(
                Arc::new(RwLock::new(tracks)),
                queue.clone(),
                library.clone(),
            )
            .as_boxed_view_ext()
        })
    }

    fn share_url(&self) -> Option<String> {
        self.id
            .clone()
            .map(|id| format!("https://open.spotify.com/track/{}", id))
    }

    fn album(&self, queue: Arc<Queue>) -> Option<Album> {
        let spotify = queue.get_spotify();

        match self.album_id {
            Some(ref album_id) => spotify.album(&album_id).map(|ref fa| fa.into()),
            None => None,
        }
    }

    fn artist(&self) -> Option<Artist> {
        Some(Artist::new(
            self.artist_ids[0].clone(),
            self.artists[0].clone(),
        ))
    }

    fn track(&self) -> Option<Track> {
        Some(self.clone())
    }
}
