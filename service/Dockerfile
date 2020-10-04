### Builder image
FROM node:14.12.0-alpine3.10 AS builder

ENV NODE_ENV=development

WORKDIR /nrl

COPY package.json package-lock.json /nrl/
RUN npm install

COPY tsconfig.json /nrl
COPY src /nrl/src
RUN npm run build


### Runtime Image
FROM node:14.12.0-alpine3.10

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

ENV NODE_ENV=production

WORKDIR /nrl

COPY package.json package-lock.json /nrl/
RUN npm install

COPY config /nrl/config/
COPY docker/start.sh /nrl/start.sh

COPY --from=builder /nrl/output/build /nrl/dist/

CMD /nrl/start.sh