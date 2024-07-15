![sahome](/assets/banner.png)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](/docs/code_of_conduct.md)

## Getting started

### With Docker

The easiest way to get started is to use Docker. You can run the following command to start the server:

```bash
docker pull ghcr.io/sahomey-technologies/sahomedb:latest
```

This will pull the latest version of the server from the GitHub Container Registry. You can then run the server with the following command:

```bash
docker run \
    --platform linux/amd64 \
    --publish 3141:3141 \
    --env SAHOMEDB_DIMENSION=xxx \
    ghcr.io/sahomey-technologies/sahomedb:latest
```

Replace `xxx` with the dimension of your desired embedding. This will start the server on port `3141`.

### Testing the server

You can test the server by calling `GET /` using your favorite HTTP client. For example, you can use `curl`:

```bash
curl http://localhost:3141
```

You can replace `localhost` with the IP address of the server if you are running the server on a remote machine.


### `POST /values`

Create or update the value of a key. See below for the expected format of the request body.

```json
{
  "key": "string",
  "value": {
    "embedding": [0.0, 0.0, ...],
    "data": {}
  }
}
```

The `embedding` field is a list of floating-point numbers with the dimension specified by the `SAHOMEDB_DIMENSION` environment variable.

The `data` field is an object that can be used to store additional information about the key-value pair. Currently, this only support string keys and values.

### `POST /build`

Build the HNSW index. This operation is required before you can search the index. We use HNSW as the underlying algorithm for the embedding index and for that, we use [instant-distance](https://github.com/instant-labs/instant-distance) crate.

Optionally, you can specify `ef_construction` and `ef_search` in the request body. These are the parameters for the HNSW algorithm. By default, we use `100` for both parameters.

```json
{
  "ef_construction": 100,
  "ef_search": 100
}
```

### `POST /search`

Search the index given an embedding. See below for the expected format of the request body.

```json
{
  "embedding": [0.0, 0.0, ...],
  "count": 10
}
```

The dimension of `embedding` must match the dimension specified by the `SAHOMEDB_DIMENSION` environment variable.

## Disclaimer

This project is still in the early stages of development. We are actively working on it and we expect the API and functionality to change. We do not recommend using this in production yet.

We also don't have a benchmark yet. We are working on it and we will publish the results once we have them.

## Contributing

We welcome contributions from the community. Please see [contributing.md](/docs/contributing.md) for more information.

## Code of Conduct

We are committed to creating a welcoming community. Any participant in our project is expected to act respectfully and to follow the [Code of Conduct](/docs/code_of_conduct.md).