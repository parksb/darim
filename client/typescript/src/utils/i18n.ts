enum I18nLanguages {
  KO = 'ko',
  EN = 'en',
}

interface I18nText {
  [key: string]: {
    ko: string
    en: string
  };
}

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

class I18n {
  private readonly texts: I18nText;

  constructor(texts: I18nText) {
    this.texts = { ...presetTexts, ...texts };
  }

  static getUserLanguage(): I18nLanguages {
    const language = navigator.language;
    switch (language) {
      case 'ko-KR':
        return I18nLanguages.KO;
      case 'en-US':
        return I18nLanguages.EN;
      default:
        return I18nLanguages.EN;
    }
  }

  text(key: string): string {
    return this.texts[key][I18n.getUserLanguage()] as string;
  }
}

export default I18n;
