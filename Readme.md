# Just for fun cli tool for searching pod by ip.  

### Build deb.

Set k8s api version.
Cargo.toml set features for k8s-openapi (`features = ["v1_22"]`)
```toml
k8s-openapi = { version = "0.15.0", default-features = false, features = ["v1_22"] }
```
Set env environment.

```bash
export DISTRO_RELEASE="rust-deb"
export DISTRO_NAME="1.59.0"
export PRIVATE_REPO="https://hub.docker.com"
```
Run `build.sh`

### Usage

```bash
k8s-search-pod -a 10.250.1.1
+----------------+-----------------------------------------+----------------+-------------------------------+
| Request IP     | Pod name                                | Pod namespace  | Pod on node                   |
+----------------+-----------------------------------------+----------------+-------------------------------+
| 10.250.1.1     | test                                    | test-ns        | some-node-in-cluster          |
+----------------+-----------------------------------------+----------------+-------------------------------+
```
