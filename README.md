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

sudo apt-get install javascriptcoregtk-4.1 libsoup-3.0 webkit2gtk-4.1 -y

```

Finally, to run, do 

```bash

pnpm i

pnpm run tauri dev

```

## Design

Here are just some thoughts on how one should go about implementing EuFootballGM.

Each game should be inside its own game struct. Realistically, this should all be stored in a DB because it will get too slow to cache everything into memory but for now, I will assume memory can hold it.

Inside a game, we will have leagues. Each league will have divisions will a set number of teams. After each season, a certain amount of teams will be relegated, anda  certain amount will be promoted. Top finishers in the highest division of every league will play in a European tournament. Each season, there will also be a domestic cup or two.

Each team should have to manage its players and finances. These include:

1) Players
    * Tactics
    * Injuries
    * Transfers
    * Academy
    
2) Finances
    * Stadium
    * Transfers
    * Fans
    * Food Sales + Prices
    * Facilities

Players should all begin with some level of attributes. All players come from an academy: depending on the quality of the academy, there will be a larger range in potential player development. Players have a specific position and will see reduced performance if they play out of position. You and other teams can offer to transfer players for a fee or for a combination of players. A player will have a satisfcation rating: a higher satisfaction will see the player perform better and make him more likely to extend his contract. Players can get injured: if the injuries are often, this can impact the players performance and will drive down their attributes.

Finances are a bit more tricky. Each team has serveral streams of income, whether from league earnings, winning trophies, ticket sales or merch/food sales. The amount a team can spend is strictly dependent on the budget given by the higher-ups. A team can make money but be less willing to spend than a lower income team. However, poor financials and team results will ultimately lead to you getting sacked.


The goal every manger has is to win games and make money.


## Game Simulation

1) Take a query of each team's starting roster, the pressure of the game, the environment and the players health. Generate a team strength value to represent each team

2) Randomly select from a set of events on what happens 

## Todo

- [ ] Randomly generate players and allow for player progression
- [ ] Simulate games between two teams
- [ ] Simulate transfers between clubs
- [ ] Allow playesr to manage the financials of a club