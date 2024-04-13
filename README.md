# Docker Stats Exporter

Docker Stats Exporter is a tool designed to monitor the performance of your Docker containers. It exports Docker container statistics to an endpoint that Prometheus can easily consume, simplifying the process of container performance monitoring and leveraging the powerful capabilities of Prometheus with minimal setup.

## Getting Started

To get started with Docker Stats Exporter, you need to have Docker installed on your machine. If you don't have Docker installed, you can download it from the [official Docker website](https://www.docker.com/get-started).

Once you have Docker installed, you can run the Docker Stats Exporter using Docker Compose. Here is a sample `docker-compose.yml` file that you can use:

```yaml
version: "3.8"
services:
  docker-stats-exporter:
      image: lekesoldat/docker-stats-exporter:latest
      container_name: docker-stats-exporter
      volumes:
        - /var/run/docker.sock:/var/run/docker.sock
      ports:
        - 3069:3069
```

The volume `- /var/run/docker.sock:/var/run/docker.sock` is necessary because Docker Stats Exporter needs to communicate with the Docker daemon to retrieve container statistics. The Docker daemon listens on a Unix socket located at `/var/run/docker.sock` by default. By mapping this socket to the same location in the Docker Stats Exporter container, the tool can communicate with the Docker daemon as if it were running directly on the host machine.

To run the Docker Stats Exporter, navigate to the directory containing your `docker-compose.yml` file and run the following command:

```bash
docker-compose up
```

This will start the Docker Stats Exporter, and it will begin exporting Docker stats to the `/docker-stats/metrics` endpoint.

## Usage

Once the Docker Stats Exporter is running, you can access the stats at `http://localhost:3069/docker-stats/metrics`. This endpoint will provide a detailed breakdown of the performance of each of your Docker containers, including CPU usage, memory usage, and network I/O.

You can then configure Prometheus to scrape this endpoint to collect and store the Docker stats. For more information on how to do this, refer to the [Prometheus documentation](https://prometheus.io/docs/prometheus/latest/configuration/configuration/).

## Contributing

Contributions to Docker Stats Exporter are welcome! Please feel free to open an issue or submit a pull request if you have any improvements or features you'd like to add.

## License

Docker Stats Exporter is open source and is licensed under the MIT License.