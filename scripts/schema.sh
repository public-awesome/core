cd contracts/fair-burn
cargo schema
rm -rf schema/raw
cd ../..

for d in contracts/vip/*; do
  if [ -d "$d" ]; then
    cd $d
    cargo schema
    rm -rf schema/raw
    cd ../../..
  fi
done

