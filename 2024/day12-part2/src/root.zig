const std = @import("std");
const testing = std.testing;
pub const Location = struct {
    x: i32,
    y: i32,
};

pub const Region = struct {
    area: u32,
    perimeter: u32,
    sides: u32,
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

pub fn freeMatix(matrix: anytype) void {
    const allocator = std.heap.page_allocator;
    for (matrix) |row| {
        allocator.free(row);
    }
    allocator.free(matrix);
}

pub fn generateProcesedRegions(input: [][]const u8) ![][]bool {
    const allocator = std.heap.page_allocator;
    var processed = try allocator.alloc([]bool, input.len);
    var i: usize = 0;
    while (i<input.len): (i+=1) {
        processed[i] = try allocator.alloc(bool, input[i].len);
        @memset(processed[i], false);

    }
    return processed;
}

fn checkUperBorder(map: [][]const u8, border: [][]bool, regionType: u8, x: usize, y: usize) u32 {
    if (!border[y][x] and (y==0 or map[y-1][x] != regionType)) {
        border[y][x] = true;
        var left = @subWithOverflow(x, 1);
        while (left[1]==0 and left[0]>=0 and map[y][left[0]] == regionType and (y==0 or map[y-1][left[0]] != regionType)) {
            border[y][left[0]] =  true;
            left = @subWithOverflow(left[0], 1);
        }
        var right = x + 1;
        while (right < map[y].len and map[y][right] == regionType and (y==0 or map[y-1][right] != regionType)) {
            border[y][right] =  true;
            right += 1;
        }
        return 1;
    }
    return 0;
}

fn checkLowerBorder(map: [][]const u8, border: [][]bool, regionType: u8, x: usize, y: usize) u32 {
    if (!border[y][x] and (y+1 == map.len or map[y+1][x] != regionType)) {
        border[y][x] = true;
        var left = @subWithOverflow(x, 1);
        while (left[1]==0 and left[0]>=0 and map[y][left[0]] == regionType and (y+1 == map.len  or map[y+1][left[0]] != regionType)) {
            border[y][left[0]] =  true;
            left = @subWithOverflow(left[0], 1);
        }
        var right = x + 1;
        while (right < map[y].len and map[y][right] == regionType and (y+1 == map.len  or map[y+1][right] != regionType)) {
            border[y][right] =  true;
            right += 1;
        }
        return 1;
    }
    return 0;
}

fn checkLeftBorder(map: [][]const u8, border: [][]bool, regionType: u8, x: usize, y: usize) u32 {
    if (!border[y][x] and (x==0 or map[y][x-1] != regionType)) {
        border[y][x] = true;
        var up = @subWithOverflow(y, 1);
        while (up[0]>=0 and up[1]==0 and map[up[0]][x] == regionType and (x==0 or map[up[0]][x-1] != regionType)) {
            border[up[0]][x] =  true;
            up = @subWithOverflow(up[0], 1);
        }
        var down = y + 1;
        while (down < map.len and map[down][x] == regionType and (x==0 or map[down][x-1] != regionType)) {
            border[down][x] =  true;
            down += 1;
        }
        return 1;
    }
    return 0;
}

fn checkRightBorder(map: [][]const u8, border: [][]bool, regionType: u8, x: usize, y: usize) u32 {
    if (!border[y][x] and (x+1 == map[y].len or map[y][x+1] != regionType)) {
        border[y][x] = true;
        var up = @subWithOverflow(y, 1);
        while (up[0]>=0 and up[1]==0 and map[up[0]][x] == regionType and (x+1==map[y].len or map[up[0]][x+1] != regionType)) {
            border[up[0]][x] =  true;
            up = @subWithOverflow(up[0], 1);
        }
        var down = y + 1;
        while (down < map.len and map[down][x] == regionType and (x+1==map[y].len or map[down][x+1] != regionType)) {
            border[down][x] =  true;
            down += 1;
        }
        return 1;
    }
    return 0;
}

pub fn processRegion(map: [][]const u8, procesedRegions: [][]bool, i: i32 , j: i32, regionType: u8) !Region {
    const upperBorders = try generateProcesedRegions(map);
    defer freeMatix(upperBorders);
    const lowerBorders = try generateProcesedRegions(map);
    defer freeMatix(lowerBorders);
    const leftBorders = try generateProcesedRegions(map);
    defer freeMatix(leftBorders);
    const rightBorders = try generateProcesedRegions(map);
    defer freeMatix(rightBorders);
    const allocator = std.heap.page_allocator;
    var visitedMap = std.AutoHashMap(Location, void).init(allocator);
    defer {
        visitedMap.clearAndFree();
        visitedMap.deinit();
    }
    var region = Region{.area = 0, .perimeter = 0, .sides = 0};
    const CellQueue = std.TailQueue(Location);
    var pendingCells = CellQueue{};
    var node = try allocator.create(CellQueue.Node);
    node .data = Location{ .x = j, .y = i};
    pendingCells.append(node);
    while (pendingCells.pop()) |current| {
        const location = current.data;
        const x = location.x;
        const y = location.y;
        if (y >= 0 and y < map.len) {
            const uy: usize = @intCast(y);
            if (x >= 0 and x < map[uy].len){
                const ux: usize = @intCast(x);
                if (map[uy][ux] == regionType) {
                    if (!visitedMap.contains(location)){
                        region.area += 1;
                        procesedRegions[uy][ux] = true;
                        node = try allocator.create(CellQueue.Node);
                        node .data = Location{ .x = x-1, .y = y};
                        pendingCells.append(node);
                        node = try allocator.create(CellQueue.Node);
                        node .data = Location{ .x = x+1, .y = y};
                        pendingCells.append(node);
                        node = try allocator.create(CellQueue.Node);
                        node .data = Location{ .x = x, .y = y-1};
                        pendingCells.append(node);
                        node = try allocator.create(CellQueue.Node);
                        node .data = Location{ .x = x, .y = y+1};
                        pendingCells.append(node);
                        region.sides += checkUperBorder(map, upperBorders, regionType, ux, uy);
                        region.sides += checkLowerBorder(map, lowerBorders, regionType, ux, uy);
                        region.sides += checkLeftBorder(map, leftBorders, regionType, ux, uy);
                        region.sides += checkRightBorder(map, rightBorders, regionType, ux, uy);
                    }
                    _ = try visitedMap.put(location, {});
                } else {
                    region.perimeter += 1;
                }
            }else {
                region.perimeter += 1;
            }
        }else {
            region.perimeter += 1;
        }
        allocator.destroy(current);
    }
    return region;
}

test "test case" {
    const input =
        \\OOOOO
        \\OXXXX
        \\OOOOO
        \\OXXXX
        \\OOOOO
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const items = result.items;
    const procesedRegions = try generateProcesedRegions(items);
    defer freeMatix(procesedRegions);
    for (items, 0..) |line, i| {
        const cleanLine  = std.mem.trim(u8, line, "\r\n");
        for (cleanLine, 0..) |c, j| {
            if (!procesedRegions[i][j]) {
                const region = try processRegion(items, procesedRegions, @intCast(i) , @intCast(j), c);
                total += region.area * region.sides;
            }
        }
    }

    try testing.expectEqual(236, total);
}

test "test case 2" {
    const input =
        \\AAAAAA
        \\AAABBA
        \\AAABBA
        \\ABBAAA
        \\ABBAAA
        \\AAAAAA
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const items = result.items;
    const procesedRegions = try generateProcesedRegions(items);
    defer freeMatix(procesedRegions);
    for (items, 0..) |line, i| {
        const cleanLine  = std.mem.trim(u8, line, "\r\n");
        for (cleanLine, 0..) |c, j| {
            if (!procesedRegions[i][j]) {
                const region = try processRegion(items, procesedRegions, @intCast(i) , @intCast(j), c);
                total += region.area * region.sides;
            }
        }
    }

    try testing.expectEqual(368, total);
}

test "test case 3" {
    const input =
        \\RRRRIICCFF
        \\RRRRIICCCF
        \\VVRRRCCFFF
        \\VVRCCCJFFF
        \\VVVVCJJCFE
        \\VVIVCCJJEE
        \\VVIIICJJEE
        \\MIIIIIJJEE
        \\MIIISIJEEE
        \\MMMISSJEEE
    ;
    const result = try divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const items = result.items;
    const procesedRegions = try generateProcesedRegions(items);
    defer freeMatix(procesedRegions);
    for (items, 0..) |line, i| {
        const cleanLine  = std.mem.trim(u8, line, "\r\n");
        for (cleanLine, 0..) |c, j| {
            if (!procesedRegions[i][j]) {
                const region = try processRegion(items, procesedRegions, @intCast(i) , @intCast(j), c);
                total += region.area * region.sides;
            }
        }
    }

    try testing.expectEqual(1206, total);
}
