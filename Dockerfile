FROM rust:1.67.1-bullseye AS build
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install libclang-dev -y
#COPY ./assets/geometry_collection.rs /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/geo-types-0.7.7/src/geometry/geometry_collection.rs
RUN cargo build --release
RUN mkdir -p /app/lib
RUN ls
RUN cp -LR $(ldd ./target/release/actix-surreal | grep "=>" | cut -d ' ' -f 3) /app/lib


FROM scratch AS app
WORKDIR /app
COPY --from=build /app/lib /app/lib
COPY --from=build /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
COPY --from=build /app/target/release/actix-surreal actix-surreal
ENV LD_LIBRARY_PATH=/app/lib
ENTRYPOINT ["./actix-surreal"]