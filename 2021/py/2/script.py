


def read_input(fn):
    with open(fn) as f:
        lines =  f.read().strip().split("\n")
        directions = []
        for i in lines:
            directions.append(i.split(" "))
    return directions


hpos = 0
vpos = 0 # note that this is depth

dict_rules_h = {
    "forward": 1 ## no backwards?
}

dict_rules_v = {
    "up": -1,
    "down": 1
}

input_commands = read_input("input.txt")

### p1 ###
for i in range(len(input_commands)):
    cmd = input_commands[i][0]
    val = int(input_commands[i][1])
    if cmd in dict_rules_h:
        hpos = hpos + val * dict_rules_h[cmd]
    elif cmd in dict_rules_v:
        vpos = vpos + val * dict_rules_v[cmd]
    else:
        raise ValueError("Unknown command: {}".format(cmd))

part1_sol = hpos * vpos
mssg = "Part 1 solution: {}".format(part1_sol)
print(mssg)

#### p2 #####

aim = 0
hpos = 0
vpos = 0 # note that this is depth

dict_rules_h = {
    "forward": 1 ## no backwards?
}


for i in range(len(input_commands)):
    cmd = input_commands[i][0]
    val = int(input_commands[i][1])
    if cmd in dict_rules_h:
        hpos = hpos + val * dict_rules_h[cmd]
        vpos = vpos + val * aim
    elif cmd in dict_rules_v:
        aim = aim + val * dict_rules_v[cmd]
    else:
        raise ValueError("Unknown command: {}".format(cmd))

part2_sol = hpos * vpos
mssg = "Part 2 solution: {}".format(part2_sol)
print(mssg)

