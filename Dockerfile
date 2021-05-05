FROM suborbital/atmo:v0.2.0

COPY --chown=atmo:atmo ./runnables.wasm.zip .

RUN chmod -R 777 ./runnables.wasm.zip

USER root

ENTRYPOINT ["atmo"]
