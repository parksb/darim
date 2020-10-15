import React, { useState } from 'react';

import Calendar from './Calendar';
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
          <Calendar session={session} />
        );
      case ViewMode.LIST:
        return (
          <List session={session} />
        );
    }
  };

  return <Container fullWidth fullHeight>
    <TimelineHeader viewModeState={[viewMode, setViewMode]} />
    {viewTimelineBody(viewMode)}
  </Container>
};

export default Timeline;
