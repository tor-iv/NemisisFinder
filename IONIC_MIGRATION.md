# Ionic Migration Guide

This guide walks through converting the existing Angular application to an Ionic + Capacitor cross-platform app.

## Why Ionic?

`★ Key Concepts`
- **Ionic** provides mobile-optimized UI components built on web standards
- **Capacitor** is the native runtime that packages your web app for iOS/Android
- **One Codebase** → Deploy to web, iOS, and Android simultaneously
- **Native APIs** → Access device features (camera, push notifications, storage)

## What Changes?

### Stays the Same ✅
- Angular framework and TypeScript
- All services, guards, and business logic
- Firebase integration (@angular/fire)
- Routing structure
- State management
- Rust matching engine

### Changes 🔄
- **UI Components**: Angular Material → Ionic components
- **Styling**: CSS → Ionic's CSS variables + theme
- **Navigation**: Angular Router → Ionic's ion-router-outlet
- **Build Process**: Angular CLI → Ionic CLI
- **Deployment**: Single web target → Web + iOS + Android

---

## Migration Steps

### Step 1: Install Ionic CLI

```bash
# Install globally
npm install -g @ionic/cli

# Verify installation
ionic --version
```

**What this does**: The Ionic CLI extends Angular CLI with mobile-specific commands like `ionic serve`, `ionic cap run ios`, etc.

---

### Step 2: Add Ionic to Existing Angular Project

```bash
cd frontend

# Add Ionic Angular package
npm install @ionic/angular

# Add Ionic core styles
npm install @ionic/core
```

**What happens**:
- `@ionic/angular` adds Ionic components as Angular modules
- `@ionic/core` provides the underlying Web Components

---

### Step 3: Update angular.json

Add Ionic styles to your build configuration:

```json
{
  "projects": {
    "frontend": {
      "architect": {
        "build": {
          "options": {
            "styles": [
              "node_modules/@ionic/angular/css/core.css",
              "node_modules/@ionic/angular/css/normalize.css",
              "node_modules/@ionic/angular/css/structure.css",
              "node_modules/@ionic/angular/css/typography.css",
              "node_modules/@ionic/angular/css/padding.css",
              "node_modules/@ionic/angular/css/float-elements.css",
              "node_modules/@ionic/angular/css/text-alignment.css",
              "node_modules/@ionic/angular/css/text-transformation.css",
              "node_modules/@ionic/angular/css/flex-utils.css",
              "src/styles.css"
            ]
          }
        }
      }
    }
  }
}
```

**Why**: These CSS files provide Ionic's base styles, utility classes, and mobile-optimized layout system.

---

### Step 4: Update app.module.ts

Import Ionic modules:

```typescript
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { RouteReuseStrategy } from '@angular/router';

// Import Ionic modules
import { IonicModule, IonicRouteStrategy } from '@ionic/angular';

import { AppComponent } from './app.component';
import { AppRoutingModule } from './app-routing.module';

@NgModule({
  declarations: [AppComponent],
  imports: [
    BrowserModule,
    IonicModule.forRoot(), // Initialize Ionic
    AppRoutingModule
  ],
  providers: [
    // Use Ionic's routing strategy for better mobile navigation
    { provide: RouteReuseStrategy, useClass: IonicRouteStrategy }
  ],
  bootstrap: [AppComponent]
})
export class AppModule {}
```

**Key concepts**:
- `IonicModule.forRoot()` initializes Ionic globally with default configuration
- `IonicRouteStrategy` optimizes page transitions for mobile (caching, animations)

---

### Step 5: Update app.component.html

Replace the root component with Ionic structure:

**Before (Angular)**:
```html
<router-outlet></router-outlet>
```

**After (Ionic)**:
```html
<ion-app>
  <ion-router-outlet></ion-router-outlet>
</ion-app>
```

**Why**:
- `<ion-app>` is the root container for all Ionic apps
- `<ion-router-outlet>` extends Angular's router-outlet with mobile navigation animations

---

### Step 6: Convert Components to Ionic

#### Example: Login Component

**Before (Angular Material)**:
```html
<div class="login-container">
  <mat-card>
    <mat-card-header>
      <mat-card-title>Login</mat-card-title>
    </mat-card-header>

    <mat-card-content>
      <form [formGroup]="loginForm">
        <mat-form-field>
          <input matInput type="email" formControlName="email" placeholder="Email">
        </mat-form-field>

        <mat-form-field>
          <input matInput type="password" formControlName="password" placeholder="Password">
        </mat-form-field>

        <button mat-raised-button color="primary" (click)="login()">
          Login
        </button>
      </form>
    </mat-card-content>
  </mat-card>
</div>
```

**After (Ionic)**:
```html
<ion-header>
  <ion-toolbar>
    <ion-title>Login</ion-title>
  </ion-toolbar>
</ion-header>

<ion-content>
  <ion-card>
    <ion-card-header>
      <ion-card-title>Welcome Back</ion-card-title>
    </ion-card-header>

    <ion-card-content>
      <form [formGroup]="loginForm">
        <ion-item>
          <ion-label position="floating">Email</ion-label>
          <ion-input type="email" formControlName="email"></ion-input>
        </ion-item>

        <ion-item>
          <ion-label position="floating">Password</ion-label>
          <ion-input type="password" formControlName="password"></ion-input>
        </ion-item>

        <ion-button expand="block" (click)="login()">
          Login
        </ion-button>
      </form>
    </ion-card-content>
  </ion-card>
</ion-content>
```

**Component mapping**:

| Angular Material | Ionic | Purpose |
|-----------------|-------|---------|
| `<mat-card>` | `<ion-card>` | Card container |
| `<mat-form-field>` | `<ion-item>` | Form field wrapper |
| `<input matInput>` | `<ion-input>` | Text input |
| `<button mat-raised-button>` | `<ion-button>` | Button |
| N/A | `<ion-header>` | Page header |
| N/A | `<ion-content>` | Scrollable content area |
| N/A | `<ion-toolbar>` | Header toolbar |

**Key differences**:
- Ionic requires `<ion-header>` and `<ion-content>` for proper mobile layout
- `<ion-item>` is the container for form inputs (similar to mat-form-field)
- `position="floating"` makes the label float above the input when focused
- `expand="block"` makes the button full-width

---

### Step 7: Install Capacitor

```bash
# Install Capacitor CLI and core
npm install @capacitor/core @capacitor/cli

# Initialize Capacitor
npx cap init

# When prompted:
# App name: NemisisFinder
# App ID: com.nemisisfinder.app (reverse domain notation)
# Web directory: dist/frontend (Angular build output)
```

**What this creates**:
- `capacitor.config.json` - Capacitor configuration
- Native project folders (after adding platforms)

---

### Step 8: Add Native Platforms

```bash
# Add iOS platform (macOS only)
ionic cap add ios

# Add Android platform
ionic cap add android
```

**What this creates**:
- `ios/` folder - Xcode project
- `android/` folder - Android Studio project

These are **real native projects** that you can open and customize in Xcode/Android Studio.

---

### Step 9: Configure capacitor.config.json

```json
{
  "appId": "com.nemisisfinder.app",
  "appName": "NemisisFinder",
  "webDir": "www",
  "server": {
    "androidScheme": "https"
  },
  "plugins": {
    "SplashScreen": {
      "launchShowDuration": 2000,
      "backgroundColor": "#ffffffff",
      "androidScaleType": "CENTER_CROP",
      "showSpinner": false
    }
  }
}
```

**Important settings**:
- `webDir`: Where Ionic builds your web assets (usually `www` or `dist`)
- `androidScheme`: Use HTTPS for Android to avoid mixed content issues
- Plugins configuration for native features

---

### Step 10: Update package.json Scripts

Add Ionic-specific commands:

```json
{
  "scripts": {
    "start": "ionic serve",
    "build": "ionic build",
    "build:prod": "ionic build --prod",
    "sync": "ionic cap sync",
    "ios": "ionic cap run ios",
    "android": "ionic cap run android",
    "open:ios": "ionic cap open ios",
    "open:android": "ionic cap open android"
  }
}
```

---

### Step 11: Add Capacitor Plugins

Install plugins for native features:

```bash
# Push notifications
npm install @capacitor/push-notifications

# Camera
npm install @capacitor/camera

# Storage (for offline data)
npm install @capacitor/preferences

# Status bar customization
npm install @capacitor/status-bar

# Splash screen
npm install @capacitor/splash-screen
```

---

### Step 12: Example - Using Camera Plugin

```typescript
import { Camera, CameraResultType, CameraSource } from '@capacitor/camera';

@Component({
  selector: 'app-profile',
  templateUrl: './profile.page.html'
})
export class ProfilePage {

  async takePicture() {
    try {
      // Request camera permission and take photo
      const image = await Camera.getPhoto({
        quality: 90,
        allowEditing: true,
        resultType: CameraResultType.Uri,
        source: CameraSource.Camera
      });

      // image.webPath contains the file URI
      const photoUrl = image.webPath;

      // Upload to Firebase Storage
      this.uploadPhoto(photoUrl);

    } catch (error) {
      console.error('Error taking photo:', error);
    }
  }
}
```

**What this demonstrates**:
- Capacitor APIs use modern `async/await` syntax
- Automatic permission handling
- Works on iOS, Android, and web (uses browser's file picker as fallback)

---

### Step 13: Example - Push Notifications

#### Install Firebase Cloud Messaging Plugin

```bash
npm install @capacitor-firebase/messaging
```

#### Configure in TypeScript

```typescript
import { FirebaseMessaging } from '@capacitor-firebase/messaging';
import { Capacitor } from '@capacitor/core';

@Injectable({ providedIn: 'root' })
export class NotificationService {

  async initialize() {
    if (Capacitor.isNativePlatform()) {
      // Request permission
      const result = await FirebaseMessaging.requestPermissions();

      if (result.receive === 'granted') {
        // Get FCM token
        const { token } = await FirebaseMessaging.getToken();

        // Store token in Firestore for this user
        await this.saveTokenToFirestore(token);

        // Listen for incoming notifications
        await FirebaseMessaging.addListener('notificationReceived', (notification) => {
          console.log('Notification received:', notification);
        });
      }
    }
  }

  private async saveTokenToFirestore(token: string) {
    const userId = this.authService.currentUserId;
    await this.firestore.doc(`users/${userId}`).update({
      fcmToken: token,
      platform: Capacitor.getPlatform() // 'ios' or 'android'
    });
  }
}
```

**Key concepts**:
- `Capacitor.isNativePlatform()` detects if running on iOS/Android
- `Capacitor.getPlatform()` returns 'ios', 'android', or 'web'
- Store FCM token in Firestore to send targeted notifications

#### Send Notification from Cloud Function

```typescript
// functions/src/index.ts
import * as admin from 'firebase-admin';

export const sendMatchNotification = functions.firestore
  .document('matches/{matchId}')
  .onCreate(async (snap, context) => {
    const match = snap.data();

    // Get user's FCM token
    const userDoc = await admin.firestore()
      .doc(`users/${match.user_id}`)
      .get();

    const fcmToken = userDoc.data()?.fcmToken;

    if (fcmToken) {
      // Send push notification
      await admin.messaging().send({
        token: fcmToken,
        notification: {
          title: 'New Match Found! 🎉',
          body: 'You have been matched with your opposite. Tap to view.',
        },
        data: {
          matchId: context.params.matchId,
          type: 'new_match'
        }
      });
    }
  });
```

---

### Step 14: Build and Test

#### Test on Web
```bash
ionic serve
```
Opens at http://localhost:8100

#### Test on iOS Simulator
```bash
# Build the web assets
ionic build

# Sync to native projects
ionic cap sync

# Run on iOS simulator
ionic cap run ios
```

#### Test on Android Emulator
```bash
# Build the web assets
ionic build

# Sync to native projects
ionic cap sync

# Run on Android emulator
ionic cap run android
```

#### Live Reload on Device
```bash
# Find your local IP (e.g., 192.168.1.5)
ionic cap run ios --livereload --external
ionic cap run android --livereload --external
```

**What this does**:
- Runs the dev server on your local IP
- Device connects to your computer
- Changes auto-reload on the device (fast development!)

---

## Component Migration Checklist

### Authentication Pages
- [ ] Convert login form (mat-form-field → ion-item + ion-input)
- [ ] Convert register form
- [ ] Add ion-header and ion-content wrappers
- [ ] Update button styling (mat-button → ion-button)

### Questionnaire
- [ ] Replace mat-slider with ion-range for 1-7 scale
- [ ] Convert mat-stepper to ion-slides or custom ion-card progression
- [ ] Add ion-progress-bar for completion tracking
- [ ] Update form validation styling

### Profile Page
- [ ] Convert mat-card to ion-card
- [ ] Add ion-avatar for profile picture
- [ ] Add ion-fab for camera button
- [ ] Implement Capacitor Camera API

### Matches Display
- [ ] Convert mat-list to ion-list
- [ ] Convert mat-list-item to ion-item
- [ ] Add ion-thumbnail for user avatars
- [ ] Implement swipe gestures with ion-item-sliding
- [ ] Add pull-to-refresh with ion-refresher

### Navigation
- [ ] Add ion-tabs for bottom navigation (optional)
- [ ] Add ion-menu for side menu (optional)
- [ ] Update routing animations

---

## Ionic Theming

### Create theme variables in src/theme/variables.css

```css
:root {
  /** Primary brand color **/
  --ion-color-primary: #3880ff;
  --ion-color-primary-rgb: 56, 128, 255;
  --ion-color-primary-contrast: #ffffff;
  --ion-color-primary-contrast-rgb: 255, 255, 255;
  --ion-color-primary-shade: #3171e0;
  --ion-color-primary-tint: #4c8dff;

  /** Add custom colors for NemisisFinder **/
  --ion-color-nemesis: #ff3366;
  --ion-color-match: #00cc66;

  /** Typography **/
  --ion-font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}
```

**Usage in templates**:
```html
<ion-button color="primary">Primary Button</ion-button>
<ion-button color="nemesis">Custom Color</ion-button>
```

---

## Firebase Configuration for Capacitor

### Update firebase.json

```json
{
  "hosting": {
    "public": "www",
    "ignore": [
      "firebase.json",
      "**/.*",
      "**/node_modules/**"
    ],
    "rewrites": [
      {
        "source": "**",
        "destination": "/index.html"
      }
    ]
  }
}
```

### Add platform-specific Firebase config files

#### iOS: GoogleService-Info.plist
Download from Firebase Console → iOS App → Download config file
Place in `ios/App/App/GoogleService-Info.plist`

#### Android: google-services.json
Download from Firebase Console → Android App → Download config file
Place in `android/app/google-services.json`

---

## Testing Checklist

### Functionality Tests
- [ ] Authentication (login, register, logout)
- [ ] Questionnaire submission
- [ ] Real-time Firestore listeners
- [ ] Navigation between pages
- [ ] Form validation

### Mobile-Specific Tests
- [ ] Camera access
- [ ] Push notifications
- [ ] Offline mode (Capacitor Storage)
- [ ] Responsive layout (phone, tablet)
- [ ] Back button navigation (Android)
- [ ] Status bar styling

### Platform Tests
- [ ] Web browser (Chrome, Safari, Firefox)
- [ ] iOS Simulator
- [ ] Android Emulator
- [ ] Physical iOS device
- [ ] Physical Android device

---

## Deployment

### Web (Firebase Hosting)
```bash
ionic build --prod
firebase deploy --only hosting
```

### iOS (App Store)
1. Open in Xcode: `ionic cap open ios`
2. Update signing & capabilities
3. Set version and build number
4. Archive and upload to App Store Connect

### Android (Play Store)
1. Open in Android Studio: `ionic cap open android`
2. Update version in `android/app/build.gradle`
3. Generate signed APK or Bundle
4. Upload to Play Console

---

## Common Issues & Solutions

### Issue: White screen on device
**Solution**: Check `capacitor.config.json` webDir matches your build output folder

### Issue: Plugins not working
**Solution**: Run `ionic cap sync` after installing new plugins

### Issue: iOS build fails
**Solution**: Update CocoaPods: `cd ios && pod install`

### Issue: Android permissions denied
**Solution**: Add permissions to `android/app/src/main/AndroidManifest.xml`

```xml
<uses-permission android:name="android.permission.CAMERA" />
<uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
```

---

## Learning Resources

### Official Documentation
- [Ionic Framework Docs](https://ionicframework.com/docs)
- [Capacitor Docs](https://capacitorjs.com/docs)
- [Ionic Angular Guide](https://ionicframework.com/docs/angular/overview)

### Video Tutorials
- Ionic Academy (ionic.io/academy)
- Angular Firebase (fireship.io)

### Component Reference
- [Ionic Components](https://ionicframework.com/docs/components)
- [Capacitor Plugins](https://capacitorjs.com/docs/plugins)

---

## Next Steps

After completing the migration:

1. **Add Native Features**
   - Camera for profile photos
   - Push notifications for matches
   - Biometric authentication
   - Offline mode

2. **Optimize Performance**
   - Lazy load modules
   - Image optimization
   - Bundle size reduction

3. **Test Thoroughly**
   - Unit tests
   - E2E tests on iOS/Android
   - Beta testing with TestFlight/Play Console

4. **Prepare for Launch**
   - App Store screenshots
   - Privacy policy
   - Terms of service
   - App descriptions

---

## Summary

This migration transforms your Angular web app into a full-fledged cross-platform mobile application. The beauty of Ionic + Capacitor is that you:

✅ Keep your existing Angular code and logic
✅ Gain native iOS and Android apps
✅ Access device features through simple APIs
✅ Deploy to web, App Store, and Play Store from one codebase

**Key Philosophy**: Write once, run everywhere - without sacrificing native functionality or user experience.
