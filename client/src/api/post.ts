import Storage from '../utils/storage';
import Http from '../utils/http';
import Secret from '../utils/secret';
import { getI18n } from '../utils/i18n';
import { serverBaseUrl, localStoragePrivateKey } from '../constants';
import { Post, SummarizedPost } from '../models';

interface CreatePostBody {
  title: string;
  date: string;
  content: string;
}

interface UpdatePostBody {
  title?: string;
  date?: string;
  content?: string;
}

async function fetchPosts(publicKey: string, accessToken: string): Promise<SummarizedPost[]> {
  if (!publicKey) {
    return [];
  }

  try {
    const url = `${serverBaseUrl}/summarized_posts`;
    const posts = await Http.get<SummarizedPost[]>(url, accessToken);

    const keyFromLocalStorage = Storage.get(localStoragePrivateKey);
    if (!keyFromLocalStorage) {
      throw new Error('Failed to load posts');
    }

    const encryptedPrivateKey = Secret.parseUtf8ToString(keyFromLocalStorage);
    const privateKey = Secret.decryptAES(encryptedPrivateKey, publicKey);

    if (privateKey) {
      return posts.map((post): SummarizedPost | null => {
        const { id, title, date } = post;
        const decryptedTitle = Secret.decryptAES(title, privateKey);

        if (decryptedTitle) {
          return {
            id,
            title: decryptedTitle,
            date,
          };
        }

        return null;
      }).filter((post) => post !== null) as SummarizedPost[];
    }
    return [];
  } catch (e) {
    return [];
  }
}

async function fetchPost(id: number, publicKey: string, accessToken: string): Promise<Post | null> {
  const url = `${serverBaseUrl}/posts/${id}`;
  const post = await Http.get<Post>(url, accessToken);

  try {
    const keyFromLocalStorage = Storage.get(localStoragePrivateKey);
    if (!keyFromLocalStorage) {
      throw new Error('Failed to load posts');
    }

    const encryptedPrivateKey = Secret.parseUtf8ToString(keyFromLocalStorage);
    const privateKey = Secret.decryptAES(encryptedPrivateKey, publicKey);

    if (privateKey) {
      const {
        title, content, date, created_at: createdAt, updated_at: updatedAt,
      } = post;
      return {
        id,
        title: Secret.decryptAES(title, privateKey),
        content: Secret.decryptAES(content, privateKey),
        date,
        created_at: createdAt,
        updated_at: updatedAt,
      };
    }
    return null;
  } catch (e) {
    return null;
  }
}

async function createPost(publicKey: string, title: string, date: string, content: string, accessToken: string): Promise<number | null> {
  if (!title && !date && !content) {
    return null;
  }

  try {
    const keyFromLocalStorage = Storage.get(localStoragePrivateKey);
    if (!keyFromLocalStorage) {
      throw new Error('Failed to load posts');
    }

    const encryptedPrivateKey = Secret.parseUtf8ToString(keyFromLocalStorage);

    const privateKey = Secret.decryptAES(encryptedPrivateKey, publicKey);
    const encryptedTitle = Secret.encryptAES(title, privateKey);
    const encryptedContent = Secret.encryptAES(content, privateKey);

    const url = `${serverBaseUrl}/posts`;
    const body: CreatePostBody = {
      title: encryptedTitle,
      date,
      content: encryptedContent,
    };

    return await Http.post<CreatePostBody, number>(url, body, accessToken);
  } catch (e) {
    const i18n = getI18n({
      error: {
        ko: '저장에 실패했습니다',
        en: 'Failed to save',
      },
    });

    alert(i18n.text('error'));
  }

  return null;
}

async function updatePost(publicKey: string, accessToken: string, id: number, title?: string, date?: string, content?: string): Promise<boolean> {
  if (!title && !date && !content) {
    return false;
  }

  try {
    const keyFromLocalStorage = Storage.get(localStoragePrivateKey);
    if (!keyFromLocalStorage) {
      throw new Error('Failed to load posts');
    }

    const encryptedPrivateKey = Secret.parseUtf8ToString(keyFromLocalStorage);

    const privateKey = Secret.decryptAES(encryptedPrivateKey, publicKey);
    const encryptedTitle = title && Secret.encryptAES(title, privateKey);
    const encryptedContent = content && Secret.encryptAES(content, privateKey);

    const url = `${serverBaseUrl}/posts/${id}`;
    const body: UpdatePostBody = {
      title: encryptedTitle,
      date,
      content: encryptedContent,
    };

    return await Http.patch<UpdatePostBody, boolean>(url, body, accessToken);
  } catch (e) {
    const i18n = getI18n({
      error: {
        ko: '저장에 실패했습니다',
        en: 'Failed to save',
      },
    });

    alert(i18n.text('error'));
  }

  return false;
}

async function deletePost(id: number, accessToken: string): Promise<boolean> {
  try {
    const url = `${serverBaseUrl}/posts/${id}`;
    return await Http.delete<boolean>(url, accessToken);
  } catch (e) {
    const i18n = getI18n({
      error: {
        ko: '삭제에 실패했습니다',
        en: 'Failed to delete',
      },
    });

    alert(i18n.text('error'));
  }

  return false;
}

export {
  fetchPosts, createPost, updatePost, fetchPost, deletePost,
};
