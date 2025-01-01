const std = @import("std");
const root = @import("root.zig");
const Robot = root.Robot;

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const allocator = std.heap.page_allocator;
    var robots = std.ArrayList(root.Robot).init(allocator);
    defer robots.deinit();
    for (result.items) |line| {
        const cleanLine = std.mem.trim(u8, line, "\n\r");
        try robots.append(try root.parseRobot(cleanLine));
        total += line.len;
    }
    const maxX:i32 = 101;
    const maxY:i32 = 103;
    const stdout = std.io.getStdOut().writer();
    const seconds = 10000;
    for (0..seconds) |s|{
        for (0..robots.items.len) | i| {
            robots.items[i].move(maxX, maxY);
        }
        var rowCount = std.AutoHashMap(i32, i32).init(allocator);
        defer rowCount.deinit();
        for (robots.items) | robot| {
            try rowCount.put(robot.pY, (rowCount.get(robot.pY) orelse 0) + 1);
        }
        var it = rowCount.keyIterator();
        while (it.next()) |key| {
            if (rowCount.get(key.*) == 32 ) {
                try stdout.print("Posible tree found at {d} seconds\n", .{s+1});
                try root.printRobots(robots.items, maxX, maxY, stdout);
            }
        }

    }
}
