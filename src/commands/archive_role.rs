use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("archiverole")
        .description("Sets a role of your choice to override permissions in archived channels")
        .create_option(|option| {
            option
                .name("role")
                .description("Everyone should have this role")
                .kind(CommandOptionType::Role)
                .required(true)
        })
}
