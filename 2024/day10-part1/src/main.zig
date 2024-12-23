const std = @import("std");
const root  = @import("root.zig");
const location = root.location;

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u32 = 0;
    const allocator = std.heap.page_allocator;
    const topoMap = try root.convertMap(result.items);
    defer {
        for (topoMap) |value| {
            allocator.free(value);
        }
        allocator.free(topoMap);
    }
    for (topoMap, 0..) |row, i| {
        for (row, 0..) |heigh, j| {
            if (heigh==0) {
                var headsHistory = std.AutoHashMap(location, void).init(allocator);
                total += root.trailHeadValue(topoMap, heigh, i, j, &headsHistory);
                headsHistory.clearAndFree();
                headsHistory.deinit();
            }
        }
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The sum of the scores of all trailheads on the topographic map is {d}\n", .{total});
}