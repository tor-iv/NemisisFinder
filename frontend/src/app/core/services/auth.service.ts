import { Injectable, inject, signal } from '@angular/core';
import {
  Auth,
  createUserWithEmailAndPassword,
  signInWithEmailAndPassword,
  signOut,
  user,
  User as FirebaseUser,
  updateProfile
} from '@angular/fire/auth';
import { Firestore, doc, setDoc, getDoc } from '@angular/fire/firestore';
import { Router } from '@angular/router';
import { Observable } from 'rxjs';
import { User } from '../models/user.model';

@Injectable({ providedIn: 'root' })
export class AuthService {
  private auth = inject(Auth);
  private firestore = inject(Firestore);
  private router = inject(Router);

  user$ = user(this.auth);
  currentUser = signal<User | null>(null);

  constructor() {
    // Subscribe to auth state changes
    this.user$.subscribe(async (firebaseUser) => {
      if (firebaseUser) {
        const userDoc = await this.getUserDocument(firebaseUser.uid);
        this.currentUser.set(userDoc);
      } else {
        this.currentUser.set(null);
      }
    });
  }

  async register(email: string, password: string, displayName: string): Promise<void> {
    try {
      const credential = await createUserWithEmailAndPassword(
        this.auth,
        email,
        password
      );

      // Update Firebase Auth profile
      await updateProfile(credential.user, { displayName });

      // Create user document in Firestore
      const userRef = doc(this.firestore, `users/${credential.user.uid}`);
      const userData: User = {
        uid: credential.user.uid,
        email: email,
        displayName: displayName,
        createdAt: new Date(),
        updatedAt: new Date(),
        hasCompletedQuestionnaire: false,
        isMatched: false,
        preferences: {
          notifications: true,
          emailUpdates: true
        }
      };

      await setDoc(userRef, userData);

      // Navigate to questionnaire after successful registration
      await this.router.navigate(['/questionnaire']);
    } catch (error: any) {
      console.error('Registration error:', error);
      throw new Error(this.getErrorMessage(error.code));
    }
  }

  async login(email: string, password: string): Promise<void> {
    try {
      await signInWithEmailAndPassword(this.auth, email, password);

      // Check if user has completed questionnaire
      const user = this.currentUser();
      if (user && !user.hasCompletedQuestionnaire) {
        await this.router.navigate(['/questionnaire']);
      } else {
        await this.router.navigate(['/profile']);
      }
    } catch (error: any) {
      console.error('Login error:', error);
      throw new Error(this.getErrorMessage(error.code));
    }
  }

  async logout(): Promise<void> {
    try {
      await signOut(this.auth);
      await this.router.navigate(['/login']);
    } catch (error) {
      console.error('Logout error:', error);
      throw error;
    }
  }

  private async getUserDocument(uid: string): Promise<User | null> {
    try {
      const userRef = doc(this.firestore, `users/${uid}`);
      const userSnap = await getDoc(userRef);

      if (userSnap.exists()) {
        return userSnap.data() as User;
      }
      return null;
    } catch (error) {
      console.error('Error fetching user document:', error);
      return null;
    }
  }

  private getErrorMessage(errorCode: string): string {
    switch (errorCode) {
      case 'auth/email-already-in-use':
        return 'This email is already registered. Please log in instead.';
      case 'auth/invalid-email':
        return 'Invalid email address.';
      case 'auth/operation-not-allowed':
        return 'Email/password accounts are not enabled.';
      case 'auth/weak-password':
        return 'Password is too weak. Please use at least 6 characters.';
      case 'auth/user-not-found':
        return 'No account found with this email.';
      case 'auth/wrong-password':
        return 'Incorrect password.';
      case 'auth/invalid-credential':
        return 'Invalid email or password.';
      default:
        return 'An error occurred. Please try again.';
    }
  }
}
