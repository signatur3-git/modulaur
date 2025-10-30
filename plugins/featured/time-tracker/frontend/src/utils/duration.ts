/**
 * Format seconds to HH:MM:SS
 */
export function formatDuration(seconds: number): string {
  if (!seconds || seconds < 0) return '00:00:00';

  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);

  return [hours, minutes, secs]
    .map(v => v.toString().padStart(2, '0'))
    .join(':');
}

/**
 * Format seconds to human-readable format (e.g., "2h 15m")
 */
export function formatDurationShort(seconds: number): string {
  if (!seconds || seconds < 0) return '0m';

  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);

  if (hours > 0) {
    return minutes > 0 ? `${hours}h ${minutes}m` : `${hours}h`;
  }

  return `${minutes}m`;
}

/**
 * Format seconds to decimal hours
 */
export function formatHours(seconds: number): string {
  if (!seconds || seconds < 0) return '0.00';
  return (seconds / 3600).toFixed(2);
}

/**
 * Calculate duration between two ISO timestamps
 */
export function calculateDuration(startTime: string, endTime: string): number {
  const start = new Date(startTime).getTime();
  const end = new Date(endTime).getTime();
  return Math.floor((end - start) / 1000);
}

/**
 * Get elapsed time since start (for active timer)
 */
export function getElapsedTime(startTime: string): number {
  const start = new Date(startTime).getTime();
  const now = Date.now();
  return Math.floor((now - start) / 1000);
}

