echo "512000"
dd bs=1024 count=512000 </dev/urandom > ./random_test_data/512000_random_bytess
echo "1024000"
dd bs=1024 count=1024000 </dev/urandom > ./random_test_data/1024000_random_bytes
echo "2048000"
dd bs=1024 count=2048000 </dev/urandom > ./random_test_data/2048000_random_bytes
echo "4096000"
dd bs=1024 count=4096000 </dev/urandom > ./random_test_data/4096000_random_bytes