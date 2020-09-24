import SHA3 from 'crypto-js/sha3';
import { Http } from 'snowball-js';

import { getI18n } from '../utils/i18n';
import { serverBaseUrl } from '../constants';
import { Session } from '../models';

interface LoginBody {
  email: string;
  password: string;
}

interface SetSignUpTokenBody {
  name: string;
  email: string;
  password: string;
  avatar_url: string | null;
}

interface SetPasswordTokenBody {
  email: string;
}

async function fetchSession(): Promise<Session | null> {
  const url = `${serverBaseUrl}/auth`;

  try {
    return await Http.get<Session>(url);
  } catch (e) {
    return null;
  }
}

async function refreshSession(): Promise<Session | null> {
  const url = `${serverBaseUrl}/auth`;

  try {
    return await Http.postWithoutBody<Session>(url);
  } catch (e) { /**/ }

  return null;
}

async function login(email: string, password: string): Promise<Session | null> {
  const url = `${serverBaseUrl}/auth/login`;
  const hashedPassword = SHA3(password, { outputLength: 512 }).toString();

  const body: LoginBody = {
    email,
    password: hashedPassword,
  };

  try {
    return await Http.post<LoginBody, Session>(url, body);
  } catch (e) {
    const i18n = getI18n({
      error404: {
        ko: '이메일이나 비밀번호가 잘못되었습니다',
        en: 'Incorrect email or password',
      },
      error: {
        ko: '로그인에 실패했습니다',
        en: 'Failed to sign in',
      },
    });

    if (e.message === '404') {
      alert(i18n.text('error404'));
    } else {
      alert(i18n.text('error'));
    }
  }

  return null;
}

async function logout(): Promise<boolean | null> {
  const url = `${serverBaseUrl}/auth/logout`;

  try {
    return await Http.postWithoutBody<boolean>(url);
  } catch (e) {
    const i18n = getI18n({
      error: {
        ko: '로그아웃에 실패했습니다',
        en: 'Failed to sign out',
      },
    });

    alert(i18n.text('error'))
  }

  return null;
}

async function setSignUpToken(name: string, email: string, password: string, avatarUrl?: string): Promise<string | null> {
  const url = `${serverBaseUrl}/auth/token/sign_up`;
  const hashedPassword = SHA3(password, { outputLength: 512 }).toString();

  const body: SetSignUpTokenBody = {
    name,
    email,
    password: hashedPassword,
    avatar_url: avatarUrl || null,
  };

  try {
    return await Http.post<SetSignUpTokenBody, string>(url, body);
  } catch (e) {
    const i18n = getI18n({
      error: {
        ko: '토큰 설정에 실패했습니다',
        en: 'Failed to set token',
      },
    });

    alert(i18n.text('error'));
  }

  return null;
}

async function setPasswordToken(email: string): Promise<boolean | null> {
  const url = `${serverBaseUrl}/auth/token/password`;
  const body: SetPasswordTokenBody = {
    email,
  };

  try {
    return await Http.post<SetPasswordTokenBody, boolean>(url, body);
  } catch (e) {
    const i18n = getI18n({
      error404: {
        ko: '계정을 찾을 수 없습니다',
        en: 'Cannot find the account',
      },
      error: {
        ko: '토큰 설정에 실패했습니다',
        en: 'Failed to set token',
      },
    });

    if (e.message === '404') {
      alert(i18n.text('error404'));
    } else {
      alert(i18n.text('error'));
    }
  }

  return null;
}

export { fetchSession, refreshSession, login, logout, setSignUpToken, setPasswordToken };
