export interface User {
  uid: string;
  email: string;
  displayName: string;
  photoURL?: string;
  createdAt: Date;
  updatedAt: Date;
  hasCompletedQuestionnaire: boolean;
  isMatched: boolean;
  preferences?: {
    notifications: boolean;
    emailUpdates: boolean;
  };
}
