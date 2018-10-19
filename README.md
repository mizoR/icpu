# icpu

## Requirements

```sh
rustup default beta
npm install npm@latest -g
```

Also need to install [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)

## Quickstart

Build app:

```sh
wasm-pack build

pushd ./pkg

npm link

popd

pushd ./www

npm install

npm link icpu

popd
```

Run app:

```sh
pushd ./www; npm run start; popd
```

## Reference

- https://rustwasm.github.io/book/game-of-life/setup.html

