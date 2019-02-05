rustup target add wasm32-unknown-unknown --toolchain nightly
cargo +nightly install wasm-bindgen-cli

curl https://raw.githubusercontent.com/Thomspoon/rugl/master/webpack.config.js > webpack.config.js
curl https://raw.githubusercontent.com/Thomspoon/rugl/master/package.json > package.json
curl https://raw.githubusercontent.com/Thomspoon/rugl/master/build.rs > build.rs 
curl https://raw.githubusercontent.com/Thomspoon/rugl/master/index.html > index.html
curl https://raw.githubusercontent.com/Thomspoon/rugl/master/index.js > index.js 