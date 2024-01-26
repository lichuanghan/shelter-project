strip shelter_main
docker build -t shelter .
docker run -d -p 8080:8080 --name shelter shelter:latest