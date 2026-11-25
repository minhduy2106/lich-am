// Chạy: node test.mjs
import assert from 'node:assert';
import * as L from './lunar.js';

const cases = [
  [[29, 1, 2025], [1, 1, 2025, 0]], // Tết Ất Tỵ
  [[17, 2, 2026], [1, 1, 2026, 0]], // Tết Bính Ngọ
  [[10, 2, 2024], [1, 1, 2024, 0]], // Tết Giáp Thìn
  [[21, 1, 1985], [1, 1, 1985, 0]], // Tết 1985: Việt Nam khác Trung Quốc (20/2)
  [[22, 3, 2023], [1, 2, 2023, 1]], // mùng 1 tháng 2 nhuận 2023
  [[25, 7, 2025], [1, 6, 2025, 1]], // mùng 1 tháng 6 nhuận 2025
  [[6, 10, 2025], [15, 8, 2025, 0]], // Trung thu 2025
  [[7, 5, 2054], [30, 3, 2054, 0]], // sóc ngay sau nửa đêm: trước đây ra "ngày 0"
  [[9, 4, 2062], [30, 2, 2062, 0]],
];
for (const [solar, lunar] of cases) {
  assert.deepStrictEqual(L.convertSolar2Lunar(...solar), lunar, `${solar.join('/')}`);
}

// Năm Ất Tỵ: từ Tết 2025 đến hết 30 Tết 2026, có tháng 6 nhuận nên 13 lần mùng 1.
const from = L.jdFromDate(29, 1, 2025);
const days = L.jdFromDate(17, 2, 2026) - from;
const ev = L.buildEvents(from, days, [{ name: 'ông nội', day: 30, month: 12 }, { name: 'bà', day: 10, month: 6 }], { mung1: true, ram: true });
assert.strictEqual(ev.filter((e) => e.title.startsWith('Mùng 1')).length, 13);
assert.strictEqual(ev.filter((e) => e.title.startsWith('Rằm')).length, 13);
// Giỗ 30 tháng Chạp luôn rơi vào ngày trước Tết, kể cả khi tháng Chạp chỉ có 29 ngày.
const gioChap = ev.filter((e) => e.title.startsWith('Giỗ ông nội'));
assert.deepStrictEqual(gioChap.map((e) => e.solar), [[16, 2, 2026]]);
// Giỗ tháng 6 chỉ một lần, không lặp lại ở tháng 6 nhuận.
assert.strictEqual(ev.filter((e) => e.title.startsWith('Giỗ bà')).length, 1);

const ics = L.toIcs(ev, '2026-01-01T00:00:00.000Z');
assert.strictEqual(ics.match(/BEGIN:VEVENT/g).length, ev.length);
assert.ok(ics.includes('SUMMARY:Giỗ ông nội (30/12 âm)\r\nTRANSP'));
assert.ok(ics.includes('DTSTART;VALUE=DATE:20260216\r\nDTEND;VALUE=DATE:20260217'));
// Giờ nhắc: giỗ báo 9h sáng 3 ngày trước và 18h hôm trước; mùng 1, rằm chỉ 18h hôm trước.
const triggers = (prefix) => ics.split('BEGIN:VEVENT').find((b) => b.includes(`SUMMARY:${prefix}`)).match(/TRIGGER:\S+/g);
assert.deepStrictEqual(triggers('Giỗ'), ['TRIGGER:-PT63H', 'TRIGGER:-PT6H']);
assert.deepStrictEqual(triggers('Mùng 1'), ['TRIGGER:-PT6H']);
assert.deepStrictEqual(triggers('Rằm'), ['TRIGGER:-PT6H']);

// Cùng người nhập hai lần, hoặc hai tên có hash trùng ('cụ Ly'/'cụ MZ'): UID vẫn phải khác nhau.
const dup = L.buildEvents(from, days, [{ name: 'cụ Ly', day: 5, month: 3 }, { name: 'cụ MZ', day: 5, month: 3 }, { name: 'cụ Ly', day: 5, month: 3 }], {});
const uids = L.toIcs(dup, '2026-01-01T00:00:00.000Z').match(/^UID:.*$/gm);
assert.strictEqual(uids.length, 3);
assert.strictEqual(new Set(uids).size, 3);

console.log(`OK: ${cases.length} ngày chuyển đổi, ${ev.length} sự kiện`);
