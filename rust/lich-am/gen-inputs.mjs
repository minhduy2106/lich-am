// Sinh input thật cho `rust-anything record --inputs`. Chạy: node gen-inputs.mjs > real-inputs.jsonl
import * as L from '../../lunar.js';
const out = (fn, args) => console.log(JSON.stringify({ fn, args }));
let seed = 42;
const rnd = (n) => ((seed = (seed * 1103515245 + 12345) % 2147483648) % n);
const start = L.jdFromDate(1, 1, 1900), span = L.jdFromDate(31, 12, 2100) - start;
for (let i = 0; i < 2000; i++) {
  const [d, m, y] = L.jdToDate(start + rnd(span));
  out('convertSolar2Lunar', [d, m, y]);
  if (i % 10 === 0) out('convertSolar2Lunar', [d, m, y, 8]);
  if (i % 7 === 0) { out('jdFromDate', [d, m, y]); out('jdToDate', [start + rnd(span)]); }
}
// Tết và tháng nhuận đã biết, cùng ngày ngay trước đó.
for (const [d, m, y] of [[29,1,2025],[17,2,2026],[10,2,2024],[21,1,1985],[22,3,2023],[25,7,2025],[6,10,2025],[1,1,1900],[31,12,2100]]) {
  out('convertSolar2Lunar', [d, m, y]);
  out('convertSolar2Lunar', L.jdToDate(L.jdFromDate(d, m, y) - 1));
}
const gioSets = [
  [],
  [{ name: 'ông nội', day: 30, month: 12 }, { name: 'bà', day: 10, month: 6 }],
  [{ name: 'Cụ; tổ, \\ "x"', day: 29, month: 1 }, { name: 'bố\nmẹ', day: 30, month: 6 }, { name: '😀', day: 1, month: 11 }],
  [{ name: 'A', day: 15, month: 8 }, { name: 'B', day: 30, month: 2 }, { name: 'C', day: 30, month: 11 }],
];
const optSets = [{ mung1: true, ram: true }, { mung1: false, ram: true }, { mung1: true, ram: false }, { mung1: false, ram: false }];
for (let i = 0; i < 40; i++) {
  const [d, m, y] = L.jdToDate(L.jdFromDate(1, 1, 1990) + rnd(365 * 50));
  const args = [L.jdFromDate(d, m, y), 30 + rnd(800), gioSets[i % gioSets.length], optSets[i % optSets.length]];
  out('buildEvents', args);
  if (i % 4 === 0) out('toIcs', [L.buildEvents(...args), '2026-11-25T08:30:15.123Z']);
}
out('buildEvents', [L.jdFromDate(29, 1, 2025), 384, gioSets[1], optSets[0]]);
out('toIcs', [[], '2026-01-01T00:00:00.000Z']);
out('toIcs', [L.buildEvents(L.jdFromDate(29, 1, 2025), 384, gioSets[2], optSets[0]), '1999-12-31T23:59:59.999Z']);
