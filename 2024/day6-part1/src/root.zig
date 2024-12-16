const std = @import("std");
const testing = std.testing;

pub const Guard = struct {
    posX :usize,
    posY :usize,
    dirX :i8,
    dirY :i8,

    pub fn parseGuard(x: usize, y: usize, c: u8) Guard {
        var dirX :i8 = 0;
        var dirY :i8 = 0;
        switch (c) {
            '^' => {
                dirX = 0;
                dirY = -1;
            },
            'v' => {
                dirX = 0;
                dirY = 1;
            },
            '<' => {
                dirX = -1;
                dirY = 0;
            },
            '>' => {
                dirX = 1;
                dirY = 0;
            },
            else => {}
        }
        return Guard{.posX = x, .posY = y, .dirX = dirX, .dirY = dirY};
    }

    fn turnRight(self: *Guard) void {
        if (self.dirX == 0) {
            self.dirX -= self.dirY;
            self.dirY = 0;
        } else {
            self.dirY += self.dirX;
            self.dirX = 0;
        }
    }

    fn advance(self: *Guard) bool {
        if((self.dirX >= 0 or self.posX >= 1) and (self.dirY >= 0 or self.posY >= 1)){
            if (self.dirX >= 0){
                self.posX += @intCast(self.dirX);
            }  else {
                self.posX -= @intCast(-self.dirX);
            }
            if (self.dirY >= 0){
                self.posY += @intCast(self.dirY);
            } else {
                self.posY -= @intCast(-self.dirY);
            }
            return true;
        }
        return false;
    }

    pub fn move(self: *Guard, map: [][]bool) !void {
        const oldX = self.posX;
        const oldY = self.posY;
        if (!self.advance()){
            return error.OutOfBoundsError;
        }
        if(self.posY >= map.len or self.posX >= map[0].len) {
            return error.OutOfBoundsError;
        }
        if (!map[self.posY][self.posX]){
            self.turnRight();
            self.posX = oldX;
            self.posY = oldY;
        }
    }

    pub fn getPosition(self: Guard) struct {x:usize, y:usize} {
        return .{.x=self.posX, .y=self.posY};
    }
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


pub fn readFile(path: []const u8) ![] const u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return try file.readToEndAlloc(std.heap.page_allocator, @as(usize, std.math.maxInt(usize)));
}

pub fn transformLine(line: []const u8) struct {map :std.ArrayList(bool), visited: std.ArrayList(bool)} {
    const allocator = std.heap.page_allocator;
    var transformed = std.ArrayList(bool).init(allocator);
    var visited = std.ArrayList(bool).init(allocator);
    for (line) |value| {
        transformed.append(value != '#') catch unreachable;
        visited.append(false) catch unreachable;
    }
    return .{.map = transformed, .visited = visited};
}

pub fn locateGuard(line: []const u8) struct {located: bool, column: usize} {
    for (line, 0..) |value, i| {
        if (value == '^' or value == 'v' or value == '<' or value == '>') {
            return .{.located=true, .column=i};
        }
    }
    return .{.located = false, .column=0};
}






test "test case" {
    const input =
        \\....#.....
        \\.........#
        \\..........
        \\..#.......
        \\.......#..
        \\..........
        \\.#..^.....
        \\........#.
        \\#.........
        \\......#...
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u32 = 0;
    const allocator = std.heap.page_allocator;
    var map = std.ArrayList([] bool).init(allocator);
    defer map.deinit();
    var positionsVissited = std.ArrayList([] bool).init(allocator);
    defer positionsVissited.deinit();
    var guard: Guard = undefined;
    for (result.items, 0..) |line, row| {
        const cleanLine = std.mem.trim(u8, line, " \r\n");
        const transformed = transformLine(cleanLine);
        try map.append(transformed.map.items);
        try positionsVissited.append(transformed.visited.items);
        const guadrPresent = locateGuard(line);
        if(guadrPresent.located){
            guard = Guard.parseGuard(guadrPresent.column, row, line[guadrPresent.column]);
        }
    }
    var guardPos = guard.getPosition();
    positionsVissited.items[guardPos.y][guardPos.x] = true;
    var moving = true;
    while(moving) {
        guard.move(map.items) catch {
            moving = false;
            break;
        };
        guardPos = guard.getPosition();
        positionsVissited.items[guardPos.y][guardPos.x] = true;
    }
    for (positionsVissited.items) |row| {
        for (row) |value| {
            if (value) {
                total += 1;
            }
        }
    }
    try testing.expectEqual(41, total);

}


