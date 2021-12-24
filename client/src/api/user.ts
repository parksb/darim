import SHA3 from 'crypto-js/sha3';

import Http from '../utils/http';
import { getI18n } from '../utils/i18n';
import { serverBaseUrl } from '../constants';
import User from '../models/User';

interface CreateUserBody {
  user_public_key: string;
  token_key: string;
  token_pin: string;
  recaptcha_token: string;
}

interface UpdateUserBody {
  name?: string;
  password?: string;
  avatar_url?: string;
}

interface ResetPasswordBody {
  email: string;
  token_id: string;
  temporary_password: string;
  new_password: string;
}

async function fetchUser(accessToken: string): Promise<User | null> {
  const url = `${serverBaseUrl}/users/me`;

  try {
    return await Http.get<User>(url, accessToken);
  } catch (e) {
    const i18n = getI18n({
      error: {
        ko: '사용자 정보를 가져오지 못했습니다',
        en: 'Failed to fetch user information',
      },
    });

    alert(i18n.text('error'));
  }

  return null;
}

async function createUser(userPublicKey: string, tokenKey: string, tokenPin: string, recaptchaToken: string): Promise<boolean | null> {
  const url = `${serverBaseUrl}/users`;

  const body: CreateUserBody = {
    user_public_key: userPublicKey,
    token_key: tokenKey,
    token_pin: tokenPin,
    recaptcha_token: recaptchaToken,
  };

  try {
    return await Http.post<CreateUserBody, boolean>(url, body);
  } catch (e) {
    const i18n = getI18n({
      error: {
        ko: '이메일 인증에 실패했습니다',
        en: 'Failed to verify email',
      },
    });

    alert(i18n.text('error'));
  }

  return null;
}

async function updateUser(userId: string, accessToken: string, password?: string, name?: string, avatar?: string): Promise<boolean | null> {
  const url = `${serverBaseUrl}/users/${userId}`;

  const body: UpdateUserBody = {
    password: password ? SHA3(password, { outputLength: 512 }).toString() : undefined,
    name: name,
    avatar_url: avatar,
  };

  try {
    return await Http.patch<UpdateUserBody, boolean>(url, body, accessToken);
  } catch (e) {
    const i18n = getI18n({
      error: {
        ko: '변경에 실패했습니다',
        en: 'Failed to update',
      },
    });

    alert(i18n.text('error'));
  }

  return null;
}

async function resetPassword(email: string, tokenId: string, temporaryPassword: string, newPassword: string): Promise<boolean | null> {
  const url = `${serverBaseUrl}/users/password`;

  const body: ResetPasswordBody = {
    email,
    token_id: tokenId,
    temporary_password: temporaryPassword,
    new_password: SHA3(newPassword, { outputLength: 512 }).toString(),
  };

  try {
    return await Http.post<ResetPasswordBody, boolean>(url, body);
  } catch (e) {
    const i18n = getI18n({
      error: {
        ko: '변경에 실패했습니다',
        en: 'Failed to reset',
      },
    });

    alert(i18n.text('error'));
  }

  return null;
}

export { fetchUser, createUser, updateUser, resetPassword };
