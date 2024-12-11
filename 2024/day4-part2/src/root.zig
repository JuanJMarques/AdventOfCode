const std = @import("std");
const testing = std.testing;

const direction = enum {
    up,
    down,
    left,
    right,
    upleft,
    upright,
    downleft,
    downright,

    pub fn values() *const [8]direction {
        return &[_]direction{ direction.up, direction.down, direction.left, direction.right, direction.upleft, direction.upright, direction.downleft, direction.downright };
    }

    pub fn compute(self: direction, x: usize, y: usize) !struct { x: usize, y: usize } {
        switch (self) {
            direction.up => {
                if (y == 0) {
                    return error.underflow;
                }
                return .{ .x = x, .y = y - 1 };
            },
            direction.down => {
                return .{ .x = x, .y = y + 1 };
            },
            direction.left => {
                if (x == 0) {
                    return error.underflow;
                }
                return .{ .x = x - 1, .y = y };
            },
            direction.right => {
                return .{ .x = x + 1, .y = y };
            },
            direction.upleft => {
                if (x == 0 or y == 0) {
                    return error.underflow;
                }
                return .{ .x = x - 1, .y = y - 1 };
            },
            direction.upright => {
                if (y == 0) {
                    return error.underflow;
                }
                return .{ .x = x + 1, .y = y - 1 };
            },
            direction.downleft => {
                if (x == 0) {
                    return error.underflow;
                }
                return .{ .x = x - 1, .y = y + 1 };
            },
            direction.downright => {
                return .{ .x = x + 1, .y = y + 1 };
            },
        }
        return error.unknown;
    }
};

const XMAS = "XMAS";

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

pub fn searchForX_MAS(matrix: [][]const u8, x: usize, y: usize) u32 {
    if (x + 2 < matrix[y].len and y + 2 < matrix.len) {
        if ((matrix[y][x] == 'M' or matrix[y][x] == 'S') and (matrix[y + 1][x + 1] == 'A') and (matrix[y + 2][x] == 'M' or matrix[y + 2][x] == 'S') and (matrix[y][x + 2] == 'M' or matrix[y][x + 2] == 'S') and (matrix[y + 2][x + 2] == 'M' or matrix[y + 2][x + 2] == 'S') and matrix[y][x] != matrix[y + 2][x + 2] and matrix[y][x + 2] != matrix[y + 2][x]) {
            return 1;
        }
    }
    return 0;
}

test "test case" {
    const input =
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u32 = 0;
    for (result.items, 0..) |_, i| {
        for (result.items[i], 0..) |_, j| {
            total += searchForX_MAS(result.items, j, i);
        }
    }

    try testing.expectEqual(9, total);
}