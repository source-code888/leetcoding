def commit_and_push [branch: string, title: string, body: string] {
   git add .
   git commit -m $'($title)\n\n($body)'
   git push origin $branch
}