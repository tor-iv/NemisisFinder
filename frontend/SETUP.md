# NemisisFinder Frontend Setup Guide

This is the Angular frontend for NemisisFinder, built with Angular 17+, Firebase, and Angular Material.

## Prerequisites

- Node.js 18+ and npm
- Firebase account
- Firebase project created

## Setup Instructions

### 1. Install Dependencies

```bash
npm install
```

### 2. Configure Firebase

1. Go to [Firebase Console](https://console.firebase.google.com/)
2. Create a new project or select an existing one
3. Go to Project Settings > General
4. Scroll down to "Your apps" and click "Add app" > Web app
5. Copy your Firebase configuration

6. Update `src/environments/environment.ts` and `src/environments/environment.prod.ts` with your Firebase config:

```typescript
export const environment = {
  production: false, // true for prod
  firebase: {
    apiKey: "YOUR_API_KEY",
    authDomain: "YOUR_PROJECT_ID.firebaseapp.com",
    projectId: "YOUR_PROJECT_ID",
    storageBucket: "YOUR_PROJECT_ID.appspot.com",
    messagingSenderId: "YOUR_MESSAGING_SENDER_ID",
    appId: "YOUR_APP_ID"
  }
};
```

### 3. Enable Firebase Services

In the Firebase Console:

#### Enable Authentication:
1. Go to Build > Authentication
2. Click "Get Started"
3. Enable "Email/Password" sign-in method
4. (Optional) Enable "Google" sign-in method

#### Enable Firestore:
1. Go to Build > Firestore Database
2. Click "Create database"
3. Choose "Start in test mode" for development
4. Select a location
5. Click "Enable"

#### Set up Firestore Security Rules (for development):

In Firestore > Rules tab, use:

```javascript
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    match /users/{userId} {
      allow read, write: if request.auth != null && request.auth.uid == userId;
    }

    match /responses/{responseId} {
      allow read: if request.auth != null && resource.data.userId == request.auth.uid;
      allow create: if request.auth != null && request.resource.data.userId == request.auth.uid;
    }

    match /matches/{matchId} {
      allow read: if request.auth != null && (
        resource.data.user1Id == request.auth.uid ||
        resource.data.user2Id == request.auth.uid
      );
    }

    match /questionnaires/{questionnaireId} {
      allow read: if request.auth != null;
    }
  }
}
```

### 4. Run Development Server

```bash
ng serve
```

Navigate to `http://localhost:4200/`

The application will automatically reload if you change any of the source files.

## Project Structure

```
src/
├── app/
│   ├── core/                 # Core functionality
│   │   ├── guards/          # Route guards (auth.guard.ts)
│   │   ├── models/          # TypeScript interfaces
│   │   └── services/        # Services (auth, questionnaire)
│   │
│   ├── features/            # Feature modules
│   │   ├── auth/           # Login and register components
│   │   ├── questionnaire/  # Questionnaire form
│   │   └── profile/        # User profile
│   │
│   ├── shared/             # Shared components
│   │   └── components/    # Navbar, etc.
│   │
│   ├── app.config.ts      # App configuration (Firebase setup)
│   ├── app.routes.ts      # Routing configuration
│   └── app.ts             # Root component
│
└── environments/          # Environment configurations
```

## Available Features

### Authentication
- Email/password registration
- Email/password login
- Protected routes with auth guards
- Automatic navigation based on auth state

### Questionnaire
- 10 sample questions with 1-7 scale
- Progress tracking
- Form validation
- Submission to Firestore

### User Profile
- View user information
- Check questionnaire completion status
- Check match status

## Build

Build the project for production:

```bash
ng build --configuration production
```

The build artifacts will be stored in the `dist/` directory.

## Deploy to Firebase Hosting (Optional)

1. Install Firebase CLI:
```bash
npm install -g firebase-tools
```

2. Login to Firebase:
```bash
firebase login
```

3. Initialize Firebase in your project:
```bash
firebase init
```
- Select "Hosting"
- Choose your Firebase project
- Set public directory to: `dist/frontend/browser`
- Configure as single-page app: Yes
- Set up automatic builds: No

4. Build and deploy:
```bash
ng build --configuration production
firebase deploy --only hosting
```

## Troubleshooting

### Firebase Configuration Error
If you see "Firebase: Error (auth/invalid-api-key)", double-check your Firebase configuration in `src/environments/environment.ts`.

### Authentication Not Working
Make sure Email/Password authentication is enabled in Firebase Console > Authentication > Sign-in method.

### Firestore Permission Denied
Update your Firestore security rules to allow authenticated users to read/write their own data.

## Next Steps

1. Set up Firebase Cloud Functions for the matching algorithm
2. Create the Rust matching engine
3. Implement real-time match notifications
4. Add more questionnaire questions
5. Implement matches display page

## Additional Commands

### Generate Component
```bash
ng generate component features/your-component
```

### Run Tests
```bash
ng test
```

### Run Linting
```bash
ng lint
```

## Support

For issues or questions, check:
- [Angular Documentation](https://angular.dev)
- [Firebase Documentation](https://firebase.google.com/docs)
- [AngularFire Documentation](https://github.com/angular/angularfire)
