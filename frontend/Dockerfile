FROM node:current-alpine

WORKDIR /app

COPY package.json ./
COPY package-lock.json ./
COPY ./ ./

RUN npm i

run npm install -g serve

RUN npm run build

EXPOSE 3000

CMD ["serve", "-s", "build"]