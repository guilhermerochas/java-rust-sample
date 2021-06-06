#!/usr/bin/env bash

mvn package
ls $(pwd)/target/*.jar | java -jar $(tee)