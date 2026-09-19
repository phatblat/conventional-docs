# Build doc for linux and windows

## Problem

`doc` builds for macOS arm64 only. Any contributor or CI runner on linux
amd64/arm64 or windows cannot install or run it.

## Outcome

`doc` builds and passes its test suite on linux amd64, linux arm64, and
windows, in addition to macOS arm64.
