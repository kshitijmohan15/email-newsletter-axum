FROM rust:latest as builder

RUN USER=root cargo new --bin email_newsletter_axum
WORKDIR /email_newsletter_axum
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
RUN rm ./target/release/deps/email_newsletter_axum*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /email_newsletter_axum/target/release/email_newsletter_axum ${APP}/email_newsletter_axum

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./email_newsletter_axum"]