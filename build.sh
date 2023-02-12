cd popup && wasm-pack build --target web 
cd ..
echo "Copying popup to Extension Directory" 
cp -r popup/pkg ext/

cd background && wasm-pack build --target web 
cd ..
echo "Copying background scripts to Extension Directory" 
cp -r background/pkg ext/

echo "Done!"


