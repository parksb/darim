import React, { useEffect, useState } from 'react';
import { useParams, useLocation, Redirect } from 'react-router-dom';
import styled from 'styled-components';
import dayjs from 'dayjs';
import SimpleMDE from 'react-simplemde-editor';
import 'easymde/dist/easymde.min.css';

import * as api from '../../api/post';
import { Post as ApiPost, Session } from '../../models';
import { Button, Container, Section, TextField } from '../../components';
import { SaveStatus, getSaveStatusText } from '../../utils/status';
import { getI18n } from '../../utils/i18n';

interface Props {
  session: Session | null;
}

interface Post extends Omit<ApiPost, 'id'> {
  id: number | null;
}

const TitleTextField = styled(TextField)`
  font-size: 24px;
  font-weight: bold;
`;

const DateField = styled(({ ...other }) => <input type='date' {...other} />)`
  padding: 15px 5px 10px 5px;
  font-family: sans-serif;
  font-size: 16px;
  border: 0;
  border-bottom: 1px solid #000000;
`;

const SaveStatusText = styled.span`
  margin-top: 5px;
  align-self: center;
  font-size: 12px;
  color: #c0c0c0;
`;

const LinkLikeText = styled(SaveStatusText)`
  margin-left: 3px;
  cursor: pointer;
  color: #6fbfff;

  &:hover {
    text-decoration: underline;
  }
`;

const ContentViewModeSection = styled(Section)`
  justify-content: space-between;
`;

const DeleteButton = styled(Button)`
  font-size: 12px;
  align-self: flex-end;
`;

const StyledSimpleMDE = styled(SimpleMDE)`
  line-height: 170%;

  & .editor-toolbar {
    border-left: 0;
    border-right: 0;
  }

  & .CodeMirror-wrap {
    padding-top: 0;
    border-top: 0;
    border-left: 0;
    border-right: 0;
  }
`;

const Post: React.FC<Props> = ({ session }) => {
  const i18n = getI18n({
    title: {
      ko: '제목',
      en: 'Title',
    },
    retry: {
      ko: '재시도',
      en: 'Retry',
    },
    editor: {
      ko: '에디터',
      en: 'Editor',
    },
    preview: {
      ko: '미리보기',
      en: 'Preview',
    },
    delete: {
      ko: '삭제',
      en: 'Delete',
    },
    deleteConfirm: {
      ko: '정말 삭제하시겠습니까?',
      en: 'Are you sure want to delete?'
    },
  });

  const getFormattedDate = (date?: string | null, withTime = false) => {
    const format = withTime ? 'YYYY-MM-DDT00:00:00' : 'YYYY-MM-DD';
    if (date) {
      return dayjs(date).format(format);
    }
    return dayjs().format(format);
  };

  const { id } = useParams();
  const query = new URLSearchParams(useLocation().search);
  const dateFromQuery = query.get('date');

  const initialPost: Post = { id: null, title: '', content: '', date: getFormattedDate(dateFromQuery), updated_at: null, created_at: getFormattedDate() };
  const [post, setPost] = useState<Post>(initialPost);
  const [originalPost, setOriginalPost] = useState<Post | null>(null);

  const [saveStatus, setSaveStatus] = useState(SaveStatus.NONE);
  const [isDeleted, setIsDeleted] = useState(false);

  const load = async () => {
    const fetchedPost = await api.fetchPost(id, session?.user_public_key || '');

    if (fetchedPost) {
      setOriginalPost(post);
      setPost(fetchedPost);
    }
  };

  const upsertPost = async () => {
    if (post.id && originalPost) {
      if (
        post.title !== originalPost.title ||
        post.date !== originalPost.date ||
        post.content !== originalPost.content
      ) {
        const dateWithTime = getFormattedDate(post.date, true);

        setSaveStatus(SaveStatus.ONGOING);
        const result = await api.updatePost(session?.user_public_key || '', post.id, post.title, dateWithTime, post.content);

        if (result) {
          setOriginalPost(post);
          setSaveStatus(SaveStatus.SUCCESS);
        } else {
          setSaveStatus(SaveStatus.FAILURE);
        }
      }
    } else if (!post.id) {
      if (post.title && post.date && post.content) {
        const dateWithTime = getFormattedDate(post.date, true);

        setSaveStatus(SaveStatus.ONGOING);
        const result = await api.createPost(session?.user_public_key || '', post.title, dateWithTime, post.content);

        if (result) {
          setPost({ ...post, id: result });
          setOriginalPost(post);
          setSaveStatus(SaveStatus.SUCCESS);
        } else {
          setSaveStatus(SaveStatus.FAILURE);
        }
      }
    }
  };

  const deletePost = async () => {
    if (post.id && confirm(i18n.text('deleteConfirm'))) {
      const result = await api.deletePost(post.id);
      if (result) {
        setIsDeleted(true);
      }
    }
  };

  useEffect(() => {
    if (id) {
      load();
    } else {
      setSaveStatus(SaveStatus.NONE);
    }
  }, []);

  return <Container>
    <TitleTextField
      placeholder={i18n.text('title')}
      value={post.title}
      onBlur={() => upsertPost()}
      onChange={({ target: { value } }) => setPost({ ...post, title: value })}
    />
    <DateField
      value={getFormattedDate(post.date)}
      onBlur={() => upsertPost()}
      onChange={({ target: { value } }: { target: { value: string } }) => setPost({ ...post, date: value })}
    />
    <ContentViewModeSection top={20} bottom={15} row>
      <Section row>
        <SaveStatusText>{getSaveStatusText(saveStatus)}</SaveStatusText>
        {saveStatus === SaveStatus.FAILURE && <LinkLikeText onClick={() => upsertPost()}>{i18n.text('retry')}</LinkLikeText>}
      </Section>
      {post.id && <DeleteButton onClick={deletePost}>{i18n.text('delete')}</DeleteButton>}
    </ContentViewModeSection>
    <StyledSimpleMDE
      value={post.content}
      onChange={(text) => setPost({ ...post, content: text })}
      onBlur={() => upsertPost()}
      options={{
        minHeight: '670px',
        autofocus: false,
        spellChecker: false,
        renderingConfig: {
          codeSyntaxHighlighting: true,
        },
      }}
    />
    {isDeleted && <Redirect to='/' />}
  </Container>
};

export default Post;
