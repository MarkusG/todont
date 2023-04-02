#!/bin/bash

rsync -rv . mark@rudy:/home/mark/todont \
    --exclude '.git' \
    --exclude '*/target' \
