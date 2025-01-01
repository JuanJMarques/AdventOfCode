const std = @import("std");
const testing = std.testing;
const Writer = std.io.Writer(std.fs.File, std.posix.WriteError, std.fs.File.write);

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

    pub fn clone(self: Self) Self {
        return Self{
            .pX = self.pX,
            .pY = self.pY,
            .vX = self.vX,
            .vY = self.vY,
        };
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

pub fn printRobots(robots :[]Robot, maxX: usize, maxY: usize, writer: Writer) !void {
    for (0..maxY) | i| {
        for (0..maxX) | j| {
            var c :u8 = '.';
            for (robots) |robot| {
                if (robot.pX==j and robot.pY == i) {
                    c = '#';
                }
            }
            try writer.print("{c}", .{c});
        }
        try writer.print("\n", .{});
    }
    try writer.print("\n", .{});
}
