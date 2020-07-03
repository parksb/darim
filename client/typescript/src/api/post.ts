import * as CryptoJS from 'crypto-js';

import Http from '../utils/http';
import Post from "../models/Post";

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
  const url = `${Http.baseUrl}/posts`;
  const posts = await Http.get<Post[]>(url);

  if (publicKey) {
    const encryptedPrivateKey = localStorage.getItem('key');

    if (encryptedPrivateKey) {
      const privateKey = CryptoJS.AES.decrypt(encryptedPrivateKey, publicKey).toString(CryptoJS.enc.Utf8);
      return posts.map((post) => {
        const { id, title, content, date, created_at, updated_at } = post;
        return {
          id,
          title: CryptoJS.AES.decrypt(title, privateKey).toString(CryptoJS.enc.Utf8),
          content: CryptoJS.AES.decrypt(content, privateKey).toString(CryptoJS.enc.Utf8),
          date,
          created_at,
          updated_at,
        }
      });
    }
  }

  return [];
}

async function fetchPost(id: number, publicKey: string): Promise<Post | null> {
  const url = `${Http.baseUrl}/posts/${id}`;
  const post = await Http.get<Post>(url);

  if (publicKey) {
    const encryptedPrivateKey = localStorage.getItem('key');

    if (encryptedPrivateKey) {
      const privateKey = CryptoJS.AES.decrypt(encryptedPrivateKey, publicKey).toString(CryptoJS.enc.Utf8);
      const { id, title, content, date, created_at, updated_at } = post;
      return {
        id,
        title: CryptoJS.AES.decrypt(title, privateKey).toString(CryptoJS.enc.Utf8),
        content: CryptoJS.AES.decrypt(content, privateKey).toString(CryptoJS.enc.Utf8),
        date,
        created_at,
        updated_at,
      };
    }
  }

  return null;
}

async function createPost(publicKey: string, title: string, date: string, content: string): Promise<number | null> {
  if (!title && !date && !content) {
    return null;
  }

  const encryptedPrivateKey = localStorage.getItem('key');
  if (encryptedPrivateKey) {
    const privateKey = CryptoJS.AES.decrypt(encryptedPrivateKey, publicKey).toString(CryptoJS.enc.Utf8);
    const encryptedTitle = CryptoJS.AES.encrypt(title, privateKey).toString();
    const encryptedContent = CryptoJS.AES.encrypt(content, privateKey).toString();

    const url = `${Http.baseUrl}/posts`;
    const body: CreatePostBody = {
      title: encryptedTitle,
      date,
      content: encryptedContent,
    };

    try {
      return await Http.post<CreatePostBody, number>(url, body);
    } catch (e) {
      alert('Failed to save post');
    }
  }

  return null;
}

async function updatePost(publicKey: string, id: number, title?: string, date?: string, content?: string): Promise<boolean> {
  if (!title && !date && !content) {
    return false;
  }

  const encryptedPrivateKey = localStorage.getItem('key');
  if (encryptedPrivateKey) {
    const privateKey = CryptoJS.AES.decrypt(encryptedPrivateKey, publicKey).toString(CryptoJS.enc.Utf8);
    const encryptedTitle = title && CryptoJS.AES.encrypt(title, privateKey).toString();
    const encryptedContent = content && CryptoJS.AES.encrypt(content, privateKey).toString();

    const url = `${Http.baseUrl}/posts/${id}`;
    const body: UpdatePostBody = {
      title: encryptedTitle,
      date,
      content: encryptedContent,
    };

    try {
      return await Http.patch<UpdatePostBody, boolean>(url, body);
    } catch (e) {
      alert('Failed to save post')
    }
  }

  return false;
}

export { fetchPosts, createPost, updatePost, fetchPost };
