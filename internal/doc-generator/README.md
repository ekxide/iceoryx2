# doc-generator

Tooling for generating user documentation for iceoryx2.

## Setup

1. Install `poetry` (if not already installed):
    ```console
    curl -sSL https://install.python-poetry.org | python3 -
    ```
1. Install dependencies
    ```console
    cd internal/doc-generator/
    make install
    ```

## Development

### Running the Development Server

```console
cd internal/doc-generator/
make serve
```

### Building the Documentation

```console
cd internal/doc-generator/
make html
```
