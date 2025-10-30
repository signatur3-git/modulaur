// Built-in panel definitions - treated as system plugins
// This allows the core app to be plugin-agnostic

import TextPanel from '../components/panels/TextPanel.vue'
import ChartPanel from '../components/panels/ChartPanel.vue'
import TablePanel from '../components/panels/TablePanel.vue'
import KanbanPanel from '../components/panels/KanbanPanel.vue'
import TicketKanbanPanel from '../components/panels/TicketKanbanPanel.vue'

export interface BuiltInPanelType {
  id: string
  name: string
  icon: string
  component: any
  configSchema?: PanelConfigSchema
}

export interface PanelConfigSchema {
  fields: PanelConfigField[]
}

export interface PanelConfigField {
  key: string
  label: string
  type: 'text' | 'textarea' | 'select' | 'number' | 'checkbox'
  options?: { value: string | number | boolean; label: string }[]
  placeholder?: string
  required?: boolean
  min?: number
  max?: number
  rows?: number
  help?: string
}

// Built-in panel definitions
export const BUILT_IN_PANELS: BuiltInPanelType[] = [
  {
    id: 'text',
    name: 'Text',
    icon: '📝',
    component: TextPanel,
    configSchema: {
      fields: [
        {
          key: 'content',
          label: 'Content',
          type: 'textarea',
          required: true,
        },
      ],
    },
  },
  {
    id: 'chart',
    name: 'Chart',
    icon: '📊',
    component: ChartPanel,
    configSchema: {
      fields: [
        {
          key: 'chartType',
          label: 'Chart Type',
          type: 'select',
          options: [
            { value: 'bar', label: 'Bar' },
            { value: 'line', label: 'Line' },
            { value: 'pie', label: 'Pie' },
            { value: 'doughnut', label: 'Doughnut' },
          ],
          required: true,
        },
        {
          key: 'groupBy',
          label: 'Group By',
          type: 'select',
          options: [
            { value: 'status', label: 'Status' },
            { value: 'source', label: 'Source' },
            { value: 'type', label: 'Record Type' },
            { value: 'time', label: 'Time (Timeseries)' },
          ],
          required: true,
        },
        {
          key: 'timeBucket',
          label: 'Time Bucket',
          type: 'select',
          options: [
            { value: 'hour', label: 'Hourly' },
            { value: 'day', label: 'Daily' },
            { value: 'week', label: 'Weekly' },
            { value: 'month', label: 'Monthly' },
          ],
        },
        {
          key: 'dataTransform',
          label: 'Data Transformation',
          type: 'select',
          options: [
            { value: 'count', label: 'Count' },
            { value: 'sum', label: 'Sum' },
            { value: 'avg', label: 'Average' },
          ],
          required: true,
        },
        {
          key: 'recordType',
          label: 'Record Type',
          type: 'select',
          options: [
            { value: '', label: 'All Types' },
            { value: 'rest_api', label: 'REST API' },
            { value: 'gitlab_pipeline', label: 'GitLab Pipeline' },
            { value: 'gitlab_job', label: 'GitLab Job' },
          ],
        },
        {
          key: 'dataSource',
          label: 'Data Source',
          type: 'text',
          placeholder: 'Leave empty for all sources',
        },
      ],
    },
  },
  {
    id: 'table',
    name: 'Table',
    icon: '📋',
    component: TablePanel,
    configSchema: {
      fields: [
        {
          key: 'dataSource',
          label: 'Data Source',
          type: 'text',
          placeholder: 'Leave empty for all sources',
        },
        {
          key: 'recordType',
          label: 'Record Type',
          type: 'select',
          options: [
            { value: '', label: 'All Types' },
            { value: 'rest_api', label: 'REST API' },
            { value: 'gitlab_pipeline', label: 'GitLab Pipeline' },
            { value: 'gitlab_job', label: 'GitLab Job' },
          ],
        },
        {
          key: 'pageSize',
          label: 'Page Size',
          type: 'number',
          min: 5,
          max: 100,
          placeholder: '20',
        },
      ],
    },
  },
  {
    id: 'kanban',
    name: 'Kanban',
    icon: '📌',
    component: KanbanPanel,
  },
  {
    id: 'ticket-kanban',
    name: 'Ticket Kanban',
    icon: '🎯',
    component: TicketKanbanPanel,
  },
]

// Helper function to get built-in panel by ID
export function getBuiltInPanel(id: string): BuiltInPanelType | undefined {
  return BUILT_IN_PANELS.find(panel => panel.id === id)
}

// Helper function to get all built-in panels
export function getAllBuiltInPanels(): BuiltInPanelType[] {
  return BUILT_IN_PANELS
}
