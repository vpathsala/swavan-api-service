## SwaVan middlelayer Service

This repo contain rest api code that [SwaVan]("https://swavan.io/") use for data mocking.

SwaWan does not have it's own mock server, it use mocky to push the mock data over the internet. So you might be thinking why swavan does not directly talk with mocky api and the reason is to avoid multiple trip to mocky server.

SwaVan allow user to add, edit and delete the data in bulk and mocky api does not support bulk operation, which is why this middlelayer service exists.

### Flow Diagram

![Image of SwaVanDataFlow](https://miro.medium.com/max/1140/1*MKGcoWB9JM3RRqJa7by2uA.png)


### Why and how to use code

#### Why?
SwaVan cannot mock api without this middlelayer service as you can see in the flow diagram, this service sit in the middle between the SwaVan and Mocky.

#### How ?

Run the swavan-api middlelayer service using docker

Command to pull the image from dockerhub or github
```
docker pull biplabsamu/swavan-api:0.0.19

OR

docker pull docker.pkg.github.com/vpathsala/swavan-api-service/swavan-api:0.0.19
```

If you like to build image locally ( Clone this repo first )
```
 DOCKER_BUILDKIT=1 docker build --tag swavan-api:0.0.19 .
```

Command to run docker image
```
docker run --rm --name swavan-api-service -p 5000:8080 biplabsamu/swavan-api:0.0.19

or 

docker run --rm --name swavan-api-service -p 5000:8080 docker.pkg.github.com/vpathsala/swavan-api-service/swavan-api:0.0.19

```
