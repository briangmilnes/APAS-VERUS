import glob
import re
import os

markdown_table = [
    "<style>\n",
    "body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }\n",
    ".markdown-body { max-width: 100% !important; width: 100% !important; }\n",
    ".container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }\n",
    "table { width: 100% !important; table-layout: fixed; }\n",
    "</style>\n",
    "# Broadcast Axiom Audit for Holed Modules\n",
    "| Chapter | File | Holes | Spec Types Used | vstd Broadcasts | vstdplus Broadcasts |",
    "|---------|------|-------|-----------------|-----------------|---------------------|"
]

for report in glob.glob("src/Chap*/analyses/proof-holes.txt"):
    chap = report.split('/')[1]
    
    with open(report, "r") as f:
        content = f.read()
        
    blocks = re.split(r'❌ ', content)[1:]
    
    for block in blocks:
        lines = block.strip().split('\n')
        filename = lines[0].strip()
        
        hole_count = "0"
        for line in lines:
            m = re.search(r'Holes:\s+(\d+)\s+total', line)
            if m:
                hole_count = m.group(1)
                break
                
        if hole_count == "0" or not hole_count.isdigit() or int(hole_count) == 0:
            continue
            
        rs_path = f"src/{chap}/{filename}"
        
        spec_types = set()
        vstd_broadcasts = set()
        vstdplus_broadcasts = set()
        
        if os.path.exists(rs_path):
            with open(rs_path, "r") as rsf:
                rs_content = rsf.read()
                
                if re.search(r'\bSet<', rs_content): spec_types.add("Set")
                if re.search(r'\bSeq<', rs_content): spec_types.add("Seq")
                if re.search(r'\bMap<', rs_content): spec_types.add("Map")
                if re.search(r'\bMultiset<', rs_content): spec_types.add("Multiset")
                
                bd_uses = re.findall(r'broadcast use\s+([^;{]+);', rs_content)
                blocks_bd = re.findall(r'broadcast use\s*\{([^}]+)\}', rs_content)
                for b in blocks_bd:
                    bd_uses.extend(item.strip() for item in b.split(','))
                
                for bu in bd_uses:
                    bu = bu.strip()
                    if not bu or bu.startswith("//"): continue
                    if "vstd::" in bu:
                        if "vstd::set" in bu: vstd_broadcasts.add("Set")
                        if "vstd::seq" in bu: vstd_broadcasts.add("Seq")
                        if "vstd::map" in bu: vstd_broadcasts.add("Map")
                        if "vstd::multiset" in bu: vstd_broadcasts.add("Multiset")
                    elif "vstdplus" in bu:
                        vstdplus_broadcasts.add(bu)
                        
        spec_str = ", ".join(sorted(spec_types)) if spec_types else "None"
        vstd_str = ", ".join(sorted(vstd_broadcasts)) if vstd_broadcasts else "None"
        vstdplus_str = "Yes" if vstdplus_broadcasts else "None"
        
        # Calculate Proposals
        prop_vstd = []
        prop_vstdplus = []
        
        if "Set" in spec_types:
            prop_vstd.extend(["vstd::set::group_set_axioms", "vstd::set_lib::group_set_lib_default"])
        if "Seq" in spec_types:
            prop_vstd.extend(["vstd::seq::group_seq_axioms", "vstd::seq_lib::group_seq_properties", "vstd::seq_lib::group_to_multiset_ensures"])
            prop_vstdplus.append("crate::vstdplus::seq_set::*") 
        if "Map" in spec_types:
            prop_vstd.extend(["vstd::map::group_map_axioms", "vstd::map_lib::group_map_lib_default"])
        if "Multiset" in spec_types or ("Set" in spec_types and "Seq" in spec_types):
            prop_vstd.append("vstd::multiset::group_multiset_axioms")
            prop_vstdplus.append("crate::vstdplus::multiset::*")

        if "Seq" in spec_types or "Set" in spec_types or "Map" in spec_types:
            prop_vstdplus.append("crate::vstdplus::feq::feq::group_feq_axioms")
            
        p_vstd_str = "<br>".join([f"`{x}`" for x in sorted(set(prop_vstd))]) if prop_vstd else "None"
        p_vplus_str = "<br>".join([f"`{x}`" for x in sorted(set(prop_vstdplus))]) if prop_vstdplus else "None"
        
        markdown_table.append(f"| {chap} | {filename} | {hole_count} | {spec_str} | {vstd_str} | {vstdplus_str} |")
        markdown_table.append(f"| | *↳ Proposed* | | | {p_vstd_str} | {p_vplus_str} |")

print("\n".join(markdown_table))
