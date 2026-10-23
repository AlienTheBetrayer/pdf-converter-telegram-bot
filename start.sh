#!/bin/sh
echo "Starting Telegram bot..."
echo "Checking executable:"
ls -lh ./telegram-hi-bot

chmod +x ./telegram-hi-bot

./telegram-hi-bot 2>&1
STATUS=$?

echo "Bot exited with status: $STATUS"
sleep 300