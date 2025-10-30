import type { TimeEntry } from '../composables/useTimeTracker';

/**
 * Generate CSV from time entries
 */
export function generateCSV(entries: TimeEntry[]): string {
  let csv = 'Date,Start,End,Duration (hours),Project,Description,Billable,Rate,Total,Tags,Notes\n';

  entries.forEach(entry => {
    const date = entry.timestamp.split('T')[0];
    const start = new Date(entry.timestamp).toLocaleTimeString();
    const end = entry.data.end_time ? new Date(entry.data.end_time).toLocaleTimeString() : '';
    const duration = (entry.data.duration_seconds || 0) / 3600;
    const project = escapeCSV(entry.data.project);
    const description = escapeCSV(entry.data.description);
    const billable = entry.data.billable ? 'Yes' : 'No';
    const rate = entry.data.hourly_rate || '';
    const total = entry.data.billable && entry.data.hourly_rate
      ? (duration * entry.data.hourly_rate).toFixed(2)
      : '';
    const tags = (entry.data.tags || []).join(';');
    const notes = escapeCSV(entry.data.notes || '');

    csv += `${date},${start},${end},${duration.toFixed(2)},${project},${description},${billable},${rate},${total},${tags},${notes}\n`;
  });

  return csv;
}

/**
 * Escape CSV field (handle commas and quotes)
 */
function escapeCSV(value: string): string {
  if (!value) return '';

  // If contains comma, quote, or newline, wrap in quotes and escape quotes
  if (value.includes(',') || value.includes('"') || value.includes('\n')) {
    return '"' + value.replace(/"/g, '""') + '"';
  }

  return value;
}

/**
 * Download CSV file
 */
export function downloadCSV(csv: string, filename: string) {
  const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
  const link = document.createElement('a');
  const url = URL.createObjectURL(blob);

  link.setAttribute('href', url);
  link.setAttribute('download', filename);
  link.style.visibility = 'hidden';

  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);

  URL.revokeObjectURL(url);
}

