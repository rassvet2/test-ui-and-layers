# test-ui-and-layers
A test app on UI and RenderLayers.

## Scene overview
### Objects
1. Blue plane - background, layer = 3
2. Green cube - scene object, layer = 2
3. Red plane - foreground, layer = 1

### Cameras
1. Background camera - priority = 1, layer = 3
2. Scene camera - priority = 2, layer = 2,
3. Foreground camera - priority = 3, layer = 1

## Render Result
<img width="400" alt="pic-app" src="https://user-images.githubusercontent.com/50271984/193084346-7c2ca7f2-8aec-4323-b8e4-b889f44fedf6.png">

|Background|Foreground|Scene|
|:-|:-|:-|
|<img width="100" alt="pic-bg" src="https://user-images.githubusercontent.com/50271984/193084353-afe9a9fa-8d85-45a7-af4a-f24324bc0fad.png">|<img width="100" alt="pic-fg" src="https://user-images.githubusercontent.com/50271984/193084355-913ab286-f6a7-4f0d-93aa-ca85f663fb9e.png">|<img width="100" alt="pic-scene" src="https://user-images.githubusercontent.com/50271984/193084361-b56887ea-b2db-4647-8b81-468293b84080.png">|

## Expected Result (using GIMP)
<img width="400" alt="pic-expected" src="https://user-images.githubusercontent.com/50271984/193086529-bde02547-2e26-4a0b-9151-0d7f2da76c2f.png">
