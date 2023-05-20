FROM node:current-alpine

WORKDIR /app

COPY package.json ./
COPY package-lock.json ./
COPY ./ ./

RUN npm install -g serve
RUN npm i
RUN npm run build

EXPOSE 3000

CMD ["serve", "-s", "build"]