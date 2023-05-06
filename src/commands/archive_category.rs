use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("archivecategory")
        .description("Sets a category of your choice to be used as the main channel")
        .create_option(|option| {
            option
                .name("category")
                .description("Select the category")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
}
