use channels_lite::channels::{channel_subscriber, Network};
use failure::Fallible;

mod user_input;
use user_input::get_words;

#[tokio::main]
async fn main() -> Fallible<()> {
  println!("Please enter the subscriber seed");
  let seed = get_words(1)?;
  println!("Subscriber seed:{:?}", seed[0].to_uppercase());
  let seed_subscriber = Some(seed[0].clone().to_uppercase());
  println!("Please enter the channel_address and tag from the author");
  let link = get_words(2)?;
  let channel_address = link[0].clone();
  let announcement_tag = link[1].clone();
  //Create Channel Instance for subscriber
  let mut channel_subscriber = channel_subscriber::Channel::new(
    Network::Custom("https://node04.iotatoken.nl:443", 14),
    channel_address,
    announcement_tag,
    seed_subscriber,
  );

  //Connect to channel
  let subscription_tag = channel_subscriber.connect().unwrap();
  println!(
    "Subscriber: Connected to channel with tag: {}",
    subscription_tag
  );

  println!("Please enter the keyload_tag from the author");
  let keyload_tag = get_words(1)?;
  channel_subscriber
    .update_keyload(keyload_tag[0].clone())
    .unwrap();

  // println!("Please enter the signed_packed_tag_public and change_key_tag_public from the author");
  // let tags = get_words(2)?;
  // let signed_packed_tag_public = tags[0].clone();
  // // let change_key_tag_public = Some(tags[1].clone());
  // let change_key_tag_public = None;
  // //Read all signed messages
  // let list_signed_public: Vec<(Option<String>, Option<String>)> = channel_subscriber
  //   .read_signed(signed_packed_tag_public, change_key_tag_public)
  //   .unwrap();
  // println!("Subscriber: Reading signed public messages");
  // for msg in list_signed_public.iter() {
  //   let (public, masked) = msg;
  //   println!(
  //     "Subscriber: Found Signed Public Message -> Public: {:?} -- Masked: {:?}",
  //     public, masked
  //   )
  // }

  println!("Please enter the signed_packed_tag_masked and change_key_tag_masked from the author");
  let tags_masked = get_words(2)?;
  let signed_packed_tag_masked = tags_masked[0].clone();
  // let change_key_tag_masked = Some(tags_masked[1].clone());
  let change_key_tag_masked = None;
  let list_signed_masked: Vec<(Option<String>, Option<String>)> = channel_subscriber
    .read_signed(signed_packed_tag_masked, change_key_tag_masked)
    .unwrap();
  println!("Subscriber: Reading signed masked messages");
  for msg in list_signed_masked.iter() {
    let (public, masked) = msg;
    println!(
      "Subscriber: Found Signed Masked Message -> Public: {:?} -- Masked: {:?}",
      public, masked
    )
  }

  // println!("Please enter the signed_packed_tag_masked from the author");
  // let tagged_packed = get_words(1)?;
  // let tagged_packed_tag = tagged_packed[0].clone();
  // //Read all tagged messages
  // let list_tagged: Vec<(Option<SensorData>, Option<SensorData>)> =
  //   channel_subscriber.read_tagged(tagged_packed_tag).unwrap();
  // println!("Subscriber: Reading tagged messages");
  // for msg in list_tagged.iter() {
  //   let (public, masked) = msg;
  //   println!(
  //     "Subscriber: Found Tagged Message -> Public: {:?} -- Masked: {:?}",
  //     public, masked
  //   )
  // }
  Ok(())
}
