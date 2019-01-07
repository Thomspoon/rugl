# rugl

A clone of [regl](regl.party) a functional abstraction for wegbl.

The goal of this crate is to provide a simple Rust macro for stateless WebGL animations with minimal work!

## Example
```
rugl! {
    vertex: `
        precision mediump float;
        attribute vec2 position;
        void main() {
            gl_Position = vec4(position, 0, 1);
        }
    `;
    fragment: `
        precision mediump float;
        uniform vec3 color;
        void main() {
            gl_FragColor = color;
        }
    `;

    attributes: {
        position: [
            [-1, 0],
            [0, -1],
            [1, 1]
        ]
    }

    uniforms: {
        color: [1, 0, 0, 1]
    }
}
```

## TODO
- [ ] Everything
