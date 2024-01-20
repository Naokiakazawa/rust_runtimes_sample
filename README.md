# Runtime sample

[![Lint & Test](https://github.com/Naokiakazawa/rust_runtimes_sample/actions/workflows/check.yml/badge.svg)](https://github.com/Naokiakazawa/rust_runtimes_sample/actions/workflows/check.yml) [![Docker CI](https://github.com/Naokiakazawa/rust_runtimes_sample/actions/workflows/docker.yml/badge.svg)](https://github.com/Naokiakazawa/rust_runtimes_sample/actions/workflows/docker.yml)

## For Mac

### Install onnxruntime
`brew install onnxruntime`

### set environment variables
```bash
export ORT_STRATEGY=system
export ORT_LIB_LOCATION=/path/to/onnxruntime
```

### Notice

- If you use Apple silicon, you may fail to build docker containers.