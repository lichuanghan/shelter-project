FROM scratch

COPY ./shelter_main  /shelter_main
COPY ./config.json /config.json
COPY ./.env /.env

EXPOSE 8080

ENTRYPOINT [ "/shelter_main","serve" ]
