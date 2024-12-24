const std = @import("std");
const testing = std.testing;
pub const Location = struct {
    x: i32,
    y: i32,
};

pub const Region = struct {
    area: u32,
    perimeter: u32,
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

pub fn processRegion(map: [][]const u8, procesedRegions: [][]bool, i: i32 , j: i32, regionType: u8) !Region {
    const allocator = std.heap.page_allocator;
    var visitedMap = std.AutoHashMap(Location, void).init(allocator);
    defer {
        visitedMap.clearAndFree();
        visitedMap.deinit();
    }
    var region = Region{.area = 0, .perimeter = 0};
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
            if (x >= 0 and x < map[@intCast(y)].len){
                if (map[@intCast(y)][@intCast(x)] == regionType) {
                    if (!visitedMap.contains(location)){
                        region.area += 1;
                        procesedRegions[@intCast(y)][@intCast(x)] = true;
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
        \\OXOXO
        \\OOOOO
        \\OXOXO
        \\OOOOO
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    // const allocator = std.heap.page_allocator;
    const items = result.items;
    const procesedRegions = try generateProcesedRegions(items);
    for (items, 0..) |line, i| {
        const cleanLine  = std.mem.trim(u8, line, "\r\n");
        for (cleanLine, 0..) |c, j| {
            if (!procesedRegions[i][j]) {
                const region = try processRegion(items, procesedRegions, @intCast(i) , @intCast(j), c);
                total += region.area * region.perimeter;
            }
        }
    }

    try testing.expectEqual(772, total);
}

test "test case2" {
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
    // const allocator = std.heap.page_allocator;
    const items = result.items;
    const procesedRegions = try generateProcesedRegions(items);
    for (items, 0..) |line, i| {
        const cleanLine  = std.mem.trim(u8, line, "\r\n");
        for (cleanLine, 0..) |c, j| {
            if (!procesedRegions[i][j]) {
                const region = try processRegion(items, procesedRegions, @intCast(i) , @intCast(j), c);
                total += region.area * region.perimeter;
            }
        }
    }

    try testing.expectEqual(1930, total);
}