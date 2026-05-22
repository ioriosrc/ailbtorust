 ```json
{
  "name": "Image Mode Empty Layout",
  "description": "Test for empty image mode layout when certain topics are not available.",
  "type": "component",
  "parameters": {
    "type": { "title": "Type", "values": ["no-topics", "no-image-topics", "no-messages", "image-topic-DNE"] }
  },
  "testCases": [
    {
      "id": "type-no-topics-empty-state",
      "name": "Test for empty state when no topics are available",
      "parameters": { "type": "no-topics" },
      "assertions": [
        {
          "title": "Check if the image mode layout is rendered correctly without any topics",
          "selector": ".image-mode-layout",
          "expectedContent": ""
        }
      ]
    },
    {
      "id": "type-no-image-topics-empty-state",
      "name": "Test for empty state when no image topic is available",
      "parameters": { "type": "no-image-topics" },
      "assertions": [
        {
          "title": "Check if the image mode layout is rendered correctly without an image topic",
          "selector": ".image-mode-layout",
          "expectedContent": ""
        }
      ]
    },
    {
      "id": "type-no-messages-empty-state",
      "name": "Test for empty state when no messages are available",
      "parameters": { "type": "no-messages" },
      "assertions": [
        {
          "title": "Check if the image mode layout is rendered correctly without any messages",
          "selector": ".image-mode-layout",
          "expectedContent": ""
        }
      ]
    },
    {
      "id": "type-image-topic-DNE-empty-state",
      "name": "Test for empty state when only the image topic does not exist",
      "parameters": { "type": "image-topic-DNE" },
      "assertions": [
        {
          "title": "Check if the image mode layout is rendered correctly without an image topic",
          "selector": ".image-mode-layout",
          "expectedContent": ""
        }
      ]
    }
  ]
}
```