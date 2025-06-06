
LOCAL_DFX_DIR=".dfx/local/canisters/backend/"
IC_DFX_DIR=".dfx/ic/canisters/backend/"

for OLD in $(ls ${LOCAL_DFX_DIR} | grep service)
do
  NEW="${OLD//service/backend}"
  OLD_FILE=${LOCAL_DFX_DIR}$OLD
  NEW_FILE=${LOCAL_DFX_DIR}$NEW
  echo "old file:"$OLD_FILE
  echo "new:"$NEW_FILE
  mv $OLD_FILE $NEW_FILE
done


for OLD in $(ls ${IC_DFX_DIR} | grep service)
do
  NEW="${OLD//service/backend}"
  OLD_FILE=${IC_DFX_DIR}$OLD
  NEW_FILE=${IC_DFX_DIR}$NEW
  echo "old:"$OLD_FILE
  echo "new:"$NEW_FILE
  mv $OLD_FILE $NEW_FILE
done
