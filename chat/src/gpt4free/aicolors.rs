use inline_python::{ python, Context };

use std::panic::catch_unwind;

use anyhow::bail;

pub fn generate(prompt: &str) -> anyhow::Result<String> {
  match catch_unwind(|| {
    let c = Context::new();
    c.set("prompt", prompt);
    c.run(python! {
      import sys
      import os

      from gpt4free import aicolors

      try:
        result = str( aicolors.Completion.create(prompt) )
        reslt = True
      except OSError as err:
        result = ("OS Error! {0}".format(err))
        reslt = False
      except RuntimeError as err:
        result = ("Runtime Error! {0}".format(err))
        reslt = False
    }); ( c.get::<bool>("reslt")
        , c.get::<String>("result") )
  }) {
    Ok((r,m)) => {
      if r { Ok(m) } else {
        bail!("No tokens generated: {:?}", m)
      }
    }, Err(_) => { bail!("Failed to to use gpt4free::aicolors now!") }
  }
}

#[cfg(test)]
mod colour_tests {
  use super::*;
  #[test]
  fn colour_test() {
    let chat_response =
      generate("red color");
    assert!(chat_response.is_ok());
  }
}
