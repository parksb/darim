import { I18n, I18nText } from 'snowball-js';

const presetTexts: I18nText = {
  email: {
    ko: '이메일',
    en: 'Email',
  },
  password: {
    ko: '비밀번호',
    en: 'Password',
  },
  name: {
    ko: '이름',
    en: 'Name',
  },
  avatar: {
    ko: '프로필 사진 URL',
    en: 'Avatar URL',
  },
  save: {
    ko: '저장',
    en: 'Save',
  },
};

function getI18n(texts: I18nText): I18n {
  return new I18n({ ...presetTexts, ...texts });
}

export { getI18n };
