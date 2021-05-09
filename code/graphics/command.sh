#for i in {0..10} ; do
#    rm ./a.png
#    /bin/time -f "%e, %M, %I, %O" ./target/release/graphics -i ~/ImageProcessing/images/floor.bmp -o ./a.png --white-black Smooth1 |& tee -a times_wb.csv
#done
#
#for i in {0..10} ; do
#    rm ./a.png
#    /bin/time -f "%e, %M, %I, %O" ./target/release/graphics -i ~/ImageProcessing/images/floor.bmp -o ./a.png --white-black Smooth2 |& tee -a times_wb.csv
#done
#
#for i in {0..10} ; do
#    rm ./a.png
#    /bin/time -f "%e, %M, %I, %O" ./target/release/graphics -i ~/ImageProcessing/images/floor.bmp -o ./a.png --white-black Flat |& tee -a times_wb.csv
#done

# Format: Time, Mem, Read, Write

what=floor

executable=./target/release/graphics
input_image=~/ImageProcessing/images/$what.bmp
output_image=./a.png

csv_file="time_noise_filter_$what.csv"

function time_command {
    /bin/time -f "%e, %M, %I, %O" $executable --input $input_image --output $output_image $1
}

for i in {0..100}; do
  time_command "--noise-filter $i" |& tee -a $csv_file
done