use std::any::Any;
use crate::commands;
use crate::settings::Settings;
use lazy_static::lazy_static;
use serenity::async_trait;
use serenity::futures::stream::iter;
use serenity::futures::TryFutureExt;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::application_command::CommandDataOptionValue::Channel;
use serenity::model::channel::{ChannelCategory, ChannelType, Message, PartialChannel, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::event::ChannelPinsUpdateEvent;
use serenity::model::gateway::Ready;
use serenity::model::guild::Change::GuildId;
use serenity::model::id::ChannelId;
use serenity::prelude::*;

lazy_static! {
    static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings::new());
}
pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn channel_pins_update(&self, ctx: Context, pin: ChannelPinsUpdateEvent) {
        let guild = pin.guild_id.unwrap();
        let mut channels = guild.channels(&ctx).await.expect("Err channels");
        let mut cat = channels
            .get(&pin.channel_id)
            .expect("Err channel id")
            .parent_id;

        if SETTINGS.lock().await.main_channel == 0 {
            pin.channel_id.say(&ctx.http,"Warning: No main channel has been set for this server! Run /mainchannel").await.expect("asd");
        }
        if SETTINGS.lock().await.archive_category == 0 {
            pin.channel_id.say(&ctx.http, "Warning: No archive channel has been set for this server! Run /archivecategory").await.expect("asd");
        }

        let pin_len = pin.channel_id.pins(&ctx).await.expect("Err expecting channel pins").len();
        if pin_len == 50 && pin.channel_id == SETTINGS.lock().await.main_channel {
            pin.channel_id.say(&ctx, format!("Pins updated! Max pins reached!")).await.expect("asd");

            let arch = SETTINGS.lock().await.archive_category.to_channel(&ctx).await.unwrap();
            SETTINGS.lock().await.main_channel.to_channel(&ctx).await.unwrap().category().as_mut().unwrap().edit(&ctx, |s| {
                s.category(Some(arch.id()))
            });
        } else {
            pin.channel_id.say(&ctx, format!("Pins updated! Current pins: {}", pin_len)).await.expect("asd");
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Successfully logged in!");

        Command::create_global_application_command(&ctx.http, |command| {
            commands::main_channel::register(command)
        })
        .await
        .expect("asd");
        Command::create_global_application_command(&ctx.http, |command| {
            commands::archive_category::register(command)
        })
        .await
        .expect("asd");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {

            match command.data.name.as_str() {
                "mainchannel" => {
                    let mut result = "Successfully set the new main channel";
                    let raw: &CommandDataOptionValue = command.data.options[0].resolved.as_ref().unwrap();

                    let channel = if let Channel(a) = raw {
                        a.id
                    } else {
                        result = "Error setting the main channel, make sure to select a valid channel";
                        SETTINGS.lock().await.main_channel
                    };
                    let kind = command.guild_id.unwrap().channels(&ctx).await.expect("asd").get(&channel).unwrap().kind;
                    if kind == ChannelType::Category {
                        result = "Provide a valid channel, this is a category";
                    } else {
                        SETTINGS.lock().await.set_main_channel(channel);
                    }

                    command.create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(result))
                    }).await.expect("asd");
                },
                "archivecategory" => {
                    let mut result = "Successfully set the archive category";
                    let raw: &CommandDataOptionValue = command.data.options[0].resolved.as_ref().unwrap();

                    let channel = if let Channel(a) = raw {
                        a.id
                    } else {
                        result = "Error setting the main channel, make sure to select a valid channel";
                        SETTINGS.lock().await.archive_category
                    };
                    let kind = command.guild_id.unwrap().channels(&ctx).await.expect("asd").get(&channel).unwrap().kind;
                    if kind == ChannelType::Text {
                        result = "Provide a valid category, this is a text channel";
                    } else {
                        SETTINGS.lock().await.set_archive_category(channel);
                    }

                    command.create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(result))
                    }).await.expect("asd");
                },
                _ => {}
            };
        }
    }
}
