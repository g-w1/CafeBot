// Feed people's gambling addictions
use rand::{thread_rng, Rng};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::database::database::{get_money, money_increment};

#[command]
async fn coin_flip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let bet: i32 = match args.rest().trim().parse() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(&ctx.http, "Please enter an amount to bet as an argument.")
                .await?;
            return Ok(());
        }
    };
    if bet > get_money(&msg.author)? || bet < 0 {
        msg.reply(&ctx.http, "You can't bet more money than you have.")
            .await?;
        return Ok(());
    }
    if thread_rng().gen_bool(0.5) {
        money_increment(&msg.author, bet)?;
        let response = format!("You got heads! You got **{}** monies.", bet);
        msg.reply(&ctx.http, response).await?;
    } else {
        money_increment(&msg.author, -bet)?;
        let response = format!("You got tails! You lost **{}** monies.", bet);
        msg.reply(&ctx.http, response).await?;
    }
    Ok(())
}
