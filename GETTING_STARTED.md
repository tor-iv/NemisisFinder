# Getting Started with NemisisFinder

## 🎯 What You Have Now

A fully functional Angular frontend application with:
- ✅ User authentication (login/register)
- ✅ Firebase integration
- ✅ 10-question questionnaire
- ✅ User profile management
- ✅ Beautiful, responsive UI
- ✅ Protected routes
- ✅ Real-time database sync

## 📋 What You Need to Do Next

### STEP 1: Set Up Firebase (Required to Run)

**You cannot run the app without this step!**

1. Go to https://console.firebase.google.com/
2. Create a new project (or use existing)
3. Get your Firebase configuration
4. Enable Authentication (Email/Password)
5. Enable Firestore Database

**👉 Follow the detailed guide:** [frontend/QUICKSTART.md](frontend/QUICKSTART.md)

### STEP 2: Configure Your App

Edit these two files with your Firebase config:

**File 1:** `frontend/src/environments/environment.ts`
**File 2:** `frontend/src/environments/environment.prod.ts`

```typescript
export const environment = {
  production: false, // change to true in prod file
  firebase: {
    apiKey: "YOUR_API_KEY",              // ← Replace with your values
    authDomain: "YOUR_PROJECT.firebaseapp.com",
    projectId: "YOUR_PROJECT_ID",
    storageBucket: "YOUR_PROJECT.appspot.com",
    messagingSenderId: "YOUR_SENDER_ID",
    appId: "YOUR_APP_ID"
  }
};
```

### STEP 3: Run the App

```bash
cd frontend
npm start
```

Open http://localhost:4200 in your browser!

## 🧪 Testing the App

1. **Register a new account**
   - Click "Register here"
   - Enter: Display name, email, password
   - You'll be auto-logged in

2. **Complete the questionnaire**
   - Answer all 10 questions (1-7 scale)
   - Click through with Next/Previous
   - Submit when done

3. **View your profile**
   - See your info and questionnaire status
   - Check Firebase Console to see your data in Firestore

## 📁 What's Where

```
NemisisFinder/
├── frontend/                    # ← Your Angular app
│   ├── src/
│   │   ├── app/
│   │   │   ├── features/
│   │   │   │   ├── auth/          # Login & Register
│   │   │   │   ├── questionnaire/ # The quiz
│   │   │   │   └── profile/       # User profile
│   │   │   ├── core/              # Services & logic
│   │   │   └── shared/            # Navbar, etc.
│   │   └── environments/          # ← UPDATE THESE FILES!
│   │       ├── environment.ts     # Development config
│   │       └── environment.prod.ts # Production config
│   └── QUICKSTART.md              # ← START HERE for Firebase setup
│
├── README.md                       # Project overview
├── FIREBASE_IMPLEMENTATION_PLAN.md # Full technical plan
└── CLAUDE.md                       # Guide for AI assistants
```

## 🔧 Common Commands

```bash
# Start development server
cd frontend
npm start

# Build for production
npm run build

# Install dependencies (if needed)
npm install

# Generate new component
ng generate component my-component
```

## ❌ Troubleshooting

### "ng: command not found"
**Solution:** The Angular CLI is now installed globally. Try:
```bash
ng version
```
If it still doesn't work:
```bash
npm install -g @angular/cli
```

### "Firebase: Error (auth/invalid-api-key)"
**Solution:** You haven't configured Firebase yet!
1. Go through STEP 1 and STEP 2 above
2. Make sure you copied your Firebase config correctly

### "Permission denied" in Firestore
**Solution:** Set up Firestore security rules (see [frontend/QUICKSTART.md](frontend/QUICKSTART.md))

### App won't start or shows errors
**Solution:**
```bash
cd frontend
rm -rf node_modules package-lock.json
npm install
npm start
```

## 🚀 What's Next?

Now that the frontend is working, the next phase is:

### Phase 1: Backend Development
- [ ] Set up Firebase Cloud Functions project
- [ ] Build Rust matching algorithm
- [ ] Compile Rust to WebAssembly
- [ ] Create Cloud Function to run matching
- [ ] Deploy to Firebase

### Phase 2: Matching Features
- [ ] Create "Matches" page to display results
- [ ] Add real-time match notifications
- [ ] Build messaging between matches
- [ ] Add match preferences/filters

### Phase 3: Enhancement
- [ ] Google Sheets integration for bulk imports
- [ ] Enhanced questionnaire with categories
- [ ] Analytics dashboard
- [ ] Match explanations (why you were matched)

## 📚 Documentation

- **Quick Setup:** [frontend/QUICKSTART.md](frontend/QUICKSTART.md) ← Start here!
- **Detailed Setup:** [frontend/SETUP.md](frontend/SETUP.md)
- **Full Implementation Plan:** [FIREBASE_IMPLEMENTATION_PLAN.md](FIREBASE_IMPLEMENTATION_PLAN.md)
- **AI Development Guide:** [CLAUDE.md](CLAUDE.md)

## 💡 Tips

1. **Firebase Console is your friend**
   - Check Authentication tab to see registered users
   - Check Firestore tab to see questionnaire responses
   - Use it to debug data issues

2. **Chrome DevTools**
   - Open with F12 or Cmd+Option+I
   - Check Console for errors
   - Use Network tab to see Firebase requests

3. **Hot Reload**
   - The app auto-reloads when you edit files
   - No need to restart the server

4. **Testing Different Users**
   - Use incognito/private browsing for multiple accounts
   - Or log out and register new accounts

## 🆘 Need Help?

1. Check the error message in the browser console (F12)
2. Look at the detailed guides in the `frontend/` folder
3. Check Firebase Console for any service issues
4. Review the [FIREBASE_IMPLEMENTATION_PLAN.md](FIREBASE_IMPLEMENTATION_PLAN.md)

## ✅ Success Checklist

- [ ] Firebase project created
- [ ] Authentication enabled (Email/Password)
- [ ] Firestore database created
- [ ] Environment files configured
- [ ] `npm install` completed
- [ ] `npm start` runs without errors
- [ ] Can access http://localhost:4200
- [ ] Can register a new account
- [ ] Can complete questionnaire
- [ ] Can view profile
- [ ] Data appears in Firebase Console

Once you've checked all these boxes, you're ready to move to the next phase!

---

**You're all set!** The frontend is complete and ready to use. Follow the steps above to get it running. 🎉
