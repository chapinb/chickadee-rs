#!/bin/bash

curl -X POST \
    $1/api/v1/bom \
    -H "X-Api-Key: $2" \
    -H "Content-Type: multipart/form-data" \
    -F bom=@bom.xml \
    -F "project=$3"
