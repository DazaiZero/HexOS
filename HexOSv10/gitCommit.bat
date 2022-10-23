@Echo off
git add "D:\Projects\HexOS\HexOSv10"
set message= %1
git commit -m message
git push origin dev