import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterLink } from '@angular/router';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { AuthService } from '../../core/services/auth.service';

@Component({
  selector: 'app-profile',
  standalone: true,
  imports: [CommonModule, RouterLink, MatCardModule, MatButtonModule],
  template: `
    <div class="profile-container">
      <mat-card class="profile-card">
        <mat-card-header>
          <mat-card-title>Your Profile</mat-card-title>
        </mat-card-header>

        <mat-card-content>
          @if (authService.currentUser(); as user) {
            <div class="profile-info">
              <div class="info-row">
                <strong>Name:</strong>
                <span>{{ user.displayName }}</span>
              </div>
              <div class="info-row">
                <strong>Email:</strong>
                <span>{{ user.email }}</span>
              </div>
              <div class="info-row">
                <strong>Questionnaire:</strong>
                <span class="status" [class.completed]="user.hasCompletedQuestionnaire">
                  {{ user.hasCompletedQuestionnaire ? 'Completed' : 'Not Completed' }}
                </span>
              </div>
              <div class="info-row">
                <strong>Match Status:</strong>
                <span class="status" [class.matched]="user.isMatched">
                  {{ user.isMatched ? 'Matched!' : 'Waiting for match' }}
                </span>
              </div>
            </div>

            @if (!user.hasCompletedQuestionnaire) {
              <div class="alert">
                <p>You haven't completed the questionnaire yet.</p>
                <button mat-raised-button color="primary" routerLink="/questionnaire">
                  Complete Questionnaire
                </button>
              </div>
            }

            @if (user.hasCompletedQuestionnaire && !user.isMatched) {
              <div class="info-message">
                <p>We're working on finding your opposite match. Check back soon!</p>
              </div>
            }
          }
        </mat-card-content>
      </mat-card>
    </div>
  `,
  styles: [`
    .profile-container {
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: calc(100vh - 64px);
      padding: 20px;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    }

    .profile-card {
      max-width: 600px;
      width: 100%;
    }

    mat-card-header {
      display: flex;
      flex-direction: column;
      align-items: center;
      margin-bottom: 20px;
    }

    mat-card-title {
      font-size: 24px;
      font-weight: 500;
    }

    .profile-info {
      margin: 20px 0;
    }

    .info-row {
      display: flex;
      justify-content: space-between;
      padding: 12px;
      border-bottom: 1px solid #eee;
    }

    .info-row strong {
      color: #666;
    }

    .status.completed,
    .status.matched {
      color: #4caf50;
      font-weight: 500;
    }

    .alert {
      background-color: #fff3cd;
      border: 1px solid #ffc107;
      border-radius: 4px;
      padding: 16px;
      margin-top: 20px;
      text-align: center;
    }

    .alert p {
      margin-bottom: 12px;
    }

    .info-message {
      background-color: #e3f2fd;
      border: 1px solid #2196f3;
      border-radius: 4px;
      padding: 16px;
      margin-top: 20px;
      text-align: center;
    }
  `]
})
export class ProfileComponent {
  authService = inject(AuthService);
}
