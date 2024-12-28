const std = @import("std");
const root = @import("root.zig");
const Button = root.Button;

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
    const seconds = 100;
    const maxX:i32 = 101;
    const maxY:i32 = 103;
    for (0..seconds) |_|{
        for (0..robots.items.len) | i| {
            robots.items[i].move(maxX, maxY);
        }
    }
    const hSep = @divTrunc(maxX, 2) + 1;
    const vSep = @divTrunc(maxY, 2) + 1;
    var robotQuadrants =[4]u64{0,0,0,0};
    for (robots.items) | robot| {
        const quadrant = root.getQuadrant(robot, hSep, vSep);
        if (quadrant > 0) {
            robotQuadrants[quadrant-1] += 1;
        }
    }
    total = 1;
    for (robotQuadrants) |robotCount| {
        total *= robotCount;
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The  safety factor be after exactly 100 secondss is {d}\n", .{total});
}
