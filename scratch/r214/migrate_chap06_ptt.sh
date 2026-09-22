#!/bin/bash
# r214: migrate one Chap06 graph PTT file from the pre-09.13 SetStEphIter model
# to the prophetic hash_set::Iter model. Reads the old file, writes the new one
# to stdout. Setup code, requires and test names are copied unchanged; each
# iterator loop is replaced by the iterator_ptt_standard.rs template with
# orig = into_iter_hash_keys(it0). Graph iterators that no longer exist
# (iter_vertices/iter_arcs/iter_edges outside DirGraphStEph) map to the
# accessor's SetStEph::iter().
# Usage: migrate_chap06_ptt.sh <old file> <keep_iter_fns: yes|no>
IN=$1; KEEP=$2
skip=0
emit_body() { # $1 loop|for  $2 iter expr  $3 elem type
  local PAT=$1 ITER=$2 T=$3
  echo "            let it0 = $ITER;"
  echo "            let ghost orig: Seq<$T> = into_iter_hash_keys(it0);"
  echo "            let mut collected: Vec<$T> = Vec::new();"
  if [ "$PAT" = loop ]; then cat <<EOF
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
EOF
  else cat <<EOF
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
EOF
  fi
}
map_expr() { # $1 expr $2 elem type
  local E=$1 T=$2
  if [ "$KEEP" = yes ]; then echo "$E"; return; fi
  case "$E" in
    "g.iter_vertices()") echo "g.vertices().iter()";;
    "g.iter_arcs()") if [[ $T == LabEdge* ]]; then echo "g.labeled_arcs().iter()"; else echo "g.arcs().iter()"; fi;;
    "g.iter_edges()") if [[ $T == LabEdge* ]]; then echo "g.labeled_edges().iter()"; else echo "g.edges().iter()"; fi;;
    *) echo "UNMAPPED($E)";;
  esac
}
re='^ *let (mut )?it: SetStEphIter<(.*)> = (.*);$'
while IFS= read -r line; do
  if [ $skip = 1 ]; then
    if [[ "$line" == "    } => Ok(())" ]]; then skip=0; echo "$line"; fi
    continue
  fi
  if [[ "$line" =~ $re ]]; then
    if [ -n "${BASH_REMATCH[1]}" ]; then pat=loop; else pat=for; fi
    T=${BASH_REMATCH[2]}; E=$(map_expr "${BASH_REMATCH[3]}" "$T")
    emit_body $pat "$E" "$T"; skip=1; continue
  fi
  case "$line" in
    "        use vstd::prelude::*;")
      echo "$line"; echo "        use vstd::std_specs::iter::*;"; echo "        use vstd::std_specs::hash::into_iter_hash_keys;";;
    "//! Proof tests for "*" iterators") echo "$line (prophetic model, r214)";;
    *"docs/APAS-VERUSIterators.rs"*) echo "${line/docs\/APAS-VERUSIterators.rs/src\/standards\/iterator_ptt_standard.rs}";;
    *) if [[ "$line" == "//!"* && "$KEEP" != yes ]]; then
         l=${line//for x in iter:/for x in it:}
         l=${l//g.iter_vertices()/g.vertices().iter()}
         if [[ "$IN" == *Lab* || "$IN" == *Weighted* ]]; then l=${l//g.iter_arcs()/g.labeled_arcs().iter()}; l=${l//g.iter_edges()/g.labeled_edges().iter()}
         else l=${l//g.iter_arcs()/g.arcs().iter()}; l=${l//g.iter_edges()/g.edges().iter()}; fi
         echo "$l"
       elif [[ "$line" == "//!"* ]]; then echo "${line//for x in iter:/for x in it:}"
       else echo "$line"; fi;;
  esac
done < "$IN"
