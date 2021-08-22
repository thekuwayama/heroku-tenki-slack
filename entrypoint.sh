#!/bin/bash

if [ -z "$PORT" ]; then
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

PORT=${PORT} SLACK_OAUTH_TOKEN=${SLACK_OAUTH_TOKEN} SLACK_CHANNEL=${SLACK_CHANNEL} REDIRECT_PAGE=${REDIRECT_PAGE} heroku_tenki_slack
