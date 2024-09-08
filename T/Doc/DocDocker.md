


## Install

## Setup on Mac

[stackoverflow](https://stackoverflow.com/questions/40112083/can-i-use-docker-for-installing-ubuntu-on-a-mac) 
	
docker run -it --name ubuntu ubuntu:latest

To start again after a reboot: ^85e314

```
docker start ubuntu
docker exec -it ubuntu bash
```

If you want save your changes:

```
docker commit ubuntu
docker images
```


# LOG

### 2023-11-17 Docker for SV laptop setup
^2023-11-17


// My extras; mapping folder from outside in
RUN mkdir /Users
VOLUME ["/Users"]
WORKDIR /Users/oblinger

// Build command
docker build -t x86-buntu .
docker run -v /Users:/Users -it x86-buntu



cat > Dockerfile
FROM ubuntu:latest
RUN mkdir /Users
VOLUME ["/Users"]
WORKDIR /Users/oblinger
RUN apt-get update && \
    apt-get install -y wget less git
RUN wget https://repo.anaconda.com/miniconda/Miniconda3-latest-MacOSX-x86_64.sh -O miniconda.sh
RUN bash miniconda.sh -b -p /opt/miniconda
ENV PATH="/opt/miniconda/bin:$PATH"





### 2023-11-16  Creating my own docker image

// In host folder
cat > Dockerfile
FROM ubuntu:latest
RUN mkdir /Users
VOLUME ["/Users"]
WORKDIR /Users/oblinger
RUN apt-get update && \
    apt-get install -y wget less git
RUN wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-aarch64.sh -O miniconda.sh
RUN bash miniconda.sh -b -p /opt/miniconda
ENV PATH="/opt/miniconda/bin:$PATH"
