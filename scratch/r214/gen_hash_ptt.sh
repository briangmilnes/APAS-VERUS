#!/bin/bash
# r214: emit one hash-set-iterator PTT body (prophetic model) to stdout.
# Usage: gen_hash_ptt.sh <test_name> <pattern: loop|for> <uses (;-separated)> <requires or -> <setup stmt> <iter expr> <elem type>
NAME=$1; PAT=$2; USES=$3; REQ=$4; SETUP=$5; ITER=$6; T=$7
echo "test_verify_one_file! {"
echo "    #[test] $NAME verus_code! {"
echo "        use vstd::prelude::*;"
echo "        use vstd::std_specs::iter::*;"
echo "        use vstd::std_specs::hash::into_iter_hash_keys;"
IFS=';' read -ra U <<< "$USES"; for u in "${U[@]}"; do echo "        use $u;"; done
echo ""
if [ "$REQ" = "-" ]; then echo "        fn test_body() {"; else
echo "        fn test_body()"; echo "            requires $REQ"; echo "        {"; fi
echo "            $SETUP"
echo "            let it0 = $ITER;"
echo "            let ghost orig: Seq<$T> = into_iter_hash_keys(it0);"
echo "            let mut collected: Vec<$T> = Vec::new();"
if [ "$PAT" = "loop" ]; then
cat <<EOF
            let mut it = it0;
            let ghost mut pos: int = 0;
            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
                    collected.len() == pos,
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == orig[i],
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    Some(x) => {
                        proof {
                            pos = pos + 1;
                            assert(orig[old_pos] == *x);
                        }
                        collected.push(*x);
                    },
                    None => {
                        assert(pos == orig.len());
                        assert(collected@ =~= orig);
                        break;
                    },
                }
            }
        }
    } => Ok(())
}
EOF
else
cat <<EOF
            for x in it: it0
                invariant
                    it.seq().unref() == orig,
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == *it.seq()[i],
            {
                collected.push(*x);
            }
            assert(collected@ =~= orig);
        }
    } => Ok(())
}
EOF
fi
