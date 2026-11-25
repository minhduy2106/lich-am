// Sinh input biên cho `rust-anything record --inputs`. Chạy:
// node gen-edge-inputs.mjs > edge-inputs.jsonl && cat real-inputs.jsonl edge-inputs.jsonl > all-inputs.jsonl
import * as L from '../../lunar.js';
const out = (fn, args) => console.log(JSON.stringify({ fn, args }));
let seed = 7;
const rnd = (n) => ((seed = (seed * 1103515245 + 12345) % 2147483648) % n);
const J = (d, m, y) => L.jdFromDate(d, m, y);

// convertSolar2Lunar: mọi ngày 1999-2001, 2029-2031; ngày ngẫu nhiên 1800-2199 với nhiều tz.
for (const [y0, y1] of [[1999, 2001], [2029, 2031]])
  for (let jd = J(1, 1, y0); jd <= J(31, 12, y1); jd++) out('convertSolar2Lunar', L.jdToDate(jd));
const lo = J(1, 1, 1800), span = J(31, 12, 2199) - lo;
for (let i = 0; i < 600; i++) {
  const dmy = L.jdToDate(lo + rnd(span));
  out('convertSolar2Lunar', dmy);
  for (const tz of [7, 8, 0, 9, 7.5, null]) if (rnd(3) === 0) out('convertSolar2Lunar', [...dmy, tz]);
}
for (const tz of [7, 8, 0, 9, 7.5, null]) out('convertSolar2Lunar', [29, 1, 2025, tz]);

// jdFromDate / jdToDate: cuối tháng, 29/2, 1/1, 31/12, mốc đổi lịch Gregory.
for (const y of [1582, 1600, 1700, 1800, 1900, 2000, 2024, 2100, 2199]) {
  for (const m of [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]) {
    const last = L.jdToDate(J(1, m % 12 + 1, m === 12 ? y + 1 : y) - 1);
    out('jdFromDate', last);
    out('jdToDate', [J(...last)]);
  }
  out('jdFromDate', [29, 2, y]); out('jdFromDate', [1, 1, y]); out('jdFromDate', [31, 12, y]);
  out('jdToDate', [J(1, 1, y)]); out('jdToDate', [J(31, 12, y)]);
}
for (const jd of [2299159, 2299160, 2299161, 2299162]) out('jdToDate', [jd]);
for (const dmy of [[4, 10, 1582], [5, 10, 1582], [14, 10, 1582], [15, 10, 1582]]) out('jdFromDate', dmy);

// buildEvents.
const all = { mung1: true, ram: true };
const tet25 = J(29, 1, 2025), year = 384;
const events = [];
const be = (...args) => { out('buildEvents', args); events.push(L.buildEvents(...args)); };
for (const days of [0, 1, 3653]) be(J(1, 1, 2020), days, [{ name: 'A', day: 1, month: 1 }], all);
// Giỗ 29 và 30 ở mọi tháng (gồm tháng thiếu), qua vài năm.
const g2930 = [];
for (let m = 1; m <= 12; m++) g2930.push({ name: `n${m}a`, day: 29, month: m }, { name: `n${m}b`, day: 30, month: m });
be(J(1, 1, 2023), 365 * 3, g2930, {});
// Tháng 1 và 12 quanh Tết; tháng nhuận 6/2025 và 2/2023.
be(J(1, 1, 2024), 800, [{ name: 'giêng', day: 1, month: 1 }, { name: 'chạp', day: 30, month: 12 }, { name: 'chạp2', day: 23, month: 12 }], all);
be(tet25, year, [{ name: 'sáu', day: 10, month: 6 }, { name: 'sáu30', day: 30, month: 6 }], all);
be(J(22, 1, 2023), year, [{ name: 'hai', day: 10, month: 2 }, { name: 'hai30', day: 30, month: 2 }], all);
// Tên lạ, trùng lặp, day dạng chuỗi.
const odd = [
  { name: '😀🎉 cụ', day: 5, month: 3 }, { name: 'a;b,c\\d\ne\r\nf\rg', day: 5, month: 3 }, { name: '', day: 5, month: 3 },
  { name: 42, day: 5, month: 3 }, { name: 1.5, day: 5, month: 3 }, { name: null, day: 5, month: 3 }, { day: 5, month: 3 },
  { name: true, day: 5, month: 3 }, { name: 'dup', day: 5, month: 3 }, { name: 'dup', day: 5, month: 3 },
  { name: 'chuỗi', day: '30', month: 3 }, { name: 'chuỗi2', day: '5', month: 3 }, { name: 'tháng chuỗi', day: 5, month: '3' },
];
be(tet25, year, odd, all);
// `g.day > ld` ép kiểu: day không phải số ở các tháng thiếu (2, 4, 5 năm 2025).
const coerce = [];
for (const day of ['30', ' 30 ', '0x1e', '0b11110', '3e1', 'Infinity', '-Infinity', '+-Infinity', 'inf', 'abc', '', true, null, false])
  for (const month of [2, 4, 5]) coerce.push({ name: JSON.stringify(day), day, month });
be(tet25, year, coerce, {});
for (const v of [true, false, 0, 1, '', 'x', null]) {
  be(tet25, 60, [{ name: 'x', day: 15, month: 1 }], { mung1: v, ram: v });
  be(tet25, 60, [], { mung1: v });
  be(tet25, 60, [], { ram: v });
}
be(tet25, 60, [], {});

// toIcs.
const now = '2026-11-25T08:30:15.123Z';
for (const ev of events) out('toIcs', [ev, now]);
out('toIcs', [[], now]);
out('toIcs', [[
  { solar: [1, 1, 2025], kind: 'gio', title: 'a\r\nb' },
  { solar: [28, 2, 2024], kind: 'le', title: 'lone\rcr' },
  { solar: [29, 2, 2024], kind: 'other', title: '😀 emoji; x, y\\z' },
  { solar: [31, 12, 2025], kind: '', title: '' },
  { solar: [30, 4, 2025], kind: 'GIO', title: '\n\r\n\r' },
  { solar: [15, 10, 1582], title: 'no kind' },
  { solar: [4, 10, 1582], kind: 'gio', title: 'pre-gregorian' },
], now]);
