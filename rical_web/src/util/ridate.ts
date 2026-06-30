export type Ridate = {
  year: number;
  /** 1-indexed */
  month: number;
  /** 1-indexed */
  dayOfMonth: number;
};

function toDate(ridate: Ridate) {
  return new Date(ridate.year, ridate.dayOfMonth - 1, ridate.dayOfMonth);
}

export function monthName(ridate: Ridate) {
  console.log('month: ' + ridate.month);
  return toDate(ridate).toLocaleString('default', { month: 'short' });
}

export function eq(ridate1: Ridate, ridate2: Ridate) {
  return (
    (ridate1.year === ridate2.year)
    && (ridate1.month === ridate2.month)
    && (ridate1.dayOfMonth === ridate2.dayOfMonth)
  );
}
