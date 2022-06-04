# Gravity Substream

## Usage

### Get an API key

First, you need to get an API token to access the substreams service:

1. Get a StreamingFast API key from https://app.dfuse.io/ and store it in the
   `STREAMINGFAST_KEY` environment variable.

2. Run
   ```sh
   export SUBSTREAMS_API_TOKEN=$(curl https://auth.dfuse.io/v1/auth/issue -s --data-binary '{"api_key":"'$STREAMINGFAST_KEY'"}' | jq -r .token)
   ```

### Run a prebuilt release of the substream

```
docker run \
  -e SUBSTREAMS_API_TOKEN=$SUBSTREAMS_API_TOKEN \
  -it ghcr.io/streamingfast/substreams \
  run -e api-dev.streamingfast.io:443 \
  https://github.com/jannis/gravity-substream/releases/download/v0.0.1/gravity-v0.1.0.spkg \
  gravatars
```

### Run your local copy of the substream

```
make build

docker run \
  -e SUBSTREAMS_API_TOKEN=$SUBSTREAMS_API_TOKEN \
  -w "$(pwd)" -v "$(pwd):$(pwd)" \
  -it ghcr.io/streamingfast/substreams \
  run -e api-dev.streamingfast.io:443 \
  substreams.yaml \
  gravatars
```

## Copyright

Licensed under Apache License 2.0.
