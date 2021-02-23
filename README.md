# RbxAPI-rs
RbxAPI-rs is an async Rust port of my [other](https://github.com/PythonicIconic/RbxAPI) Roblox API wrapper. [Discord](https://discord.gg/pdstefSahB) for following the project!

## Usage
The majority of this library currently revolves around the `Client` type. With this, you can get information about and perform various requests toauthentication-required
endpoints, without the hassle of dealing with the authentication!

Example of retrieving a given user, three different ways!
```rust
use rbxapi;

#[tokio::main]
async fn main() {
    let cookie = "your_cookie";
    let client = rbxapi::Client.new().cookie(cookie).await;
    
    let my_user = client.current_user().await;
    let str_user = client.user("builderman").await;
    let int_user = client.user(156).await;
}
```

## Notes
This is just the start of the project. There is essentially no error handling and not much functionality other than viewing a User or Game's general information. I am simply
publishing the basis of the project to give people an idea of what it's going to be. The goal, especially with it being a Rust port, is for this to be the single most efficient
way of interacting with Roblox's API. There is also no documentation quite yet due to this being a very first iteration. I will spend time on this where I can!
