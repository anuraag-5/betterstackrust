FROM rustlang/rust:nightly

WORKDIR /app
COPY . .
ENV RUSTC_BOOTSTRAP=1

CMD ["cargo", "run", "--release", "--bin", "api"]