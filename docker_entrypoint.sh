#!/bin/sh

set -euo pipefail

configurator
exec tini ddclient -- -foreground
