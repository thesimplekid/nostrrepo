## Message Kinds

| Message Kind        | `kind` |
| ------------------- |--------|
| Repository          | 124    |
| Issue               | 125    |
| Issue Comment       | 126    |
| Status              | 127    | 
| Patch               | 128    |


## Publish a Repository
A publish repository event is kind 124 with the tag `n` of the `repo_name` and `r` of the `git_url` and content the repo description 
```json
{
tags: [[n, <repo_name>],[r, <git_url>]]
content: <repo_descriptiopn>
}
```

## Publish an Issue
A publish issue event is a kind 125 with the an "e" tag of the `event id` of the publish repository, with the content being a JSON-serialized string of:
```json
{
    title: "",
    description: ""
}
```

## Publish an Issue Comment
A publish issue event is a kind 126 with an "e" tag of the `event id` of the publish issue issue event and the content the comment. 

## Publish Issue Status Update
A publish issue status event is a kind 127 with the "e" tag the `event id` of the publish issue event and the content being a JSON-serialized sting of the status 

```json
{
    CloseCompleted/Close/Open
}
```

While there is no way to stop anyone from publishing an issue status those not published by the issue author or the repository owner should be ignored.

## Publish a patch
A publish patch event is a kind 128 with the "e" tag the `event id` of the publish issue event and the content being a JSON-serialized sting of:
```json 
{
    title: "",
    description: "",
    patch: "<generated with git format-patch>"
}
``` 

## TODO:
- [ ] Alot of the content should be moved to tags 
- [ ] Make async
- [ ] Add example events
- [ ] Efficiency improvements (few places that are a little hacky)
    - [ ] Petname calls
    - [ ] A lot of functions have more arguments then i would like
    this can probably be reduced using DB. [redb](https://github.com/cberner/redb)
- [ ] Error handling    