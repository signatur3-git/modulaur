import type { TimeEntry } from '../composables/useTimeTracker';

interface ProjectSummary {
  duration: number;
  amount: number;
  entries: number;
}

interface TimeSummary {
  totalDuration: number;
  billableDuration: number;
  totalAmount: number;
  entriesCount: number;
  byProject: Record<string, ProjectSummary>;
  byDate: Record<string, number>;
}

/**
 * Calculate time summary from entries
 */
export function calculateSummary(entries: TimeEntry[]): TimeSummary {
  const summary: TimeSummary = {
    totalDuration: 0,
    billableDuration: 0,
    totalAmount: 0,
    entriesCount: entries.length,
    byProject: {},
    byDate: {}
  };

  entries.forEach(entry => {
    const duration = entry.data.duration_seconds || 0;
    summary.totalDuration += duration;

    // Billable tracking
    if (entry.data.billable) {
      summary.billableDuration += duration;
      if (entry.data.hourly_rate) {
        summary.totalAmount += (duration / 3600) * entry.data.hourly_rate;
      }
    }

    // By project
    const project = entry.data.project;
    if (!summary.byProject[project]) {
      summary.byProject[project] = { duration: 0, amount: 0, entries: 0 };
    }
    summary.byProject[project].duration += duration;
    summary.byProject[project].entries += 1;
    if (entry.data.billable && entry.data.hourly_rate) {
      summary.byProject[project].amount += (duration / 3600) * entry.data.hourly_rate;
    }

    // By date
    const date = entry.timestamp.split('T')[0];
    summary.byDate[date] = (summary.byDate[date] || 0) + duration;
  });

  return summary;
}

/**
 * Get today's entries
 */
export function getTodayEntries(entries: TimeEntry[]): TimeEntry[] {
  const today = new Date().toISOString().split('T')[0];
  return entries.filter(e => e.timestamp.startsWith(today));
}

/**
 * Get this week's entries
 */
export function getThisWeekEntries(entries: TimeEntry[]): TimeEntry[] {
  const now = new Date();
  const monday = new Date(now);
  monday.setDate(now.getDate() - now.getDay() + 1);
  monday.setHours(0, 0, 0, 0);

  const mondayISO = monday.toISOString();

  return entries.filter(e => e.timestamp >= mondayISO);
}

/**
 * Get entries for date range
 */
export function getEntriesInRange(
  entries: TimeEntry[],
  startDate: string,
  endDate: string
): TimeEntry[] {
  return entries.filter(e => {
    return e.timestamp >= startDate && e.timestamp <= endDate;
  });
}

/**
 * Group entries by project
 */
export function groupByProject(entries: TimeEntry[]): Record<string, TimeEntry[]> {
  const grouped: Record<string, TimeEntry[]> = {};

  entries.forEach(entry => {
    const project = entry.data.project;
    if (!grouped[project]) {
      grouped[project] = [];
    }
    grouped[project].push(entry);
  });

  return grouped;
}

/**
 * Group entries by date
 */
export function groupByDate(entries: TimeEntry[]): Record<string, TimeEntry[]> {
  const grouped: Record<string, TimeEntry[]> = {};

  entries.forEach(entry => {
    const date = entry.timestamp.split('T')[0];
    if (!grouped[date]) {
      grouped[date] = [];
    }
    grouped[date].push(entry);
  });

  return grouped;
}

/**
 * Get this month's entries
 */
export function getThisMonthEntries(entries: TimeEntry[]): TimeEntry[] {
  const now = new Date();
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1);
  const firstDayISO = firstDay.toISOString();

  return entries.filter(e => e.timestamp >= firstDayISO);
}

/**
 * Get summary with additional metrics
 */
export function getSummary(entries: TimeEntry[]) {
  const summary = calculateSummary(entries);
  return {
    totalDuration: summary.totalDuration,
    billableDuration: summary.billableDuration,
    totalBillable: summary.totalAmount,
    entriesCount: summary.entriesCount,
    projectCount: Object.keys(summary.byProject).length
  };
}

/**
 * Get breakdown by project
 */
export function getBreakdownByProject(entries: TimeEntry[]) {
  const grouped = groupByProject(entries);
  const breakdown = Object.entries(grouped).map(([project, projectEntries]) => {
    const duration = projectEntries.reduce(
      (sum, e) => sum + (e.data.duration_seconds || 0),
      0
    );
    const billable = projectEntries.reduce((sum, e) => {
      if (e.data.billable && e.data.hourly_rate) {
        return sum + ((e.data.duration_seconds || 0) / 3600) * e.data.hourly_rate;
      }
      return sum;
    }, 0);

    return {
      project,
      duration,
      entries: projectEntries.length,
      billable: billable > 0 ? billable : undefined
    };
  });

  return breakdown.sort((a, b) => b.duration - a.duration);
}

/**
 * Get breakdown by day
 */
export function getBreakdownByDay(entries: TimeEntry[]) {
  const grouped = groupByDate(entries);
  const breakdown = Object.entries(grouped).map(([date, dayEntries]) => {
    const duration = dayEntries.reduce(
      (sum, e) => sum + (e.data.duration_seconds || 0),
      0
    );
    const dateObj = new Date(date);
    const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    const label = `${dayNames[dateObj.getDay()]} ${dateObj.getMonth() + 1}/${dateObj.getDate()}`;

    return {
      date,
      label,
      duration
    };
  });

  return breakdown.sort((a, b) => a.date.localeCompare(b.date));
}

/**
 * Get breakdown by week
 */
export function getBreakdownByWeek(entries: TimeEntry[]) {
  const weekMap: Record<string, TimeEntry[]> = {};

  entries.forEach(entry => {
    const date = new Date(entry.timestamp);
    const monday = new Date(date);
    monday.setDate(date.getDate() - date.getDay() + 1);
    monday.setHours(0, 0, 0, 0);
    const weekKey = monday.toISOString().split('T')[0];

    if (!weekMap[weekKey]) {
      weekMap[weekKey] = [];
    }
    weekMap[weekKey].push(entry);
  });

  const breakdown = Object.entries(weekMap).map(([weekStart, weekEntries]) => {
    const duration = weekEntries.reduce(
      (sum, e) => sum + (e.data.duration_seconds || 0),
      0
    );
    const startDate = new Date(weekStart);
    const endDate = new Date(startDate);
    endDate.setDate(startDate.getDate() + 6);
    const label = `${startDate.getMonth() + 1}/${startDate.getDate()} - ${endDate.getMonth() + 1}/${endDate.getDate()}`;

    return {
      week: weekStart,
      label,
      duration
    };
  });

  return breakdown.sort((a, b) => a.week.localeCompare(b.week));
}

