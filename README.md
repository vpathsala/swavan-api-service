## SwaVan middleware Service

This repo contain rest api code that [SwaVan]("https://swavan.io/") use for data mocking.

SwaWan does not have it's own mock server, it use mocky to push the mock data over the internet. So you might be thinking why swavan does not directly talk with mocky api and the reason is to avoid multiple trip to mocky server.

SwaVan allow user to add, edit and delete the data in bulk and mocky api does not support bulk operation, which is why this middleware service exists.

### Flow Diagram

![Image of SwaVanDataFlow]
(https://miro.medium.com/max/1140/1*MKGcoWB9JM3RRqJa7by2uA.png)


### Why and how to use code

#### Why?
SwaVan cannot mock api without this middleware service as you can see in the flow diagram, this service sit in the middle between the SwaVan and Mocky.

#### How ?
Run the service using docker

Command to build docker image
```
 DOCKER_BUILDKIT=1 docker build --tag swavan-api:0.0.19 .
```

Command to run docker image
```
docker run --rm -p 8080:8080 swavan-api:1.0
```
