export interface Response {
  id: string;
  userId: string;
  questionnaireId: string;
  questionnaireVersion: number;
  answers: Answer[];
  submittedAt: Date;
  source: 'web' | 'google_forms';
  isProcessed: boolean;
}

export interface Answer {
  questionId: number;
  value: number;  // 1-7
}
