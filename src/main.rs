use async_trait::async_trait;
use nvim_rs::{compat::tokio::Compat, create::tokio as create, Handler, Neovim};
use rmpv::Value;
use tokio::io::Stdout;
use tokio::time;

#[derive(Clone)]
struct NeovimHandler {}

#[async_trait]
impl Handler for NeovimHandler {
    type Writer = Compat<Stdout>;

    async fn handle_notify(&self, name: String, _args: Vec<Value>, neovim: Neovim<Compat<Stdout>>) {
        match name.as_ref() {
            "start" => {
                tokio::spawn(async move {
                    let mut interval = time::interval(time::Duration::from_secs(1));
                    loop {
                        interval.tick().await;
                        let maybe_output = 
                            tokio::process::Command::new("defaults")
                            .args(&["read", "-g", "AppleInterfaceStyle"])
                            .output()
                            .await;
                        if let Ok(output) = maybe_output {
                            if let Ok(data) = String::from_utf8(output.stdout.to_vec()) {
                                if data.contains("Dark") {
                                    neovim.command("colorscheme gruvbox-material").await.unwrap();
                                } else {
                                    neovim.command("colorscheme elflord").await.unwrap();
                                }
                            }
                        }
                    }
                });
            },
            _ => {}
        }
    }
    async fn handle_request(
        &self,
        _name: String,
        _args: Vec<Value>,
        _neovim: Neovim<Compat<Stdout>>,
    ) -> Result<Value, Value> {
        Ok(Value::Nil)
    }
}

#[tokio::main]
async fn main() {
    let handler = NeovimHandler{};
    let (nvim, io_handler) = create::new_parent(handler).await;
    match io_handler.await {
        Err(joinerr) => eprintln!("Error joining IO loop: '{}'", joinerr),
        Ok(Err(err)) => {
            if !err.is_reader_error() {
                // One last try, since there wasn't an error with writing to the
                // stream
                nvim.err_writeln(&format!("Error: '{}'", err))
                    .await
                    .unwrap_or_else(|e| {
                        // We could inspect this error to see what was happening, and
                        // maybe retry, but at this point it's probably best
                        // to assume the worst and print a friendly and
                        // supportive message to our users
                        eprintln!("Well, hmm... '{}'", e);
                    });
            }

            if !err.is_channel_closed() {
                // Closed channel usually means neovim quit itself, or this plugin was
                // told to quit by closing the channel, so it's not always an error
                // condition.
                eprintln!("Error: '{}'", err);

                /*let mut source = err.source();

                while let Some(e) = source {
                  eprintln!("Caused by: '{}'", e);
                  source = e.source();
                }*/
            }
        }
        Ok(Ok(())) => {}
    }
}
