# Pull a Docker Image

Docker images are stored in repositories, similar to how Github stores code projects in repositories. Each image get's it's own repo, which makes it easy to know which image we're creating a container from. When we ran the command `docker run -it ubuntu` the `ubuntu` was the name of the repository.

Something to note, single name repositories like `ubuntu` are __official__ repositories. As in they are created by Docker or an official organization. For example the `ubuntu` repo is managed by the Ubuntu team at Canonical. If you see a double name repository like `ubuntu:24.04` then the `24.04` is a tag, which is a version of the image.

To pull a docker image we can use the `docker pull` command. For example to pull the `ubuntu` image we can run the command `docker pull ubuntu`.

```sh
docker pull ubuntu
```

This will pull the latest version of the `ubuntu` image. If you want to pull a specific version of the image you can use the `:tag` syntax. For example to pull the `24.04` version of the `ubuntu` image we can run the command `docker pull ubuntu:24.04`.

```sh
docker pull ubuntu:24.04
```

This will pull the `24.04` version of the `ubuntu` image.

Once you have pulled an image you can view it with the `docker images` command.

```sh
docker images
```

This will show you a list of all the images that you have pulled.

```sh
$ docker images
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
ubuntu              latest              728785b59223        6 days ago          139MB
```

This will show you a list of all the images that you have pulled.

```sh
$ docker images
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
ubuntu              latest              728785b59223        6 days ago          139MB
```

This will show you a list of all the images that you have pulled.
