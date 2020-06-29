# streams-pubsub

Example from https://github.com/AleBuser/iota-channels-lite edited and splitted into two binary targets

Run both at the same time and share the address and tags

Author: `cargo run --release --bin pub`

```
Running `target\release\pub.exe`
Please enter the author seed
AUTHORSEED
Author seed:"AUTHORSEED"
Author: Announced channel address and tag: YGXTLBBZQNYFZZHQWKWYFXWZEJJXIAZYCYEFUMZPMCMBYNR9MHI9KXZRDPWODYBDV9SZOTLHQRVUIIIIA OUWDCFHJWDOBELJUNGIRPDSKMYE
Enter tag from subscriber
AFXNFYBDPJBGDXVZGJPB9CWIKBB
Author keyload_tag: ACBJU9ZJHPPQQJTPEHEFEQIUGUJ
Please enter the message you want to publish
Hey, message sent with streams
Author: Sent signed masked message DNWKWARZP9CHHQURYZPGMFAWIAD None
```

Subscriber: `cargo run --release --bin sub`

```
Running `target\release\sub.exe`
Please enter the subscriber seed
SUBSCRIBERSEED
Subscriber seed:"SUBSCRIBERSEED"
Please enter the channel_address and tag from the author
YGXTLBBZQNYFZZHQWKWYFXWZEJJXIAZYCYEFUMZPMCMBYNR9MHI9KXZRDPWODYBDV9SZOTLHQRVUIIIIA OUWDCFHJWDOBELJUNGIRPDSKMYE
Subscriber: Connected to channel with tag: AFXNFYBDPJBGDXVZGJPB9CWIKBB
Please enter the keyload_tag from the author
ACBJU9ZJHPPQQJTPEHEFEQIUGUJ
Please enter the signed_packed_tag_masked and change_key_tag_masked from the author
DNWKWARZP9CHHQURYZPGMFAWIAD None
Subscriber: Reading signed masked messages
Subscriber: Found Signed Masked Message -> Public: None -- Masked: Some("Hey, message sent with streams")
```