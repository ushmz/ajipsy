# ajipsy 

Simple command to set Slack status.

# Usage

Set your slack OAuth access token and member ID in `.env` file.
```.env
# Slack OAuth token
AJIPSY_ACCESS_TOKEN=${YOUR_TOKEN}
AJIPSY_USERID=${YOUR_MEMBERID}
```

```sh
ajipsy --room2525 # Set Slack Status 'Room2525'
ajipsy --room2719 # Set Slack Status 'Room2719'
ajipsy --home     # Set SLack Status 'Home'
ajipsy --out      # Set SLack Status 'Going out'
ajipsy --reset    # Reset Slack Status
```

You can also specify your status as you like by `-t` and `-e` option.
```
-e, --emoji <EMOJI>    Status emoticon(like :school:)
-t, --text <TEXT>      Status text
```

Show help

```sh
repo [-h|--help]
```
