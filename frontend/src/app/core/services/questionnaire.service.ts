import { Injectable, inject } from '@angular/core';
import {
  Firestore,
  collection,
  addDoc,
  doc,
  updateDoc,
  getDocs,
  query,
  where,
  limit,
  Timestamp
} from '@angular/fire/firestore';
import { Auth } from '@angular/fire/auth';
import { Questionnaire, Question } from '../models/questionnaire.model';
import { Response, Answer } from '../models/response.model';

@Injectable({ providedIn: 'root' })
export class QuestionnaireService {
  private firestore = inject(Firestore);
  private auth = inject(Auth);

  // Sample questionnaire data (in production, this would come from Firestore)
  private sampleQuestions: Question[] = [
    {
      id: 1,
      text: 'Government should play a major role in regulating businesses',
      category: 'politics',
      scaleMinLabel: 'Strongly Disagree',
      scaleMaxLabel: 'Strongly Agree',
      order: 1
    },
    {
      id: 2,
      text: 'Traditional family values are essential to society',
      category: 'values',
      scaleMinLabel: 'Strongly Disagree',
      scaleMaxLabel: 'Strongly Agree',
      order: 2
    },
    {
      id: 3,
      text: 'I prefer spontaneous activities over planned ones',
      category: 'lifestyle',
      scaleMinLabel: 'Strongly Disagree',
      scaleMaxLabel: 'Strongly Agree',
      order: 3
    },
    {
      id: 4,
      text: 'Environmental protection should be prioritized over economic growth',
      category: 'politics',
      scaleMinLabel: 'Strongly Disagree',
      scaleMaxLabel: 'Strongly Agree',
      order: 4
    },
    {
      id: 5,
      text: 'I enjoy trying new and exotic foods',
      category: 'preferences',
      scaleMinLabel: 'Strongly Disagree',
      scaleMaxLabel: 'Strongly Agree',
      order: 5
    },
    {
      id: 6,
      text: 'People should rely more on themselves than on government assistance',
      category: 'politics',
      scaleMinLabel: 'Strongly Disagree',
      scaleMaxLabel: 'Strongly Agree',
      order: 6
    },
    {
      id: 7,
      text: 'Art and culture are essential parts of a good life',
      category: 'values',
      scaleMinLabel: 'Strongly Disagree',
      scaleMaxLabel: 'Strongly Agree',
      order: 7
    },
    {
      id: 8,
      text: 'I prefer city living to rural living',
      category: 'lifestyle',
      scaleMinLabel: 'Strongly Disagree',
      scaleMaxLabel: 'Strongly Agree',
      order: 8
    },
    {
      id: 9,
      text: 'Science and reason should guide most decisions',
      category: 'values',
      scaleMinLabel: 'Strongly Disagree',
      scaleMaxLabel: 'Strongly Agree',
      order: 9
    },
    {
      id: 10,
      text: 'I enjoy quiet evenings at home more than social gatherings',
      category: 'preferences',
      scaleMinLabel: 'Strongly Disagree',
      scaleMaxLabel: 'Strongly Agree',
      order: 10
    }
  ];

  getQuestions(): Question[] {
    return this.sampleQuestions;
  }

  async submitResponse(answers: Answer[]): Promise<void> {
    const user = this.auth.currentUser;
    if (!user) {
      throw new Error('User must be authenticated to submit responses');
    }

    try {
      // Create response document
      const response: Omit<Response, 'id'> = {
        userId: user.uid,
        questionnaireId: 'default-v1',
        questionnaireVersion: 1,
        answers: answers,
        submittedAt: new Date(),
        source: 'web',
        isProcessed: false
      };

      // Add to Firestore
      const responsesRef = collection(this.firestore, 'responses');
      await addDoc(responsesRef, {
        ...response,
        submittedAt: Timestamp.fromDate(response.submittedAt)
      });

      // Update user document to mark questionnaire as completed
      const userRef = doc(this.firestore, `users/${user.uid}`);
      await updateDoc(userRef, {
        hasCompletedQuestionnaire: true,
        updatedAt: Timestamp.now()
      });

    } catch (error) {
      console.error('Error submitting response:', error);
      throw new Error('Failed to submit questionnaire. Please try again.');
    }
  }

  async hasUserCompletedQuestionnaire(userId: string): Promise<boolean> {
    try {
      const responsesRef = collection(this.firestore, 'responses');
      const q = query(
        responsesRef,
        where('userId', '==', userId),
        limit(1)
      );

      const querySnapshot = await getDocs(q);
      return !querySnapshot.empty;
    } catch (error) {
      console.error('Error checking questionnaire status:', error);
      return false;
    }
  }
}
