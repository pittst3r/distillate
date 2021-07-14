const PRE: &[u8] = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width">
<title>Robbie's Home on the Web</title>
</head>

<body>
<h1><a href="/" title="Go to homepage">Robbie's Home on the&nbsp;Web</a></h1>
"#.as_bytes();

const POST: &[u8] = r#"<hr />
<p>© 2021 Robbie Pitts</p>
</body>

</html>
"#.as_bytes();

pub fn wrap_body(body: &mut Vec<u8>) {
    let mut page: Vec<u8> = vec![];

    page.extend(PRE);
    page.extend(body.iter());
    page.extend(POST);

    *body = page;
}
