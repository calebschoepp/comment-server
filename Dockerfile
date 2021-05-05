FROM suborbital/atmo:v0.2.0

COPY --chown=atmo:atmo ./runnables.wasm.zip .

ENTRYPOINT ["atmo"]
