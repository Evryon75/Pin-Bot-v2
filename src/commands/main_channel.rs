use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("mainchannel")
        .description("Sets a channel of your choice to be used as the main channel")
        .create_option(|option| {
            option
                .name("channel")
                .description("Select the channel")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
}
