use crate::{errors::Error, repository::RepoInfo, types::PatchInfo, Portan};

use nostr_rust::{
    bech32::{to_bech32, ToBech32Kind},
    events::Event,
};

use nostr_types::{Id, PublicKeyHex, Url};

/// Truncate the public key
/// ```
/// use portan::utils;
/// let pubkey = "npub1qjgcmlpkeyl8mdkvp4s0xls4ytcux6my606tgfx9xttut907h0zs76lgjw";
/// let pubkey = utils::truncated_npub(pubkey);
/// assert_eq!(pubkey.unwrap(), "npub1qjgcm...76lgjw");
/// let pubkey = "04918dfc36c93e7db6cc0d60f37e1522f1c36b64d3f4b424c532d7c595febbc5";
/// let pubkey = utils::truncated_npub(pubkey);
/// assert_eq!(pubkey.unwrap(), "npub1qjgcm...76lgjw");
/// ```
pub fn truncated_npub(hex_pub: &str) -> Result<String, Error> {
    let mut pub_key = hex_pub.to_string();
    if !pub_key.starts_with("npub1") {
        if hex_pub.len().ne(&64) {
            return Err(Error::InvalidKey);
        }
        pub_key = to_bech32(ToBech32Kind::PublicKey, &pub_key)?;
    }
    pub_key.replace_range(10..57, "...");
    Ok(pub_key)
}

impl Portan {
    /// Convert event to repo info
    /// ```rust
    /// use portan::utils;
    /// use portan::repository::RepoInfo;
    /// use nostr_rust::events::Event;
    ///
    /// let event = Event {
    ///    id: "1439f6c41f050365048726478f350c5ebe5ee4219d53ef788273257693c0db25".to_string(),
    ///    pub_key: "04918dfc36c93e7db6cc0d60f37e1522f1c36b64d3f4b424c532d7c595febbc5".to_string(),
    ///    created_at: 1673389764,
    ///    kind: 124,
    ///    tags: [["r".to_string(),"https://github.com/nostr-protocol/nips".to_string()].to_vec(),["n".to_string(),"nips".to_string()].to_vec()].to_vec(),
    ///    content: "".to_string(),
    ///    sig: "54af604bc8bc88449ad52facf65ae59b839497f7f17b9da71c356e1d897688e76562bc8424313039881850c99209972aca9d7c2470632aaac22a9090a8c0f256".to_string(),
    /// };
    ///
    /// let repo_info = utils::event_to_repo_info(&event).unwrap();
    ///
    /// let r = RepoInfo {
    ///     id: "1439f6c41f050365048726478f350c5ebe5ee4219d53ef788273257693c0db25".to_string(),
    ///     owner_pub_key: "04918dfc36c93e7db6cc0d60f37e1522f1c36b64d3f4b424c532d7c595febbc5".to_string(),
    ///     name: "nips".to_string(),
    ///     description: "".to_string(),
    ///     git_url: "https://github.com/nostr-protocol/nips".to_string(),
    /// };
    ///
    /// assert_eq!(repo_info, r);
    ///
    /// ```
    ///
    pub fn event_to_repo_info(&self, event: &Event) -> Result<RepoInfo, Error> {
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
            owner_pub_key: PublicKeyHex(event.pub_key.clone()),
            id: event.id.clone(),
            name: name.unwrap(),
            description: event.content.clone(),
            git_url: Url::new(&git_url.unwrap()),
        })
    }
}
/// Converts a nostr event to patch info
pub fn event_to_patch_info(event: &Event) -> Result<PatchInfo, Error> {
    if event.verify().is_err() {
        return Err(Error::EventInvalid);
    }
    let content: PatchInfo = serde_json::from_str(&event.content)?;

    let mut name: Option<String> = None;
    for v in &event.tags {
        if v[0].as_str() == "n" {
            name = Some(v[1].clone())
        }
    }

    if name.is_none() {
        return Err(Error::EventInvalid);
    }

    Ok(PatchInfo {
        id: event.id.clone(),
        author: event.pub_key.clone(),
        name: name.unwrap(),
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
pub fn encode_id_to_number(id: &Id) -> u32 {
    let mut sum = 0;
    for c in id.as_hex_string().chars() {
        sum += c as u32;
    }
    sum % 100000
}
