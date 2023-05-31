# for d in contracts/*; do
#   if [ -d "$d" ]; then
#     cd $d
#     cargo schema
#     rm -rf schema/raw
#     cd ../..
#   fi
# done

cd contracts/fair-burn
cargo schema
rm -rf schema/raw
cd ../..