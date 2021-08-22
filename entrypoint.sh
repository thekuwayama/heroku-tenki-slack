#!/bin/bash

if [ -z "$SERVER_PORT" ]; then
    PORT=3000
fi

if [ -z "$SLACK_OAUTH_TOKEN" ]; then
    exit 1
fi

if [ -z "$SLACK_CHANNEL" ]; then
    exit 1
fi

if [ -z "$REDIRECT_PAGE" ]; then
    exit 1
fi

heroku_tenki_slack
