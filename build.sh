#!/usr/bin/env sh
set -x

if [ -n "${PRIVATE_REPO}" ]; then
  _PRIVATE_REPO=${PRIVATE_REPO}/
fi

docker run --rm \
    --interactive \
    --workdir /workdir \
    --volume $(pwd):/workdir \
    --env HTTP_PROXY=${_HTTP_PROXY:=} \
    --env USERID=$(id -u) \
    --env GROUPID=$(id -g) \
    --entrypoint /usr/local/cargo/bin/cargo-deb \
  ${_PRIVATE_REPO}build/${DISTRO_NAME}:${DISTRO_RELEASE}
