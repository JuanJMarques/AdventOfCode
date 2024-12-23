const std = @import("std");
const testing = std.testing;
pub const  ArragementList = std.SinglyLinkedList(u64);

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

pub fn parseLine(line: []const u8) !ArragementList {
    const allocator = std.heap.page_allocator;
    var arragementList = ArragementList{};
    var arrangementIt = std.mem.splitSequence(u8, line, " ");
    while (arrangementIt.next()) |arrangementStr| {
        const arrangement = try std.fmt.parseInt(u64, arrangementStr, 10);
        const node = try allocator.create(ArragementList.Node);
        node.data = arrangement;
        arragementList.prepend(node);
    }
    ArragementList.Node.reverse(&arragementList.first);
    return arragementList;
}

pub fn blink(stones: *ArragementList) !void {
    const allocator = std.heap.page_allocator;
    var current = stones.first;
    while (current) |stone| {
        if (stone.data == 0) {
            stone.data = 1;
            current = stone.next;
        }else {
            const digitLength = std.math.log10(stone.data) + 1;
            if (@mod(digitLength, 2) == 0) {
                const middle = digitLength / 2;
                const divisor = std.math.pow(@TypeOf(stone.data), 10, middle);
                const first = stone.data / divisor;
                const firstStone = try allocator.create(ArragementList.Node);
                firstStone.data = first;
                const second = @rem(stone.data, divisor);
                const secondStone = try allocator.create(ArragementList.Node);
                secondStone.data = second;
                current = stone.next;
                stone.insertAfter(secondStone);
                stone.insertAfter(firstStone);
                stones.remove(stone);
                allocator.destroy(stone);
            } else {
                stone.data = stone.data * 2024;
                current = stone.next;
            }
        }
    }
}

test "test case" {
    const input =
        \\125 17
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const allocator = std.heap.page_allocator;
    const blinkCount :usize = 25;
    for (result.items) |line| {
        const cleanLine  = std.mem.trim(u8, line, "\r\n");
        var arragementList = try parseLine(cleanLine);
        defer {
            while (arragementList.popFirst()) |node| {
                allocator.destroy(node);
            }
        }
        var i :usize = 0;
        while (i<blinkCount): (i+=1) {
            try blink(&arragementList);
        }
        total = @intCast(arragementList.len());
    }

    try testing.expectEqual(55312, total);
}
