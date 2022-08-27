# Attendace admin app

__A quick webapp - desktop app using Yew + Tauri framework with [Rust](https://www.rust-lang.org/)__

<!-- [![Gitter](https://badge.fury.io/js/electron-markdownify.svg)](https://badge.fury.io/js/electron-markdownify)
[![Markdownify](https://badges.gitter.im/amitmerchant1990/electron-markdownify.svg)](https://gitter.im/amitmerchant1990/electron-markdownify)
[![Say_Thanks](https://img.shields.io/badge/SayThanks.io-%E2%98%BC-1EAEDB.svg)](https://saythanks.io/to/bullredeyes@gmail.com)
[![Paypal](https://img.shields.io/badge/$-donate-ff69b4.svg?maxAge=2592000&style=flat)](https://www.paypal.me/AmitMerchant) -->

# Table of contents

- [Features](#features)
  - [Web](#web-features)
  - [Desktop-App](#app-features)

- [Download](#download)
- [How To Use](#how-to-use)
- [Credits](#credits)
- [Roadmap](#roadmap)
- [License](#license)

<!-- ![screenshot](https://raw.githubusercontent.com/amitmerchant1990/electron-markdownify/master/app/img/markdownify.gif) -->

## Features

### Web Features

- Login with the given account and using security stuff by google
- Track student informations and there attend date
- Quickly search through the database, __regex__ supported

### App Features

- Same as web features but i need some content to write
- __NOTICE!__: Mobile support is under developement right now.

## How To Use

Simply go to our website and login with your given account.
[Website](#roadmap)

## Download

### You can download this application in many ways

- __Clone__ this repo and run it for your self
- __Download__ the desktop aplication
- __Use__ the web version of this app, *See [How To Use](#how-to-use) up there*

### Clone this repo

  Simply use git commands or download the project as a zip file, it doesnt matter.

  ```bash
  # Clone the repo
  $ git clone https://github.com/thaiminh2022/attendance-admin.git attendance-admin
  
  ```

  Note that your project name should have __hyphen-case__ or __snake_case__ naming conventions since this is required by Rust.

#### __Setup__

- Go to <https://www.rust-lang.org/> and install Rust.
- [NPM](https://nodejs.org/en/download/) is required but the aplication will still mostly work without it.
- Make sure __Trunk__ and __WASM__ are installed since those are the back-bone of our project

```bash
  # Install trunk
  $ cargo install trunk
  
  # Install wasm
  $ rustup target add wasm32-unknown-unknown
```

__NOTICES__: This project's yew version is [__0.19.3__](/frontend/Cargo.toml), yew latest version and there's syntax may change in future version, check [this link](https://yew.rs/docs/getting-started/introduction) for a better install guild.

#### __Run the aplication__

```bash
  # Install all dependencies
  $ npm install

  # Optional, firebase-stuff, follow theres tutorial at firebase.google.com
  $ firebase login
  $ firebase init # Hosting, realtime database, authentication
  
  # Run the web version
  $ cd frontend
  $ >frontend> trunk serve

  # Run the app version
  $ cargo tauri dev
  
  # Build the app version
  $ cargo tauri build
  
```

Note: __For better fundamental understand, check tauri, yew, firebase docs__

## Credits

Although it is not require, we are really grateful for these.

- [Yew](https://yew.rs/)
- [Trunk](https://trunkrs.dev/)
- [Tauri](https://tauri.app)

__And__ all the creator of the dependencies use in project.

## Roadmap

- [x] Write a readme file
- [ ] Support for mobile version
- [ ] Intergrate with an actual attendance hardware
- [ ] Get a price for the idea

## License

Idk what to write here lol
