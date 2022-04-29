export enum Profile {
  DEV,
  PRODUCTION,
}

function decideProfile(profile: string) {
  if (profile === 'production') {
    return Profile.PRODUCTION;
  }
  return Profile.DEV;
}

export const serverBaseUrl = process.env.SERVER_BASE_URL || 'http://localhost:9600';
export const localStoragePrivateKey = process.env.LOCAL_STORAGE_PRIVATE_KEY || 'key';
export const reCAPTCHASiteKey = process.env.RECAPTCHA_SITE_KEY || 'key';
export const profile = process.env.PROFILE ? decideProfile(process.env.PROFILE) : Profile.DEV;
export const localStorageViewModeKey = 'view-mode';
