const std = @import("std");
const root  = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u32 = 0;
    var parsingRules = true;
    const allocator = std.heap.page_allocator;
    var pageRules = root.rules{ .rulesMap = std.StringHashMap(std.ArrayList([]const u8)).init(allocator) };
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
    const stdout = std.io.getStdOut().writer();
    try stdout.print("the sum of valid pages middle elements is: {d}\n", .{total});
}