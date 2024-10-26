use serenity::{
  all::{
    ActivityData, Context, EventHandler as Handler, GuildChannel, Message, OnlineStatus, Ready,
  },
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

  async fn message(&self, ctx: Context, new_message: Message) {
    if new_message.content.contains("No such file or directory") {
      if let Err(error) = new_message
        .reply(&ctx.http, "Did you run `git add .`?")
        .await
      {
        println!("Error sending message: {:?}", error);
      }
    }
  }

  async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
    if let Err(error) = thread.id.join_thread(&ctx.http).await {
      println!("Error joining thread: {:?}", error);
    }
  }
}
