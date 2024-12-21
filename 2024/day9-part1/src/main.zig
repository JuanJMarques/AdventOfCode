const std = @import("std");
const root  = @import("root.zig");
const location = root.location;
const Disk = root.Disk;
const DiskBlock = root.DiskBlock;

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const allocator = std.heap.page_allocator;
    var disk = Disk{};
    defer {
        var current = disk.first;
        while (current) |node|  {
            const next = node.next;
            allocator.destroy(node);
            current = next;
        }
    }
    for (result.items) |line| {
        const cleanline = std.mem.trim(u8, line, "\r\n");
        disk = try root.parseLine(cleanline);
        total +=1;
    }

    try root.defragment(&disk);
    total =  root.checksum(disk);
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The filesystem checaksum is {d}\n", .{total});
}