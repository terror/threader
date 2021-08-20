use crate::common::*;

#[macro_use]
pub mod test_utils;

mod client;
mod common;
mod error;
mod file;
mod opt;
mod thread;
mod tweet;
mod util;

///        \`-._           __
///         \\  `-..____,.'  `.
///          :`.         /    \`.
///          :  )       :      : \
///           ;'        '   ;  |  :
///           )..      .. .:.`.;  :
///          /::...  .:::...   ` ;
///          ; _ '    __        /:\
///          `:o>   /\o_>      ;:. `.
///         `-`.__ ;   __..--- /:.   \
///         === \_/   ;=====_.':.     ;
///          ,/'`--'...`--....        ;
///               ;                    ;
///             .'                      ;
///           .'                        ;
///         .'     ..     ,      .       ;
///        :       ::..  /      ;::.     |
///       /      `.;::.  |       ;:..    ;
///      :         |:.   :       ;:.    ;
///      :         ::     ;:..   |.    ;
///       :       :;      :::....|     |
///       /\     ,/ \      ;:::::;     ;
///     .:. \:..|    :     ; '.--|     ;
///    ::.  :''  `-.,,;     ;'   ;     ;
/// .-'. _.'\      / `;      \,__:      \
/// `---'    `----'   ;      /    \,.,,,/

#[tokio::main]
async fn main() {
  dotenv().ok();
  match Opt::from_args().run().await {
    Ok(()) => println!("Thread sent! (you should probably check)"),
    Err(e) => eprintln!("{}", e),
  }
}
