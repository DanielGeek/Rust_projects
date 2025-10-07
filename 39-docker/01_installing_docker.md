# Installing Docker

Docker is easy to set up and install on our computers. If you have a Mac or Windows system then fallow the instructions at [Docker getting started](https://docs.docker.com/get-started/) page.

If you have a Linux system you can just install the Docker engine without Docker Desktop directly. Here are the instructions for installing Docker engine on [Ubuntu](https://docs.docker.com/engine/install/ubuntu/). There are also instructions for other Linux repos like Debian and Red Hat.

Once you have Docker installed, you can check to make sure that it is running with the command `docker image`.

If you get an error like `Cannot connect to the Docker daemon at unix:///Users/thepunisher/.docker/run/docker.sock. Is the docker daemon running?` then Docker is not running yet. Make sure that Docker is set up and running before continuing on.

If you get output that looks like

```sh
$ docker images
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
```

then Docker is up and running on your system.
