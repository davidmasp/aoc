

def read_input(fn):
    with open(fn) as f:
        return f.read().strip().split("\n")

def calculate_increments(input_arr, mode = "positive"):
    int_arr = [int(x) for x in input_arr]
    count = 0
    increment_arr = [0]
    for i in range(len(int_arr)):
        if i == 0:
            pass
        else:
            increment = int_arr[i] - int_arr[i-1]
            #print(increment)
            increment_arr.append(increment)
            if mode == "positive" and increment > 0:
                count += 1
            elif mode == "incremental" and increment > increment_arr[i-1]:
                count += 1
    return count

def calculate_sliding_win(input_arr, win_len = 3):
    total_win = len(input_arr) - win_len + 1
    input_arr_int = [int(i) for i in input_arr]
    min_i = 0
    max_i = min_i + win_len
    tvals = []
    for i in range(total_win):
        total_value = sum(input_arr_int[min_i:max_i])
        tvals.append(total_value)
        min_i = min_i + 1
        max_i = min_i + win_len
    return tvals

if __name__ == "__main__":
    # first part
    input_arr = read_input("input.txt")
    incr = calculate_increments(input_arr)
    print("Solution 1) {}".format(incr))
    # second part
    input_arr2 = read_input("input2.txt")
    wins = calculate_sliding_win(input_arr2)
    incr2 = calculate_increments(wins, mode = "incremental")
    print("Solution 2) {}".format(incr2))
