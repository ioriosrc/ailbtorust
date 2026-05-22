 ```javascript
describe("messagePathParser", () => {
  const parseMessagePath = (path: string): MessagePathResult | undefined => {
    // Implementation of the path parsing logic
    return undefined;
  };

  it("returns the parsed message path", () => {
    expect(parseMessagePath('/tf.transforms[:]{child_frame_id=="foo"}')).toEqual({
      nameLoc: 123,
      valueLoc: 0,
      repr: "",
      operator: "==",
      pathParts: [
        { type: "name", name: "tf" },
        {
          type: "filter",
          path: ["transforms"],
          value: undefined,
          nameLoc: 124,
          valueLoc: 0,
          repr: "",
          operator: ":",
        },
        {
          type: "name",
          name: "child_frame_id",
          repr: "child_frame_id",
        },
        { type: "filter", path: ["=="], value: '"foo"', nameLoc: 147, valueLoc: 0, repr: "", operator: "=" },
      ],
    });
  });

  it("returns an error for invalid paths", () => {
    expect(parseMessagePath('/tf.transforms[:{child_frame_id=="foo"}')).toEqual({
      message: "Invalid path",
      location: [1, 3],
    });
  });
});
```