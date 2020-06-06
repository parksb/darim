import React, {useEffect, useState} from 'react';
import { useParams } from "react-router-dom";
import styled from 'styled-components';
import dayjs from "dayjs";

import * as api from './api';
import { Post } from '../../models';
import { Section, TextField } from '../../components';

const Container = styled(Section)`
  margin-bottom: 30px;
`;

const TitleTextField = styled(TextField)`
  font-size: 24px;
  font-weight: bold;
`;

const TextArea = styled.textarea`
  max-width: 100%;
  margin-top: 30px;
  padding: 5px;
  font-family: sans-serif;
  font-size: 16px;
  border: 0;
  border-bottom: 1px solid #000000;
  resize: none;
  line-height: 150%;
`;

const DateField = styled(({ ...other }) => <input type='date' {...other} />)`
  padding: 15px 5px 10px 5px;
  font-family: sans-serif;
  font-size: 16px;
  border: 0;
  border-bottom: 1px solid #000000;
`;

const Post: React.FC = () => {
  const [title, setTitle] = useState('');
  const [date, setDate] = useState('');
  const [content, setContent] = useState('');
  const [originalPost, setOriginalPost] = useState<Post | null>(null);

  const { id } = useParams();

  const getFormattedDate = (date: string, withTime: boolean = false) => {
    const format = withTime ? 'YYYY-MM-DDT00:00:00' : 'YYYY-MM-DD';
    if (date) {
      return dayjs(date).format(format);
    }
    return dayjs().format(format);
  };

  const load = async () => {
    const post = await api.fetchPost(id);

    if (post) {
      const { title, content, date } = post;
      setOriginalPost(post);
      setTitle(title);
      setDate(date);
      setContent(content);
    }
  };

  const updatePost = async (newTitle: string | null = null, newDate: string | null = null, newContent: string | null = null) => {
    if (id && originalPost) {
      if (
        newTitle && newTitle !== originalPost.title ||
        newDate && newDate !== getFormattedDate(originalPost.date) ||
        newContent && newContent !== originalPost.content
      ) {
        const dateWithTime = getFormattedDate(date, true);
        const result = await api.updatePost(id, title, dateWithTime, content);

        if (!result) {
          alert('Failed to save post');
        }
      }
    }
  };

  useEffect(() => {
    if (id) {
      load();
    }
  }, []);

  return <Container>
    <TitleTextField placeholder='Title' value={title} onBlur={({ target: { value } }) => updatePost(value)} onChange={({ target: { value } }) => setTitle(value)} />
    <DateField value={getFormattedDate(date)} onBlur={({ target: { value } }) => updatePost(null, value)} onChange={({ target: { value } }) => setDate(value)} />
    <TextArea rows={30} placeholder='Content' value={content} onBlur={({ target: { value } }) => updatePost(null, null, value)} onChange={({ target: { value } }) => setContent(value)} />
  </Container>
};

export default Post;
