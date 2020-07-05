import SHA3 from 'crypto-js/sha3';

import Http from '../utils/http';
import I18n from "../utils/i18n";
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

async function fetchSession(): Promise<Session | null> {
  const url = `${Http.baseUrl}/auth`;

  try {
    return await Http.get<Session>(url);
  } catch (e) {
    return null;
  }
}

async function login(email: string, password: string): Promise<Session | null> {
  const url = `${Http.baseUrl}/auth/login`;
  const hashedPassword = SHA3(password, { outputLength: 512 }).toString();

  const body: LoginBody = {
    email,
    password: hashedPassword,
  };

  try {
    return await Http.post<LoginBody, Session>(url, body);
  } catch (e) {
    const i18n = new I18n({
      error404: {
        'ko-KR': '이메일이나 비밀번호가 잘못되었습니다',
        'en-US': 'Incorrect email or password',
      },
      error: {
        'ko-KR': '로그인에 실패했습니다',
        'en-US': 'Failed to sign in',
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
  const url = `${Http.baseUrl}/auth/logout`;

  try {
    return await Http.postWithoutBody<boolean>(url);
  } catch (e) {
    const i18n = new I18n({
      error: {
        'ko-KR': '로그아웃에 실패했습니다',
        'en-US': 'Failed to sign out',
      },
    });

    alert(i18n.text('error'))
  }

  return null;
}

async function setSignUpToken(name: string, email: string, password: string, avatarUrl?: string): Promise<boolean | null> {
  const url = `${Http.baseUrl}/auth/token`;
  const hashedPassword = SHA3(password, { outputLength: 512 }).toString();

  const body: SetSignUpTokenBody = {
    name,
    email,
    password: hashedPassword,
    avatar_url: avatarUrl || null,
  };

  try {
    return await Http.post<SetSignUpTokenBody, boolean>(url, body);
  } catch (e) {
    const i18n = new I18n({
      error: {
        'ko-KR': '토큰 설정에 실패했습니다',
        'en-US': 'Failed to set token',
      },
    });

    alert(i18n.text('error'));
  }

  return null;
}

export { fetchSession, login, logout, setSignUpToken };
