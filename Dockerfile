# Build phase
FROM rust:1.58.1 as builder

RUN USER=root cargo new --bin HeroesGame_Backend
WORKDIR /HeroesGame_Backend
# Build dependencies
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
# Build release binary
ADD . ./

RUN rm ./target/release/deps/heroes_game_backend*
RUN cargo build --release

# Run phase
FROM debian:buster-slim
ARG APP=/usr/src/app

# Install necessary packages
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libmariadb-dev-compat libmariadb-dev \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser
# Create new user to run the binary
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /HeroesGame_Backend/.env ${APP}/.env
COPY --from=builder /HeroesGame_Backend/Rocket.toml ${APP}/Rocket.toml
COPY --from=builder /HeroesGame_Backend/target/release/heroes_game_backend ${APP}/heroes_game_backend

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./heroes_game_backend"]
