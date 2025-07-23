#!/usr/bin/env bash
# This is a test script for the Luna Project.

# for DIR in [01]*; do
#     DIRNAME=$(basename "$DIR")
#     echo "==> $DIRNAME <=="
#     (cd $DIR && cargo test -q > /dev/null && cargo clippy)
# done

# echo "Done."

#!/usr/bin/env bash

# 定义颜色
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # 无颜色（重置）

for DIR in [01]*; do
    DIRNAME=$(basename "$DIR")
    printf "${YELLOW}==> %s <==${NC}\n" "$DIRNAME"

    (
      cd "$DIR" || exit
      if cargo test > /dev/null; then
        printf "${GREEN}Tests passed in %s${NC}\n" "$DIRNAME"
        cargo clippy -- -D warnings
      else
        printf "${RED}Tests failed in %s, skipping clippy${NC}\n" "$DIRNAME"
      fi
    )
done

echo -e "${GREEN}Done.${NC}"
