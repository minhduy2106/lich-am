// Âm lịch Việt Nam (múi giờ +7), thuật toán của Hồ Ngọc Đức.
const TZ = 7;

export function jdFromDate(dd, mm, yy) {
  const a = Math.floor((14 - mm) / 12);
  const y = yy + 4800 - a;
  const m = mm + 12 * a - 3;
  let jd = dd + Math.floor((153 * m + 2) / 5) + 365 * y + Math.floor(y / 4) - Math.floor(y / 100) + Math.floor(y / 400) - 32045;
  if (jd < 2299161) jd = dd + Math.floor((153 * m + 2) / 5) + 365 * y + Math.floor(y / 4) - 32083;
  return jd;
}

export function jdToDate(jd) {
  let b, c;
  if (jd > 2299160) {
    const a = jd + 32044;
    b = Math.floor((4 * a + 3) / 146097);
    c = a - Math.floor((b * 146097) / 4);
  } else {
    b = 0;
    c = jd + 32082;
  }
  const d = Math.floor((4 * c + 3) / 1461);
  const e = c - Math.floor((1461 * d) / 4);
  const m = Math.floor((5 * e + 2) / 153);
  const day = e - Math.floor((153 * m + 2) / 5) + 1;
  const month = m + 3 - 12 * Math.floor(m / 10);
  const year = b * 100 + d - 4800 + Math.floor(m / 10);
  return [day, month, year];
}

function getNewMoonDay(k, tz) {
  const T = k / 1236.85, T2 = T * T, T3 = T2 * T, dr = Math.PI / 180;
  let Jd1 = 2415020.75933 + 29.53058868 * k + 0.0001178 * T2 - 0.000000155 * T3;
  Jd1 += 0.00033 * Math.sin((166.56 + 132.87 * T - 0.009173 * T2) * dr);
  const M = 359.2242 + 29.10535608 * k - 0.0000333 * T2 - 0.00000347 * T3;
  const Mpr = 306.0253 + 385.81691806 * k + 0.0107306 * T2 + 0.00001236 * T3;
  const F = 21.2964 + 390.67050646 * k - 0.0016528 * T2 - 0.00000239 * T3;
  let C1 = (0.1734 - 0.000393 * T) * Math.sin(M * dr) + 0.0021 * Math.sin(2 * dr * M);
  C1 = C1 - 0.4068 * Math.sin(Mpr * dr) + 0.0161 * Math.sin(dr * 2 * Mpr);
  C1 = C1 - 0.0004 * Math.sin(dr * 3 * Mpr);
  C1 = C1 + 0.0104 * Math.sin(dr * 2 * F) - 0.0051 * Math.sin(dr * (M + Mpr));
  C1 = C1 - 0.0074 * Math.sin(dr * (M - Mpr)) + 0.0004 * Math.sin(dr * (2 * F + M));
  C1 = C1 - 0.0004 * Math.sin(dr * (2 * F - M)) - 0.0006 * Math.sin(dr * (2 * F + Mpr));
  C1 = C1 + 0.0010 * Math.sin(dr * (2 * F - Mpr)) + 0.0005 * Math.sin(dr * (2 * Mpr + M));
  const deltat = T < -11
    ? 0.001 + 0.000839 * T + 0.0002261 * T2 - 0.00000845 * T3 - 0.000000081 * T * T3
    : -0.000278 + 0.000265 * T + 0.000262 * T2;
  return Math.floor(Jd1 + C1 - deltat + 0.5 + tz / 24);
}

function getSunLongitude(jdn, tz) {
  const T = (jdn - 2451545.5 - tz / 24) / 36525, T2 = T * T, dr = Math.PI / 180;
  const M = 357.5291 + 35999.0503 * T - 0.0001559 * T2 - 0.00000048 * T * T2;
  const L0 = 280.46645 + 36000.76983 * T + 0.0003032 * T2;
  let DL = (1.9146 - 0.004817 * T - 0.000014 * T2) * Math.sin(dr * M);
  DL += (0.019993 - 0.000101 * T) * Math.sin(dr * 2 * M) + 0.00029 * Math.sin(dr * 3 * M);
  let L = (L0 + DL) * dr;
  L = L - Math.PI * 2 * Math.floor(L / (Math.PI * 2));
  return Math.floor((L / Math.PI) * 6);
}

function getLunarMonth11(yy, tz) {
  const off = jdFromDate(31, 12, yy) - 2415021;
  const k = Math.floor(off / 29.530588853);
  const nm = getNewMoonDay(k, tz);
  return getSunLongitude(nm, tz) >= 9 ? getNewMoonDay(k - 1, tz) : nm;
}

function getLeapMonthOffset(a11, tz) {
  const k = Math.floor((a11 - 2415021.076998695) / 29.530588853 + 0.5);
  let last, i = 1;
  let arc = getSunLongitude(getNewMoonDay(k + i, tz), tz);
  do {
    last = arc;
    i++;
    arc = getSunLongitude(getNewMoonDay(k + i, tz), tz);
  } while (arc !== last && i < 14);
  return i - 1;
}

// Trả về [ngày, tháng, năm âm, nhuận (0/1)].
export function convertSolar2Lunar(dd, mm, yy, tz = TZ) {
  const dayNumber = jdFromDate(dd, mm, yy);
  const k = Math.floor((dayNumber - 2415021.076998695) / 29.530588853);
  let monthStart = getNewMoonDay(k + 1, tz);
  // k có thể cao hơn 1 khi sóc rơi ngay sau nửa đêm (vd 7/5/2054): lùi tới khi monthStart <= dayNumber,
  // nếu chỉ lùi một lần thì ra "ngày 0". Giới hạn 3 bước để input NaN không lặp mãi.
  for (let i = 0; monthStart > dayNumber && i < 3; i++) monthStart = getNewMoonDay(k - i, tz);
  let a11 = getLunarMonth11(yy, tz);
  let b11 = a11;
  let lunarYear;
  if (a11 >= monthStart) {
    lunarYear = yy;
    a11 = getLunarMonth11(yy - 1, tz);
  } else {
    lunarYear = yy + 1;
    b11 = getLunarMonth11(yy + 1, tz);
  }
  const lunarDay = dayNumber - monthStart + 1;
  const diff = Math.floor((monthStart - a11) / 29);
  let lunarLeap = 0;
  let lunarMonth = diff + 11;
  if (b11 - a11 > 365) {
    const leapMonthDiff = getLeapMonthOffset(a11, tz);
    if (diff >= leapMonthDiff) {
      lunarMonth = diff + 10;
      if (diff === leapMonthDiff) lunarLeap = 1;
    }
  }
  if (lunarMonth > 12) lunarMonth -= 12;
  if (lunarMonth >= 11 && diff < 4) lunarYear -= 1;
  return [lunarDay, lunarMonth, lunarYear, lunarLeap];
}

const lunarOfJd = (jd) => convertSolar2Lunar(...jdToDate(jd));

// gios: [{name, day, month}] theo âm lịch. opts: {mung1, ram}.
export function buildEvents(fromJd, days, gios, opts) {
  const events = [];
  let next = lunarOfJd(fromJd);
  for (let jd = fromJd; jd < fromJd + days; jd++) {
    const [ld, lm, , leap] = next;
    next = lunarOfJd(jd + 1);
    const solar = jdToDate(jd);
    const thang = `tháng ${lm}${leap ? ' nhuận' : ''}`;
    if (opts.mung1 && ld === 1) events.push({ solar, kind: 'le', title: `Mùng 1 ${thang} (âm)` });
    if (opts.ram && ld === 15) events.push({ solar, kind: 'le', title: `Rằm ${thang} (âm)` });
    const lastDayOfMonth = next[0] === 1;
    for (const g of gios) {
      // Giỗ cúng vào tháng thường, không cúng tháng nhuận; tháng thiếu thì giỗ ngày 30 dời về ngày 29.
      if (!leap && g.month === lm && (g.day === ld || (lastDayOfMonth && g.day > ld))) {
        events.push({ solar, kind: 'gio', title: `Giỗ ${g.name} (${g.day}/${g.month} âm)` });
      }
    }
  }
  return events;
}

function hash(s) {
  let h = 0;
  for (const c of s) h = (h * 31 + c.codePointAt(0)) >>> 0;
  return h.toString(36);
}

// shortcut: không gập dòng dài hơn 75 byte; iPhone và Google vẫn đọc được, cần thì thêm folding.
// now: chuỗi ISO, ví dụ new Date().toISOString() (chuỗi thì truyền qua wasm được, Date thì không).
export function toIcs(events, now) {
  const p = (n) => String(n).padStart(2, '0');
  const ymd = ([d, m, y]) => `${y}${p(m)}${p(d)}`;
  const esc = (s) => s.replace(/[\\;,]/g, (c) => '\\' + c).replace(/[\r\n]+/g, ' ');
  const stamp = now.replace(/[-:]/g, '').slice(0, 15) + 'Z';
  const lines = ['BEGIN:VCALENDAR', 'VERSION:2.0', 'PRODID:-//lich-am//VI', 'CALSCALE:GREGORIAN', 'X-WR-CALNAME:Lịch âm'];
  // UID trùng thì Google chỉ giữ một sự kiện: thêm -2, -3 cho lần lặp (cùng người nhập hai lần, hoặc hash trùng).
  const seen = new Map();
  for (const e of events) {
    const start = ymd(e.solar);
    const base = `${start}-${hash(e.title)}`;
    const n = (seen.get(base) || 0) + 1;
    seen.set(base, n);
    lines.push(
      'BEGIN:VEVENT',
      `UID:${base}${n > 1 ? `-${n}` : ''}@lich-am`,
      `DTSTAMP:${stamp}`,
      `DTSTART;VALUE=DATE:${start}`,
      `DTEND;VALUE=DATE:${ymd(jdToDate(jdFromDate(...e.solar) + 1))}`,
      `SUMMARY:${esc(e.title)}`,
      'TRANSP:TRANSPARENT'
    );
    // Sự kiện cả ngày bắt đầu lúc 0h: -PT6H = 18h hôm trước, -PT63H = 9h sáng 3 ngày trước.
    const alarms = e.kind === 'gio' ? ['-PT63H', '-PT6H'] : ['-PT6H'];
    for (const t of alarms) {
      lines.push('BEGIN:VALARM', 'ACTION:DISPLAY', `DESCRIPTION:${esc(e.title)}`, `TRIGGER:${t}`, 'END:VALARM');
    }
    lines.push('END:VEVENT');
  }
  lines.push('END:VCALENDAR');
  return lines.join('\r\n') + '\r\n';
}

