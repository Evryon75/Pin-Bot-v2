use serenity::model::channel::{ChannelType, PartialChannel};
use serenity::model::id::ChannelId;

pub(crate) struct Settings {
    pub main_channel: ChannelId,
    pub archive_category: ChannelId,
}

impl Settings {
    pub fn new() -> Settings {
        Settings { main_channel: ChannelId(0), archive_category: ChannelId(0)}
    }
    pub fn set_main_channel(&mut self, channel_id: ChannelId) {
        self.main_channel = channel_id;
    }
    pub fn set_archive_category(&mut self, category: ChannelId) {
        self.archive_category = category;
    }
}
