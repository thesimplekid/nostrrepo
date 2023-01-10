use crate::{errors::Error, repository::RepoInfo, types::PatchInfo};

use nostr_rust::{
    bech32::{to_bech32, ToBech32Kind},
    events::Event,
};

/// Truncate the public key
/// ```
/// use portan::utils;
/// let pubkey = "npub15nr2zfan778slpf3lhql42z4ldwzrvdxlq4d6jea7nz94hlc63ps2vza9s";
/// let pubkey = utils::truncated_npub(pubkey);
/// assert_eq!(pubkey, "npub15nr2z...2vza9s");
/// ```
pub fn truncated_npub(hex_pub: &str) -> Result<String, Error> {
    if hex_pub.len().ne(&64) {
        return Err(Error::InvalidKey);
    }
    let mut pub_key = hex_pub.to_string().clone();
    if !pub_key.starts_with("npub1") {
        pub_key = to_bech32(ToBech32Kind::PublicKey, &pub_key)?;
    }
    pub_key.replace_range(10..57, "...");
    Ok(pub_key)
}

/// Convert event to repo info
/// ```rust
/// use portan::utils;
/// use portan::repository::RepoInfo;
/// use nostr_rust::events::Event;
///
/// let event = Event {
///    id: "98b64f3400ba1c358cd22c864cf789ba9ab7b091546f51ba8b973411e2ee2564".to_string(),
///    pub_key: "5333d5f643a05fd7ad4fe64feba217fda853013e8a15cf85753aecc90a05a64a".to_string(),
///    created_at: 1673058145,
///    kind: 124,
///    tags: [].to_vec(),
///    content: "{\"name\":\"repo-name\",\"description\":\"the description\",\"git_url\":\"https::git.thesimplekid.com/repo-name\"}".to_string(),
///    sig: "e284efa85a5e702cc7ef55c858bbfc5c487c42588cbe0c00123400d09333450189f7051e7562f8fb3a0d56eef92c9dc58d4c55c298225af0e6e1c5caf24963bc".to_string(),
/// };
///
/// let repo_info = utils::event_to_repo_info(&event).unwrap();
///
/// let r = RepoInfo {
///     id: "98b64f3400ba1c358cd22c864cf789ba9ab7b091546f51ba8b973411e2ee2564".to_string(),
///     owner_pub_key: "5333d5f643a05fd7ad4fe64feba217fda853013e8a15cf85753aecc90a05a64a".to_string(),
///     name: "repo-name".to_string(),
///     description: "the description".to_string(),
///     git_url: "https::git.thesimplekid.com/repo-name".to_string()
/// };
///
/// assert_eq!(repo_info, r);
///
/// ```
///
pub fn event_to_repo_info(event: &Event) -> Result<RepoInfo, Error> {
    if event.verify().is_err() {
        return Err(Error::EventInvalid);
    }
    // let content: RepoEventContent = serde_json::from_str(&event.content).unwrap();
    let mut git_url: Option<String> = None;
    let mut name: Option<String> = None;

    for v in &event.tags {
        match v[0].as_str() {
            "r" => git_url = Some(v[1].clone()),
            "n" => name = Some(v[1].clone()),
            _ => (),
        }
    }

    if name.is_none() || git_url.is_none() {
        return Err(Error::RepoUndefined);
    }

    Ok(RepoInfo {
        owner_pub_key: event.pub_key.clone(),
        id: event.id.clone(),
        name: name.unwrap(),
        description: event.content.clone(),
        git_url: git_url.unwrap(),
        local_path: None,
    })
}

pub fn event_to_patch_info(event: &Event) -> Result<PatchInfo, Error> {
    if event.verify().is_err() {
        return Err(Error::EventInvalid);
    }
    let content: PatchInfo = serde_json::from_str(&event.content)?;
    Ok(PatchInfo {
        id: event.id.clone(),
        author: event.pub_key.clone(),
        title: content.title,
        description: content.description,
        patch: content.patch,
    })
}

/// Encode event is to number
/// Since there is no global state of nostr, simply incrementing issue numbers will not work
/// However, having simple human friendly issues numbers is still desirable so the id of the event is encoded to a 4 digit number
/// ```rust
/// use portan::utils;
///
/// let id = "24f2e615551e03e06032826bc5aa2eff701091fc9f4dd0c520a4969f141feff5";
/// let num =  utils::encode_id_to_number(id);
///
/// assert_eq!(num , 4422);
///
/// ```
pub fn encode_id_to_number(id: &str) -> u32 {
    let mut sum = 0;
    for c in id.chars() {
        sum += c as u32;
    }
    sum % 100000
}
