FROM node:14

RUN useradd -ms /bin/bash myuser
USER myuser

WORKDIR /code

COPY --chown=myuser:myuser . .

RUN npm install

ENV API_PORT=3000
ENV API_URI=http://localhost

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 CMD curl -f http://localhost:$API_PORT || exit 1

CMD ["npm", "run", "test:watch", "--verbose", "--maxConcurrency=1", "-w", "1"]