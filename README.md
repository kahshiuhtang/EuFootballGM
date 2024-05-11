# EuFootballGM

## Description

Desktop application simulate the EU football season. The idea behind this simulator is based off of the ZenGM American Sports Simulator games. I intend for the game to be written in Rust as a Desktop Application.

## Installation

Rust is needed for this program. To run, use: 

```bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

Tauri is used to create the desktop app. To install it, you will need to install some libraries:

```bash

sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

```

Finally, to run, do 

```bash

npm run tauri dev

```







## Todo

- [ ] Randomly generate players and allow for player progression
- [ ] Simulate games between two teams
- [ ] Simulate transfers between clubs
- [ ] Allow playesr to manage the financials of a club