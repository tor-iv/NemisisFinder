# Quick Start Guide - NemisisFinder Frontend

## 🚀 Before You Start

You **MUST** configure Firebase before running the app, or you'll get errors.

## Step 1: Create Firebase Project

1. Go to [Firebase Console](https://console.firebase.google.com/)
2. Click "Add project" or select existing project
3. Follow the setup wizard

## Step 2: Get Firebase Configuration

1. In Firebase Console, click the gear icon ⚙️ next to "Project Overview"
2. Click "Project settings"
3. Scroll down to "Your apps"
4. Click the web icon `</>` to add a web app
5. Register your app with name "NemisisFinder"
6. Copy the `firebaseConfig` object

It will look like this:
```javascript
const firebaseConfig = {
  apiKey: "AIza...",
  authDomain: "your-app.firebaseapp.com",
  projectId: "your-app",
  storageBucket: "your-app.appspot.com",
  messagingSenderId: "123456789",
  appId: "1:123456789:web:abc123"
};
```

## Step 3: Update Environment Files

### Edit `src/environments/environment.ts`:
```typescript
export const environment = {
  production: false,
  firebase: {
    apiKey: "YOUR_API_KEY_HERE",
    authDomain: "YOUR_PROJECT_ID.firebaseapp.com",
    projectId: "YOUR_PROJECT_ID",
    storageBucket: "YOUR_PROJECT_ID.appspot.com",
    messagingSenderId: "YOUR_MESSAGING_SENDER_ID",
    appId: "YOUR_APP_ID"
  }
};
```

### Edit `src/environments/environment.prod.ts`:
```typescript
export const environment = {
  production: true,
  firebase: {
    // Same config as above
    apiKey: "YOUR_API_KEY_HERE",
    authDomain: "YOUR_PROJECT_ID.firebaseapp.com",
    projectId: "YOUR_PROJECT_ID",
    storageBucket: "YOUR_PROJECT_ID.appspot.com",
    messagingSenderId: "YOUR_MESSAGING_SENDER_ID",
    appId: "YOUR_APP_ID"
  }
};
```

## Step 4: Enable Firebase Authentication

1. In Firebase Console, go to **Build > Authentication**
2. Click "Get started"
3. Click on "Email/Password"
4. **Enable** the toggle for "Email/Password"
5. Click "Save"

## Step 5: Enable Firestore Database

1. In Firebase Console, go to **Build > Firestore Database**
2. Click "Create database"
3. Select **"Start in test mode"** (for development)
4. Choose a location (e.g., us-central)
5. Click "Enable"

## Step 6: Set Firestore Security Rules (Optional but Recommended)

1. In Firestore, click on the **"Rules"** tab
2. Replace the rules with:

```javascript
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    // Users can only read/write their own user document
    match /users/{userId} {
      allow read, write: if request.auth != null && request.auth.uid == userId;
    }

    // Users can only read their own responses
    match /responses/{responseId} {
      allow read: if request.auth != null && resource.data.userId == request.auth.uid;
      allow create: if request.auth != null && request.resource.data.userId == request.auth.uid;
    }

    // Users can read matches they're part of
    match /matches/{matchId} {
      allow read: if request.auth != null && (
        resource.data.user1Id == request.auth.uid ||
        resource.data.user2Id == request.auth.uid
      );
    }

    // Anyone authenticated can read questionnaires
    match /questionnaires/{questionnaireId} {
      allow read: if request.auth != null;
    }
  }
}
```

3. Click "Publish"

## Step 7: Run the Application

Now you're ready to run the app!

```bash
cd frontend
npm start
```

Or using npx:
```bash
npx ng serve
```

The app will be available at: **http://localhost:4200**

## Step 8: Test the Application

1. **Register a new account**:
   - Go to http://localhost:4200
   - Click "Register here"
   - Fill in display name, email, and password
   - Click "Create Account"

2. **Complete the questionnaire**:
   - After registration, you'll be redirected to the questionnaire
   - Answer all 10 questions (1-7 scale)
   - Click "Submit"

3. **View your profile**:
   - After submitting, you'll see a success message
   - Click "Go to Profile"
   - You should see your profile with questionnaire marked as "Completed"

## Troubleshooting

### Error: "Firebase: Error (auth/invalid-api-key)"
- Double-check your Firebase config in `src/environments/environment.ts`
- Make sure you copied the entire config correctly

### Error: "Permission denied" when submitting questionnaire
- Make sure you've enabled Firestore and set up the security rules
- Check that you're logged in (navbar should show "Welcome, [your name]")

### Port 4200 is already in use
```bash
ng serve --port 4201
```

### Can't find 'ng' command
```bash
npm install -g @angular/cli
# Or use npx:
npx ng serve
```

## What's Next?

Now that the frontend is running, you can:

1. **Explore the UI**: Test registration, login, questionnaire, and profile
2. **Check Firebase Console**: See users and responses being created in real-time
3. **Next Phase**: Build the Cloud Functions for the matching algorithm

## Available Commands

```bash
# Start development server
npm start

# Build for production
npm run build

# Run tests
npm test

# Build and watch for changes
npm run watch
```

## Need Help?

- Check [SETUP.md](SETUP.md) for detailed documentation
- Review [Firebase Documentation](https://firebase.google.com/docs)
- Check the [Angular Documentation](https://angular.dev)

---

**Important**: Never commit your Firebase config with real API keys to a public repository! Consider using environment variables for production.
