FROM suborbital/atmo:v0.2.0

COPY --chown=atmo:atmo ./runnables.wasm.zip .
COPY --chown=atmo:atmo ./run.sh .

RUN chmod -R 777 ./runnables.wasm.zip
RUN chmod -R 777 ./run.sh

USER root

WORKDIR /home/atmo

# RUN export ATMO_HTTP_PORT=$PORT

CMD [ "./run.sh" ]
