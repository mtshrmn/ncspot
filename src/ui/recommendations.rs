use std::sync::{Arc, RwLock};

use cursive::view::ViewWrapper;
use cursive::Cursive;

use crate::command::Command;
use crate::commands::CommandResult;
use crate::library::Library;
use crate::queue::Queue;
use crate::track::Track;
use crate::traits::ViewExt;
use crate::ui::listview::ListView;

pub struct RecommendationsView {
    list: ListView<Track>,
}

impl RecommendationsView {
    pub fn new(content: Arc<RwLock<Vec<Track>>>, queue: Arc<Queue>, library: Arc<Library>) -> Self {
        Self {
            list: ListView::new(content, queue, library),
        }
    }
}

impl ViewWrapper for RecommendationsView {
    wrap_impl!(self.list: ListView<Track>);
}

impl ViewExt for RecommendationsView {
    fn title(&self) -> String {
        "Similar Tracks".to_string()
    }

    fn on_command(&mut self, s: &mut Cursive, cmd: &Command) -> Result<CommandResult, String> {
        self.list.on_command(s, cmd)
    }
}
