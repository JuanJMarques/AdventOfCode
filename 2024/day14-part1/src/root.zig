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

pub const Robot = struct {
    const Self = @This();
    pX: i32,
    pY: i32,
    vX: i32,
    vY: i32,

    pub fn move(self: *Self, maxX:  i32, maxY: i32) void {
        self.pX = @mod(self.pX+self.vX, maxX);
        self.pY = @mod(self.pY+self.vY, maxY);
    }
};

pub fn parseRobot(line: []const u8) !Robot {
    const posStr = line[2..(std.mem.indexOfScalar(u8, line, 'v') orelse 0)-1];
    const velStr = line[(std.mem.indexOfScalar(u8, line, 'v') orelse 0)+2..];
    const  posX = try std.fmt.parseInt(i32, posStr[0..std.mem.indexOfScalar(u8, posStr, ',') orelse 0], 10);
    const  posY = try std.fmt.parseInt(i32, posStr[(std.mem.indexOfScalar(u8, posStr, ',') orelse 0) + 1..], 10);
    const velX = try std.fmt.parseInt(i32, velStr[0..std.mem.indexOfScalar(u8, velStr, ',') orelse 0], 10);
    const velY = try std.fmt.parseInt(i32, velStr[(std.mem.indexOfScalar(u8, velStr, ',') orelse 0) + 1..], 10);
    return Robot{
        .pX = posX,
        .pY = posY,
        .vX = velX,
        .vY = velY,
    };
}

pub fn getQuadrant(robot: Robot, hSep: i32, vSep: i32) usize {
    if(robot.pX + 1 == hSep or robot.pY + 1 == vSep) {
        return 0;
    }
    return @intCast((@divTrunc(robot.pX, hSep) + (@divTrunc(robot.pY, vSep) << 1) + 1));
}

test "test case" {
    const input =
        \\p=0,4 v=3,-3
        \\p=6,3 v=-1,-3
        \\p=10,3 v=-1,2
        \\p=2,0 v=2,-1
        \\p=0,0 v=1,3
        \\p=3,0 v=-2,-2
        \\p=7,6 v=-1,-3
        \\p=3,0 v=-1,-2
        \\p=9,3 v=2,3
        \\p=7,3 v=-1,2
        \\p=2,4 v=2,-3
        \\p=9,5 v=-3,-3
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const allocator = std.heap.page_allocator;
    var robots = std.ArrayList(Robot).init(allocator);
    defer robots.deinit();
    for (result.items) |line| {
        const cleanLine = std.mem.trim(u8, line, "\n\r");
        try robots.append(try parseRobot(cleanLine));
        total += line.len;
    }
    const seconds = 100;
    const maxX:i32 = 11;
    const maxY:i32 = 7;
    for (0..seconds) |_|{
        for (0..robots.items.len) | i| {
            robots.items[i].move(maxX, maxY);
        }
    }
    const hSep = @divTrunc(maxX, 2) + 1;
    const vSep = @divTrunc(maxY, 2) + 1;
    var robotQuadrants =[4]u64{0,0,0,0};
    for (robots.items) | robot| {
        const quadrant = getQuadrant(robot, hSep, vSep);
        if (quadrant > 0) {
            robotQuadrants[quadrant-1] += 1;
        }
    }
    total = 1;
    for (robotQuadrants) |robotCount| {
        total *= robotCount;
    }
    try testing.expectEqual(12, total);
}