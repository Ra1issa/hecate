import os

file_sizes = [10, 100, 1000, 10000, 100000]

# os.chdir(os.pardir)

text_file = open("../data/msgs/lorem.txt", "r")
lorem = text_file.read()
text_file.close()

for i in range(len(file_sizes)):
    file_name = "message"+str(i)+".txt";
    path = "../data/msgs/"+file_name;
    f = open(path, "w+")
    f.write(lorem[0:file_sizes[i]])
    f.close()
