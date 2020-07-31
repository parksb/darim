import Post from '../models/Post';
import Http from '../utils/http';
import Secret from '../utils/secret';
import I18n from '../utils/i18n';

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

async function fetchPosts(publicKey: string): Promise<Post[]> {
  if (!publicKey) {
    return [];
  }

  try {
    const url = `${Http.baseUrl}/posts`;
    const posts = await Http.get<Post[]>(url);

    const keyFromLocalStorage = Secret.getPrivateKeyFromLocalStorage();
    const encryptedPrivateKey = Secret.parseUtf8ToString(keyFromLocalStorage);
    const privateKey = Secret.decryptAES(encryptedPrivateKey, publicKey);

    if (privateKey) {
      return posts.map((post) => {
        const { id, title, content, date, created_at, updated_at } = post;
        const decryptedTitle = Secret.decryptAES(title, privateKey);
        const decryptedContent = Secret.decryptAES(content, privateKey);

        if (decryptedTitle && decryptedContent) {
          return {
            id,
            title: decryptedTitle,
            content: decryptedContent,
            date,
            created_at,
            updated_at,
          } as Post;
        }

        return null;
      }).filter((post) => post !== null) as Post[];
    } else {
      return [];
    }
  } catch (e) {
    return [];
  }
}

async function fetchPost(id: number, publicKey: string): Promise<Post | null> {
  const url = `${Http.baseUrl}/posts/${id}`;
  const post = await Http.get<Post>(url);

  try {
    const keyFromLocalStorage = Secret.getPrivateKeyFromLocalStorage();
    const encryptedPrivateKey = Secret.parseUtf8ToString(keyFromLocalStorage);
    const privateKey = Secret.decryptAES(encryptedPrivateKey, publicKey);

    if (privateKey) {
      const { id, title, content, date, created_at, updated_at } = post;
      return {
        id,
        title: Secret.decryptAES(title, privateKey),
        content: Secret.decryptAES(content, privateKey),
        date,
        created_at,
        updated_at,
      };
    } else {
      return null;
    }
  } catch (e) {
    return null;
  }
}

async function createPost(publicKey: string, title: string, date: string, content: string): Promise<number | null> {
  if (!title && !date && !content) {
    return null;
  }

  try {
    const keyFromLocalStorage = Secret.getPrivateKeyFromLocalStorage();
    const encryptedPrivateKey = Secret.parseUtf8ToString(keyFromLocalStorage);

    const privateKey = Secret.decryptAES(encryptedPrivateKey, publicKey);
    const encryptedTitle = Secret.encryptAES(title, privateKey);
    const encryptedContent = Secret.encryptAES(content, privateKey);

    const url = `${Http.baseUrl}/posts`;
    const body: CreatePostBody = {
      title: encryptedTitle,
      date,
      content: encryptedContent,
    };

    return await Http.post<CreatePostBody, number>(url, body);
  } catch (e) {
    const i18n = new I18n({
      error: {
        ko: '저장에 실패했습니다',
        en: 'Failed to save',
      },
    });

    alert(i18n.text('error'));
  }

  return null;
}

async function updatePost(publicKey: string, id: number, title?: string, date?: string, content?: string): Promise<boolean> {
  if (!title && !date && !content) {
    return false;
  }

  try {
    const keyFromLocalStorage = Secret.getPrivateKeyFromLocalStorage();
    const encryptedPrivateKey = Secret.parseUtf8ToString(keyFromLocalStorage);

    const privateKey = Secret.decryptAES(encryptedPrivateKey, publicKey);
    const encryptedTitle = title && Secret.encryptAES(title, privateKey);
    const encryptedContent = content && Secret.encryptAES(content, privateKey);

    const url = `${Http.baseUrl}/posts/${id}`;
    const body: UpdatePostBody = {
      title: encryptedTitle,
      date,
      content: encryptedContent,
    };

    return await Http.patch<UpdatePostBody, boolean>(url, body);
  } catch (e) {
    const i18n = new I18n({
      error: {
        ko: '저장에 실패했습니다',
        en: 'Failed to save',
      },
    });

    alert(i18n.text('error'));
  }

  return false;
}

async function deletePost(id: number): Promise<boolean> {
  try {
    const url = `${Http.baseUrl}/posts/${id}`;
    return await Http.delete<boolean>(url);
  } catch (e) {
     const i18n = new I18n({
      error: {
        ko: '삭제에 실패했습니다',
        en: 'Failed to delete',
      },
    });

    alert(i18n.text('error'));
  }

  return false;
}

export { fetchPosts, createPost, updatePost, fetchPost, deletePost };
