use serenity::model::id::{ChannelId, RoleId};

pub(crate) struct Settings {
    pub main_channel: ChannelId,
    pub archive_category: ChannelId,
    pub archive_role: RoleId
}

impl Settings {
    pub fn new() -> Settings {
        Settings { main_channel: ChannelId(0), archive_category: ChannelId(0), archive_role: RoleId(0) }
    }
    pub fn set_main_channel(&mut self, channel: ChannelId) {
        self.main_channel = channel;
    }
    pub fn set_archive_category(&mut self, category: ChannelId) {
        self.archive_category = category;
    }
    pub fn set_archive_role(&mut self, role: RoleId) {
        self.archive_role = role;
    }
}
