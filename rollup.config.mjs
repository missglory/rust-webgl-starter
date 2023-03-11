import rust from "@wasm-tool/rollup-plugin-rust";
import serve from 'rollup-plugin-serve'

export default {
    input: {
        example: "Cargo.toml",
    },
    output: {
        dir: "dist/js",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: "/js/",
        }),
        serve({
            contentBase: 'dist',
            port: 9000
        })
    ],
};
