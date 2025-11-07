# NemisisFinder Implementation Plan

## Executive Summary

This document outlines the complete implementation plan for NemisisFinder, a matchmaking application that connects users with opposite opinions. The system uses Angular for the frontend, Django for the backend API, Rust for the high-performance matching algorithm, and MongoDB for data persistence.

---

## Technology Stack

### Frontend
- **Framework**: Angular 17+ (latest stable)
- **Language**: TypeScript
- **UI Library**: Angular Material or PrimeNG
- **State Management**: NgRx or Angular Signals
- **HTTP Client**: Angular HttpClient with interceptors
- **Testing**: Jasmine + Karma, Cypress for E2E

### Backend
- **Framework**: Django 5.x with Django REST Framework (DRF)
- **Language**: Python 3.11+
- **Authentication**: djangorestframework-simplejwt
- **Database ORM**: MongoEngine or Djongo
- **API Documentation**: drf-spectacular (OpenAPI/Swagger)
- **Testing**: pytest with pytest-django

### Matching Engine
- **Language**: Rust (latest stable)
- **Python Integration**: PyO3 (Rust bindings for Python)
- **Alternative**: Standalone microservice with REST API
- **WebAssembly**: wasm-pack for potential client-side matching
- **Testing**: Built-in Rust testing framework

### Database
- **Primary**: MongoDB 7.x
- **Schema**: User profiles, questionnaire responses, match results
- **Indexing**: Optimized for matching algorithm queries

### DevOps & Infrastructure
- **Containerization**: Docker + Docker Compose
- **API Gateway**: Nginx (reverse proxy)
- **Environment Management**: python-dotenv, environment files

---

## Architecture Overview

### System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Client Layer                         │
│  ┌───────────────────────────────────────────────────────┐  │
│  │           Angular SPA (Port 4200)                     │  │
│  │  - Components (Questionnaire, Matches, Profile)      │  │
│  │  - Services (API, Auth, State)                       │  │
│  │  - Guards & Interceptors                             │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ HTTPS/REST API
┌─────────────────────────────────────────────────────────────┐
│                         API Layer                            │
│  ┌───────────────────────────────────────────────────────┐  │
│  │        Django REST API (Port 8000)                    │  │
│  │  - Authentication & Authorization                     │  │
│  │  - User Management                                    │  │
│  │  - Questionnaire CRUD                                 │  │
│  │  - Match Orchestration                                │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                    │                      │
                    │                      ▼ FFI/PyO3
         ▼ MongoDB Driver          ┌──────────────────┐
┌──────────────────┐                │  Rust Engine     │
│   MongoDB        │                │  - Algorithm     │
│  - Users         │                │  - Optimization  │
│  - Responses     │◄───────────────┤  - Scoring       │
│  - Matches       │  Store Results └──────────────────┘
│  - Questionnaire │
└──────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    External Services                         │
│  - Google Forms (Data Collection)                           │
│  - Google Sheets API (Initial Import)                       │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow Sequence

1. **User Registration & Questionnaire**
   - User accesses Angular app
   - Registers/logs in via Django API (JWT authentication)
   - Fills out questionnaire (1-7 scale responses)
   - Angular submits to Django `/api/responses/`
   - Django validates and stores in MongoDB

2. **Matching Process**
   - Admin or scheduled task triggers matching
   - Django calls Rust matching engine via PyO3
   - Rust engine:
     - Loads all user responses from Django
     - Calculates pairwise absolute differences
     - Identifies optimal opposite pairs
     - Returns match results to Django
   - Django stores matches in MongoDB

3. **Viewing Matches**
   - User requests matches via Angular
   - Django API returns match data
   - Angular displays match profiles and scores

---

## Project Structure

```
NemisisFinder/
├── backend/                    # Django project
│   ├── nemisis_finder/        # Main Django project
│   │   ├── settings/
│   │   │   ├── base.py
│   │   │   ├── development.py
│   │   │   └── production.py
│   │   ├── urls.py
│   │   └── wsgi.py
│   ├── apps/
│   │   ├── users/             # User management
│   │   ├── questionnaire/     # Questions & responses
│   │   ├── matching/          # Match orchestration
│   │   └── integrations/      # Google Sheets import
│   ├── requirements/
│   │   ├── base.txt
│   │   ├── development.txt
│   │   └── production.txt
│   ├── manage.py
│   └── pytest.ini
│
├── frontend/                   # Angular project
│   ├── src/
│   │   ├── app/
│   │   │   ├── core/          # Services, guards, interceptors
│   │   │   ├── shared/        # Shared components
│   │   │   ├── features/
│   │   │   │   ├── auth/
│   │   │   │   ├── questionnaire/
│   │   │   │   ├── matches/
│   │   │   │   └── profile/
│   │   │   └── app.component.ts
│   │   ├── environments/
│   │   └── assets/
│   ├── angular.json
│   ├── package.json
│   └── tsconfig.json
│
├── matching-engine/            # Rust crate
│   ├── src/
│   │   ├── lib.rs             # Library entry point
│   │   ├── algorithm.rs       # Core matching logic
│   │   ├── python_bindings.rs # PyO3 interface
│   │   └── models.rs          # Data structures
│   ├── tests/
│   ├── Cargo.toml
│   └── README.md
│
├── docker/
│   ├── backend.Dockerfile
│   ├── frontend.Dockerfile
│   └── nginx.conf
│
├── scripts/
│   ├── setup.sh               # Initial setup script
│   ├── deploy.sh              # Deployment script
│   └── import_google_sheets.py
│
├── docker-compose.yml
├── .env.example
├── .gitignore
├── README.md
├── CLAUDE.md
└── IMPLEMENTATION_PLAN.md
```

---

## Database Schema

### MongoDB Collections

#### users
```javascript
{
  _id: ObjectId,
  email: String (unique, indexed),
  username: String (unique, indexed),
  password_hash: String,
  first_name: String,
  last_name: String,
  created_at: DateTime,
  updated_at: DateTime,
  is_active: Boolean,
  has_completed_questionnaire: Boolean
}
```

#### questionnaires
```javascript
{
  _id: ObjectId,
  title: String,
  description: String,
  questions: [
    {
      id: Number,
      text: String,
      category: String,  // e.g., "politics", "lifestyle", "values"
      scale_min_label: String,  // e.g., "Strongly Disagree"
      scale_max_label: String   // e.g., "Strongly Agree"
    }
  ],
  is_active: Boolean,
  created_at: DateTime,
  version: Number
}
```

#### responses
```javascript
{
  _id: ObjectId,
  user_id: ObjectId (indexed),
  questionnaire_id: ObjectId,
  answers: [
    {
      question_id: Number,
      value: Number (1-7)
    }
  ],
  submitted_at: DateTime,
  source: String  // "google_forms" or "web_app"
}
```

#### matches
```javascript
{
  _id: ObjectId,
  user_1_id: ObjectId (indexed),
  user_2_id: ObjectId (indexed),
  difference_score: Number,  // Total absolute difference
  question_scores: [
    {
      question_id: Number,
      difference: Number,
      user_1_answer: Number,
      user_2_answer: Number
    }
  ],
  matched_at: DateTime,
  questionnaire_version: Number,
  status: String  // "pending", "viewed", "contacted"
}
```

---

## Rust Matching Algorithm

### Core Algorithm Pseudocode

```rust
// Structure for user response
struct UserResponse {
    user_id: String,
    answers: Vec<i32>,  // Array of 1-7 values
}

// Structure for match result
struct MatchResult {
    user_1_id: String,
    user_2_id: String,
    total_difference: i32,
    question_differences: Vec<i32>,
}

// Calculate absolute difference between two users
fn calculate_difference(user1: &UserResponse, user2: &UserResponse) -> i32 {
    user1.answers
        .iter()
        .zip(user2.answers.iter())
        .map(|(a1, a2)| (a1 - a2).abs())
        .sum()
}

// Find optimal opposite matches
fn find_opposite_matches(users: Vec<UserResponse>) -> Vec<MatchResult> {
    let mut matches = Vec::new();
    let mut matched_users = HashSet::new();

    // Calculate all pairwise differences
    let mut pairs: Vec<_> = users.iter()
        .enumerate()
        .flat_map(|(i, u1)| {
            users[i+1..].iter().map(move |u2| {
                (u1, u2, calculate_difference(u1, u2))
            })
        })
        .collect();

    // Sort by difference (descending - highest difference first)
    pairs.sort_by(|a, b| b.2.cmp(&a.2));

    // Greedily match users with highest differences
    for (user1, user2, diff) in pairs {
        if !matched_users.contains(&user1.user_id)
           && !matched_users.contains(&user2.user_id) {
            matches.push(create_match_result(user1, user2, diff));
            matched_users.insert(&user1.user_id);
            matched_users.insert(&user2.user_id);
        }
    }

    matches
}
```

### Performance Optimizations
- Use SIMD instructions for vectorized difference calculations
- Parallel processing with Rayon for large datasets
- Memory-efficient data structures
- Caching for repeated calculations

---

## API Endpoints

### Authentication
- `POST /api/auth/register/` - User registration
- `POST /api/auth/login/` - Login (returns JWT)
- `POST /api/auth/refresh/` - Refresh JWT token
- `POST /api/auth/logout/` - Logout

### Users
- `GET /api/users/me/` - Get current user profile
- `PATCH /api/users/me/` - Update user profile
- `GET /api/users/{id}/` - Get user by ID (limited info)

### Questionnaire
- `GET /api/questionnaires/active/` - Get active questionnaire
- `POST /api/responses/` - Submit questionnaire responses
- `GET /api/responses/me/` - Get current user's responses

### Matching
- `POST /api/matches/run/` - Trigger matching algorithm (admin)
- `GET /api/matches/me/` - Get matches for current user
- `GET /api/matches/{id}/` - Get specific match details
- `PATCH /api/matches/{id}/status/` - Update match status

### Admin
- `POST /api/admin/import-sheets/` - Import from Google Sheets
- `GET /api/admin/stats/` - System statistics

---

## Hosting Recommendations

### Option 1: Cloud Platform (Recommended for Production)

#### **AWS (Amazon Web Services)**

**Services:**
- **EC2**: Django backend (t3.medium or t3.large)
- **S3**: Static file storage (Angular build, media)
- **CloudFront**: CDN for frontend distribution
- **DocumentDB**: MongoDB-compatible managed database
- **Elastic Load Balancer**: Load balancing and SSL termination
- **Route 53**: DNS management
- **Certificate Manager**: Free SSL certificates

**Estimated Monthly Cost**: $150-300 (small to medium scale)

**Pros:**
- Highly scalable
- Excellent reliability (99.99% uptime SLA)
- Comprehensive monitoring (CloudWatch)
- Strong security features

**Cons:**
- More complex setup
- Higher cost for small projects
- Steeper learning curve

---

#### **Google Cloud Platform (GCP)**

**Services:**
- **Compute Engine** or **Cloud Run**: Backend hosting
- **Cloud Storage**: Static files
- **Cloud CDN**: Content delivery
- **MongoDB Atlas** (GCP marketplace): Managed MongoDB
- **Cloud Load Balancing**: Traffic distribution
- **Cloud DNS**: Domain management

**Estimated Monthly Cost**: $100-250

**Pros:**
- Good integration with Google services (Sheets API)
- Generous free tier
- Strong ML/AI capabilities for future features
- Excellent documentation

**Cons:**
- Smaller community than AWS
- Fewer third-party integrations

---

### Option 2: Platform as a Service (Easiest Setup)

#### **Render** (Recommended for MVP/Small Scale)

**Services:**
- **Web Service**: Django backend (auto-deploy from Git)
- **Static Site**: Angular frontend
- **MongoDB Atlas**: External managed MongoDB

**Configuration:**
```yaml
# render.yaml
services:
  - type: web
    name: nemisisfinder-api
    env: python
    buildCommand: "pip install -r requirements/production.txt"
    startCommand: "gunicorn nemisis_finder.wsgi:application"
    envVars:
      - key: MONGODB_URI
        sync: false
      - key: SECRET_KEY
        generateValue: true

  - type: web
    name: nemisisfinder-frontend
    env: static
    buildCommand: "npm install && ng build --configuration production"
    staticPublishPath: dist/frontend
```

**Estimated Monthly Cost**: $25-75

**Pros:**
- Extremely simple deployment (Git push)
- Free SSL certificates
- Automatic deployments
- Good for MVPs and small projects
- Free tier available

**Cons:**
- Less control over infrastructure
- Limited scaling options
- Higher cost at large scale

---

#### **Heroku**

**Services:**
- **Dyno**: Django backend
- **Heroku Static**: Frontend (or use S3)
- **MongoDB Atlas**: Add-on

**Estimated Monthly Cost**: $50-150

**Pros:**
- Very easy deployment
- Great developer experience
- Extensive add-on marketplace

**Cons:**
- More expensive than alternatives
- Dynos sleep on free tier

---

### Option 3: Self-Hosted (Maximum Control)

#### **DigitalOcean Droplet** or **Linode VPS**

**Setup:**
- **Droplet/VPS**: $12-24/month (4GB RAM)
- **MongoDB Atlas**: Free tier or $9/month
- **Docker Compose**: All services containerized

**Estimated Monthly Cost**: $12-35

**Pros:**
- Full control over environment
- Most cost-effective for small-medium projects
- Simple pricing model
- Good performance

**Cons:**
- Manual setup and maintenance
- You manage security updates
- No auto-scaling

---

### Option 4: Hybrid Approach (Best Value)

**Recommended Configuration:**
- **Frontend**: Vercel or Netlify (free tier, excellent for Angular)
- **Backend**: Railway or Render ($7-25/month)
- **Database**: MongoDB Atlas (free tier up to 512MB)
- **Matching Engine**: AWS Lambda or Cloud Functions (pay-per-use)

**Estimated Monthly Cost**: $0-25 (MVP), $25-75 (production)

**Pros:**
- Best of all worlds
- Extremely cost-effective
- Easy to set up and deploy
- Each component uses optimal platform

**Cons:**
- Multiple platforms to manage
- Potential latency between services

---

## Deployment Strategy

### Development Environment
```bash
# Using Docker Compose
docker-compose up

# Services:
# - Angular: http://localhost:4200
# - Django: http://localhost:8000
# - MongoDB: localhost:27017
```

### Production Deployment (Recommended: Hybrid)

#### Frontend (Vercel)
```bash
# Connect GitHub repo to Vercel
# Auto-deploy on push to main branch
# Environment variables set in Vercel dashboard
```

#### Backend (Render)
```bash
# Connect GitHub repo to Render
# Auto-deploy on push to main branch
# Set environment variables:
MONGODB_URI=<mongodb_atlas_connection_string>
SECRET_KEY=<django_secret_key>
ALLOWED_HOSTS=api.nemisisfinder.com
CORS_ALLOWED_ORIGINS=https://nemisisfinder.com
```

#### Database (MongoDB Atlas)
- Create free cluster
- Whitelist Render IP addresses
- Create database user
- Use connection string in Django

---

## Environment Variables

### Backend (.env)
```
# Django
SECRET_KEY=your-secret-key-here
DEBUG=False
ALLOWED_HOSTS=api.nemisisfinder.com,localhost

# Database
MONGODB_URI=mongodb+srv://user:pass@cluster.mongodb.net/nemisisfinder

# CORS
CORS_ALLOWED_ORIGINS=https://nemisisfinder.com,http://localhost:4200

# JWT
JWT_SECRET_KEY=your-jwt-secret
JWT_ACCESS_TOKEN_LIFETIME=60  # minutes
JWT_REFRESH_TOKEN_LIFETIME=7  # days

# Google Sheets
GOOGLE_SHEETS_API_KEY=your-api-key
GOOGLE_SHEETS_SPREADSHEET_ID=your-sheet-id

# Email (optional)
EMAIL_HOST=smtp.gmail.com
EMAIL_PORT=587
EMAIL_HOST_USER=your-email@gmail.com
EMAIL_HOST_PASSWORD=your-app-password
```

### Frontend (environment.ts)
```typescript
export const environment = {
  production: true,
  apiUrl: 'https://api.nemisisfinder.com/api',
  appUrl: 'https://nemisisfinder.com'
};
```

---

## Implementation Phases

### Phase 1: Foundation (Weeks 1-2)
- [ ] Initialize Git repository structure
- [ ] Set up Django project with MongoDB
- [ ] Create Angular application
- [ ] Initialize Rust crate for matching engine
- [ ] Set up Docker development environment
- [ ] Configure CI/CD pipelines

### Phase 2: Authentication & User Management (Week 3)
- [ ] Django user models and API endpoints
- [ ] JWT authentication implementation
- [ ] Angular authentication service and guards
- [ ] Login/register UI components

### Phase 3: Questionnaire System (Week 4)
- [ ] Django questionnaire models and API
- [ ] Angular questionnaire form component
- [ ] Response submission and validation
- [ ] Google Sheets import functionality

### Phase 4: Matching Engine (Weeks 5-6)
- [ ] Rust algorithm implementation
- [ ] PyO3 Python bindings
- [ ] Integration with Django
- [ ] Performance testing and optimization
- [ ] Unit tests for algorithm correctness

### Phase 5: Match Display & User Experience (Week 7)
- [ ] Django match API endpoints
- [ ] Angular match display components
- [ ] User profile pages
- [ ] Match history and status tracking

### Phase 6: Testing & Refinement (Week 8)
- [ ] Comprehensive unit tests
- [ ] Integration tests
- [ ] E2E tests with Cypress
- [ ] Performance testing
- [ ] Security audit

### Phase 7: Deployment (Week 9)
- [ ] Set up production hosting
- [ ] Configure domain and SSL
- [ ] Deploy frontend to Vercel/Netlify
- [ ] Deploy backend to Render/Railway
- [ ] Set up MongoDB Atlas production cluster
- [ ] Configure monitoring and logging

### Phase 8: Launch & Iteration (Week 10+)
- [ ] Soft launch with test users
- [ ] Gather feedback
- [ ] Fix bugs and improve UX
- [ ] Add analytics
- [ ] Scale as needed

---

## Testing Strategy

### Backend (Django)
```python
# pytest with pytest-django
# tests/test_matching.py
def test_matching_algorithm():
    """Test that opposite users are matched correctly"""
    user1 = create_test_user(answers=[1, 1, 1, 1])
    user2 = create_test_user(answers=[7, 7, 7, 7])
    user3 = create_test_user(answers=[4, 4, 4, 4])

    matches = run_matching_algorithm()

    # user1 and user2 should be matched (highest difference)
    assert matches[0].contains_users(user1, user2)
    assert matches[0].difference_score == 24  # 4*6
```

### Rust (Matching Engine)
```rust
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

    let diff = calculate_difference(&user1, &user2);
    assert_eq!(diff, 12); // |1-7| + |2-6| + |3-5| = 6+4+2
}
```

### Frontend (Angular)
```typescript
// Jasmine/Karma
describe('QuestionnaireComponent', () => {
  it('should submit valid responses', () => {
    component.answers = [5, 4, 6, 3, 7, 2, 1];
    component.submit();

    expect(apiService.submitResponse).toHaveBeenCalledWith({
      answers: [5, 4, 6, 3, 7, 2, 1]
    });
  });
});
```

---

## Security Considerations

### Authentication & Authorization
- JWT tokens with short expiration (60 minutes)
- Refresh tokens stored securely
- HTTPS only in production
- CORS properly configured
- Rate limiting on API endpoints

### Data Protection
- Passwords hashed with Django's PBKDF2
- User data anonymized in match displays
- MongoDB connection encrypted
- Environment variables for secrets
- No sensitive data in version control

### API Security
- Input validation on all endpoints
- SQL/NoSQL injection prevention
- XSS protection in Angular
- CSRF tokens for state-changing operations
- API rate limiting (django-ratelimit)

---

## Monitoring & Logging

### Application Monitoring
- **Sentry**: Error tracking (free tier available)
- **LogRocket**: Frontend session replay
- **Django logging**: Structured logs to file/stdout

### Performance Monitoring
- **Django Debug Toolbar**: Development performance
- **Angular DevTools**: Frontend performance
- **MongoDB Compass**: Database query analysis

### Uptime Monitoring
- **UptimeRobot**: Free uptime monitoring
- **Healthchecks.io**: Cron job monitoring

---

## Future Enhancements

### Short-term (3-6 months)
- Email notifications for new matches
- In-app messaging between matches
- More sophisticated matching (weighted questions)
- User preferences and filters
- Match explanations (why you were matched)

### Medium-term (6-12 months)
- Mobile app (React Native or Flutter)
- Social features (profiles, bios, photos)
- Matching analytics dashboard
- A/B testing for questionnaire optimization
- Multiple questionnaire versions

### Long-term (12+ months)
- Machine learning for improved matching
- Video profiles
- Events and meetups for matched users
- Premium features and monetization
- API for third-party integrations

---

## Cost Estimates

### MVP (Months 1-3)
- **Hosting**: $0-25/month (free tiers + Render hobby plan)
- **Domain**: $12/year
- **Total**: ~$25-50/month

### Growth Phase (100-1000 users)
- **Hosting**: $75-150/month
- **Database**: $25-50/month (MongoDB Atlas M10)
- **CDN**: $10-20/month
- **Monitoring**: $0-30/month
- **Total**: ~$110-250/month

### Scale Phase (1000+ users)
- **Hosting**: $200-500/month
- **Database**: $50-150/month
- **CDN**: $30-100/month
- **Monitoring**: $50-100/month
- **Total**: ~$330-850/month

---

## Conclusion

This implementation plan provides a complete roadmap for building NemisisFinder using modern, scalable technologies. The Angular + Django + Rust stack offers:

- **Performance**: Rust for computationally intensive matching
- **Developer Experience**: Angular and Django with excellent tooling
- **Scalability**: Cloud-native architecture ready to grow
- **Cost-Effectiveness**: Start small, scale as needed

**Recommended First Steps:**
1. Set up development environment with Docker Compose
2. Initialize Django backend with MongoDB
3. Create Angular frontend boilerplate
4. Implement Rust matching algorithm prototype
5. Deploy MVP to Render + Vercel (free tiers)

**Timeline**: 8-10 weeks for MVP, 3-4 months for production-ready launch.
