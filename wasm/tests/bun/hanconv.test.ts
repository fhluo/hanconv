import {expect, test} from "bun:test";

import init, {hk2s, hk2t, jp2t, s2hk, s2t, s2tw, s2twp, t2hk, t2jp, t2s, t2tw, tw2s, tw2sp, tw2t} from "hanconv";

await init();

type Case = [
    name: string,
    convert: (input: string) => string,
    input: string,
    expected: string,
];

const cases: Case[] = [
    ["s2t", s2t, "编译", "編譯"],
    ["t2s", t2s, "編譯", "编译"],
    ["s2tw", s2tw, "启用", "啟用"],
    ["tw2s", tw2s, "啟用", "启用"],
    ["s2twp", s2twp, "缓存", "快取"],
    ["tw2sp", tw2sp, "快取", "缓存"],
    ["t2tw", t2tw, "羣組", "群組"],
    ["tw2t", tw2t, "群組", "羣組"],
    ["s2hk", s2hk, "图像锐化", "圖像鋭化"],
    ["hk2s", hk2s, "圖像鋭化", "图像锐化"],
    ["t2hk", t2hk, "說明文件", "説明文件"],
    ["hk2t", hk2t, "説明文件", "說明文件"],
    ["t2jp", t2jp, "圖形", "図形"],
    ["jp2t", jp2t, "図形", "圖形"],
];

for (const [name, convert, input, expected] of cases) {
    test(name, () => {
        expect(convert(input)).toBe(expected);
    });
}
