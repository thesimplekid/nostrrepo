### NostrRepo

The goal of this project is to create a tool to coordinate code development over [Nostr](https://github.com/nostr-protocol/nostr).
It is in its very early stages and much is left to do.
Currently it supports issues, sending and receiving patches(very crude) and even that not very well. 

## State
This should be seen as Alpha level software for testing and building in the open and not a usable tool for development, yet. More details on the nostr events that are published and read can be found here [portan/README.md](portan/README.md).

This [template](https://github.com/emilk/eframe_template) was used to start this project some dead code related to compiling to WASM is left around and not used ~~as I have not decided if I will works towards WASM support, I would like to support WASM but it a may limit me as [nostr_rust](https://github.com/0xtlt/nostr_rust) would have to be changed or reimplemented in order to be WASM compatible~~. I'm almost definitely not going to work towards WASM support as I think a better desktop client and web client can be built separately.

The UI is really ugly, I'm hesitant to put too much time into fixing this at the moment for two reasons the first being I think its more important to get more functionality then make it look good for now. The second is I started this project in [Iced](https://github.com/iced-rs/iced) before realizing it didn't have multiline text input, and I like the [Elm](https://guide.elm-lang.org/architecture/) architecture that Iced uses so I would like to move back to Iced if I can.

Some more informal ideas that may or may not get implemented can be found [here](THOUGHTS.md).

## Development Environment  
Currently it will publish to relays in the .env file, I have this set by default to my relay as I don't want to spam other relays during testing, relays can be added or changed in this file or within the GUI.  A `.env-dev` file can also be added that will take priority over the `.env` file. 

The nostr secret key can also be set here, if one is not set a new one will be generated on start up (but not saved)

**WARNNING NOSTR KEYS**
Be cautious that if you generate a new key and have not saved the private key you you will not be able to post as the repository owner. 
This will hopefully get fixed soon as it's obviously terrible.
You can login with your own private key by pasting it in the settings or .env file. **You should think twice about pasting your private key anywhere.** However, this is written in rust so its fine. (/s)
But it is not a web app and it does not persistent (yet) so there are worse places to paste it, up to you.

For now it will default to only publish and query my relay, as I'm sure there are many inefficient calls and don't want to spam other relays during testing, relays can be added or removed in the settings.


## Events
At this point I can't promise that there won't be backwards incompatible changes to the event structures. This may cause issues with viewing previously published events. 

A more detailed look at the events can be found [here](portan/README.md)

## Publishing a Repository
Anyone can publish a repository associated with their nostr identity. Publishing a repository isn't really publishing the code(yet) it is announcing to the nostr relays that the code exists somewhere along with a name and a description. This allows others to query the nostr relays and find published repositories. 

## Issues

### Publish Issue
An issue is published with a nostr event kind 125 the e tag being the event id of the publish repo event and the event content being the serialized json of the issue title and description (content).  

### Publish Issue Comment
Issues can be commented on by publishing a nostr event with kind 126 with the content being the comment text.

### Publish Issue Status
The status of an issue can be updated by publishing a nostr kind 127 event with content being either Close, CloseCompleted or Open. The client should discard any events not posted by the the pubkey of the issue author or the pubkey of the repository owner*. 

*this should be updated later to support multi repo owners or groups probably by publishing some sort of owners similar to issue status. Some sort of [delegation](https://github.com/nostr-protocol/nips/blob/master/26.md) might be used here.

## Publish a Patch
A publish patch event is a kind 128 with the "e" tag the `event id` of the publish issue event. Patches that have been published in the patches section of the repository for now, the patch has to be manually applied via either saving the patch or copying it. 


## License 
Code is under the [BSD 3-Clause License](LICENSE-BSD-3) or the [Apache-2.0 License](LICENSE-APACHE)

Icons are used under [MIT License](assets/iconoir/LICENSE) from [https://iconoir.com/](https://iconoir.com/)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Contact
I can be contacted for comments or questions on nostr at _@thesimplekid.com (npub1qjgcmlpkeyl8mdkvp4s0xls4ytcux6my606tgfx9xttut907h0zs76lgjw) or via email tsk@thesimplekid.com.


## TODO (an incomplete list):
- [x] Close vs close completed 
- [x] Names shouldn't allow spaces
- [x] Petnames
- [ ] Handle the unwraps (there are alot)
- [x] Login (sorta)
- [x] Add relays
- [x] Display at least a link to the code repo (pretty lame)
- [x] Separate closed vs open issues
- [x] Issues should have a human friendly number or id. Encode the ID somehow?
- [ ] New repo refresh
- [ ] Issue status don't refresh correctly
- [x] Verify events
- [x] Send patches/pull requests
- [x] Download Patches
- [x] env var for relays
- [x] Nostr key from env
- [x] Select local folder that matches repo title
- [ ] Should be able to comment on patches
- [ ] Status of patches
- [ ] Add a DB
- [ ] Publish repo using hash of the first two commits
- [ ] Show code of repo
- [ ] Async
- [ ] Markdown support
- [ ] Reactions on comments
- [ ] Spam
    - [ ] Repo owner should be able to mark comments to hide
- [ ] Styling (it's super ugly) 
    - [ ] Bold repo names
    - [ ] Bold Issue Title
## Bugs (an incomplete list)
- [x] Get stuck on a issue for a repo need to have a back to issues
- [x] Creating an issue should add it to local list
- [ ] Sometimes my cursor will disappear after coming back from other windows (think its something with wayland).
