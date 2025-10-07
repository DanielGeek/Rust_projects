# List Docker Images

When we ran the `docker run` command Docker automatically pulled down the Docker image **ubuntu:latest** for us. We can view the images that are on our system with the command `docker images`.

```sh
$ docker images
REPOSITORY                    TAG       IMAGE ID       CREATED       SIZE
ubuntu                        latest    728785b59223   6 days ago    139MB
```

Let's go through the table that Docker displays for us.

## REPOSITORY

Docker stores it's images in repositories similar to how Github stores code projects in repositories. Each image get's it's own repo, which makes it easy to know which image we're creating a container from. When we ran the command `docker run -it ubuntu` the `ubuntu` was the name of the repository.
