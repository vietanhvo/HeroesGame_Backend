# Heroes Game Project

[![Rust](https://github.com/vietanhvo/HeroesGame_Backend/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/vietanhvo/HeroesGame_Backend/actions/workflows/rust.yml)

## Summary

Heroes game is a web-based game project for our WebApp course. In our game, users may form their own heroes squad to combat monsters. As a result, heroes gain experience and gold to help them level up. Initially, we give new users 100.000 gold to purchase any heroes in our store. Besides, if users want to upgrade their heroes' stars, they also need to buy gems to satisfy the requirement. In general, the technologies we used for this project are as follows:
* Frontend: [Next.js](https://nextjs.org/)
* Backend: [Rocket.rs](https://rocket.rs/) server & [Diesel.rs](https://diesel.rs/) ORM
* Game Engine: [Cocos Creator](https://www.cocos.com/en/)
* Database: MySQL 

## Prerequisites

We understand that installing everything is hard, time-consuming, and prone to conflict. As a result, we Dockerized everything for ease of use.
* Git
* Docker

## Installation

You need to clone our repository first:
```git
git clone https://github.com/vietanhvo/HeroesGame_Backend.git
```
Next, take a look at our .env_sample and create your own .env file. We will use some of your machine's ports to expose Docker's ports: 3000, 8000, and 3306. So, please stop any processes running on these ports before running.

## Usage

By using the following command you can start up the application. This process can take a long time. Please be patient!
```sh
docker-compose up
```
Go to http://localhost:3000/ and enjoy :beers:. Have a nice day!

You can also run docker-compose in detached mode using -d flag, then you can stop it whenever needed by the following command:
```sh
docker-compose stop
```
In the end, you need to remove the containers entirely, with the down command and `-v` flag to delete the data volume:
```sh
docker-compose down -v
```

## Troubleshooting

Please read carefully our instructions in the .env_sample file. Your .env file is the only factor that decides whether Docker can run or not.
1. `openssl rand -base64 32` cannot run on my machine?

Unlike Ubuntu, Windows does not have a built-in openssl executable, you need to install it manually. For more information, read the instruction [here](https://www.openssl.org/source/)

2. Why has Docker still crashed after all of the problems were fixed?

On the first run, Docker created the images on your machine. Those images won't update automatically, so you need to build and run them again:
```sh
docker-compose build
```
Or, you can add `-- build` flag with the command `docker-compose up --build` to build images before starting containers.

## Authors
1. Vo Anh Viet - ITITIU19243 - Project Leader
2. Nguyen Duc Anh Tai - ITITIU19204
