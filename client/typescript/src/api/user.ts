import Http from '../utils/http';
import I18n from "../utils/i18n";

interface CreateUserBody {
  user_public_key: string;
  token_key: string;
  token_pin: string;
}

async function createUser(user_public_key: string, token_key: string, token_pin: string): Promise<boolean | null> {
  const url = `${Http.baseUrl}/users`;

  const body: CreateUserBody = {
    user_public_key,
    token_key,
    token_pin,
  };

  try {
    return await Http.post<CreateUserBody, boolean>(url, body);
  } catch (e) {
    const i18n = new I18n({
      error: {
        'ko-KR': '이메일 인증에 실패했습니다',
        'en-US': 'Failed to verify email',
      },
    });

    alert(i18n.text('error'));
  }

  return null;
}

export { createUser };
