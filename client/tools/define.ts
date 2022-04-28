export const define = {
  'process.env.SERVER_BASE_URL': process.env.SERVER_BASE_URL ? JSON.stringify(process.env.SERVER_BASE_URL) : null,
  'process.env.LOCAL_STORAGE_PRIVATE_KEY': process.env.LOCAL_STORAGE_PRIVATE_KEY ? JSON.stringify(process.env.LOCAL_STORAGE_PRIVATE_KEY) : null,
  'process.env.RECAPTCHA_SITE_KEY': process.env.RECAPTCHA_SITE_KEY ? JSON.stringify(process.env.RECAPTCHA_SITE_KEY) : null,
  'process.env.PROFILE': process.env.PROFILE ? JSON.stringify(process.env.PROFILE) : null,
};
