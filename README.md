# Autonomous network-manager

Project `aunema` - is autonomous manager of social networks

## Usage:

To begin development just run:

```bash
docker-compose up --build
```

## Production build:

To build lightweight production image under just run:

```
docker build -f docker/prod.Dockerfile -t test --build-arg PROJECT=aunema .
```
