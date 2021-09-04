#!/bin/bash

if [ -z "$SLACK_OAUTH_TOKEN" ]; then
    exit 1
fi

if [ -z "$SLACK_CHANNEL" ]; then
    exit 1
fi

if [ -z "$REDIRECT_PAGE" ]; then
    exit 1
fi

PORT=3000 heroku_tenki_slack
