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

class I18n {
  private readonly texts: I18nText;

  constructor(texts: I18nText) {
    this.texts = texts;
  }

  static getUserLanguage(): I18nLanguages {
    const { language } = navigator;

    if (language.startsWith(I18nLanguages.KO)) {
      return I18nLanguages.KO;
    } if (language.startsWith(I18nLanguages.EN)) {
      return I18nLanguages.EN;
    }
    return I18nLanguages.EN;
  }

  text(key: string): string {
    return this.texts[key][I18n.getUserLanguage()] as string;
  }
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

function getI18n(texts: I18nText): I18n {
  return new I18n({ ...presetTexts, ...texts });
}

export { I18n, I18nLanguages, getI18n };
