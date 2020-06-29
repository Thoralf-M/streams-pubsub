use channels_lite::channels::{channel_author, Network};
use channels_lite::utils::payload::json::PayloadBuilder;
use failure::Fallible;

mod user_input;
use user_input::get_words;

#[tokio::main]
async fn main() -> Fallible<()> {
    println!("Please enter the author seed");
    let seed = get_words(1)?;
    println!("Author seed:{:?}", seed[0].to_uppercase());
    let seed_author = Some(seed[0].clone().to_uppercase());

    //Create Channel Instance for author
    let mut channel_author = channel_author::Channel::new(
        Network::Custom("https://node04.iotatoken.nl:443", 14),
        seed_author,
    );
    //Open Channel
    let (channel_address, announcement_tag) = channel_author.open().unwrap();
    println!(
        "Author: Announced channel address and tag: {} {}",
        channel_address, announcement_tag
    );

    //wait for sub
    println!("Enter tag from subscriber");
    let subtag = get_words(1)?;
    let subscription_tag = subtag[0].clone();
    //Add subscriber
    let keyload_tag = channel_author.add_subscriber(subscription_tag).unwrap();
    println!("Author keyload_tag: {}", keyload_tag);

    // //Write signed public message
    // println!("Please enter the message you want to publish");
    // let message = get_words(0)?;
    // let response_write_signed = channel_author
    //     .write_signed(
    //         false,
    //         PayloadBuilder::new().public(&message.join(" "))?.build(),
    //     )
    //     .unwrap();

    // let signed_packed_tag_public = response_write_signed.signed_message_tag;
    // let change_key_tag_public = response_write_signed.change_key_tag;
    // println!(
    //     "Author: Sent signed public message {} {:?}",
    //     signed_packed_tag_public, change_key_tag_public
    // );

    //Write signed masked message
    println!("Please enter the message you want to publish");
    let message = get_words(0)?;
    let response_write_signed = channel_author
        .write_signed(
            true,
            PayloadBuilder::new().masked(&message.join(" "))?.build(),
        )
        .unwrap();
    let signed_packed_tag_masked = response_write_signed.signed_message_tag;
    let change_key_tag_masked = response_write_signed.change_key_tag;
    println!(
        "Author: Sent signed masked message {} {:?}",
        signed_packed_tag_masked, change_key_tag_masked
    );

    // //Write tagged message
    // let tagged_packed_tag: String = channel_author
    //     .write_tagged(
    //         PayloadBuilder::new()
    //             .public(&SensorData::new(17.0))?
    //             .masked(&SensorData::new(19.0))?
    //             .build(),
    //     )
    //     .unwrap();
    // println!("Author: Sent tagged message: {}", tagged_packed_tag);

    Ok(())
}
