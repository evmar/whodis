#!/bin/sh

set -e

./glue/build.sh
(cd web && npx tsc && npm run build)
cp web/{index.html,style.css,bundle.js,glue_bg.wasm} deploy
