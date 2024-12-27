const std = @import("std");
const testing = std.testing;
pub const Button  = struct {
    x: i64,
    y: i64,
};

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

pub fn parseButton(line: []const u8, ch: u8) !Button {
    var x: i64 = 0;
    var y: i64 = 0;
    const values = line[(std.mem.indexOfScalar(u8, line, ':') orelse 0) + 1 ..];
    var it2 = std.mem.split(u8, values, ",");
    if (it2.next()) |xStr| {
        const cleanVal = xStr[(std.mem.indexOfScalar(u8, xStr, ch) orelse 0) + 1 ..];
        x = try std.fmt.parseInt(i32, cleanVal, 10);
    }
    if (it2.next()) |yStr| {
        const cleanVal = yStr[(std.mem.indexOfScalar(u8, yStr, ch) orelse 0) + 1 ..];
        y = try std.fmt.parseInt(i32, cleanVal, 10);
    }
    return Button{.x = x, .y = y};
}


pub fn minimizeTokens(aButton: Button, bButton: Button, prize: Button) !i64 {
    // in this part as the pricess are to big an different too small is assured that they form a
    // linearly independent equations system
    var matrix = [2][3]i64 {
        [_]i64{aButton.x, bButton.x, prize.x},
        [_]i64{aButton.y, bButton.y, prize.y}
    };
    var j: u8 = 0;
    const firstMul =  matrix[0][0];
    const secondMul = matrix[1][0];
    while (j < 3): (j+=1) {
        matrix[1][j] = matrix[1][j] * firstMul - matrix[0][j] * secondMul;
    }
    if (matrix[1][1] < 0 or matrix[1][2] < 0) {
        j = 0;
        while (j < 3): (j+=1) {
            matrix[1][j] = matrix[1][j] * -1;
        }
    }
    if (matrix[1][1] <= 0 or @rem(matrix[1][2], matrix[1][1]) != 0) {
        return 0;
    }
    matrix[1][2] = @divExact(matrix[1][2], matrix[1][1]);
    matrix[1][1] = 1;
    j = 0;
    const thirdMul = matrix[0][1];
    while (j < 3): (j+=1) {
        matrix[0][j] = matrix[0][j] - matrix[1][j] * thirdMul;

    }
    const asd = @rem(matrix[0][2], matrix[0][0]);
    if (matrix[0][0] == 0 or asd != 0 or std.math.sign(matrix[0][0]) != std.math.sign(matrix[0][2])) {
        return 0;
    }
    if (matrix[0][2] < 0) {
        matrix[0][0] *= -1;
        matrix[0][2] *= -1;
    }

    matrix[0][2] = @divExact(matrix[0][2], matrix[0][0]);
    matrix[0][0] = 1;
    const aTokens: i32 = 3;
    const bTokens: i32 = 1;

    return matrix[0][2]*aTokens+matrix[1][2]*bTokens;
}


