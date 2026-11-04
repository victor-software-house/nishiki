FROM alpine:3.20 AS base
ONBUILD RUN apk add --no-cache git
onbuild COPY . /src
ONBUILD ENTRYPOINT ["/src/run"]
CMD ["sh", "-c", "echo \"done\""]
LABEL note='it\'s here'
