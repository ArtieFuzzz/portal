set shell := ["powershell.exe", "-c"]

deploy:
  wrangler pubish

dev:
  wrangler dev

cleanup:
  cargo clean