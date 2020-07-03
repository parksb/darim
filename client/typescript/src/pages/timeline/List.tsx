import React, { useEffect, useState } from 'react';

import * as api from '../../api/post';
import ListItem from './ListItem';
import {Post, Session} from '../../models';
import { Section } from "../../components";

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

  return <Section>
    {posts.map((post: Post) => {
      return <ListItem key={post.id} post={post}/>
    })}
  </Section>
};

export default List;
