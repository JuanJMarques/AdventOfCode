const std = @import("std");
const root  = @import("root.zig");
const Guard = root.Guard;

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
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
        const transformed = root.transformLine(cleanLine);
        try map.append(transformed.map.items);
        try positionsVissited.append(transformed.visited.items);
        const guadrPresent = root.locateGuard(line);
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
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Distinct posistions visited by the guard before leaving are {d}\n", .{total});
}