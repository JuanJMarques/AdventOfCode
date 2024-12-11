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

pub fn readFile(path: []const u8) ![]const u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return try file.readToEndAlloc(std.heap.page_allocator, @as(usize, std.math.maxInt(usize)));
}

pub fn sumLine(line: []const u8) u32 {
    var total: u32 = 0;
    for (line, 0..) |value, i| {
        if (value == 'm') {
            total += sumMul(line, i);
        }
    }
    return total;
}

fn sumMul(line: []const u8, index: usize) u32 {
    if (line.len < index + 4) {
        return 0;
    }
    const prefix = line[index .. index + 4];
    if (!std.mem.eql(u8, prefix, "mul(")) {
        return 0;
    }
    const sum1 = parseNum(line, index + 4);
    if (sum1.endpos >= line.len or sum1.endpos <= index + 4 or sum1.endpos > index + 7 or line[sum1.endpos] != ',') {
        return 0;
    }
    const sum2 = parseNum(line, sum1.endpos + 1);
    if (sum2.endpos >= line.len or line[sum2.endpos] != ')') {
        return 0;
    }
    return sum1.value * sum2.value;
}

fn parseNum(str: []const u8, index: usize) struct { value: u32, endpos: usize } {
    var i = index;
    var sum: u32 = 0;
    while (i < str.len) {
        const currChar = str[i .. i + 1];
        const digit = std.fmt.parseInt(u32, currChar, 10) catch return .{ .value = sum, .endpos = i };
        sum = sum * 10 + digit;
        i += 1;
    }
    return .{ .value = sum, .endpos = i };
}

test "test case" {
    const input =
        \\xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u32 = 0;
    for (result.items) |line| {
        total += sumLine(line);
    }
    try testing.expectEqual(161, total);
}