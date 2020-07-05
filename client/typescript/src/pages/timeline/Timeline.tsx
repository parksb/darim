import React, { useState } from 'react';

import Calendar from "./Calendar";
import List from './List';
import TimelineHeader from './TimelineHeader';
import { Session } from '../../models';
import { Container } from '../../components'

interface Props {
  session: Session | null;
}

export enum ViewMode {
  CALENDAR,
  LIST,
}

const Timeline: React.FC<Props> = ({ session }) => {
  const [viewMode, setViewMode] = useState(ViewMode.CALENDAR);

  const viewTimelineBody = (mode: ViewMode) => {
    switch (mode) {
      case ViewMode.CALENDAR:
        return (
          <Container fullWidth>
            <Calendar session={session} />
          </Container>
        );
      case ViewMode.LIST:
        return (
          <Container>
            <List session={session} />
          </Container>
        );
    }
  };

  return <Container bottom={30} fullWidth>
    <TimelineHeader viewModeState={[viewMode, setViewMode]} />
    {viewTimelineBody(viewMode)}
  </Container>
};

export default Timeline;
