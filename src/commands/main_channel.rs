use serenity::builder;
use serenity::model::application::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::command::CommandOptionType;

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected channel")
        .resolved
        .as_ref()
        .expect("Expected channel");

    if let CommandDataOptionValue::Channel(chan) = option {
        format!("{:#?}'s id is {}", chan.name, chan.id)
    } else {
        "Please provide a valid channel".to_string()
    }
}

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
