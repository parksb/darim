export enum Profile {
  DEV,
  PRODUCTION,
}

function decideProfile(profile: string) {
  if (process.env.PROFILE == 'production') {
    return Profile.PRODUCTION;
  } else {
    return Profile.DEV;
  }
}

export const serverBaseUrl = process.env.SERVER_BASE_URL || 'http://localhost:9600';
export const localStoragePrivateKey = process.env.LOCAL_STORAGE_PRIVATE_KEY || 'key';
export const reCAPTCHASiteKey = process.env.RECAPTCHA_SITE_KEY || 'key';
export const profile = process.env.PROFILE ? decideProfile(process.env.PROFILE) : Profile.DEV;
export const localStorageViewModeKey = 'view-mode';
