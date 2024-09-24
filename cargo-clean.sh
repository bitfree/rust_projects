#!/bin/bash

# 기본 경로 설정 (현재 디렉토리)
BASE_DIR="."

# 하위 디렉토리를 찾고, 각각의 디렉토리에서 명령어를 수행
find "$BASE_DIR" -maxdepth 1 -type d ! -path . | while read -r DIR; do
  echo "Processing directory: $DIR"
  # 여기서 수행하고자 하는 명령어를 입력하세요.
  # 예시로 해당 디렉토리의 파일 목록을 출력
  cd $DIR
  cargo clean
#  ls "$DIR"
  cd ..
done
