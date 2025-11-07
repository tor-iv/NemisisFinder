# NemisisFinder - Firebase Implementation Plan

## Executive Summary

This document outlines the complete implementation plan for NemisisFinder using **Firebase** as the backend platform. Firebase dramatically simplifies backend development with managed services for authentication, database, hosting, and serverless functions. Combined with Angular and Rust, this stack provides a modern, scalable, and cost-effective solution.

---

## Why Firebase?

### Advantages
✅ **Zero Backend Code Initially** - Firebase SDK in Angular handles most backend operations
✅ **Real-time Data Sync** - Firestore provides live updates to Angular components
✅ **Built-in Authentication** - Email, Google, Facebook, etc. with minimal code
✅ **Generous Free Tier** - Perfect for MVP and small-scale projects
✅ **Auto-scaling** - Serverless architecture scales automatically
✅ **Integrated Hosting** - Deploy Angular app to Firebase Hosting instantly
✅ **Google Integration** - Seamless with Google Sheets and Forms
✅ **Security Rules** - Declarative database security

### Considerations
⚠️ **Vendor Lock-in** - Tied to Google Cloud ecosystem
⚠️ **Cost at Scale** - Can become expensive with heavy usage
⚠️ **Complex Queries** - Firestore has limitations vs. SQL databases
⚠️ **Cold Starts** - Cloud Functions may have latency on first invocation

---

## Technology Stack

### Frontend
- **Framework**: Angular 17+
- **Firebase SDK**: @angular/fire (official Angular integration)
- **UI**: Angular Material
- **State**: Firebase real-time listeners + Angular Signals
- **Testing**: Jasmine, Karma, Cypress

### Backend (Firebase)
- **Authentication**: Firebase Authentication
- **Database**: Cloud Firestore (NoSQL document database)
- **Functions**: Cloud Functions for Firebase (Node.js/TypeScript)
- **Hosting**: Firebase Hosting (CDN-backed static hosting)
- **Storage**: Cloud Storage (if needed for user photos)
- **Analytics**: Google Analytics for Firebase

### Matching Engine
- **Language**: Rust
- **Compilation Target**: WebAssembly (WASM) via wasm-pack
- **Integration**: Node.js bindings for Cloud Functions
- **Alternative**: Direct WASM in browser for small datasets

### Development Tools
- **Firebase CLI**: Project management and deployment
- **Emulator Suite**: Local development environment
- **TypeScript**: Cloud Functions and Angular
- **Firebase Extensions**: Pre-built integrations

---

## Architecture Overview

### System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Firebase Hosting                       │
│           Angular SPA (Static Files + CDN)               │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Components                                       │  │
│  │  - Auth (Login/Register)                         │  │
│  │  - Questionnaire Form                            │  │
│  │  - Match Display                                 │  │
│  │  - User Profile                                  │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                         │
                         ▼ Firebase SDK (Real-time)
┌─────────────────────────────────────────────────────────┐
│                  Firebase Services                       │
│                                                          │
│  ┌────────────────┐  ┌──────────────────────────────┐  │
│  │ Authentication │  │   Cloud Firestore            │  │
│  │ - Email/Pass   │  │   Collections:               │  │
│  │ - Google OAuth │  │   - users                    │  │
│  │ - JWT Tokens   │  │   - questionnaires           │  │
│  └────────────────┘  │   - responses                │  │
│                      │   - matches                  │  │
│  ┌────────────────┐  └──────────────────────────────┘  │
│  │ Cloud Functions│                                     │
│  │ (Node.js TS)   │                                     │
│  │                │                                     │
│  │ 1. importSheets│  ┌───────────────────────┐        │
│  │ 2. matchUsers  │──│ Rust WASM Engine      │        │
│  │ 3. onNewResponse│ │ - Calculate diffs     │        │
│  │ 4. sendNotifs  │  │ - Find best pairs     │        │
│  └────────────────┘  └───────────────────────┘        │
│                                                          │
└─────────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│              External Services                           │
│  - Google Forms (Questionnaire)                         │
│  - Google Sheets (Data import)                          │
└─────────────────────────────────────────────────────────┘
```

### Data Flow Sequence

#### 1. User Registration & Authentication
```
User → Angular Auth Component
      ↓
Firebase Authentication (createUserWithEmailAndPassword)
      ↓
Cloud Function: onCreate trigger
      ↓
Create user document in Firestore /users/{uid}
      ↓
Angular receives auth state change
      ↓
Navigate to questionnaire
```

#### 2. Questionnaire Submission
```
User fills form in Angular
      ↓
Submit → Angular Service
      ↓
Firestore.collection('responses').add({...})
      ↓
Cloud Function: onResponseCreate trigger
      ↓
Check if matching threshold reached
      ↓
If yes → Trigger matchUsers function
```

#### 3. Matching Process
```
Cloud Function: matchUsers (scheduled or triggered)
      ↓
Query all unmatched responses from Firestore
      ↓
Load Rust WASM module
      ↓
Pass data to Rust algorithm
      ↓
Rust calculates all pairwise differences
      ↓
Returns optimal opposite pairs
      ↓
Cloud Function writes to Firestore /matches
      ↓
Update user documents (matched: true)
      ↓
Send push notifications (FCM)
```

#### 4. Viewing Matches
```
Angular component subscribes to Firestore
      ↓
Firestore.collection('matches')
  .where('userId', '==', currentUser.uid)
  .onSnapshot(...)
      ↓
Real-time updates when matches are created
      ↓
Angular displays match profile
```

---

## Project Structure

```
NemisisFinder/
├── .firebaserc                 # Firebase project configuration
├── firebase.json               # Firebase services config
├── firestore.rules            # Firestore security rules
├── firestore.indexes.json     # Database indexes
├── storage.rules              # Cloud Storage rules (if used)
│
├── public/                     # Firebase Hosting (Angular dist output)
│   └── (generated by ng build)
│
├── functions/                  # Cloud Functions
│   ├── src/
│   │   ├── index.ts           # Function exports
│   │   ├── matching.ts        # Matching logic & Rust integration
│   │   ├── imports.ts         # Google Sheets import
│   │   ├── notifications.ts   # FCM notifications
│   │   └── utils/
│   │       ├── firestore.ts   # Database helpers
│   │       └── validation.ts  # Input validation
│   ├── lib/                   # Compiled JS (gitignored)
│   ├── node_modules/
│   ├── package.json
│   ├── tsconfig.json
│   └── .eslintrc.js
│
├── matching-engine/            # Rust WASM module
│   ├── src/
│   │   ├── lib.rs             # WASM entry point
│   │   ├── algorithm.rs       # Matching algorithm
│   │   └── types.rs           # Data structures
│   ├── pkg/                   # WASM output (gitignored)
│   ├── Cargo.toml
│   └── Cargo.lock
│
├── src/                        # Angular application
│   ├── app/
│   │   ├── core/
│   │   │   ├── services/
│   │   │   │   ├── auth.service.ts
│   │   │   │   ├── firestore.service.ts
│   │   │   │   ├── matching.service.ts
│   │   │   │   └── questionnaire.service.ts
│   │   │   ├── guards/
│   │   │   │   ├── auth.guard.ts
│   │   │   │   └── questionnaire.guard.ts
│   │   │   ├── interceptors/
│   │   │   └── models/
│   │   │       ├── user.model.ts
│   │   │       ├── questionnaire.model.ts
│   │   │       ├── response.model.ts
│   │   │       └── match.model.ts
│   │   │
│   │   ├── shared/
│   │   │   ├── components/
│   │   │   │   ├── navbar/
│   │   │   │   ├── footer/
│   │   │   │   └── loading-spinner/
│   │   │   └── pipes/
│   │   │
│   │   ├── features/
│   │   │   ├── auth/
│   │   │   │   ├── login/
│   │   │   │   ├── register/
│   │   │   │   └── auth.module.ts
│   │   │   │
│   │   │   ├── questionnaire/
│   │   │   │   ├── questionnaire-form/
│   │   │   │   ├── question-item/
│   │   │   │   └── questionnaire.module.ts
│   │   │   │
│   │   │   ├── matches/
│   │   │   │   ├── match-list/
│   │   │   │   ├── match-detail/
│   │   │   │   └── matches.module.ts
│   │   │   │
│   │   │   └── profile/
│   │   │       ├── profile-view/
│   │   │       ├── profile-edit/
│   │   │       └── profile.module.ts
│   │   │
│   │   ├── app.component.ts
│   │   ├── app.config.ts
│   │   └── app.routes.ts
│   │
│   ├── environments/
│   │   ├── environment.ts         # Development
│   │   └── environment.prod.ts    # Production
│   │
│   ├── assets/
│   ├── styles.scss
│   └── index.html
│
├── angular.json
├── package.json
├── tsconfig.json
├── .gitignore
├── README.md
├── CLAUDE.md
└── FIREBASE_IMPLEMENTATION_PLAN.md
```

---

## Firestore Database Schema

### Security Rules Approach
Firebase uses declarative security rules to protect data. Users can only access their own data, and Cloud Functions use admin SDK with elevated permissions.

### Collections

#### `/users/{userId}`
```typescript
interface User {
  uid: string;                    // Firebase Auth UID
  email: string;
  displayName: string;
  photoURL?: string;
  createdAt: Timestamp;
  updatedAt: Timestamp;
  hasCompletedQuestionnaire: boolean;
  isMatched: boolean;
  preferences?: {
    notifications: boolean;
    emailUpdates: boolean;
  };
}
```

**Security Rule:**
```javascript
match /users/{userId} {
  allow read: if request.auth != null && request.auth.uid == userId;
  allow create: if request.auth != null && request.auth.uid == userId;
  allow update: if request.auth != null && request.auth.uid == userId;
  allow delete: if false;  // No deletes via client
}
```

---

#### `/questionnaires/{questionnaireId}`
```typescript
interface Questionnaire {
  id: string;
  title: string;
  description: string;
  version: number;
  isActive: boolean;
  createdAt: Timestamp;
  questions: Question[];
}

interface Question {
  id: number;
  text: string;
  category: 'politics' | 'lifestyle' | 'values' | 'preferences';
  scaleMinLabel: string;  // e.g., "Strongly Disagree"
  scaleMaxLabel: string;  // e.g., "Strongly Agree"
  order: number;
}
```

**Security Rule:**
```javascript
match /questionnaires/{questionnaireId} {
  allow read: if request.auth != null;
  allow write: if false;  // Only via Cloud Functions (admin)
}
```

---

#### `/responses/{responseId}`
```typescript
interface Response {
  id: string;
  userId: string;           // Reference to user
  questionnaireId: string;  // Reference to questionnaire
  questionnaireVersion: number;
  answers: Answer[];
  submittedAt: Timestamp;
  source: 'web' | 'google_forms';
  isProcessed: boolean;     // Has been included in matching
}

interface Answer {
  questionId: number;
  value: number;  // 1-7
}
```

**Security Rule:**
```javascript
match /responses/{responseId} {
  allow read: if request.auth != null &&
              resource.data.userId == request.auth.uid;
  allow create: if request.auth != null &&
                request.resource.data.userId == request.auth.uid;
  allow update, delete: if false;
}
```

**Composite Index:**
```json
{
  "collectionGroup": "responses",
  "queryScope": "COLLECTION",
  "fields": [
    { "fieldPath": "userId", "order": "ASCENDING" },
    { "fieldPath": "submittedAt", "order": "DESCENDING" }
  ]
}
```

---

#### `/matches/{matchId}`
```typescript
interface Match {
  id: string;
  user1Id: string;
  user2Id: string;
  differenceScore: number;        // Total absolute difference
  questionDifferences: QuestionDiff[];
  questionnaireVersion: number;
  matchedAt: Timestamp;
  status: 'new' | 'viewed' | 'contacted' | 'archived';

  // Denormalized user data for quick display
  user1: {
    displayName: string;
    photoURL?: string;
  };
  user2: {
    displayName: string;
    photoURL?: string;
  };
}

interface QuestionDiff {
  questionId: number;
  user1Answer: number;
  user2Answer: number;
  difference: number;
}
```

**Security Rule:**
```javascript
match /matches/{matchId} {
  allow read: if request.auth != null && (
    resource.data.user1Id == request.auth.uid ||
    resource.data.user2Id == request.auth.uid
  );
  allow create, update, delete: if false;  // Only Cloud Functions
}
```

**Composite Indexes:**
```json
[
  {
    "collectionGroup": "matches",
    "fields": [
      { "fieldPath": "user1Id", "order": "ASCENDING" },
      { "fieldPath": "matchedAt", "order": "DESCENDING" }
    ]
  },
  {
    "collectionGroup": "matches",
    "fields": [
      { "fieldPath": "user2Id", "order": "ASCENDING" },
      { "fieldPath": "matchedAt", "order": "DESCENDING" }
    ]
  }
]
```

---

## Rust Matching Algorithm (WebAssembly)

### Cargo.toml
```toml
[package]
name = "nemisis-matching"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
js-sys = "0.3"

[profile.release]
opt-level = 3
lto = true
```

### src/lib.rs
```rust
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub user_id: String,
    pub answers: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct MatchResult {
    pub user1_id: String,
    pub user2_id: String,
    pub total_difference: i32,
    pub question_differences: Vec<i32>,
}

#[wasm_bindgen]
pub fn find_opposite_matches(users_json: &str) -> Result<String, JsValue> {
    let users: Vec<UserResponse> = serde_json::from_str(users_json)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let matches = find_matches_internal(users);

    serde_json::to_string(&matches)
        .map_err(|e| JsValue::from_str(&format!("Serialize error: {}", e)))
}

fn find_matches_internal(users: Vec<UserResponse>) -> Vec<MatchResult> {
    use std::collections::HashSet;

    let mut all_pairs: Vec<_> = Vec::new();

    // Calculate all pairwise differences
    for i in 0..users.len() {
        for j in (i + 1)..users.len() {
            let diff = calculate_difference(&users[i], &users[j]);
            all_pairs.push((i, j, diff));
        }
    }

    // Sort by difference descending (highest first)
    all_pairs.sort_by(|a, b| b.2.cmp(&a.2));

    let mut matched_indices = HashSet::new();
    let mut results = Vec::new();

    // Greedy matching: pair users with highest differences
    for (i, j, (total_diff, question_diffs)) in all_pairs {
        if !matched_indices.contains(&i) && !matched_indices.contains(&j) {
            results.push(MatchResult {
                user1_id: users[i].user_id.clone(),
                user2_id: users[j].user_id.clone(),
                total_difference: total_diff,
                question_differences: question_diffs,
            });

            matched_indices.insert(i);
            matched_indices.insert(j);
        }
    }

    results
}

fn calculate_difference(
    user1: &UserResponse,
    user2: &UserResponse,
) -> (i32, Vec<i32>) {
    let diffs: Vec<i32> = user1
        .answers
        .iter()
        .zip(user2.answers.iter())
        .map(|(a1, a2)| (a1 - a2).abs())
        .collect();

    let total: i32 = diffs.iter().sum();
    (total, diffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_difference() {
        let user1 = UserResponse {
            user_id: "1".to_string(),
            answers: vec![1, 2, 3],
        };
        let user2 = UserResponse {
            user_id: "2".to_string(),
            answers: vec![7, 6, 5],
        };

        let (total, diffs) = calculate_difference(&user1, &user2);
        assert_eq!(total, 12); // 6 + 4 + 2
        assert_eq!(diffs, vec![6, 4, 2]);
    }

    #[test]
    fn test_matching_algorithm() {
        let users = vec![
            UserResponse {
                user_id: "1".to_string(),
                answers: vec![1, 1, 1],
            },
            UserResponse {
                user_id: "2".to_string(),
                answers: vec![7, 7, 7],
            },
            UserResponse {
                user_id: "3".to_string(),
                answers: vec![4, 4, 4],
            },
        ];

        let matches = find_matches_internal(users);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].total_difference, 18); // (6*3)
        assert!(
            (matches[0].user1_id == "1" && matches[0].user2_id == "2") ||
            (matches[0].user1_id == "2" && matches[0].user2_id == "1")
        );
    }
}
```

### Build Command
```bash
wasm-pack build --target nodejs --out-dir functions/wasm
```

---

## Cloud Functions

### functions/src/index.ts
```typescript
import * as functions from 'firebase-functions';
import * as admin from 'firebase-admin';

admin.initializeApp();

export { onResponseCreate } from './matching';
export { matchUsers } from './matching';
export { importFromGoogleSheets } from './imports';
export { sendMatchNotification } from './notifications';
```

### functions/src/matching.ts
```typescript
import * as functions from 'firebase-functions';
import * as admin from 'firebase-admin';
import * as fs from 'fs';
import * as path from 'path';

// Load Rust WASM module
const wasmPath = path.join(__dirname, '../wasm/nemisis_matching_bg.wasm');
const wasmModule = require('../wasm/nemisis_matching');

interface UserResponse {
  user_id: string;
  answers: number[];
}

interface MatchResult {
  user1_id: string;
  user2_id: string;
  total_difference: number;
  question_differences: number[];
}

/**
 * Triggered when a new response is created
 * Checks if we have enough unmatched users to run matching
 */
export const onResponseCreate = functions.firestore
  .document('responses/{responseId}')
  .onCreate(async (snap, context) => {
    const firestore = admin.firestore();

    // Count unmatched users
    const unmatchedCount = await firestore
      .collection('users')
      .where('isMatched', '==', false)
      .where('hasCompletedQuestionnaire', '==', true)
      .count()
      .get();

    // If we have enough users (e.g., 10+), trigger matching
    if (unmatchedCount.data().count >= 10) {
      console.log('Threshold reached. Triggering matching...');
      // Trigger matchUsers function
      // In production, you'd use Pub/Sub or Task Queue
      await matchUsers();
    }

    return null;
  });

/**
 * Main matching function - can be called manually or scheduled
 */
export const matchUsers = functions.https.onCall(async (data, context) => {
  const firestore = admin.firestore();

  try {
    // 1. Get all unmatched users with responses
    const unmatchedUsers = await firestore
      .collection('users')
      .where('isMatched', '==', false)
      .where('hasCompletedQuestionnaire', '==', true)
      .get();

    if (unmatchedUsers.empty) {
      return { success: false, message: 'No unmatched users' };
    }

    // 2. Get their responses
    const userResponses: UserResponse[] = [];

    for (const userDoc of unmatchedUsers.docs) {
      const responseDocs = await firestore
        .collection('responses')
        .where('userId', '==', userDoc.id)
        .where('isProcessed', '==', false)
        .limit(1)
        .get();

      if (!responseDocs.empty) {
        const response = responseDocs.docs[0].data();
        userResponses.push({
          user_id: userDoc.id,
          answers: response.answers.map((a: any) => a.value),
        });
      }
    }

    if (userResponses.length < 2) {
      return { success: false, message: 'Not enough users to match' };
    }

    // 3. Call Rust WASM matching algorithm
    console.log(`Matching ${userResponses.length} users...`);
    const matchesJson = wasmModule.find_opposite_matches(
      JSON.stringify(userResponses)
    );
    const matches: MatchResult[] = JSON.parse(matchesJson);

    // 4. Store matches in Firestore
    const batch = firestore.batch();

    for (const match of matches) {
      // Get user details for denormalization
      const user1Doc = await firestore.collection('users').doc(match.user1_id).get();
      const user2Doc = await firestore.collection('users').doc(match.user2_id).get();

      const user1 = user1Doc.data()!;
      const user2 = user2Doc.data()!;

      // Create match document
      const matchRef = firestore.collection('matches').doc();
      batch.set(matchRef, {
        user1Id: match.user1_id,
        user2Id: match.user2_id,
        differenceScore: match.total_difference,
        questionDifferences: match.question_differences,
        matchedAt: admin.firestore.FieldValue.serverTimestamp(),
        status: 'new',
        user1: {
          displayName: user1.displayName,
          photoURL: user1.photoURL || null,
        },
        user2: {
          displayName: user2.displayName,
          photoURL: user2.photoURL || null,
        },
      });

      // Update users as matched
      batch.update(firestore.collection('users').doc(match.user1_id), {
        isMatched: true,
        updatedAt: admin.firestore.FieldValue.serverTimestamp(),
      });
      batch.update(firestore.collection('users').doc(match.user2_id), {
        isMatched: true,
        updatedAt: admin.firestore.FieldValue.serverTimestamp(),
      });

      // Mark responses as processed
      const resp1 = await firestore
        .collection('responses')
        .where('userId', '==', match.user1_id)
        .limit(1)
        .get();
      const resp2 = await firestore
        .collection('responses')
        .where('userId', '==', match.user2_id)
        .limit(1)
        .get();

      if (!resp1.empty) batch.update(resp1.docs[0].ref, { isProcessed: true });
      if (!resp2.empty) batch.update(resp2.docs[0].ref, { isProcessed: true });
    }

    await batch.commit();

    console.log(`Created ${matches.length} matches`);
    return { success: true, matchCount: matches.length };

  } catch (error) {
    console.error('Matching error:', error);
    throw new functions.https.HttpsError('internal', 'Matching failed');
  }
});
```

---

## Angular Implementation

### src/app/core/services/auth.service.ts
```typescript
import { Injectable, inject } from '@angular/core';
import { Auth, createUserWithEmailAndPassword, signInWithEmailAndPassword,
         signOut, user, User } from '@angular/fire/auth';
import { Firestore, doc, setDoc } from '@angular/fire/firestore';
import { Observable } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class AuthService {
  private auth = inject(Auth);
  private firestore = inject(Firestore);

  user$: Observable<User | null> = user(this.auth);

  async register(email: string, password: string, displayName: string) {
    const credential = await createUserWithEmailAndPassword(
      this.auth,
      email,
      password
    );

    // Create user document in Firestore
    const userRef = doc(this.firestore, `users/${credential.user.uid}`);
    await setDoc(userRef, {
      uid: credential.user.uid,
      email,
      displayName,
      createdAt: new Date(),
      hasCompletedQuestionnaire: false,
      isMatched: false,
    });

    return credential.user;
  }

  async login(email: string, password: string) {
    return signInWithEmailAndPassword(this.auth, email, password);
  }

  async logout() {
    return signOut(this.auth);
  }
}
```

### src/app/core/services/questionnaire.service.ts
```typescript
import { Injectable, inject } from '@angular/core';
import { Firestore, collection, addDoc, query, where,
         getDocs, doc, updateDoc } from '@angular/fire/firestore';
import { Auth } from '@angular/fire/auth';
import { Response, Answer } from '../models/response.model';

@Injectable({ providedIn: 'root' })
export class QuestionnaireService {
  private firestore = inject(Firestore);
  private auth = inject(Auth);

  async submitResponses(answers: Answer[]) {
    const user = this.auth.currentUser;
    if (!user) throw new Error('Not authenticated');

    const response: Partial<Response> = {
      userId: user.uid,
      questionnaireId: 'default-v1', // Get from active questionnaire
      questionnaireVersion: 1,
      answers,
      submittedAt: new Date(),
      source: 'web',
      isProcessed: false,
    };

    const responsesRef = collection(this.firestore, 'responses');
    await addDoc(responsesRef, response);

    // Update user document
    const userRef = doc(this.firestore, `users/${user.uid}`);
    await updateDoc(userRef, {
      hasCompletedQuestionnaire: true,
      updatedAt: new Date(),
    });
  }
}
```

### src/app/core/services/matching.service.ts
```typescript
import { Injectable, inject } from '@angular/core';
import { Firestore, collection, query, where, onSnapshot } from '@angular/fire/firestore';
import { Auth } from '@angular/fire/auth';
import { Observable } from 'rxjs';
import { Match } from '../models/match.model';

@Injectable({ providedIn: 'root' })
export class MatchingService {
  private firestore = inject(Firestore);
  private auth = inject(Auth);

  getMyMatches(): Observable<Match[]> {
    return new Observable((observer) => {
      const user = this.auth.currentUser;
      if (!user) {
        observer.error('Not authenticated');
        return;
      }

      const matchesRef = collection(this.firestore, 'matches');
      const q = query(
        matchesRef,
        where('user1Id', '==', user.uid)
      );

      // Real-time listener
      const unsubscribe = onSnapshot(q, (snapshot) => {
        const matches = snapshot.docs.map(doc => ({
          id: doc.id,
          ...doc.data()
        })) as Match[];

        observer.next(matches);
      }, (error) => {
        observer.error(error);
      });

      return () => unsubscribe();
    });
  }
}
```

---

## Hosting & Deployment

### Firebase Hosting (Recommended)

Firebase Hosting is **included free** with Firebase and perfect for Angular SPAs.

#### Features:
- Global CDN
- Automatic SSL certificates
- Custom domains
- Rollback support
- Preview channels for PRs

#### Setup:
```bash
# Initialize Firebase Hosting
firebase init hosting

# Build Angular
ng build --configuration production

# Deploy
firebase deploy --only hosting
```

#### firebase.json
```json
{
  "hosting": {
    "public": "dist/nemisis-finder/browser",
    "ignore": ["firebase.json", "**/.*", "**/node_modules/**"],
    "rewrites": [
      {
        "source": "**",
        "destination": "/index.html"
      }
    ],
    "headers": [
      {
        "source": "**/*.@(jpg|jpeg|gif|png|svg|webp)",
        "headers": [
          {
            "key": "Cache-Control",
            "value": "max-age=31536000"
          }
        ]
      }
    ]
  },
  "firestore": {
    "rules": "firestore.rules",
    "indexes": "firestore.indexes.json"
  },
  "functions": {
    "source": "functions",
    "runtime": "nodejs18"
  }
}
```

---

## Cost Analysis

### Firebase Free Tier (Spark Plan)

| Service | Free Tier | Typical MVP Usage |
|---------|-----------|-------------------|
| **Firestore** | 1 GB storage, 50K reads/day, 20K writes/day | ✅ Sufficient for 100-500 users |
| **Authentication** | Unlimited | ✅ Free forever |
| **Hosting** | 10 GB storage, 360 MB/day transfer | ✅ Good for thousands of visits |
| **Cloud Functions** | 2M invocations/month | ✅ Enough for MVP |
| **Cloud Storage** | 5 GB storage, 1 GB/day download | ✅ If using profile photos |

**MVP Cost: $0/month** 🎉

### Firebase Blaze Plan (Pay-as-you-go)

Required when you exceed free tier or need:
- Outbound networking from Cloud Functions
- More than 2M function invocations
- External API calls

#### Estimated Costs (1000 active users):

| Service | Usage | Cost |
|---------|-------|------|
| **Firestore** | 500K reads, 100K writes/day | ~$15/month |
| **Cloud Functions** | 500K invocations/month | ~$5/month |
| **Hosting** | 20 GB transfer/month | ~$1.50/month |
| **Total** | | **~$20-25/month** |

#### Estimated Costs (10,000 active users):

| Service | Usage | Cost |
|---------|-------|------|
| **Firestore** | 5M reads, 1M writes/day | ~$150/month |
| **Cloud Functions** | 5M invocations/month | ~$25/month |
| **Hosting** | 200 GB transfer/month | ~$15/month |
| **Total** | | **~$190/month** |

### Cost Optimization Tips:

1. **Use Firestore efficiently**:
   - Minimize read operations
   - Use `onSnapshot` wisely (counts as 1 read per document change)
   - Denormalize data to reduce queries

2. **Optimize Cloud Functions**:
   - Use larger instance sizes (2GB memory) for better performance
   - Implement caching
   - Batch operations

3. **CDN Caching**:
   - Set long cache headers for static assets
   - Use Firebase Hosting cache

---

## Implementation Timeline

### Phase 1: Foundation (Week 1)
- [ ] Create Firebase project
- [ ] Initialize Angular app with @angular/fire
- [ ] Set up Firebase Emulators for local development
- [ ] Configure Firestore security rules (basic)
- [ ] Create initial Firestore collections
- [ ] Set up CI/CD (GitHub Actions → Firebase)

### Phase 2: Authentication (Week 2)
- [ ] Implement Firebase Authentication
- [ ] Create login/register components
- [ ] Add auth guards and routing
- [ ] User profile creation flow

### Phase 3: Questionnaire (Week 3)
- [ ] Design questionnaire UI (Angular Material)
- [ ] Create questionnaire form component
- [ ] Implement response submission to Firestore
- [ ] Add validation and error handling

### Phase 4: Rust Matching Engine (Week 4)
- [ ] Implement matching algorithm in Rust
- [ ] Write comprehensive tests
- [ ] Compile to WebAssembly
- [ ] Create Node.js bindings for Cloud Functions

### Phase 5: Cloud Functions (Week 5)
- [ ] Set up Cloud Functions project structure
- [ ] Implement matchUsers function
- [ ] Integrate Rust WASM module
- [ ] Add Google Sheets import function
- [ ] Deploy and test functions

### Phase 6: Match Display (Week 6)
- [ ] Create match list component
- [ ] Implement real-time Firestore listeners
- [ ] Design match detail view
- [ ] Add match status updates

### Phase 7: Testing & Polish (Week 7)
- [ ] Unit tests (Angular + Cloud Functions)
- [ ] E2E tests with Cypress
- [ ] Performance optimization
- [ ] UI/UX improvements
- [ ] Security audit of Firestore rules

### Phase 8: Launch (Week 8)
- [ ] Deploy to production Firebase project
- [ ] Set up custom domain
- [ ] Configure monitoring and alerts
- [ ] Soft launch with test users
- [ ] Gather feedback and iterate

**Total Timeline: 8 weeks to MVP**

---

## Security Best Practices

### Firestore Security Rules

```javascript
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {

    // Helper functions
    function isAuthenticated() {
      return request.auth != null;
    }

    function isOwner(userId) {
      return request.auth.uid == userId;
    }

    // Users collection
    match /users/{userId} {
      allow read: if isAuthenticated() && isOwner(userId);
      allow create: if isAuthenticated() && isOwner(userId)
                    && request.resource.data.uid == userId;
      allow update: if isAuthenticated() && isOwner(userId)
                    && request.resource.data.uid == userId;
    }

    // Responses collection
    match /responses/{responseId} {
      allow read: if isAuthenticated()
                  && resource.data.userId == request.auth.uid;
      allow create: if isAuthenticated()
                    && request.resource.data.userId == request.auth.uid
                    && request.resource.data.answers.size() > 0;
      allow update, delete: if false;
    }

    // Matches collection
    match /matches/{matchId} {
      allow read: if isAuthenticated() && (
        resource.data.user1Id == request.auth.uid ||
        resource.data.user2Id == request.auth.uid
      );
      allow write: if false;  // Only via Cloud Functions
    }

    // Questionnaires (read-only for users)
    match /questionnaires/{questionnaireId} {
      allow read: if isAuthenticated();
      allow write: if false;
    }
  }
}
```

### Authentication Security
- Enable email verification
- Implement rate limiting
- Use Firebase App Check to prevent abuse
- Set up password complexity requirements

### Cloud Functions Security
- Use HTTPS callable functions for sensitive operations
- Validate all inputs
- Use Firebase Admin SDK with proper permissions
- Implement request authentication checks

---

## Monitoring & Analytics

### Firebase Analytics
```typescript
// In Angular app
import { Analytics, logEvent } from '@angular/fire/analytics';

export class QuestionnaireComponent {
  analytics = inject(Analytics);

  submitQuestionnaire() {
    logEvent(this.analytics, 'questionnaire_completed', {
      question_count: this.questions.length,
    });
  }
}
```

### Cloud Function Logging
```typescript
import * as logger from 'firebase-functions/logger';

export const matchUsers = functions.https.onCall(async (data, context) => {
  logger.info('Match function started', { userCount: data.userCount });

  try {
    // ... matching logic
    logger.info('Matching completed successfully', { matches: results.length });
  } catch (error) {
    logger.error('Matching failed', { error });
    throw error;
  }
});
```

### Performance Monitoring
- Enable Firebase Performance Monitoring in Angular
- Track page load times
- Monitor API response times
- Set up custom traces for critical paths

---

## Future Enhancements

### Short-term (3 months)
- Push notifications via Firebase Cloud Messaging
- In-app messaging between matches
- User preferences and filters
- Email notifications via SendGrid extension

### Medium-term (6 months)
- Progressive Web App (PWA) features
- Offline support with Firestore persistence
- Advanced analytics dashboard
- A/B testing for questionnaire optimization

### Long-term (12+ months)
- Mobile apps (Flutter + Firebase)
- Machine learning for improved matching (Firebase ML)
- Video profiles (Cloud Storage)
- Premium features with Stripe extension

---

## Summary

The Firebase stack provides:

✅ **Fastest Time to Market**: No backend code needed initially
✅ **Real-time Updates**: Firestore live synchronization
✅ **Scalability**: Auto-scaling serverless architecture
✅ **Cost-Effective**: Free tier covers MVP, pay-as-you-grow
✅ **Developer Experience**: Excellent tooling and documentation
✅ **Security**: Built-in authentication and declarative rules
✅ **Performance**: Rust for compute-heavy matching, WASM for efficiency

### Recommended Next Steps:

1. **Create Firebase project** at console.firebase.google.com
2. **Initialize Angular app** with `ng new nemisis-finder`
3. **Add Firebase** with `ng add @angular/fire`
4. **Set up emulators** for local development
5. **Build Rust engine** and compile to WASM
6. **Deploy MVP** to Firebase Hosting (free)

**Expected Timeline: 6-8 weeks to production-ready MVP**

---

## Support Resources

- **Firebase Docs**: https://firebase.google.com/docs
- **AngularFire**: https://github.com/angular/angularfire
- **Rust WASM**: https://rustwasm.github.io/docs/wasm-pack/
- **Firebase Community**: https://firebase.google.com/community

---

**Questions? Check CLAUDE.md for architecture overview or README.md for project vision.**
