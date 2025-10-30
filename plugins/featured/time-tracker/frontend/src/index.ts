import { App } from 'vue';
import TimeTrackerPage from './TimeTrackerPage.vue';
import TimerPanel from './components/TimerPanel.vue';
import TimeLogPanel from './components/TimeLogPanel.vue';
import TimeSummaryPanel from './components/TimeSummaryPanel.vue';
import ProjectManagerPanel from './components/ProjectManagerPanel.vue';
import TimeBreakdownPanel from './components/TimeBreakdownPanel.vue';

export default {
  install(app: App) {
    // Register components globally
    app.component('TimeTrackerPage', TimeTrackerPage);
    app.component('TimerPanel', TimerPanel);
    app.component('TimeLogPanel', TimeLogPanel);
    app.component('TimeSummaryPanel', TimeSummaryPanel);
    app.component('ProjectManagerPanel', ProjectManagerPanel);
    app.component('TimeBreakdownPanel', TimeBreakdownPanel);
  },

  // Export components for UMD loader to find them
  components: {
    TimeTrackerPage,
    TimerPanel,
    TimeLogPanel,
    TimeSummaryPanel,
    ProjectManagerPanel,
    TimeBreakdownPanel
  },

  // Plugin metadata
  meta: {
    name: 'time-tracker',
    version: '1.0.0',
    description: 'Track work hours with timers, manual entries, and reports',
    pages: [
      {
        id: 'time-tracker',
        name: 'Time Tracker',
        component: 'TimeTrackerPage',
        icon: '⏱️',
        description: 'Complete time tracking workspace'
      }
    ],
    panels: [
      {
        id: 'timer',
        name: 'Timer',
        component: 'TimerPanel',
        icon: '⏱️',
        defaultSize: { w: 4, h: 4 }
      },
      {
        id: 'time-log',
        name: 'Time Log',
        component: 'TimeLogPanel',
        icon: '📋',
        defaultSize: { w: 6, h: 6 }
      },
      {
        id: 'time-summary',
        name: 'Time Summary',
        component: 'TimeSummaryPanel',
        icon: '📊',
        defaultSize: { w: 8, h: 6 }
      },
      {
        id: 'project-manager',
        name: 'Project Manager',
        component: 'ProjectManagerPanel',
        icon: '📁',
        defaultSize: { w: 6, h: 6 }
      },
      {
        id: 'time-breakdown',
        name: 'Time Breakdown',
        component: 'TimeBreakdownPanel',
        icon: '📊',
        defaultSize: { w: 8, h: 6 }
      }
    ]
  }
};

// Named exports for direct import
export { TimerPanel, TimeLogPanel, TimeSummaryPanel };

