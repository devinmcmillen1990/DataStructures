### Repositories
#### Data-Structures
##### Commands
    1. cargo build
    2. cargo test
#### Data-Structures-Visuals
##### Commands
    1. cargo build
    2. wasm-pack build --target web --out-dir ../data-structures-ui/pkg
#### Data-Structures-UI
##### Commands
    1. .\node_modules\.bin\tailwindcss-cli.cmd -i ./styles/tailwind.css -o ./index.css
    2. python -m http.server 8080
        a. Run after Data-Structures build and Data-Structures-Visuals WASM pack