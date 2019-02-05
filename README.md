# rugl

A clone of [regl](regl.party) a functional abstraction for wegbl.

The goal of this crate is to provide a simple Rust macro for stateless WebGL animations with minimal work!

## Example
```rust
use rugl::prelude::*;

rugl! {
    vertex: "
        precision mediump float;
        attribute vec2 position;
        void main() {
            gl_Position = vec4(position, 0, 1);
        }
    ";
    fragment: "
        precision mediump float;
        uniform vec3 color;
        void main() {
            gl_FragColor = color;
        }
    ";

    attributes: {
        position: [
            [-1, 0],
            [0, -1],
            [1, 1]
        ]
    }

    uniforms: {
        color: [1, 0, 0, 1]
    },

    count: 3
}
```

## Quickstart

Add wasm to your toolchain

0. `rustup target add wasm32-unknown-unknown --toolchain nightly`

Install wasm-bindgen-cli

1. `cargo +nightly install wasm-bindgen-cli`

Create a new library via cargo

2. `cargo new --lib rugl_test && cd rugl_test`

Pull necessary files

3. `curl https://raw.githubusercontent.com/Thomspoon/rugl/master/install_rugl.sh | sh`

Replace the code in your lib.rs with the following:

```rust
use rugl::prelude::*;

rugl!(
    vertex: {
        "
            attribute vec4 position;
            void main() {
                gl_Position = position;
            }
        "
    },
    fragment: {
        "
            precision mediump float;
            uniform vec4 color;

            void main() {
                gl_FragColor = color;
            }
        "
    },
    attributes: {
        position: [
            [-0.7, -0.7, 0.0],
            [ 0.7, -0.7, 0.0],
            [ 0.0,  0.7, 0.0]
        ],
    },
    uniforms: {
        color: [0.0, 0.9, 0.5, 0.3]
    },

    count: { 3 }
);
```

4. Install npm modules

`npm install`

5. Build your crate

`cargo build`

6. Serve your crate

`npm run serve`

7. Go to http://localhost:8080, and you should see the photo below:



## TODO
- [ ] Run Clippy
- [ ] Implement Animation Capability
- [ ] Cleanup Code and Publish Crate
- [ ] Spruce Up Macro to Support Non-bracken Syntax
