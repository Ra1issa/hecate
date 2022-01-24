import os
from pathlib import Path
from shutil import copyfile

file_sizes = [10, 100, 250, 500, 750, 1000, 2500, 5000, 7500, 10000, 100000]

# using path.expanduser() getting username
dir = os.path.expanduser('~') + '/Documents/hecate/data'

Path(dir+"/msgs/").mkdir(parents=True, exist_ok=True)
copyfile("../data/msgs/lorem.txt", dir+"/msgs/lorem.txt")

text_file = open(dir+"/msgs/lorem.txt", "r")
lorem = text_file.read()
text_file.close()

for i in range(len(file_sizes)):
    file_name = "msg"+str(i)+".txt";
    path = dir+"/msgs/"+file_name;
    f = open(path, "w+")
    f.write(lorem[0:file_sizes[i]])
    f.close()
