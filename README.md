# Runtime sample

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