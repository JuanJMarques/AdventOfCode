const std = @import("std");
const testing = std.testing;
const DoublyLinkedList = std.DoublyLinkedList;
pub const Disk = DoublyLinkedList(DiskBlock);
pub const DiskBlock = struct {
    id: u32,
    length: u32,
    free: bool,
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

pub fn readFile(path: []const u8) ![]const u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return try file.readToEndAlloc(std.heap.page_allocator, @as(usize, std.math.maxInt(usize)));
}

pub fn parseLine(line: []const u8) !Disk {
    const allocator = std.heap.page_allocator;
    var disk = Disk{};
    var parsingFile = true;
    var blockId: u32 = 0;
    for (line) |blockCh| {
        const blockLength = try std.fmt.parseInt(u32, &[_]u8{blockCh}, 10);
        const node = try allocator.create(Disk.Node);
        node.* = Disk.Node{ .data = DiskBlock{
            .id = blockId,
            .length = blockLength,
            .free = !parsingFile,
        } };
        disk.append(node);
        if (parsingFile) {
            blockId += 1;
        }
        parsingFile = !parsingFile;
    }
    return disk;
}

fn getNextFreeBlock(block: ?*Disk.Node, acceptCurrent: bool) ?*Disk.Node {
    var current = block;
    if (!acceptCurrent) {
        if (current) |candidate| {
            current = candidate.next;
        }
    }
    while (current) |candidate| {
        if (candidate.data.free) {
            return candidate;
        }
        current = candidate.next;
    }
    return null;
}

fn getPrevFileBlock(block: ?*Disk.Node, acceptCurrent: bool) ?*Disk.Node {
    var current = block;
    if (!acceptCurrent) {
        if (current) |candidate| {
            current = candidate.prev;
        }
    }
    while (current) |candidate| {
        if (!candidate.data.free) {
            return candidate;
        }
        current = candidate.prev;
    }
    return null;
}

fn isBefore(b1: ?*Disk.Node, b2: ?*Disk.Node) bool {
    if (b1 == null or b2 == null) {
        return false;
    }
    const b2Ptr = b2 orelse unreachable;
    var current = b1;
    while (current) |b1Ptr| {
        if (b1Ptr == b2Ptr) {
            return true;
        }
        current = b1Ptr.next;
    }
    return false;
}

fn getBiggetsFreeBlockSize(disk: *Disk) u32 {
    var maxSize :u32= 0;
    var freeBlock = getNextFreeBlock(disk.first, true);
    while (freeBlock) |node| {
        if (node.data.length > maxSize){
            maxSize = node.data.length;
        }
        freeBlock = getNextFreeBlock(freeBlock, false);
    }
    return maxSize;
}

pub fn defragment(disk: *Disk) !void {
    const allocator = std.heap.page_allocator;
    var fileblockOpt = getPrevFileBlock(disk.last, true);
    var maxFreeSize = getBiggetsFreeBlockSize(disk);
    while (fileblockOpt != disk.first) {
        var freeBlockOpt = getNextFreeBlock(disk.first, true);
        var relocated = false;
        const fileblock = fileblockOpt orelse unreachable;
        while (isBefore(freeBlockOpt, fileblockOpt) and !relocated and fileblock.data.length <= maxFreeSize) {
        const freeBlock = freeBlockOpt orelse unreachable;
            if (freeBlock.data.length >= fileblock.data.length) {
                const node = try allocator.create(Disk.Node);
                node.* = Disk.Node{ .data = DiskBlock{
                    .id = fileblock.data.id,
                    .length = fileblock.data.length,
                    .free = fileblock.data.free,
                }};
                const freeSize = freeBlock.data.length;
                freeBlock.data.length -= fileblock.data.length;
                if (freeSize == maxFreeSize) {
                    maxFreeSize = getBiggetsFreeBlockSize(disk);
                }
                disk.insertBefore(freeBlock, node);
                fileblock.data.free = true;
                fileblockOpt = getPrevFileBlock(fileblock, false);
                relocated = true;
                if (freeBlock.data.length == 0) {
                    disk.remove(freeBlock);
                    freeBlockOpt = getNextFreeBlock(disk.first, true);
                    allocator.destroy(freeBlock);
                }
            } else {
                freeBlockOpt = getNextFreeBlock(freeBlock, false);
            }
        }
        if (!relocated){
            fileblockOpt = getPrevFileBlock(fileblockOpt, false);
        }
    }
}

pub fn checksum(disk: Disk) u64 {
    var checksumResult: u64 = 0;
    var prev: u64 = 0;
    var opt = disk.first;
    while (opt) |current| {
        if (!current.data.free) {
            var tmp = ((prev + current.data.length) * (prev + current.data.length - 1));
            if (prev > 0) {
                tmp -= (prev - 1) * prev;
                tmp /= 2;
            }
            checksumResult += tmp * current.data.id;
        }
        prev += current.data.length;
        opt = current.next;
    }
    return checksumResult;
}

test "test case" {
    const input =
        \\2333133121414131402
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const allocator = std.heap.page_allocator;
    var disk = Disk{};
    defer {
        var current = disk.first;
        while (current) |node| {
            const next = node.next;
            allocator.destroy(node);
            current = next;
        }
    }
    for (result.items) |line| {
        const cleanline = std.mem.trim(u8, line, "\r\n");
        disk = try parseLine(cleanline);
        total += 1;
    }
    try defragment(&disk);
    total = checksum(disk);

    try testing.expectEqual(2858, total);
}
