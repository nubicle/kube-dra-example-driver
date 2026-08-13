# kube-dra-example-driver

An example [Dynamic Resource Allocation (DRA)][dra] driver built on the
[`kube-dra`](https://github.com/nubicle/kube-dra) Rust library. It runs as a node-local
**kubelet plugin** (deployed as a DaemonSet) and serves as a reference for building DRA
drivers with `kube-dra`.

> **Status: early development.** The driver registers itself as a DRA kubelet plugin with
> kubelet. Device advertisement (`ResourceSlice` publishing) and claim preparation
> (`NodePrepareResources`) are still in progress, so no device is allocatable end-to-end yet.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/)
- [kind](https://kind.sigs.k8s.io/)
- [kubectl](https://kubernetes.io/docs/tasks/tools/)
- [Helm](https://helm.sh/)
- [just](https://github.com/casey/just)

> DRA is **GA in Kubernetes 1.34**, so the default `kindest/node:v1.34.0` image needs no
> feature-gate configuration. Older node images require enabling the
> `DynamicResourceAllocation` feature gate on the API server, scheduler, and kubelet.

## Working cluster setup

Bring up a cluster with the driver deployed in a single command:

```bash
just deploy
```

`deploy` runs the full pipeline:

- builds the container image for your host's platform → `ghcr.io/nubicle/example-driver:dev`.
- creates a kind cluster named `kube-dra` on a Kubernetes 1.34 node image.
- loads the image into the cluster's nodes.
- installs the chart into the `example-driver` namespace and waits for the kubelet-plugin to become ready.

Verify the plugin came up and registered:

```sh
kubectl -n example-driver get pods
kubectl -n example-driver logs -l app.kubernetes.io/instance=example-driver
```

Confirm the DRA API is available:

```sh
kubectl get resourceslices
```

Run `just` (with no target) to list available recipes.

## See also

- [`kube-dra`](https://github.com/nubicle/kube-dra) — the Rust library this driver is built on.
