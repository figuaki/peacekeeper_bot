use std::env::var;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use poise::serenity_prelude::{self as serenity, ReactionType};
use poise::Event;

type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

//config
//Botの有効・無効を切り替えるフレーズ
const WAKE_UP_PHRASE: &str = "wake_up";
//通報用スタンプが何個溜まったらメッセージを削除するかの閾値
const THRESHOLD:u64 = 9;


// 最低限のデータ
struct Data {
    enable: AtomicBool,
    poise_mentions: AtomicU32,
    user_id : AtomicU64,
    report_emoji_id : AtomicU64,
}

#[tokio::main]
pub async fn run() {
    env_logger::init();

    let options = poise::FrameworkOptions {
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(event_handler(_ctx, event, _framework, _data))
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(
            //BOTのDISCORD_トークンを環境変数から指定
            var("DISCORD_TOKEN")
                .expect("Missing `DISCORD_TOKEN` env var, see README for more information."),
        )
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    enable: AtomicBool::new(false),
                    poise_mentions: AtomicU32::new(0),
                    //実行をON/OFFを切り替えられる管理者のユーザIDを環境変数から指定
                    user_id: AtomicU64::new(var("DISCORD_USER_ID").expect("Missing `DISCORD_USER_ID` env var, see README for more information.").parse::<u64>().unwrap()),
                    //通報用の絵文字を環境変数から指定
                    report_emoji_id: AtomicU64::new(var("DISCORD_REPORT_EMOJI_ID").expect("Missing `DISCORD_REPORT_EMOJI_ID` env var, see README for more information.").parse::<u64>().unwrap())
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        //ログインイベント
        Event::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        //新規メッセージに対するイベント
        Event::Message { new_message } => {
            if new_message.author.id == data.user_id.load(Ordering::SeqCst) //var("DISCORD_USER_ID").expect("")
            && new_message.content.to_lowercase().contains(WAKE_UP_PHRASE) 
            && new_message.author.id != ctx.cache.current_user().id
            {//管理者のメッセージに含まれるキーフレーズによりBOTの有効無効を切り替える
                let mentions = data.poise_mentions.load(Ordering::SeqCst) + 1;
                let enable = !data.enable.load(Ordering::SeqCst);
                data.poise_mentions.store(mentions, Ordering::SeqCst);
                data.enable.store(enable, Ordering::SeqCst);
                new_message
                    .reply(ctx, format!("{} 回目の呼び出し → {}", mentions, if (enable) {"起動"} else {"無効"}))
                    .await?;
            }
        }
        //リアクション(Emoji)の付与イベント
        Event::ReactionAdd { add_reaction } =>
        {
            println!("detect reaction");
            let enable = data.enable.load(Ordering::SeqCst);
            if(enable)
            {
                let message = add_reaction.message(ctx).await?;
                let report_emoji_id = data.report_emoji_id.load(Ordering::SeqCst);
                //logging
                println!( "==============================================================================================");
                println!( "author:{}", message.author);
                println!( "message:{}", message.content); 
                println!( "user_id:{}", add_reaction.user_id.unwrap_or_default()); 
                for r in message.reactions.iter()
                {//メッセージに付与された絵文字を数える
                    println!( "{}, count:{}"
                            , match &r.reaction_type
                            { ReactionType::Custom{animated, id, name} => format!("custom:{}, {}", id.to_string() ,match name {Some(x)=>x, None=>""})
                            , ReactionType::Unicode(text) => format!("unicode:{}", text.to_string())
                            , _=> "_".to_string()} 
                            , r.count);

                    let id = match &r.reaction_type
                    {
                          ReactionType::Custom{animated, id, name} => Some(id.0) 
                        , ReactionType::Unicode(text) => None
                        , _=> None
                    };

                    //指定されたカスタム絵文字が一定数溜まったら削除
                    if THRESHOLD < r.count  && id.is_some() && id.unwrap() == report_emoji_id
                    {
                        //message.reply(ctx, format!("Hi, I saw {} pressed 5 times on this message", add_reaction.emoji)).await?;
                        message.reply(ctx, format!("通報によりこのメッセージを削除します")).await?;
                        message.delete(ctx).await?;
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}