import sys
import cv2

# encoder(for mp4)
fourcc = cv2.VideoWriter_fourcc('m', 'p', '4', 'v')
# output file name, encoder, fps, size(fit to image size)
video = cv2.VideoWriter('video.mp4',fourcc, 20.0, (200, 200))

if not video.isOpened():
    print("can't be opened")
    sys.exit()

for i in range(0, 134):
    # hoge0000.png, hoge0001.png,..., hoge0090.png
    img = cv2.imread('./sean%d.png' % i)

    # can't read image, escape
    if img is None:
        print("can't read")
        break

    # add
    video.write(img)
    print(i)

video.release()
print('written')