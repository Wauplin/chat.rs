#[macro_use] extern crate anyhow;

mod personality;
mod constants;

pub mod gpt4free;
pub mod opengpt;
pub mod g4f;
pub mod chimera;
pub mod phind;

pub async fn generate(msg: &str, bot_name: &str, fancy: bool) -> anyhow::Result<String> {
  let fmode =
    if fancy {
      ! (msg.contains("please")
      || msg.contains("пожалуйста")
      || msg.contains("Please")
      || msg.contains("Пожалуйста")
      || msg.contains("PLEASE"))
    } else {
      false
    };

  if let Ok(gpt4free_result)        = chimera::generate( msg, fmode, bot_name, "llama-2-70b-chat" ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = chimera::generate( msg, fmode, bot_name, "gpt-3.5-turbo-16k" ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::deepai::generate( msg, true, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::opchatgpts::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::getgpt::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::ails::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::yqcloud::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = phind::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = gpt4free::theb::generate( msg ) {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::chatgpt_ai::generate( msg, fmode, bot_name ) {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = gpt4free::gptworldAi::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = opengpt::chatbase::generate( msg ) {
    Ok(gpt4free_result)
  } else { Err(anyhow!("Failed to generate chat response")) }
}

pub async fn generate_all<'a>(msg: &str, bot_name: &str, fancy: bool)
                                -> Vec<(&'a str, anyhow::Result<String>)> {
  let fmode =
    if fancy {
      ! (msg.contains("please")
      || msg.contains("пожалуйста")
      || msg.contains("Please")
      || msg.contains("Пожалуйста")
      || msg.contains("PLEASE"))
    } else {
      false
    };

  vec![ ( "Chimera llama-2-70b-chat"
        , chimera::generate( msg, fmode, bot_name, "llama-2-70b-chat" ).await )
      , ( "gpt-3.5-turbo-16k"
        , chimera::generate( msg, fmode, bot_name, "gpt-3.5-turbo-16k" ).await )
      , ( "Deep AI"
        , g4f::deepai::generate( msg, true, bot_name ).await )
      , ( "opchatgpts"
        , g4f::opchatgpts::generate( msg, true, bot_name ).await )
      , ( "Getgpt"
        , g4f::getgpt::generate( msg, true, bot_name ).await )
      , ( "Ails"
        , g4f::ails::generate( msg, true, bot_name ).await )
      , ( "Phind"
        , phind::generate( msg, fmode, bot_name ).await )
      , ( "Yqcloud"
        , g4f::yqcloud::generate( msg, true, bot_name ).await )
      , ( "Theb"
        , gpt4free::theb::generate( msg ) )
      , ( "ChatGPT AI"
        , g4f::chatgpt_ai::generate( msg, fmode, bot_name ) )
      , ( "gpt4free::gptworldAi"
        , gpt4free::gptworldAi::generate( msg, fmode, bot_name ).await )
      , ( "OpenGPT Chatbase"
        , opengpt::chatbase::generate( msg ) )
  ]
}

pub async fn chat(msg: &str, bot_name: &str) -> anyhow::Result<String> {
  generate(msg, bot_name, true).await
}
