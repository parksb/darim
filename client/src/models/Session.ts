import User from './User';

interface Session {
  user: User;
  accessToken: string;
}

export default Session;
