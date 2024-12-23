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

pub fn convertMap(strMap: [][]const u8) ![][]u8 {
    const allocator = std.heap.page_allocator;
    var matrix = try allocator.alloc([]u8, strMap.len);
    for (strMap, 0..) |row, i| {
        matrix[i] = try allocator.alloc(u8, row.len);
        for (row, 0..) |c, j| {
            matrix[i][j] = c - '0';
        }
    }
    return matrix;
}

pub fn trailHeadValue(topoMap: [][]u8, value: u8, i: usize, j: usize) u32{
    if(value == 9) {
        return 1;
    }
    var total: u32 = 0;
    if (i > 0 and topoMap[i-1][j] == value+1) {
        total += trailHeadValue(topoMap, topoMap[i-1][j], i-1, j);
    }
    if (i+1 < topoMap.len and topoMap[i+1][j] == value+1) {
        total += trailHeadValue(topoMap, topoMap[i+1][j], i+1, j);
    }
    if (j > 0 and topoMap[i][j-1] == value+1) {
        total += trailHeadValue(topoMap, topoMap[i][j-1], i, j-1);
    }
    if (j+1 < topoMap[i].len and topoMap[i][j+1] == value+1) {
        total += trailHeadValue(topoMap, topoMap[i][j+1], i, j+1);
    }
    return total;
}

test "test case" {
    const input =
        \\89010123
        \\78121874
        \\87430965
        \\96549874
        \\45678903
        \\32019012
        \\01329801
        \\10456732
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u32 = 0;
    const allocator = std.heap.page_allocator;

    const topoMap = try convertMap(result.items);
    defer {
        for (topoMap) |value| {
            allocator.free(value);
        }
        allocator.free(topoMap);
    }
    for (topoMap, 0..) |row, i| {
        for (row, 0..) |heigh, j| {
            if (heigh==0) {
                total += trailHeadValue(topoMap, heigh, i, j);
            }
        }
    }

    try testing.expectEqual(81, total);
}
