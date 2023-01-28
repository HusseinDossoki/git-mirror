<p align="center">
  <img src="./src/assets/logo.png" height="120" />
</p>

# Git Mirror  [![license](https://img.shields.io/github/license/DAVFoundation/captain-n3m0.svg?style=flat)](https://github.com/HusseinDossoki/git-mirror/blob/dev/LICENSE)

## Libraries Used

#### Frontend

* [Vue3](https://vuejs.org/) - See [`source code`](./src)
* [bootstrap](https://getbootstrap.com/) - frontend toolkit

#### Backend

* [Rust](https://www.rust-lang.org/) and [Tauri](https://tauri.app/) framework - See [`source code`](./src-tauri)
* [reqwest](https://docs.rs/reqwest/latest/reqwest/) - The reqwest crate provides a convenient, higher-level HTTP Client.


## Set up your dev environment
1) Download and Install [Visual Studio Code](https://code.visualstudio.com/)
1) Download and Install [Rust](https://www.rust-lang.org/tools/install)
1) Download and Install [Node Js](https://nodejs.org/en/download/)

## Development

1) Clone the Git repository

```sh
git clone https://github.com/HusseinDossoki/git-mirror
```

2) CD into the folder

```sh
cd git-mirror
```

3) Install node dependencies 

```sh
npm install
```

4) Start the tauri dev server

```sh
npm tauri run dev
```