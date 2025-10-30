export interface Snippet {
  id: any;
  record_type: 'snippet';
  source: 'snippets-plugin';
  timestamp: string;
  data: {
    title: string;
    language: string;
    code: string;
    tags: string[];
    createdAt: string;
    updatedAt: string;
  };
  metadata?: {
    tags?: string[];
    status?: string | null;
    title?: string | null;
    description?: string | null;
  };
}

export type SnippetInput = {
  title: string;
  language: string;
  code: string;
  tags: string[];
};

export interface SavedSearch {
  id: any;
  record_type: 'saved_search';
  source: 'snippets-plugin';
  timestamp: string;
  data: {
    query: string;
    createdAt: string;
  };
}

