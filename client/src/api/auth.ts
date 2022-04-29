/* eslint-disable import/no-unresolved */
import SHA3 from 'crypto-js/sha3';

import Http from '../utils/http';
import { getI18n } from '../utils/i18n';
import { serverBaseUrl } from '../constants';
import ActiveUserSession from '../models/ActiveUserSession';

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

async function fetchActiveUserSessions(accessToken: string): Promise<ActiveUserSession[]> {
  const url = `${serverBaseUrl}/auth/token`;

  try {
    return Http.get<ActiveUserSession[]>(url, accessToken);
  } catch (e) {
    // do nothing
  }

  return [];
}

async function fetchAccessToken(): Promise<string | null> {
  const url = `${serverBaseUrl}/auth/token/access`;

  try {
    return Http.post<string, string | null>(url);
  } catch (e) {
    // do nothing
  }

  return null;
}

async function login(email: string, password: string): Promise<string | null> {
  const url = `${serverBaseUrl}/auth/token`;
  const hashedPassword = SHA3(password, { outputLength: 512 }).toString();

  const body: LoginBody = {
    email,
    password: hashedPassword,
  };

  try {
    return Http.post<LoginBody, string>(url, body);
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

    if (e instanceof Error && e.message === '404') {
      alert(i18n.text('error404'));
    } else {
      alert(i18n.text('error'));
    }
  }

  return null;
}

async function logout(): Promise<boolean | null> {
  const url = `${serverBaseUrl}/auth/token`;

  try {
    return await Http.delete<boolean>(url);
  } catch (e) {
    const i18n = getI18n({
      error: {
        ko: '로그아웃에 실패했습니다',
        en: 'Failed to sign out',
      },
    });

    alert(i18n.text('error'));
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
    return Http.post<SetSignUpTokenBody, string>(url, body);
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
    return Http.post<SetPasswordTokenBody, boolean>(url, body);
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

    if (e instanceof Error && e.message === '404') {
      alert(i18n.text('error404'));
    } else {
      alert(i18n.text('error'));
    }
  }

  return null;
}

export {
  login, logout, setSignUpToken, setPasswordToken, fetchAccessToken, fetchActiveUserSessions,
};
