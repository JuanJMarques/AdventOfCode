const std = @import("std");
const root = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    // const allocator = std.heap.page_allocator;
    const items = result.items;
    const procesedRegions = try root.generateProcesedRegions(items);
    for (items, 0..) |line, i| {
        const cleanLine  = std.mem.trim(u8, line, "\r\n");
        for (cleanLine, 0..) |c, j| {
            if (!procesedRegions[i][j]) {
                const region = try root.processRegion(items, procesedRegions, @intCast(i) , @intCast(j), c);
                total += region.area * region.perimeter;
            }
        }
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The total price of all regions on the maps is {d}\n", .{total});
}
