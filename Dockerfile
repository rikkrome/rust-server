FROM debian:bullseye-slim
WORKDIR /var/temp
ADD target/release/rust-graphql-example .
CMD ["/var/temp/rust-graphql-example"]