enum I18nLanguages {
  KO_KR = 'ko-KR',
  EN_US = 'en-US',
}

interface I18nText {
  [key: string]: {
    'ko-KR': string
    'en-US': string
  };
}

const presetTexts: I18nText = {
  email: {
    'ko-KR': '이메일',
    'en-US': 'Email',
  },
  password: {
    'ko-KR': '비밀번호',
    'en-US': 'Password',
  },
  name: {
    'ko-KR': '이름',
    'en-US': 'Name',
  },
  avatar: {
    'ko-KR': '프로필 사진 URL',
    'en-US': 'Avatar URL',
  },
  save: {
    'ko-KR': '저장',
    'en-US': 'Save',
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
        return I18nLanguages.KO_KR;
      case 'en-US':
        return I18nLanguages.EN_US;
      default:
        return I18nLanguages.KO_KR;
    }
  }

  text(key: string): string {
    return this.texts[key][I18n.getUserLanguage()] as string;
  }
}

export default I18n;
