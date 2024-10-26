use serenity::{
  all::{ActivityData, Context, EventHandler as Handler, OnlineStatus, Ready},
  async_trait,
};

pub struct EventHandler {}

#[async_trait]
impl Handler for EventHandler {
  async fn ready(&self, ctx: Context, ready: Ready) {
    ctx.set_presence(
      Some(ActivityData::custom("Did you run \"git add .\"?")),
      OnlineStatus::Online,
    );

    println!("Bot is now logged in as {}", ready.user.name);
  }
}
