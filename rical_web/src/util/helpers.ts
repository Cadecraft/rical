import { type Task } from '../util/types';

export function isAnythingFocused() {
  return document.activeElement &&
    document.activeElement !== document.body &&
    document.activeElement.nodeName !== "BUTTON";
}

export function isAnythingFocusedInclButton() {
  return document.activeElement &&
    document.activeElement !== document.body
}

export function leadingZero(x: number): string {
  if (x < 10) {
    return '0' + x.toString();
  } else {
    return x.toString();
  }
}

export function formatMin(minutes: number | undefined): string {
  if (minutes === undefined) {
    return '';
  }
  // TODO: leading 0 and base 24hr on user prefs
  const hours = Math.floor(minutes / 60);
  const minLeft = minutes % 60;
  if (hours === 12) {
    return `${hours}:${leadingZero(minLeft)}pm`;
  } else if (hours >= 12) {
    return `${hours - 12}:${leadingZero(minLeft)}pm`;
  } else {
    return `${hours}:${leadingZero(minLeft)}am`;
  }
}

/**
  Parse a user-inputted time shorthand string and return the minutes
  If this fails, return undefined
  E.g. 3 -> 3:00 AM
  E.g. 3am -> 3:00 AM
  E.g. 3 PM -> 3:00 PM
  E.g. 15:45 -> 3:45 PM
  E.g. 12:00 A.M. -> 0:00 PM
  E.g. 15pm -> INVALID
*/
export function parseMinString(minString: string): number | undefined {
  const normalized = minString.replace(/\s/g,'').replaceAll('.', '').toLowerCase();

  const validShorthand = /^[0-9][0-9]?(:[0-9][0-9])?(pm|am)?$/;

  if (!normalized.match(validShorthand)) {
    return undefined;
  }

  const period = normalized.endsWith('am') ? 'am' : (normalized.endsWith('pm') ? 'pm' : '24hr');
  let hours = 0;
  let minutes = 0;
  let processingMinutes = false;
  for (const c of normalized) {
    const asDigit = Number.parseInt(c);
    if (!isNaN(asDigit)) {
      if (processingMinutes) {
        minutes *= 10;
        minutes += asDigit;
      } else {
        hours *= 10;
        hours += asDigit;
      }
    } else if (c === ':') {
      processingMinutes = true;
    }
  }
  if (minutes >= 60) {
    return undefined;
  }

  // Special case: 12pm -> 12, but 12am -> 0
  if (period === 'am') {
    if (hours === 12) {
      hours = 0;
    }
  } else if (period === 'pm') {
    if (hours !== 12) {
      hours += 12;
    }
  }
  const totalMinutes = hours * 60 + minutes;
  const minutesPerDay = 24 * 60;
  if (totalMinutes >= minutesPerDay) {
    return undefined;
  }
  return totalMinutes;
}

/** Determine which task comes first in the ordering. Matches the API's ordering */
export function compareTasks(taskA: Task, taskB: Task) {
  const maxMin = 24 * 60 + 1;

  if (taskA.day < taskB.day) {
    return -1;
  } else if (taskA.day > taskB.day) {
    return 1;
  } else if ((taskA.start_min ?? maxMin) < (taskB.start_min ?? maxMin)) {
    return -1;
  } else if ((taskA.start_min ?? maxMin) > (taskB.start_min ?? maxMin)) {
    return 1;
  } else if ((taskA.end_min ?? maxMin) < (taskB.end_min ?? maxMin)) {
    return -1;
  } else if ((taskA.end_min ?? maxMin) > (taskB.end_min ?? maxMin)) {
    return 1;
  } else if ((taskA.description ?? "") < (taskB.description ?? "")) {
    return -1;
  } else if ((taskA.description ?? "") > (taskB.description ?? "")) {
    return 1;
  } else if (taskA.title < taskB.title) {
    return -1;
  } else if (taskB.title > taskA.title) {
    return 1;
  } else {
    return 0;
  }
}
