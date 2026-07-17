import { DAYS_PER_WEEK } from '../util/constants';

export type Ridate = {
  year: number;
  /** 1-indexed */
  month: number;
  /** 1-indexed */
  dayOfMonth: number;
};

function toDate(ridate: Ridate) {
  return new Date(ridate.year, ridate.month - 1, ridate.dayOfMonth);
}

function toRidate(date: Date): Ridate {
  return {
    year: date.getFullYear(),
    month: date.getMonth() + 1,
    dayOfMonth: date.getDate(),
  };
}

export function monthName(ridate: Ridate) {
  return toDate(ridate).toLocaleString('default', { month: 'short' });
}

export function eq(ridate1: Ridate, ridate2: Ridate) {
  return (
    (ridate1.year === ridate2.year)
    && (ridate1.month === ridate2.month)
    && (ridate1.dayOfMonth === ridate2.dayOfMonth)
  );
}

export function weekdayName(ridate: Ridate) {
  return toDate(ridate).toLocaleDateString('default', { weekday: 'short' });
}

/** 0-indexed, 0 being Sunday */
export function weekdayNumber(ridate: Ridate) {
  return toDate(ridate).getDay();
}

export function getDaysInMonth(year: number, month: number) {
  // Note that months are 0-indexed so passing 1 as the month returns the last day of January
  return new Date(year, month, 0).getDate();
}

export function addDays(ridate: Ridate, days: number) {
  const res = toDate(ridate);
  res.setDate(res.getDate() + days);
  return toRidate(res);
}

/** Get the 2d array of days for a calendar month (res[row][weekday] gets you a date) */
export function getCalendarFrame(year: number, month: number) {
  const res: Ridate[][] = [];
  const daysInMonth = getDaysInMonth(year, month);

  for (let dayOfMonth = 1; dayOfMonth <= daysInMonth; ++dayOfMonth) {
    const d = { year, month, dayOfMonth };
    const weekday = weekdayNumber(d);
    if (res.length === 0 || weekday === 0) {
      const blankWeek = [];
      for (let i = 0; i < DAYS_PER_WEEK; ++i) {
        blankWeek.push({year: -1, month: -1, dayOfMonth: -1});
      }
      res.push(blankWeek);
    }
    res[res.length - 1][weekday] = d;
  }

  for (let i = DAYS_PER_WEEK - 1; i >= 0; --i) {
    if (res[0][i].year === -1) {
      const nextDate = res[0][i + 1];
      res[0][i] = addDays(nextDate, -1);
    }
  }

  for (let i = 0; i < DAYS_PER_WEEK; ++i) {
    const finalWeek = res.length - 1;
    if (res[finalWeek][i].year === -1) {
      const prevDate = res[finalWeek][i - 1];
      res[finalWeek][i] = addDays(prevDate, 1);
    }
  }

  return res;
}
