FROM suborbital/atmo:v0.2.0

COPY ./runnables.wasm.zip .

ENTRYPOINT ["ls"]
