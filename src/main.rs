use secp256k1::{PublicKey, SecretKey};
use web3;
use tokio;
use web3::types::Address;
use std::str::FromStr;
use crate::wallet::{create_txn_object, sign_and_send};
use anyhow::Result;
use fltk::{app, button::Button, enums::{Align, CallbackTrigger, Color, Font, FrameType}, frame::Frame, input, prelude::*, window::Window};
mod wallet;
const URL: &str = "https://eth-holesky.g.alchemy.com/v2/VNuQWxacsBY5Lv-iMEKFk0AowRSw_J_f";
#[derive(Clone,Debug)]
  pub enum WalletMessage{
    NewWallet,
    Send,
  }

  #[tokio::main]
   async fn main() -> Result<()> {
      let app = app::App::default();
      let mut wind = Window::default().with_size( 500, 800).with_label("Simple Wallet");
      let mut but = Button::new(195, 450, 120, 45, "Create Wallet");
      let mut but2 = Button::new(200, 300, 100, 35, " SEND ");
      let mut inp1 = input::Input::new(100, 200, 225, 35, "To: ");
      let mut frame = Frame::default()
        .with_size(500, 200)
        .center_of_parent()
        .with_align(Align::Center|Align::Wrap)
        .with_label("0 Wallets");

      frame.set_label_color(Color::White);
      frame.set_label_font(Font::TimesBold);
      frame.set_label_size(24);

      wind.set_color(Color::DarkCyan);

      but.set_color(Color::White);
      but.set_label_color(Color::DarkMagenta);
      but.set_label_font(Font::TimesBold);
      but.set_frame(FrameType::FlatBox);
      but.clear_visible_focus();
      
      but2.set_color(Color::White);
      but2.set_label_color(Color::DarkMagenta);
      but2.set_label_font(Font::TimesBold);
      but2.set_frame(FrameType::FlatBox);
      but2.clear_visible_focus();
    
      inp1.set_frame(FrameType::FlatBox);

      wind.end();
      wind.show();

      inp1.set_value("Paste Address Here");
      inp1.set_trigger(CallbackTrigger::Changed);

    let (s, r) = app::channel::<WalletMessage>();

    but.emit(s.clone(), WalletMessage::NewWallet);
    but2.emit(s, WalletMessage::Send);
    

    let web3 = wallet::establish_web3_connection(URL)?;
    let mut keypairs: Vec<(PublicKey, SecretKey)> = Vec::new();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                WalletMessage::NewWallet => {
                    let (seckey, pubkey) = match wallet::create_keypair() {
                        Ok(val) => val,
                        Err(_e) => unimplemented!(),
                    };
                    keypairs.push((pubkey, seckey));
                    frame.set_label(&format!("{} Wallets", keypairs.len()));
                    println!("keypairs {:?}", keypairs);
                },
                WalletMessage::Send => {
                    if keypairs.is_empty() {
                        frame.set_label("Error: No wallet available. Please create a wallet first.");
                        continue;
                    }

                    let to_adrs = match Address::from_str(&inp1.value()) {
                        Ok(address) => address,
                        Err(_) => {
                            frame.set_label("Error: Invalid address. Please enter a valid Ethereum address.");
                            continue;
                        },
                    };

                    let tx_object = create_txn_object(to_adrs, 17)?;
                    let result = sign_and_send(web3.clone(), tx_object, keypairs[0].1).await;
                    match result {
                        Ok(val) => frame.set_label(&format!("{}", val)),
                        Err(e) => frame.set_label(&format!("{}", e)),
                    };
                }
            }
        }
    }
    app.run().unwrap();
    Ok(())
}