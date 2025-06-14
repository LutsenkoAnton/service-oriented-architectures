#!/bin/sh


KAFKA_ADVERTISED_LISTENERS="HOST://localhost:${PLEASE_PORT},DOCKER://$(hostname -i):9093"
export KAFKA_ADVERTISED_LISTENERS

/etc/kafka/docker/run
