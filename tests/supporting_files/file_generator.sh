echo "----- CREATING test_file.txt -----";
filename1="test_file.txt"
tfc="If you are seeing this message, then you are reading this properly.
Some more text...
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus lobortis sapien velit, sit amet bibendum enim maximus eget. Fusce ultricies malesuada libero, sit amet tristique quam fringilla eget. Ut ullamcorper mauris lectus, vel gravida ligula vehicula non. Etiam rhoncus finibus lectus, at cursus odio bibendum non. Pellentesque ex enim, commodo vel erat sed, ultricies aliquam sem. Fusce in purus justo. Nunc odio purus, ullamcorper sed dui ut, fringilla commodo arcu. In consectetur imperdiet lacus, non pharetra nulla laoreet facilisis. Morbi auctor lacus erat, eu volutpat enim iaculis eget. Sed vitae neque in ex tempus cursus. Suspendisse elementum, justo id porttitor efficitur, odio tellus gravida felis, a euismod nibh nulla sed quam. Ut in turpis sed arcu feugiat suscipit a vel elit. Nullam vel lacus non massa ullamcorper pharetra id eget turpis. Vestibulum nec sodales dui. Ut neque diam, convallis at efficitur ut, euismod vitae nulla. Aenean scelerisque at magna eget tempor.

Maecenas quam nisl, interdum id arcu quis, consectetur elementum lacus. Integer sed vulputate purus, vel blandit ligula. Morbi felis neque, viverra eget ornare at, ullamcorper vitae diam. Mauris sed tortor efficitur velit euismod aliquet sed at enim. Morbi interdum mauris ut arcu elementum tristique. Praesent semper libero ac ante fringilla finibus. Phasellus ornare, lectus ut consectetur dignissim, ligula odio ultricies arcu, ac faucibus lacus felis vel neque. Nam nunc augue, convallis at augue finibus, placerat aliquam quam. Pellentesque suscipit neque sed convallis ullamcorper. Donec ultrices sollicitudin sagittis. Curabitur pretium pellentesque semper. Maecenas felis lectus, consequat eget pharetra quis, semper in orci. In at facilisis quam, non lacinia elit. Nunc accumsan congue ex, id finibus enim scelerisque eu. Ut in tempor purus, nec euismod massa."

echo "$tfc" > "$filename1"


echo "----- CREATING test_invalid_json.json -----";
filename2="test_invalid_json.json"
tij="{ \"a\": \"Sreerag\" }"
echo "$tij" > "$filename2"


echo "----- CREATING test_json.json -----";
filename3="test_json.json"
tj="{ \"x\": 2, \"z\": [1, 2, 3], \"y\": [{ \"a\": 123, \"b\": \"A name\" }] }"

echo "$tj" > "$filename3"


echo "----- CREATING test_write_file.txt -----";

filename4="test_write_file.txt"
twf="If you are seeing this message, then you are reading this properly.
Some more text...
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus lobortis sapien velit, sit amet bibendum enim maximus eget. Fusce ultricies malesuada libero, sit amet tristique quam fringilla eget. Ut ullamcorper mauris lectus, vel gravida ligula vehicula non. Etiam rhoncus finibus lectus, at cursus odio bibendum non. Pellentesque ex enim, commodo vel erat sed, ultricies aliquam sem. Fusce in purus justo. Nunc odio purus, ullamcorper sed dui ut, fringilla commodo arcu. In consectetur imperdiet lacus, non pharetra nulla laoreet facilisis. Morbi auctor lacus erat, eu volutpat enim iaculis eget. Sed vitae neque in ex tempus cursus. Suspendisse elementum, justo id porttitor efficitur, odio tellus gravida felis, a euismod nibh nulla sed quam. Ut in turpis sed arcu feugiat suscipit a vel elit. Nullam vel lacus non massa ullamcorper pharetra id eget turpis. Vestibulum nec sodales dui. Ut neque diam, convallis at efficitur ut, euismod vitae nulla. Aenean scelerisque at magna eget tempor.

Maecenas quam nisl, interdum id arcu quis, consectetur elementum lacus. Integer sed vulputate purus, vel blandit ligula. Morbi felis neque, viverra eget ornare at, ullamcorper vitae diam. Mauris sed tortor efficitur velit euismod aliquet sed at enim. Morbi interdum mauris ut arcu elementum tristique. Praesent semper libero ac ante fringilla finibus. Phasellus ornare, lectus ut consectetur dignissim, ligula odio ultricies arcu, ac faucibus lacus felis vel neque. Nam nunc augue, convallis at augue finibus, placerat aliquam quam. Pellentesque suscipit neque sed convallis ullamcorper. Donec ultrices sollicitudin sagittis. Curabitur pretium pellentesque semper. Maecenas felis lectus, consequat eget pharetra quis, semper in orci. In at facilisis quam, non lacinia elit. Nunc accumsan congue ex, id finibus enim scelerisque eu. Ut in tempor purus, nec euismod massa."

echo "$twf" > "$filename4"
