use std::sync::Arc;

use cursive::view::{Margins, ViewWrapper};
use cursive::views::Dialog;
use cursive::Cursive;

use crate::command::Command;
use crate::commands::CommandResult;
use crate::library::Library;
use crate::playlist::{Playlist, PlaylistType};
use crate::queue::Queue;
use crate::traits::ViewExt;
use crate::ui::listview::ListView;
use crate::ui::modal::Modal;

pub struct PlaylistsView {
    list: ListView<Playlist>,
    library: Arc<Library>,
}

impl PlaylistsView {
    pub fn new(p_type: PlaylistType, queue: Arc<Queue>, library: Arc<Library>) -> Self {
        let list = match p_type {
            PlaylistType::Library => library.playlists.clone(),
            PlaylistType::ForYou => library.foru.clone(),
        };

        Self {
            list: ListView::new(list, queue, library.clone()),
            library,
        }
    }

    pub fn delete_dialog(&mut self) -> Option<Modal<Dialog>> {
        let store = self.library.items();
        let current = store.get(self.list.get_selected_index());

        if let Some(playlist) = current {
            let library = self.library.clone();
            let id = playlist.id.clone();
            let dialog = Dialog::text("Are you sure you want to delete this playlist?")
                .padding(Margins::lrtb(1, 1, 1, 0))
                .title("Delete playlist")
                .dismiss_button("No")
                .button("Yes", move |s: &mut Cursive| {
                    library.delete_playlist(&id);
                    s.pop_layer();
                });
            Some(Modal::new(dialog))
        } else {
            None
        }
    }
}

impl ViewWrapper for PlaylistsView {
    wrap_impl!(self.list: ListView<Playlist>);
}

impl ViewExt for PlaylistsView {
    fn on_command(&mut self, s: &mut Cursive, cmd: &Command) -> Result<CommandResult, String> {
        if let Command::Delete = cmd {
            if let Some(dialog) = self.delete_dialog() {
                s.add_layer(dialog);
            }
            return Ok(CommandResult::Consumed(None));
        }

        self.list.on_command(s, cmd)
    }
}
