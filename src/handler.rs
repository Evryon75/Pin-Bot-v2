
use crate::commands;
use crate::settings::Settings;
use lazy_static::lazy_static;
use serenity::async_trait;


use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::application::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::application::interaction::application_command::CommandDataOptionValue::{Channel, Role};
use serenity::model::channel::{ChannelType, GuildChannel, PermissionOverwrite};
use serenity::model::event::ChannelPinsUpdateEvent;
use serenity::model::gateway::Ready;
use serenity::model::guild::Guild;
use serenity::model::id::{ChannelId, RoleId};
use serenity::model::Permissions;
use serenity::model::prelude::{Activity, PermissionOverwriteType};


use serenity::prelude::*;

lazy_static! {
    static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings::new());
}
pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn channel_pins_update(&self, ctx: Context, mut pin: ChannelPinsUpdateEvent) {

        let mut w = String::new();
        if SETTINGS.lock().await.main_channel == 0 {
            w.push_str("**Warning**:\nNo main channel has been set for this server! Run /mainchannel\n\n")
        }
        if SETTINGS.lock().await.archive_category == 0 {
            w.push_str("**Warning**:\nNo archive channel has been set for this server! Run /archivecategory\n\n")
        }
        if SETTINGS.lock().await.archive_role == 0 {
            w.push_str("**Warning**:\nNo archive role has been set for this server! Run /archiverole\n\n")
        }
        if !w.is_empty() {
            pin.channel_id.say(&ctx.http, w).await.unwrap();
        }

        let pin_len = pin.channel_id.pins(&ctx).await.expect("Err expecting channel pins").len();
        if pin_len == 50 && pin.channel_id == SETTINGS.lock().await.main_channel {
            pin.channel_id.say(&ctx, format!("Pins updated! Max pins reached!")).await.expect("asd");

            let arch = SETTINGS.lock().await.archive_category;
            let rol = SETTINGS.lock().await.archive_role;

            let cat = pin.guild_id.unwrap().channels(&ctx).await.unwrap().get(&pin.channel_id).unwrap().parent_id.unwrap();
            let save = pin.guild_id.unwrap().create_channel(&ctx, |c| {
                c
                    .category(cat)
                    .name("lmao")
                    .kind(ChannelType::Text)
                    .position(0)
                    .topic("bro luiterally")
                    .permissions(vec![
                        PermissionOverwrite {
                            allow: Permissions::VIEW_CHANNEL,
                            deny: Default::default(),
                            kind: PermissionOverwriteType::Role(rol),
                        }
                    ])
            });
            ctx.cache.
            SETTINGS.lock().await.set_main_channel(save.await.unwrap().id);
            pin.channel_id.edit(&ctx, |chan| {
                chan
                    .category(arch)
                    .permissions(vec![
                        PermissionOverwrite {
                            allow: Permissions::VIEW_CHANNEL,
                            deny: Permissions::SEND_MESSAGES,
                            kind: PermissionOverwriteType::Role(rol),
                        }
                    ])

            }).await.expect("asd");
        } else if pin.channel_id == SETTINGS.lock().await.main_channel {
            pin.channel_id.say(&ctx, format!("Pins updated! Current pins: {}", pin_len)).await.expect("asd");
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Successfully logged in!");
        ctx.set_activity(Activity::watching("nothing")).await;
        ctx.idle().await;

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
        Command::create_global_application_command(&ctx.http, |command| {
            commands::archive_role::register(command)
        })
        .await
        .expect("asd");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {

            let mainchannel = async || {
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

                ctx.set_activity(Activity::watching(String::from("Channel: ") + &*SETTINGS.lock().await.main_channel.name(&ctx).await.unwrap())).await;
                ctx.online().await;
            };
            let archivecategory = async || {
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
                println!("{}", result);
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(result))
                }).await.expect("asd");
            };
            let archiverole = async || {
                let mut result = "Successfully set the archive role";
                let raw: &CommandDataOptionValue = command.data.options[0].resolved.as_ref().unwrap();

                SETTINGS.lock().await.set_archive_role(if let Role(a) = raw {
                    a.id
                } else {
                    result = "Something went wrong, default to role id 0";
                    RoleId(0)
                });
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(result))
                }).await.expect("asd");
            };
            match command.data.name.as_str() {
                "mainchannel" => { mainchannel().await; },
                "archivecategory" => { archivecategory().await; },
                "archiverole" => { archiverole().await; },
                _ => {}
            };
        }
    }
}
