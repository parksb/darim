import SHA3 from 'crypto-js/sha3';

import Http from '../utils/http';
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
    if (e.message === '404') {
      alert('Incorrect username or password');
    } else {
      alert('Failed to sign in');
    }
  }

  return null;
}

async function logout(): Promise<boolean | null> {
  const url = `${Http.baseUrl}/auth/logout`;

  try {
    return await Http.postWithoutBody<boolean>(url);
  } catch (e) {
    alert('Failed to sign out')
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
    alert('Failed to set token');
  }

  return null;
}

export { fetchSession, login, logout, setSignUpToken };
