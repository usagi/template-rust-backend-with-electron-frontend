# template-rust-backend-with-electron-frontend ( template project )

Note: This template project of the `master` branch was migrated to use [`NEON`][] version! If you want to use the `node-ffi-napi` version, then switch to the [branch/node-ffi-napi][].

[branch/master]:https://github.com/usagi/template-rust-backend-with-electron-frontend/
[branch/node-ffi-napi]:https://github.com/usagi/template-rust-backend-with-electron-frontend/tree/node-ffi-napi

This is the template project with these stacks:

1. [`Rust`][] native code library project for backend business logics
      - -> see also `Cargo.toml` and
          - `native/` directory is the marshalling part using [`NEON`][], its `Rust` with [`N-API`][] technology stacking.
          - `native/backend` directory is the native code business logic core part, its pure-`Rust`.
          - `rust-toolchain`, `.rustfmt.toml` files
2. [`Node.js`][] with [`react`][] + [`rescripts`][] + [`electron`][] + [`electron-rebuild`][] for frontend presentation logics
      - -> see also `package.json` and
          -  `src/`, `public/`, `assets/` directories
          -  `.resctiptsrc.js`, `.webpack.config.js` files

![Screenshot](https://i.imgur.com/WUJdzLI.png)

[`Rust`]:https://www.rust-lang.org/
[`Node.js`]:https://nodejs.org/
[`react`]:https://reactjs.org/
[`rescripts`]:https://github.com/harrysolovay/rescripts
[`electron`]:https://www.electronjs.org/
[`electron-rebuild`]:https://github.com/electron/electron-rebuild
[`ffi-napi`]:https://github.com/node-ffi-napi
[`NEON`]:https://neon-bindings.com/
[`N-API`]:https://nodejs.org/api/n-api.html

## Usage

Prepare these toolchains in advance:

- Rust toolchain -> <https://www.rust-lang.org/tools/install>
- Node.js toolchain -> <https://nodejs.org/en/download/>

And, these are optional technologies:

- [`git`][]
- [`cargo-generate`][]
- [`yarn`][]

And, you will need if you development with Windows(It will be required by [`electron-rebuild`][]):

- [`windows-build-tools`][] ; `npm -g i windows-build-tools`
    - with [`Visual C++`][] ; The standard toolchain of [`Node.js`][] of Windows version

Note: You can install manually or use a platform installers such as [`choco`](Windows), [`brew`](OSX), [`pacman`](Arch Linux), [`apt`](debian, Ubuntu) or etc.

[`choco`]:https://chocolatey.org/
[`brew`]:https://brew.sh/
[`pacman`]:https://wiki.archlinux.org/index.php/pacman
[`apt`]:https://wiki.debian.org/Apt
[`git`]:https://git-scm.com/
[`yarn`]:https://yarnpkg.com/
[`Visual C++`]:https://docs.microsoft.com/en-us/cpp/
[`windows-build-tools`]:https://www.npmjs.com/package/windows-build-tools

### Step-1: Generate your project from this template

- Method (a): [`cargo-generate`]; Recommended

```sh
cargo generate --git https://github.com/usagi/template-rust-backend-with-electron-frontend.git
```

[`cargo-generate`]:https://github.com/ashleygwilliams/cargo-generate

or

- Method (b): For general; But not recommended

1. Copy ( or `git clone` ) this repos to your working directory.
2. Remove `cargo-generate.toml` file.
3. Modify `Cargo.toml` to your project.

### Step-2: Modify settings to your project

1. `Cargo.toml` and several setting files for Rust technology stack:
   - `Cargo.toml`: `[workspace]` / `members` and `Cargo.toml` for each workspace.
   - `rust-toolchain` ; Change to `stable` or `beta` if you need. The default is `nightly`.
   - `.rustfmt.toml` ; Change to your style or remove it for simple if you like the standard rustfmt style. The default is like the [Allman style][].
2. `package.json` for Node.js technology stack:
   - `name`
   - `description`
   - `version`
   - `private`
   - `author`
     - `name`
     - `email`
     - `url`
   - `build`
     - `appId`
     - `productName`
     - `copyright`
     - `mac`
         - and add `win`, `linux`, ... if needed.
   - `scripts`/`electron-pack`; Change the platform argument `-w` to your target platform such as `-wml` if you need.

[Allman style]:https://en.wikipedia.org/wiki/Indentation_style#Allman_style

### Step-3: Test `cargo` and `yarn` ( `npm` ) scripts

- `cargo` ecosystem
  - `cargo check`
  - `cargo test`
- `neon`-cli ( ≃ `cargo` + building suppliments ) ecosystem
  - `neon build`; (†1)
- `yarn` ecosystem; It's customized for this stacks
  - `yarn dev` ≃ `neon build` + `start` + `electron`
  - `yarn build` (†1)
  - `yarn test`
  - `yarn electron-pack` ≃ `cargo build --release` + `build` + `electron-builder`

(†1): If you using a Windows environment then see also <https://github.com/neon-bindings/neon/issues/463> . And choose from below:

1. To use `yarn dev.release` and a release version scripts.
2. Or, modify the neon dependencies to `{ git = "https://github.com/usagi/neon", branch = "fix-463-debug-build-mode-in-windows" }` these are `neon` and `neon-build` in the `dependencies` and `build-dependencies` in the `native/Cargo.toml`.
3. Or, wait merge the fixing and new releasing.

### Step-4: Change README.md and LICENSE

- Change ( or create or remove for simple ) `README.md` and `LICENSE` files.

Note: *This process is very important if you don't want to apply the MIT licensing for your app derived from this template. You don't forget it.*

### Step-5: Upgrade packages

1. `yarn upgrade`

Note: The Rust technology stack part has no dependencies, thus you would not need update packages. Maybe, [`cargo-update`][] is useful if you need a Rust technology stack part of your app derived this template in a future.

[`cargo-update`]:https://github.com/nabijaczleweli/cargo-update

Well, done! Congratulations, you can begin to develop your app now.

- Unit testing only the backend: `yarn test` = `cargo test`
- Build the frontend and the backend: `yarn build` = `neon build`
- Run the app with build: `yarn dev` = `yarn build` + `yarn start`
- Package the app with build: `yarn electron-pack` = `build.release` + ...
- And,
  - You can use any `cargo` and `yarn` ( `npm`, `npx`, etc. ) or, Rust and Node.js technologies!
  - To see [src.rs/lib/src/lib.rs](src.rs/lib/src/lib.rs) and try to modify it at the first if you are newbie for Rust FFI technologies!
  - To see [public/preload.js](public/preload.js) and [src/App.js](src/App.js) and try to modify these at the first if you are newbie for Electron FFI technologies!

Good luck!!

## Notes

1. You don't need `eject` of `create-react-app` because this template using [`rescripts`][]
2. You can choose to any licensing for your app derived from this template. I author of this template applied MIT licensing for this template project only.

### FAQ

(Q1) How to add a more native lib?

1. `cd src.rs`
2. `cargo new extra_my_lib --lib`
3. Add the new workspace definition into the top level `Cargo.toml`
4. Modify `Cargo.toml` of the new workspace; Maybe at least, you will need `cdylib` configuration.
5. Modify `package.json`; Maybe at least, you will need add the new definition into `build.{win|mac|linux}.extraFiles`.
6. Well done! You can use your new extra lib in your front-end of electron.

(Q2) How to add a resource file for use from a native lib?

- Maybe, you will need add the new definition into `build.extraFiles`.

(Q3) Why it use `NEON`, not `node-ffi-napi`?

See: <https://github.com/usagi/neon-vs-node-ffi-napi#neon-vs-node-ffi-napi>

### Related references

#### Useful documentations for a developers use this template

- Rust side:
  - [The Rust FFI Omnibus][]; Very useful for FFI marsharing basics!
  - docs:
    - [std::os::raw][] ; `c_char`, `c_ulonglong`, ...
    - [std::ffi][] ; `CString`, `CStr`, ...
- Node.js sise:
  - [blackhat ASIA 2019 DOYENSEC Preloading Insecurity In Your Electron][]; Useful to your understanding for process isolation and IPC mechanical of Electron's preload feature
  - Electron Documentation:
    - [Docs / Guides / Security, Native Capabilities, and Your Responsibility][]
  - npm:
    - [node-ffi-napi][]
    - [node-ref-napi][]

[std::os::raw]:https://doc.rust-lang.org/std/os/raw/index.html
[std::ffi]:https://doc.rust-lang.org/std/ffi/index.html
[node-ffi-napi]:https://www.npmjs.com/package/ffi-napi
[node-ref-napi]:https://www.npmjs.com/package/ref-napi
[The Rust FFI Omnibus]:https://github.com/shepmaster/rust-ffi-omnibus
[blackhat ASIA 2019 DOYENSEC Preloading Insecurity In Your Electron]:https://doyensec.com/resources/Asia-19-Carettoni-Preloading-Insecurity-In-Your-Electron.pdf
[Docs / Guides / Security, Native Capabilities, and Your Responsibility]:https://www.electronjs.org/docs/tutorial/security

#### Official site of the elemental technology stacks

- [`Rust`][]
- [`Node.js`][]
- [`react`][]
- [`rescripts`][]
- [`electron`][]
- [`ffi-napi`][]
- [`NEON`][]

## License

[MIT](LICENSE)

Note: This licensing is applying to this original template only. It is not a copy-left. You choose any licensing to your app derived this template project! And I author of this original template would not be  effect your derived project anything!

## Author

USAGI.NETWORK / Usagi Ito <https://usagi.network/>
