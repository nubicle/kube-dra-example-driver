image       := "ghcr.io/nubicle/example-driver"
tag         := "dev"
cluster     := "kube-dra"
namespace   := "example-driver"
k8s_version := "1.34"
node_image  := "kindest/node:v" + k8s_version + ".0"

# List the available recipes (runs when `just` is invoked with no target).
_default:
    @just --list

# Build the container image for the host's platform.
build:
    #!/usr/bin/env bash
    set -euo pipefail
    case "$(uname -m)" in
        arm64 | aarch64) platform="linux/arm64" ;;
        x86_64 | amd64)  platform="linux/amd64" ;;
        *) echo "unsupported architecture: $(uname -m)" >&2; exit 1 ;;
    esac
    echo "building {{image}}:{{tag}} for ${platform}"
    docker build --platform "${platform}" --provenance=false -t {{image}}:{{tag}} .

# Create the kind cluster if it does not already exist.
cluster-up:
    #!/usr/bin/env bash
    set -euo pipefail
    if kind get clusters 2>/dev/null | grep -qx "{{cluster}}"; then
        echo "kind cluster '{{cluster}}' already exists"
    else
        echo "creating kind cluster '{{cluster}}' ({{node_image}})"
        kind create cluster --name "{{cluster}}" --image "{{node_image}}"
    fi

# Load the host-built image into the kind cluster's nodes.
_load:
    kind load docker-image {{image}}:{{tag}} --name {{cluster}}

# Build, ensure the kind cluster exists, load the image, and install the Helm chart.
deploy: build cluster-up _load
    #!/usr/bin/env bash
    set -euo pipefail
    helm upgrade --install example-driver charts/example-driver \
        --kube-context "kind-{{cluster}}" \
        --namespace "{{namespace}}" --create-namespace \
        --set image.tag={{tag}} \
        --wait --timeout 180s
