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
    var blockId :u32 = 0;
    for (line) |blockCh| {
        const blockLength = try std.fmt.parseInt(u32, &[_]u8{blockCh}, 10);
        const node = try allocator.create(Disk.Node);
        node.* = Disk.Node {
            .data = DiskBlock {
                .id = blockId,
                .length = blockLength,
                .free = !parsingFile,
            }
        };
        disk.append(node);
        if(parsingFile) {
            blockId += 1;
        }
        parsingFile = !parsingFile;
    }
    return disk;
}

fn getNextFreeBlock(block: ?*Disk.Node, acceptCurrent: bool) ?*Disk.Node {
    var current = block;
    if (!acceptCurrent){
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
    return undefined;

}

fn getPrevFileBlock(block: ?*Disk.Node, acceptCurrent: bool) ?*Disk.Node {
    var current = block;
    if (!acceptCurrent){
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

fn isBefore(b1: ?*Disk.Node, b2: ?*Disk.Node) bool{
    if (b1 == null or b2 == null) {
        return false;
    }
    const b2Ptr = b2 orelse unreachable;
    var current  = b1;
    while (current) |b1Ptr| {
        if (b1Ptr == b2Ptr) {
            return true;
        }
        current = b1Ptr.next;
    }
    return false;
}

pub fn defragment(disk: *Disk) !void {
    const allocator = std.heap.page_allocator;
    var freeBlockOpt = getNextFreeBlock(disk.first, true);
    var fileblockOpt = getPrevFileBlock(disk.last, true);
    while (isBefore(freeBlockOpt, fileblockOpt)) {
        const freeBlock = freeBlockOpt orelse unreachable;
        const fileblock = fileblockOpt orelse unreachable;
        const node = try allocator.create(Disk.Node);
        if (freeBlock.data.length >= fileblock.data.length) {
            node.* = Disk.Node {
                .data = DiskBlock {
                    .id = fileblock.data.id,
                    .length = fileblock.data.length,
                    .free = fileblock.data.free,
                }
            };
            freeBlock.data.length -= fileblock.data.length;
            disk.insertBefore(freeBlock, node);
            fileblockOpt = getPrevFileBlock(fileblock, false);
            disk.remove(fileblock);
            allocator.destroy(fileblock);
            if (freeBlock.data.length == 0) {
                freeBlockOpt = getNextFreeBlock(freeBlock, false);
                disk.remove(freeBlock);
                allocator.destroy(freeBlock);
            }
        }else {
            node.* = Disk.Node {
                .data = DiskBlock {
                    .id = fileblock.data.id,
                    .length = freeBlock.data.length,
                    .free = fileblock.data.free,
                }
            };
            fileblock.data.length -= freeBlock.data.length;
            disk.insertAfter(freeBlock, node);
            freeBlockOpt = getNextFreeBlock(freeBlock, false);
            disk.remove(freeBlock);
            allocator.destroy(freeBlock);
        }
    }
}

pub fn checksum(disk: Disk) u64 {
    var checksumResult :u64 = 0;
    var prev :u64  = 0;
    var opt = disk.first;
    while (opt) |current| {
        if(!current.data.free){
            var tmp = ((prev + current.data.length) * (prev + current.data.length - 1));
            if( prev > 0) {
                tmp -= (prev - 1) * prev;
                tmp /= 2;
            }

            checksumResult += tmp * current.data.id;
            prev += current.data.length;
        }
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
        while (current) |node|  {
            const next = node.next;
            allocator.destroy(node);
            current = next;
        }
    }
    for (result.items) |line| {
        const cleanline = std.mem.trim(u8, line, "\r\n");
        disk = try parseLine(cleanline);
        total +=1;
    }
    try defragment(&disk);
    total =  checksum(disk);

    try testing.expectEqual(1928, total);
}