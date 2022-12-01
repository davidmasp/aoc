


def read_input(fn):
    with open(fn) as f:
        lines =  f.read().strip().split("\n")
        all_lines = [ [j for j in i] for i in lines]
    return all_lines


def binary_to_decimal(binary):
    return int(binary, 2)


dat = read_input("input.txt")

common_in = []
uncommon_in = []

len_element = len(dat[0])

for j in range(len_element):
    arr = [line[j] for line in dat]
    common_in.append(max(set(arr), key=arr.count))
    uncommon_in.append(min(set(arr), key=arr.count))

a = binary_to_decimal("".join(common_in))
b = binary_to_decimal("".join(uncommon_in))
print("Solution to part 1: {}".format(a*b))


## part 2


def count_vals(dat, criteria = "common"):
    counts_one = dat.count('1')
    counts_zero = dat.count('0')
    if (criteria == "common"):
        if counts_one >= counts_zero:
            return '1'
        else: 
            return '0'
    else:
        if counts_one < counts_zero:
            return '1'
        else: 
            return '0'

most_common_criteria = []
dat_tmp = dat.copy()
for j in range(len_element):
    arr = [line[j] for line in dat_tmp]
    common_val = count_vals(arr)
    dat_tmp = [i for i in dat_tmp if i[j] == common_val]
    #most_common_criteria.append(common_val)
    if (len(dat_tmp) == 1):
        most_common_criteria = dat_tmp[0]
        break

c = binary_to_decimal("".join(most_common_criteria))


least_common_criteria = []
dat_tmp = dat.copy()
for j in range(len_element):
    arr = [line[j] for line in dat_tmp]
    arr.sort()
    common_val = count_vals(arr, "sjdfhskj")
    print(common_val)
    dat_tmp = [i for i in dat_tmp if i[j] == common_val]
    #least_common_criteria.append(common_val)
    if (len(dat_tmp) == 1):
        least_common_criteria = dat_tmp[0]
        break

d = binary_to_decimal("".join(least_common_criteria))
print("Solution to part 1: {}".format(c*d))

