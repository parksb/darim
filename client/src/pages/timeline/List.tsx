import React, { useEffect, useState } from 'react';

import * as api from '../../api/post';
import ListItem from './ListItem';
import { Post, Session } from '../../models';
import { Container } from '../../components';

interface Props {
  session: Session | null;
}

const List: React.FC<Props> = ({ session }) => {
  const [posts, setPosts] = useState<Post[]>([]);

  const load = async () => {
    const post_list = await api.fetchPosts(session?.user_public_key || '');
    setPosts(post_list);
  };

  useEffect(() => {
    load();
  }, []);

  return <Container top={50}>
    {posts.map((post: Post) => {
      return <ListItem key={post.id} post={post}/>
    })}
  </Container>
};

export default List;
