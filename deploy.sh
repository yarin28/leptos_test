#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly SSH_HOST=$1
readonly RUN_SCRIPT=./run_on_host.sh
readonly TARGET_PATH=~/Projects/leptos-start/
readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly TARGET_PATH_BIN=~/Projects/leptos-start/leptos_start
readonly TARGET_PATH_DIR=~/Projects/leptos-start/
readonly SOURCE_PATH_BIN=./target/server/aarch64-unknown-linux-gnu/release/leptos_start
readonly SOURCE_PATH_SITE=./target/site
readonly ENV_VARIABLES="LEPTOS_OUTPUT_NAME="leptos_start" LEPTOS_SITE_ROOT="site" LEPTOS_SITE_PKG_DIR="pkg" LEPTOS_SITE_ADDR="0.0.0.0:3000" LEPTOS_RELOAD_PORT="3001" LEPTOS_ASSETS_DIR="assets" LEPTOS_STYLE_FILE="output.css""
readonly COMMAND_TO_RUN_ON_TARGRT="cd ${TARGET_PATH_DIR}&& bash -c \"${ENV_VARIABLES} ${TARGET_PATH_BIN}\""
cargo leptos build --release
rsync ${SOURCE_PATH_BIN} ${SSH_HOST}:${TARGET_PATH}
rsync ${RUN_SCRIPT} ${SSH_HOST}:${TARGET_PATH}
rsync -r ${SOURCE_PATH_SITE} ${SSH_HOST}:${TARGET_PATH}
cd ${TARGET_PATH_DIR}
ssh -t ${SSH_HOST} ${COMMAND_TO_RUN_ON_TARGRT}
