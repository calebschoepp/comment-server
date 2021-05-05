FROM suborbital/atmo:v0.2.0

COPY --chown=atmo:atmo ./runnables.wasm.zip .
COPY --chown=atmo:atmo ./run.sh .

RUN chmod -R 777 ./runnables.wasm.zip
RUN chmod -R 777 ./run.sh

CMD [ "./run.sh" ]
