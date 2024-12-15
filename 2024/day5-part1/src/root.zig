const std = @import("std");
const testing = std.testing;
pub const rules = struct {
    rulesMap: std.StringHashMap(std.ArrayList([]const u8)),

    pub fn addRule(self: *rules, key: []const u8, value: []const u8) void {
        if (!self.rulesMap.contains(key)) {
            const allocator = std.heap.page_allocator;
            self.rulesMap.put(key, std.ArrayList([]const u8).init(allocator)) catch unreachable;
        }
        var ruleList = self.rulesMap.get(key).?;
        ruleList.append(value) catch unreachable;
        // put the element again in the map because the pointer may be moved to another memory addres after the append;
        self.rulesMap.put(key, ruleList) catch unreachable;
    }

    pub fn getRules(self: rules) std.StringHashMap(std.ArrayList([]const u8)) {
        return self.rulesMap;
    }

    pub fn deinit(self: *rules) void {
        var it = self.rulesMap.valueIterator();
        while (it.next()) |list| {
            list.deinit();
        }
        self.rulesMap.deinit();
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

pub fn readFile(path: []const u8) ![]const u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return try file.readToEndAlloc(std.heap.page_allocator, @as(usize, std.math.maxInt(usize)));
}

test "test case" {
    const input =
        \\47|53
        \\97|13
        \\97|61
        \\97|47
        \\75|29
        \\61|13
        \\75|53
        \\29|13
        \\97|29
        \\53|29
        \\61|53
        \\97|53
        \\61|29
        \\47|13
        \\75|47
        \\97|75
        \\47|61
        \\75|61
        \\47|29
        \\75|13
        \\53|13
        \\
        \\75,97,47,61,53
        \\61,13,29
        \\97,13,75,29,47
        \\75,47,61,53,29
        \\97,61,53,29,13
        \\75,29,13
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u32 = 0;
    var parsingRules = true;
    const allocator = std.heap.page_allocator;
    var pageRules = rules{ .rulesMap = std.StringHashMap(std.ArrayList([]const u8)).init(allocator) };
    defer pageRules.deinit();
    for (result.items) |line| {
        const cleanLine = std.mem.trim(u8, line, " \r\n");
        if (cleanLine.len == 0) {
            parsingRules = false;
        } else {
            if (parsingRules) {
                var ruleParts = std.mem.split(u8, cleanLine, "|");
                const firstPage = ruleParts.next().?;
                const secondPage = ruleParts.next().?;
                pageRules.addRule(firstPage, secondPage);
            } else {
                var indexMap = std.StringHashMap(usize).init(allocator);
                defer indexMap.deinit();
                const currentRules = pageRules.getRules();
                var page = std.mem.splitSequence(u8, cleanLine, ",");
                var pageElements = std.ArrayList([]const u8).init(allocator);
                defer pageElements.deinit();
                var pagesToVisit = std.BufSet.init(allocator);
                defer pagesToVisit.deinit();
                var index: usize = 0;
                while (page.next()) |pageNumber| {
                    try pageElements.append(pageNumber);
                    try indexMap.put(pageNumber, index);
                    index += 1;
                }
                var validPage = true;
                var pagesIt = indexMap.keyIterator();
                while (pagesIt.next()) |pagePointer| {
                    const pageKey = pagePointer.*;
                    if (currentRules.contains(pageKey)) {
                        for (currentRules.get(pageKey).?.items) |currentPageRule| {
                            if (indexMap.contains(currentPageRule)){
                                validPage = validPage and indexMap.get(pageKey).? < indexMap.get(currentPageRule).?;
                            }
                        }
                    }
                }
                if (validPage) {
                    const middleElement = pageElements.items[pageElements.items.len / 2];
                    const elemValue = try std.fmt.parseInt(u32, middleElement, 10);
                    total += elemValue;
                }
            }
        }
    }

    try testing.expectEqual(143, total);
}