const std = @import("std");
const testing = std.testing;


pub fn divideLines(input: []const u8) !std.ArrayList([]const u8) {
    const allocator = std.heap.page_allocator;
    var lineList = std.ArrayList([]const u8).init(allocator);
    var lines = std.mem.splitSequence(u8, input, "\n");
    while (lines.next()) |line| {
        try lineList.append(std.mem.trim(u8, line, "\r\n"));
    }
    return lineList;
}


pub fn readFile(path: []const u8) ![] const u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return try file.readToEndAlloc(std.heap.page_allocator, @as(usize, std.math.maxInt(usize)));
}

pub fn sumLine(line: []const u8) u32 {
    var total : u32 = 0;
    for (line, 0..) |value, i| {
        if (value == 'm') {
            total += sumMul(line, i);
        }
    }
    return total;
}

pub fn sumLine2(line: []const u8, startEnabled: bool) struct {value: u32,  enabled: bool} {
    var enabled = startEnabled;
    var total : u32 = 0;
    for (line, 0..) |value, i| {
        if (enabled and value == 'm') {
            total += sumMul(line, i);
        }
        if (value == 'd' and i+4 <= line.len){
            const doinst = line[i..i+4];
            if (std.mem.eql(u8, doinst, "do()")){
                enabled = true;
            }
        }
        if (value == 'd' and i+7 <= line.len){
            const dontinst = line[i..i+7];
            if(std.mem.eql(u8, dontinst, "don't()")){
                enabled = false;
            }
        }
    }
    return .{.value = total, .enabled = enabled};
}

fn sumMul(line: [] const u8, index: usize) u32 {
    if (line.len < index+4){
        return 0;
    }
    const prefix = line[index..index+4];
    if (!std.mem.eql(u8, prefix, "mul(")){
        return 0;
    }
    const sum1 = parseNum(line, index+4);
    if (sum1.endpos >= line.len or sum1.endpos<=index+4 or sum1.endpos > index+7 or line[sum1.endpos] != ','){
        return 0;
    }
    const sum2 = parseNum(line, sum1.endpos+1);
    if(sum2.endpos >= line.len or line[sum2.endpos] != ')') {
        return 0;
    }
    return sum1.value * sum2.value;

}

fn parseNum(str: []const u8, index: usize) struct {value: u32,  endpos: usize} {
    var i = index;
    var sum: u32  = 0;
    while (i < str.len) {
        const currChar = str[i..i+1];
        const digit = std.fmt.parseInt(u32, currChar, 10) catch return.{.value=sum, .endpos=i};
        sum = sum * 10 + digit;
        i+=1;
    }
    return .{.value= sum, .endpos=i};

}

pub fn findOperations(total: u64, operandsList: []u64, operand: u8, currAmount: u64) bool {
    if(operandsList.len == 1) {
        switch (operand) {
            '+' => {return total == currAmount + operandsList[0];},
            '*' => {return total == currAmount * operandsList[0];},
            '|' => {
                const exp = std.math.log(u64, 10, operandsList[0]) + 1;
                return total == (currAmount * (std.math.powi(u64, 10, exp) catch unreachable)) + operandsList[0];
            },
            else => {return false;}
        }
    }else {
        var newAmmount = currAmount;
        switch (operand) {
        '+' => {newAmmount += operandsList[0];},
        '*' => {newAmmount *= operandsList[0];},
        '|' => {
                const exp = std.math.log(u64, 10, operandsList[0]) + 1;
                newAmmount = (currAmount * (std.math.powi(u64, 10, exp) catch unreachable)) + operandsList[0];
            },
        else => {return false;}
        }
        if (newAmmount > total) {
            return false;
        }
        return findOperations(total, operandsList[1..], '+', newAmmount) or findOperations(total, operandsList[1..], '*', newAmmount) or findOperations(total, operandsList[1..], '|', newAmmount);
    }

}


test "test case 2" {

    const input =
        \\190: 10 19
        \\3267: 81 40 27
        \\83: 17 5
        \\156: 15 6
        \\7290: 6 8 6 15
        \\161011: 16 10 13
        \\192: 17 8 14
        \\21037: 9 7 18 13
        \\292: 11 6 16 20
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const allocator = std.heap.page_allocator;
    for (result.items) |line| {
        const cleanline = std.mem.trim(u8, line, "\r\n");
        var parts = std.mem.splitSequence(u8, cleanline, ":");
        const totalStr = parts.next().?;
        const objective = try std.fmt.parseInt(u64, totalStr, 10);
        const operands = parts.next().?;
        parts = std.mem.splitSequence(u8, operands, " ");
        var operandsList = std.ArrayList(u64).init(allocator);
        defer operandsList.deinit();
        while (parts.next()) |operandStr| {
            if(operandStr.len > 0){
                const operand = try std.fmt.parseInt(u64, operandStr, 10);
                try operandsList.append(operand);
            }
        }
        if(findOperations(objective, operandsList.items, '+', 0)){
            total += objective;
        }
    }
    try testing.expectEqual(11387, total);

}
