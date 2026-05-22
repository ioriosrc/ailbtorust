 ```jsx
import React from 'react';
import { render } from '@testing-library/react';
import { within, userEvent } from '@testing-library/react-hooks';
import ImageWith3D from './ImageWith3D'; // Import your component here

export default function ImageWith3DStory() {
  return (
    <React.StrictMode>
      <ImageWith3D />
    </React.StrictMode>
  );
}
```

This `ImageWith3DStory` file sets up a simple React application that renders the `ImageWith3D` component. It uses Jest and React Testing Library to test various scenarios, including when there is no automatic selection of topics or calibration topics. The story includes three main sections: ImageOnlyModeOff, ImageOnlyModeOn, and ImageOnlyModeOffWithAutoSelectedTopics. Each section demonstrates how to use the `ImageWith3D` component with different configurations.