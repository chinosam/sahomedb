![Oasys](/assets/banner.png)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=for-the-badge)](https://opensource.org/licenses/Apache-2.0)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=for-the-badge)](/docs/code_of_conduct.md)
[![Discord](https://img.shields.io/discord/1182432298382131200?logo=discord&logoColor=%23ffffff&label=Discord&style=for-the-badge)](https://discord.gg/bDhQrkqNdsP4)

## What is SahomeDB?

SahomeDB is a vector database that can be used to store and query high-dimensional vectors. Our goal is to make SahomeDB fast and easy to use. We are also working on making it easy to deploy and scale.

### Features

- **HTTP-based API**: All operations are exposed via a RESTful API. This makes it easy to integrate with other systems without having to install any client libraries.

- **Persistent storage**: Embeddings and graphs data are stored on disk and are persisted across restarts.

- **HNSW indexing**: SahomeDB uses the HNSW algorithm to build graphs to index embeddings. This allows for fast and accurate nearest neighbor search.

- **Multi-graph support**: SahomeDB supports multiple HNSW graphs. This allows you to version and customize your graphs to suit different use cases. For example, optimizing speed for a specific query type or optimizing accuracy for a specific dataset.

## Getting Started

### Installation

The easiest way to get started is to use Docker. You can pull the latest image from GitHub Container Registry:

```bash
docker pull ghcr.io/sahome/sahomedb:latest
```

This will pull the latest version of the server from the GitHub Container Registry. You can then run the server with the following command:

```bash
docker run \
    --publish 3141:3141 \
    --env SAHOMEDB_DIMENSION=512 \
    --env SAHOMEDB_TOKEN=token \
    ghcr.io/sahome/sahomedb:latest
```

- `SAHOMEDB_DIMENSION`: An integer representing the dimension of your embedding. Different embedding model will have different dimension. For example, OpenAI Ada 2 has a dimension of 1536.

- `SAHOMEDB_TOKEN`: A string that you will use to authenticate with the server. You need to add `x-sahomedb-token` header to your request with the value of this environment variable.

This will start SahomeDB that is accessible on port `3141`. You can change this by changing the port number in the `--publish` flag and setting the `SAHOMEDB_PORT` environment variable to the port number that you want to use.

### Testing the server

You can test the server by calling `GET /` using your favorite HTTP client. For example, you can use `curl`:

```bash
curl http://localhost:3141
```

You can replace `localhost` with the IP address of the server if you are running the server on a remote machine.

## Quickstart

To put it simply, these are the primary steps to get started with SahomeDB:

1. Set a value.
2. Create a graph.
3. Query the graph.

### Set a value

```
POST /values/<key>
```

```json
{
  "embedding": [0.1, 0.2, 0.3],
  "data": {
    "type": "fact",
    "text": "SahomeDB is awesome!"
  }
}
```

This endpoint sets a value for a given key. The value is an embedding and an optional data object. The embedding is a list of floating-point numbers. The data object is a JSON object of string keys and values.

### Create a graph

```
POST /graphs
```

Optional request body:

```json
{
  "name": "my-graph",
  "ef_construction": 10,
  "ef_search": 10,
  "filter": {
    "type": "fact"
  }
}
```

This endpoint creates a graph. The graph is used to query for nearest neighbors. If there is no data provided, the server will create a default graph with the name `default` and the default `ef_construction` and `ef_search` values of 100 for both.

The filter object is used to filter the values that are added to the graph. For example, if you only want to add values with the `type` data key set to `fact`, you can use the filter object above. The filter operation is similar to the `AND` operation. This means if you have multiple filters, the server will only add values that match all of filters.

### Query the graph

```
POST /graphs/<name>/query
```

```json
{
  "embedding": [1.0, 2.0, 3.0],
  "k": 10
}
```

This endpoint queries the graph for the nearest neighbors of the given embedding. The `k` parameter is the number of nearest neighbors to return.

### Note

- All embedding dimensions must match the dimension configured in the server using the `SAHOMEDB_DIMENSION` environment variable.
- Requests to `/graphs` and `/values` endpoints must include the `x-sahomedb-token` header with the value of the `SAHOMEDB_TOKEN` environment variable.
- SahomeDB doesn't support automatic graph building due to its versioning and filtering nature. This means whenever you add or remove a value, you need to rebuild the graph.

### More resources

For more information, please see the [SahomeDB documentation](https://www.sahome.com/docs).

## Disclaimer

This project is still in the early stages of development. We are actively working on it and we expect the API and functionality to change. We do not recommend using this in production yet.

We also don't have a benchmark yet. We are working on it and we will publish the results once we have them.

## Contributing

We welcome contributions from the community. Please see [contributing.md](/docs/contributing.md) for more information.

## Code of Conduct

We are committed to creating a welcoming community. Any participant in our project is expected to act respectfully and to follow the [Code of Conduct](/docs/code_of_conduct.md).