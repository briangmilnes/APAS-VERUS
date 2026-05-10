#!/bin/bash
set -e
cd "$(dirname "$0")"
pandoc -t beamer --slide-level=1 --pdf-engine=lualatex slidesCMUCS.md -o slidesCMUCS.pdf
echo "-> $(pwd)/slidesCMUCS.pdf"

pandoc --slide-level=1 slidesCMUCS.md -o /tmp/slides_raw.pptx
python3 - <<'EOF'
import zipfile, re

src = "/tmp/slides_raw.pptx"
dst = "/home/milnes/projects/APAS-VERUS/lectures/slidesCMUCS.pptx"
TITLE_SZ = "3200"
BODY_SZ  = "1800"
CODE_SZ  = "1100"

def extract_element(tag, text):
    """Return the first complete <tag .../> or <tag ...>...</tag> element."""
    m = re.search(f'<{tag}[^>]*>.*?</{tag}>', text, flags=re.DOTALL)
    if m:
        return m.group(0)
    m = re.search(f'<{tag}[^>]*/>', text)
    if m:
        return m.group(0)
    return ''

def fix_multiline_runs(text):
    """Split <a:t> runs that contain newlines into one <a:p> per line."""
    def split_para(m):
        para = m.group(0)
        t_match = re.search(r'<a:t>(.*?)</a:t>', para, flags=re.DOTALL)
        if not t_match or '\n' not in t_match.group(1):
            return para
        ppr = extract_element('a:pPr', para)
        rpr = extract_element('a:rPr', para)
        if not rpr:
            rpr = '<a:rPr/>'
        lines = t_match.group(1).split('\n')
        parts = []
        for line in lines:
            content = line if line.strip() else ' '
            parts.append(f'<a:p>{ppr}<a:r>{rpr}<a:t>{content}</a:t></a:r></a:p>')
        return ''.join(parts)
    return re.sub(r'<a:p>.*?</a:p>', split_para, text, flags=re.DOTALL)

with zipfile.ZipFile(src, 'r') as zin, zipfile.ZipFile(dst, 'w', zipfile.ZIP_DEFLATED) as zout:
    for item in zin.infolist():
        data = zin.read(item.filename)
        if item.filename.startswith('ppt/slides/slide') and item.filename.endswith('.xml'):
            text = data.decode('utf-8')
            def fix_shape(m):
                shape = m.group(0)
                is_title = 'type="title"' in shape or 'type="ctrTitle"' in shape
                sz = TITLE_SZ if is_title else BODY_SZ
                def fix_rpr_block(r):
                    rpr = r.group(0)
                    is_code = 'Courier' in rpr or 'monospace' in rpr.lower()
                    target = CODE_SZ if is_code else sz
                    if 'sz=' in rpr:
                        rpr = re.sub(r'sz="\d+"', f'sz="{target}"', rpr, count=1)
                    else:
                        rpr = rpr.replace('<a:rPr', f'<a:rPr sz="{target}"', 1)
                    if 'lang=' not in rpr:
                        rpr = rpr.replace('<a:rPr', '<a:rPr lang="en-US"', 1)
                    return rpr
                def fix_rpr_empty(r):
                    rpr = r.group(0)
                    if 'sz=' not in rpr:
                        rpr = rpr.replace('<a:rPr', f'<a:rPr sz="{sz}"', 1)
                    if 'lang=' not in rpr:
                        rpr = rpr.replace('<a:rPr', '<a:rPr lang="en-US"', 1)
                    return rpr
                shape = re.sub(r'<a:rPr[^>]*>.*?</a:rPr>', fix_rpr_block, shape, flags=re.DOTALL)
                shape = re.sub(r'<a:rPr[^>]*/>', fix_rpr_empty, shape)
                return shape
            text = re.sub(r'<p:sp>.*?</p:sp>', fix_shape, text, flags=re.DOTALL)
            text = fix_multiline_runs(text)
            data = text.encode('utf-8')
        zout.writestr(item, data)

slides = [n for n in zipfile.ZipFile(dst).namelist() if n.startswith('ppt/slides/slide') and n.endswith('.xml')]
print(f"Slides: {len(slides)}  ->  {dst}")
EOF
