import random

## the structure is 
## [row,row,row]
## and row is another row = []
import pdb

class BingoTable(object):
    def __init__(self, rows = None, columns = None, preset = None):
        if preset is not None:
            self.table = preset
            n_row = len(preset)
            n_col = [len(row) for row in preset]
            self.n_row = n_row
            self.n_col = n_col[0] ## should be unique and check only one!
            self.tableBool = []
            #pdb.set_trace()
            for row in range(n_row):
                tmp_row = [False for i in range(self.n_col)]
                self.tableBool.append(tmp_row)
        else:
            raise ValueError("No preset table provided")
    def checkNumber(self, number):
        for ir in  range(self.n_row):
            for ic in range(self.n_col):
                if self.table[ir][ic] == number:
                    self.tableBool[ir][ic] = True
    def checkWinner(self):
        for row in self.tableBool:
            if all(row):
                return True
        # transpose
        cols = [[0,0,0,0,0] for fsfds in range(self.n_col)]
        for i in range(len(self.tableBool)):
            row = self.tableBool[i]
            for j in range(len(row)):
                cols[j][i] = row[j]
        for col in cols:
            if all(col):
                return True
        return False
    def calculateScore(self, lastNumber):
        score = 0
        for rowi in range(self.n_row):
            for coli in range(self.n_col):
                val_bool = self.tableBool[rowi][coli]
                val_num = self.table[rowi][coli]
                if val_bool:
                    continue
                else:
                    score = score + val_num
        score_res = score * lastNumber
        return score_res

def read_input(filename):
    with open(filename, "r") as f:
        lines = f.readlines()
    numbers_raw = lines[0].strip().split(",")
    numbers = [int(n) for n in numbers_raw]
    tables_raw = []
    curr_table = []
    for i in lines[1:]:
        if i.strip() == "":
            continue
        else:
            line_raw = i.strip().split(" ")
            #pdb.set_trace()
            try:
                line = [int(x) for x in line_raw if x != ""]
            except print(0):
                pdb.set_trace()
            curr_table.append(line)
            if len(curr_table) == 5:
                table_obj = BingoTable(preset=curr_table)
                tables_raw.append(table_obj)
                curr_table = []
    return numbers, tables_raw

if __name__ == "__main__":
    ## read input
    numbers, tables = read_input("input.txt")
    ## part 1
    for n in numbers:
        for table in tables:
            table.checkNumber(n)
            if table.checkWinner():
                p1 = table.calculateScore(n)
                break
        else:
            continue  # only executed if the inner loop did NOT break
        break
    print("Part 1:", p1)
    ## part 2
    numbers, tables = read_input("input.txt")
    winners = []
    scores = []
    for table in tables:
        for i in range(len(numbers)):
            n = numbers[i]
            table.checkNumber(n)
            if table.checkWinner():
                winners.append(n)
                scores.append(table.calculateScore(n))
                break
    #pdb.set_trace()
    idx = []
    for w in winners:
        val = numbers.index(w)
        idx.append(val)
    min_value = max(idx)
    min_index = idx.index(min_value)
    print("Part 2:", scores[min_index])
        
