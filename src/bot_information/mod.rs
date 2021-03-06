use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group
    }
};

#[command]
async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
let com = "
!info
!kmp_code
!trie_run
!trie_code
!knapsack
!system_design
!http
!https
!cdn
!tcp
!tcp_header
!ip
!udp
!type_happen
!osi7layer
!osi_physical_layer
!osi_datalink_layer
!osi_network_layer
!osi_transport_layer
!osi_session_layer
!osi_presentation_layer
!osi_application_layer
!capacity
!cpu_memory_time_type
!dau
!latency
!throughput
!availablity
";
  let com  = str::replace(com, "\n", " ");
  msg.channel_id.say(&ctx.http, &com).await?;

  Ok(())
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
let info = "
Computer science, Algorithm study discord bot project v0.0.1
  - Visualization of algorithm run result. (text type display)
  - Algorithm code / data structure code.
  - Sandard library method, how to use.

input command => !commands

dev stack
- rust
- tokio
- serenity
- heroku (deployment)
";
  msg.channel_id.say(&ctx.http, info).await?;

  Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
  msg.channel_id.say(&ctx.http, "pong").await?;

  Ok(())
}

#[group]
#[commands(info, ping, commands)]
pub struct BotInformation;