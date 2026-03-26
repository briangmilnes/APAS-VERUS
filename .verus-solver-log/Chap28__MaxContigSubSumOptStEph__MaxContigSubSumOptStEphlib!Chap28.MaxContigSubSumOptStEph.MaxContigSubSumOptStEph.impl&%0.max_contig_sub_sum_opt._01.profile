[tool-version] Z3 4.12.5
[mk-app] #1 true
[mk-app] #2 false
[mk-app] #1 true
[mk-app] #2 false
[mk-app] #3 pi
[mk-app] #4 euler
[mk-var] datatype#0 0
[mk-var] datatype#1 1
[mk-app] datatype#2 insert datatype#0 datatype#1
[mk-app] datatype#3 pattern datatype#2
[mk-app] datatype#4 head datatype#2
[mk-app] datatype#5 = datatype#0 datatype#4
[mk-quant] datatype#6 constructor_accessor_axiom 2 datatype#3 datatype#5
[attach-var-names] datatype#6 (;k!0) (;List)
[mk-app] datatype#7 tail datatype#2
[mk-app] datatype#8 = datatype#1 datatype#7
[mk-quant] datatype#9 constructor_accessor_axiom 2 datatype#3 datatype#8
[attach-var-names] datatype#9 (;k!0) (;List)
[mk-app] #5 bv
[attach-meaning] #5 bv #b1
[mk-app] #6 bv
[attach-meaning] #6 bv #b0
[attach-meaning] #5 bv #b1
[attach-meaning] #6 bv #b0
[attach-meaning] #6 bv #b0
[mk-var] #7 0
[mk-var] #8 1
[mk-var] #9 2
[mk-var] #10 3
[mk-var] #11 4
[mk-var] #12 5
[mk-var] #13 6
[mk-var] #14 7
[mk-var] #15 8
[mk-var] #16 9
[mk-var] #17 10
[mk-var] #18 11
[mk-var] #19 12
[mk-var] #20 13
[mk-var] #21 14
[mk-app] #22 + #15 #13
[attach-enode] #1 0
[attach-enode] #2 0
[mk-app] #23 fuel_defaults
[mk-var] #24 0
[mk-app] #25 fuel_bool #24
[mk-app] #26 fuel_bool_default #24
[mk-app] #27 = #25 #26
[mk-app] #28 pattern #25
[mk-quant] #29 prelude_fuel_defaults 1 #28 #27
[attach-var-names] #29 (|id| ; |FuelId|)
[mk-app] #30 => #23 #29
[mk-app] #31 not #23
[mk-app] #32 or #31 #29
[inst-discovered] theory-solving 0 basic# ; #30
[mk-app] #33 = #30 #32
[instance] 0 #33
[attach-enode] #33 0
[end-of-instance]
[mk-var] #33 1
[mk-var] #34 0
[mk-app] #35 mut_ref_update_current% #33 #34
[mk-app] #36 mut_ref_current% #35
[mk-app] #37 = #36 #34
[mk-app] #38 pattern #35
[mk-quant] #39 prelude_mut_ref_update_current_current 2 #38 #37
[attach-var-names] #39 (|arg| ; |Poly|) (|m| ; |Poly|)
[mk-app] #40 mut_ref_future% #35
[mk-app] #41 mut_ref_future% #33
[mk-app] #42 = #40 #41
[mk-quant] #43 prelude_mut_ref_update_current_future 2 #38 #42
[attach-var-names] #43 (|arg| ; |Poly|) (|m| ; |Poly|)
[mk-var] #44 2
[mk-var] #45 1
[mk-var] #46 0
[mk-app] #47 MUTREF #45 #46
[mk-app] #48 has_type #44 #47
[mk-app] #49 mut_ref_current% #44
[mk-app] #50 has_type #49 #46
[mk-app] #51 => #48 #50
[mk-app] #52 pattern #48 #49
[mk-quant] #53 prelude_mut_ref_current_has_type 3 #52 #51
[attach-var-names] #53 (|t| ; |Type|) (|d| ; |Dcr|) (|m| ; |Poly|)
[mk-app] #54 not #48
[mk-app] #55 or #54 #50
[inst-discovered] theory-solving 0 basic# ; #51
[mk-app] #56 = #51 #55
[instance] 0 #56
[attach-enode] #56 0
[end-of-instance]
[mk-quant] #56 prelude_mut_ref_current_has_type 3 #52 #55
[attach-var-names] #56 (|t| ; |Type|) (|d| ; |Dcr|) (|m| ; |Poly|)
[mk-app] #57 mut_ref_future% #44
[mk-app] #58 has_type #57 #46
[mk-app] #59 => #48 #58
[mk-app] #60 pattern #48 #57
[mk-quant] #61 prelude_mut_ref_current_has_type 3 #60 #59
[attach-var-names] #61 (|t| ; |Type|) (|d| ; |Dcr|) (|m| ; |Poly|)
[mk-app] #62 or #54 #58
[inst-discovered] theory-solving 0 basic# ; #59
[mk-app] #63 = #59 #62
[instance] 0 #63
[attach-enode] #63 0
[end-of-instance]
[mk-quant] #63 prelude_mut_ref_current_has_type 3 #60 #62
[attach-var-names] #63 (|t| ; |Type|) (|d| ; |Dcr|) (|m| ; |Poly|)
[mk-var] #64 3
[mk-var] #65 2
[mk-var] #66 1
[mk-app] #67 MUTREF #65 #66
[mk-app] #68 has_type #64 #67
[mk-app] #69 has_type #34 #66
[mk-app] #70 and #68 #69
[mk-app] #71 mut_ref_update_current% #64 #34
[mk-app] #72 has_type #71 #67
[mk-app] #73 => #70 #72
[mk-app] #74 pattern #68 #71
[mk-quant] #75 prelude_mut_ref_update_has_type 4 #74 #73
[attach-var-names] #75 (|arg| ; |Poly|) (|t| ; |Type|) (|d| ; |Dcr|) (|m| ; |Poly|)
[mk-app] #76 not #70
[mk-app] #77 or #76 #72
[inst-discovered] theory-solving 0 basic# ; #73
[mk-app] #78 = #73 #77
[instance] 0 #78
[attach-enode] #78 0
[end-of-instance]
[mk-quant] #78 prelude_mut_ref_update_has_type 4 #74 #77
[attach-var-names] #78 (|arg| ; |Poly|) (|t| ; |Type|) (|d| ; |Dcr|) (|m| ; |Poly|)
[mk-var] #79 0
[mk-app] #80 sized #79
[mk-app] #81 DST #79
[mk-app] #82 sized #81
[mk-app] #83 => #80 #82
[mk-app] #84 pattern #82
[mk-quant] #85 prelude_sized_decorate_struct_inherit 1 #84 #83
[attach-var-names] #85 (|d| ; |Dcr|)
[mk-app] #86 not #80
[mk-app] #87 or #86 #82
[inst-discovered] theory-solving 0 basic# ; #83
[mk-app] #88 = #83 #87
[instance] 0 #88
[attach-enode] #88 0
[end-of-instance]
[mk-quant] #88 prelude_sized_decorate_struct_inherit 1 #84 #87
[attach-var-names] #88 (|d| ; |Dcr|)
[mk-app] #89 REF #79
[mk-app] #90 sized #89
[mk-app] #91 pattern #90
[mk-quant] #92 prelude_sized_decorate_ref 1 #91 #90
[attach-var-names] #92 (|d| ; |Dcr|)
[mk-app] #93 MUT_REF #79
[mk-app] #94 sized #93
[mk-app] #95 pattern #94
[mk-quant] #96 prelude_sized_decorate_mut_ref 1 #95 #94
[attach-var-names] #96 (|d| ; |Dcr|)
[mk-app] #97 BOX #65 #66 #79
[mk-app] #98 sized #97
[mk-app] #99 pattern #98
[mk-quant] #100 prelude_sized_decorate_box 3 #99 #98
[attach-var-names] #100 (|d2| ; |Dcr|) (|t| ; |Type|) (|d| ; |Dcr|)
[mk-app] #101 RC #65 #66 #79
[mk-app] #102 sized #101
[mk-app] #103 pattern #102
[mk-quant] #104 prelude_sized_decorate_rc 3 #103 #102
[attach-var-names] #104 (|d2| ; |Dcr|) (|t| ; |Type|) (|d| ; |Dcr|)
[mk-app] #105 ARC #65 #66 #79
[mk-app] #106 sized #105
[mk-app] #107 pattern #106
[mk-quant] #108 prelude_sized_decorate_arc 3 #107 #106
[attach-var-names] #108 (|d2| ; |Dcr|) (|t| ; |Type|) (|d| ; |Dcr|)
[mk-app] #109 GHOST #79
[mk-app] #110 sized #109
[mk-app] #111 pattern #110
[mk-quant] #112 prelude_sized_decorate_ghost 1 #111 #110
[attach-var-names] #112 (|d| ; |Dcr|)
[mk-app] #113 TRACKED #79
[mk-app] #114 sized #113
[mk-app] #115 pattern #114
[mk-quant] #116 prelude_sized_decorate_tracked 1 #115 #114
[attach-var-names] #116 (|d| ; |Dcr|)
[mk-app] #117 NEVER #79
[mk-app] #118 sized #117
[mk-app] #119 pattern #118
[mk-quant] #120 prelude_sized_decorate_never 1 #119 #118
[attach-var-names] #120 (|d| ; |Dcr|)
[mk-app] #121 CONST_PTR #79
[mk-app] #122 sized #121
[mk-app] #123 pattern #122
[mk-quant] #124 prelude_sized_decorate_const_ptr 1 #123 #122
[attach-var-names] #124 (|d| ; |Dcr|)
[mk-app] #125 $
[mk-app] #126 sized #125
[mk-var] #127 0
[mk-app] #128 CONST_INT #127
[mk-app] #129 const_int #128
[mk-app] #130 = #127 #129
[mk-app] #131 pattern #128
[mk-quant] #132 prelude_type_id_const_int 1 #131 #130
[attach-var-names] #132 (|i| ; |Int|)
[mk-var] #133 0
[mk-app] #134 CONST_BOOL #133
[mk-app] #135 const_bool #134
[mk-app] #136 = #133 #135
[mk-app] #137 pattern #134
[mk-quant] #138 prelude_type_id_const_bool 1 #137 #136
[attach-var-names] #138 (|b| ; |Bool|)
[mk-app] #139 B #133
[mk-app] #140 BOOL
[mk-app] #141 has_type #139 #140
[mk-app] #142 pattern #141
[mk-quant] #143 prelude_has_type_bool 1 #142 #141
[attach-var-names] #143 (|b| ; |Bool|)
[mk-app] #144 R #7
[mk-app] #145 REAL
[mk-app] #146 has_type #144 #145
[mk-app] #147 pattern #146
[mk-quant] #148 prelude_has_type_real 1 #147 #146
[attach-var-names] #148 (|r| ; |Real|)
[mk-app] #149 as_type #33 #46
[mk-app] #150 has_type #149 #46
[mk-app] #151 has_type #33 #46
[mk-app] #152 = #33 #149
[mk-app] #153 => #151 #152
[mk-app] #154 and #150 #153
[mk-app] #155 pattern #149
[mk-quant] #156 prelude_as_type 2 #155 #154
[attach-var-names] #156 (|t| ; |Type|) (|x| ; |Poly|)
[mk-app] #157 not #151
[mk-app] #158 or #157 #152
[inst-discovered] theory-solving 0 basic# ; #153
[mk-app] #159 = #153 #158
[instance] 0 #159
[attach-enode] #159 0
[end-of-instance]
[mk-app] #159 and #150 #158
[mk-quant] #160 prelude_as_type 2 #155 #159
[attach-var-names] #160 (|t| ; |Type|) (|x| ; |Poly|)
[mk-var] #161 0
[mk-app] #162 mk_fun #161
[mk-app] #163 = #162 #161
[mk-app] #164 pattern #162
[mk-quant] #165 prelude_mk_fun 1 #164 #163
[attach-var-names] #165 (|x| ; |%%Function%%|)
[mk-app] #166 %B #139
[mk-app] #167 = #133 #166
[mk-app] #168 pattern #139
[mk-quant] #169 prelude_unbox_box_bool 1 #168 #167
[attach-var-names] #169 (|x| ; |Bool|)
[mk-app] #170 I #127
[mk-app] #171 %I #170
[mk-app] #172 = #127 #171
[mk-app] #173 pattern #170
[mk-quant] #174 prelude_unbox_box_int 1 #173 #172
[attach-var-names] #174 (|x| ; |Int|)
[mk-app] #175 %R #144
[mk-app] #176 = #7 #175
[mk-app] #177 pattern #144
[mk-quant] #178 prelude_unbox_box_real 1 #177 #176
[attach-var-names] #178 (|x| ; |Real|)
[mk-app] #179 has_type #34 #140
[mk-app] #180 %B #34
[mk-app] #181 B #180
[mk-app] #182 = #34 #181
[mk-app] #183 => #179 #182
[mk-app] #184 pattern #179
[mk-quant] #185 prelude_box_unbox_bool 1 #184 #183
[attach-var-names] #185 (|x| ; |Poly|)
[mk-app] #186 not #179
[mk-app] #187 or #186 #182
[inst-discovered] theory-solving 0 basic# ; #183
[mk-app] #188 = #183 #187
[instance] 0 #188
[attach-enode] #188 0
[end-of-instance]
[mk-quant] #188 prelude_box_unbox_bool 1 #184 #187
[attach-var-names] #188 (|x| ; |Poly|)
[mk-app] #189 INT
[mk-app] #190 has_type #34 #189
[mk-app] #191 %I #34
[mk-app] #192 I #191
[mk-app] #193 = #34 #192
[mk-app] #194 => #190 #193
[mk-app] #195 pattern #190
[mk-quant] #196 prelude_box_unbox_int 1 #195 #194
[attach-var-names] #196 (|x| ; |Poly|)
[mk-app] #197 not #190
[mk-app] #198 or #197 #193
[inst-discovered] theory-solving 0 basic# ; #194
[mk-app] #199 = #194 #198
[instance] 0 #199
[attach-enode] #199 0
[end-of-instance]
[mk-quant] #199 prelude_box_unbox_int 1 #195 #198
[attach-var-names] #199 (|x| ; |Poly|)
[mk-app] #200 NAT
[mk-app] #201 has_type #34 #200
[mk-app] #202 => #201 #193
[mk-app] #203 pattern #201
[mk-quant] #204 prelude_box_unbox_nat 1 #203 #202
[attach-var-names] #204 (|x| ; |Poly|)
[mk-app] #205 not #201
[mk-app] #206 or #205 #193
[inst-discovered] theory-solving 0 basic# ; #202
[mk-app] #207 = #202 #206
[instance] 0 #207
[attach-enode] #207 0
[end-of-instance]
[mk-quant] #207 prelude_box_unbox_nat 1 #203 #206
[attach-var-names] #207 (|x| ; |Poly|)
[mk-app] #208 USIZE
[mk-app] #209 has_type #34 #208
[mk-app] #210 => #209 #193
[mk-app] #211 pattern #209
[mk-quant] #212 prelude_box_unbox_usize 1 #211 #210
[attach-var-names] #212 (|x| ; |Poly|)
[mk-app] #213 not #209
[mk-app] #214 or #213 #193
[inst-discovered] theory-solving 0 basic# ; #210
[mk-app] #215 = #210 #214
[instance] 0 #215
[attach-enode] #215 0
[end-of-instance]
[mk-quant] #215 prelude_box_unbox_usize 1 #211 #214
[attach-var-names] #215 (|x| ; |Poly|)
[mk-app] #216 ISIZE
[mk-app] #217 has_type #34 #216
[mk-app] #218 => #217 #193
[mk-app] #219 pattern #217
[mk-quant] #220 prelude_box_unbox_isize 1 #219 #218
[attach-var-names] #220 (|x| ; |Poly|)
[mk-app] #221 not #217
[mk-app] #222 or #221 #193
[inst-discovered] theory-solving 0 basic# ; #218
[mk-app] #223 = #218 #222
[instance] 0 #223
[attach-enode] #223 0
[end-of-instance]
[mk-quant] #223 prelude_box_unbox_isize 1 #219 #222
[attach-var-names] #223 (|x| ; |Poly|)
[mk-var] #224 1
[mk-app] #225 UINT #224
[mk-app] #226 has_type #34 #225
[mk-app] #227 => #226 #193
[mk-app] #228 pattern #226
[mk-quant] #229 prelude_box_unbox_uint 2 #228 #227
[attach-var-names] #229 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #230 not #226
[mk-app] #231 or #230 #193
[inst-discovered] theory-solving 0 basic# ; #227
[mk-app] #232 = #227 #231
[instance] 0 #232
[attach-enode] #232 0
[end-of-instance]
[mk-quant] #232 prelude_box_unbox_uint 2 #228 #231
[attach-var-names] #232 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #233 SINT #224
[mk-app] #234 has_type #34 #233
[mk-app] #235 => #234 #193
[mk-app] #236 pattern #234
[mk-quant] #237 prelude_box_unbox_sint 2 #236 #235
[attach-var-names] #237 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #238 not #234
[mk-app] #239 or #238 #193
[inst-discovered] theory-solving 0 basic# ; #235
[mk-app] #240 = #235 #239
[instance] 0 #240
[attach-enode] #240 0
[end-of-instance]
[mk-quant] #240 prelude_box_unbox_sint 2 #236 #239
[attach-var-names] #240 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #241 FLOAT #224
[mk-app] #242 has_type #34 #241
[mk-app] #243 => #242 #193
[mk-app] #244 pattern #242
[mk-quant] #245 prelude_box_unbox_sint 2 #244 #243
[attach-var-names] #245 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #246 not #242
[mk-app] #247 or #246 #193
[inst-discovered] theory-solving 0 basic# ; #243
[mk-app] #248 = #243 #247
[instance] 0 #248
[attach-enode] #248 0
[end-of-instance]
[mk-quant] #248 prelude_box_unbox_sint 2 #244 #247
[attach-var-names] #248 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #249 CHAR
[mk-app] #250 has_type #34 #249
[mk-app] #251 => #250 #193
[mk-app] #252 pattern #250
[mk-quant] #253 prelude_box_unbox_char 1 #252 #251
[attach-var-names] #253 (|x| ; |Poly|)
[mk-app] #254 not #250
[mk-app] #255 or #254 #193
[inst-discovered] theory-solving 0 basic# ; #251
[mk-app] #256 = #251 #255
[instance] 0 #256
[attach-enode] #256 0
[end-of-instance]
[mk-quant] #256 prelude_box_unbox_char 1 #252 #255
[attach-var-names] #256 (|x| ; |Poly|)
[mk-app] #257 has_type #34 #145
[mk-app] #258 %R #34
[mk-app] #259 R #258
[mk-app] #260 = #34 #259
[mk-app] #261 => #257 #260
[mk-app] #262 pattern #257
[mk-quant] #263 prelude_box_unbox_real 1 #262 #261
[attach-var-names] #263 (|x| ; |Poly|)
[mk-app] #264 not #257
[mk-app] #265 or #264 #260
[inst-discovered] theory-solving 0 basic# ; #261
[mk-app] #266 = #261 #265
[instance] 0 #266
[attach-enode] #266 0
[end-of-instance]
[mk-quant] #266 prelude_box_unbox_real 1 #262 #265
[attach-var-names] #266 (|x| ; |Poly|)
[mk-var] #267 3
[mk-var] #268 2
[mk-app] #269 = #33 #34
[mk-app] #270 ext_eq #267 #268 #33 #34
[mk-app] #271 = #269 #270
[mk-app] #272 pattern #270
[mk-quant] #273 prelude_ext_eq 4 #272 #271
[attach-var-names] #273 (|y| ; |Poly|) (|x| ; |Poly|) (|t| ; |Type|) (|deep| ; |Bool|)
[mk-app] #274 SZ
[mk-app] #275 Int
[attach-meaning] #275 arith 32
[mk-app] #276 = #274 #275
[mk-app] #277 Int
[attach-meaning] #277 arith 64
[mk-app] #278 = #274 #277
[mk-app] #279 or #276 #278
[mk-app] #280 Int
[attach-meaning] #280 arith 8
[mk-app] #281 uHi #280
[mk-app] #282 Int
[attach-meaning] #282 arith 256
[mk-app] #283 = #281 #282
[mk-app] #284 Int
[attach-meaning] #284 arith 16
[mk-app] #285 uHi #284
[mk-app] #286 Int
[attach-meaning] #286 arith 65536
[mk-app] #287 = #285 #286
[attach-meaning] #275 arith 32
[mk-app] #288 uHi #275
[mk-app] #289 Int
[attach-meaning] #289 arith 4294967296
[mk-app] #290 = #288 #289
[attach-meaning] #277 arith 64
[mk-app] #291 uHi #277
[mk-app] #292 Int
[attach-meaning] #292 arith 18446744073709551616
[mk-app] #293 = #291 #292
[mk-app] #294 Int
[attach-meaning] #294 arith 128
[mk-app] #295 uHi #294
[mk-app] #296 Int
[attach-meaning] #296 arith 1
[mk-app] #297 Int
[attach-meaning] #297 arith 340282366920938463463374607431768211455
[mk-app] #298 + #296 #297
[mk-app] #299 = #295 #298
[mk-app] #300 Int
[attach-meaning] #300 arith 340282366920938463463374607431768211456
[inst-discovered] theory-solving 0 arith# ; #298
[mk-app] #301 = #298 #300
[instance] 0 #301
[attach-enode] #301 0
[end-of-instance]
[mk-app] #301 = #295 #300
[mk-app] #302 iLo #280
[attach-meaning] #294 arith 128
[mk-app] #303 - #294
[mk-app] #304 = #302 #303
[mk-app] #305 Int
[attach-meaning] #305 arith (- 128)
[inst-discovered] theory-solving 0 arith# ; #303
[mk-app] #306 = #303 #305
[instance] 0 #306
[attach-enode] #306 0
[end-of-instance]
[mk-app] #306 = #302 #305
[attach-meaning] #284 arith 16
[mk-app] #307 iLo #284
[mk-app] #308 Int
[attach-meaning] #308 arith 32768
[mk-app] #309 - #308
[mk-app] #310 = #307 #309
[mk-app] #311 Int
[attach-meaning] #311 arith (- 32768)
[inst-discovered] theory-solving 0 arith# ; #309
[mk-app] #312 = #309 #311
[instance] 0 #312
[attach-enode] #312 0
[end-of-instance]
[mk-app] #312 = #307 #311
[attach-meaning] #275 arith 32
[mk-app] #313 iLo #275
[mk-app] #314 Int
[attach-meaning] #314 arith 2147483648
[mk-app] #315 - #314
[mk-app] #316 = #313 #315
[mk-app] #317 Int
[attach-meaning] #317 arith (- 2147483648)
[inst-discovered] theory-solving 0 arith# ; #315
[mk-app] #318 = #315 #317
[instance] 0 #318
[attach-enode] #318 0
[end-of-instance]
[mk-app] #318 = #313 #317
[attach-meaning] #277 arith 64
[mk-app] #319 iLo #277
[mk-app] #320 Int
[attach-meaning] #320 arith 9223372036854775808
[mk-app] #321 - #320
[mk-app] #322 = #319 #321
[mk-app] #323 Int
[attach-meaning] #323 arith (- 9223372036854775808)
[inst-discovered] theory-solving 0 arith# ; #321
[mk-app] #324 = #321 #323
[instance] 0 #324
[attach-enode] #324 0
[end-of-instance]
[mk-app] #324 = #319 #323
[attach-meaning] #294 arith 128
[mk-app] #325 iLo #294
[mk-app] #326 Int
[attach-meaning] #326 arith 170141183460469231731687303715884105728
[mk-app] #327 - #326
[mk-app] #328 = #325 #327
[mk-app] #329 Int
[attach-meaning] #329 arith (- 170141183460469231731687303715884105728)
[inst-discovered] theory-solving 0 arith# ; #327
[mk-app] #330 = #327 #329
[instance] 0 #330
[attach-enode] #330 0
[end-of-instance]
[mk-app] #330 = #325 #329
[mk-app] #331 iHi #280
[attach-meaning] #294 arith 128
[mk-app] #332 = #331 #294
[attach-meaning] #284 arith 16
[mk-app] #333 iHi #284
[attach-meaning] #308 arith 32768
[mk-app] #334 = #333 #308
[attach-meaning] #275 arith 32
[mk-app] #335 iHi #275
[attach-meaning] #314 arith 2147483648
[mk-app] #336 = #335 #314
[attach-meaning] #277 arith 64
[mk-app] #337 iHi #277
[attach-meaning] #320 arith 9223372036854775808
[mk-app] #338 = #337 #320
[attach-meaning] #294 arith 128
[mk-app] #339 iHi #294
[attach-meaning] #326 arith 170141183460469231731687303715884105728
[mk-app] #340 = #339 #326
[mk-app] #341 Int
[attach-meaning] #341 arith 0
[mk-app] #342 nClip #127
[mk-app] #343 <= #341 #342
[mk-app] #344 <= #341 #127
[mk-app] #345 = #127 #342
[mk-app] #346 => #344 #345
[mk-app] #347 and #343 #346
[mk-app] #348 pattern #342
[mk-quant] #349 prelude_nat_clip 1 #348 #347
[attach-var-names] #349 (|i| ; |Int|)
[mk-app] #350 Int
[attach-meaning] #350 arith (- 1)
[mk-app] #351 * #350 #342
[mk-app] #352 >= #342 #341
[inst-discovered] theory-solving 0 arith# ; #343
[mk-app] #350 = #343 #352
[instance] 0 #350
[attach-enode] #350 0
[end-of-instance]
[mk-app] #350 Int
[attach-meaning] #350 arith (- 1)
[mk-app] #351 * #350 #127
[mk-app] #353 >= #127 #341
[inst-discovered] theory-solving 0 arith# ; #344
[mk-app] #350 = #344 #353
[instance] 0 #350
[attach-enode] #350 0
[end-of-instance]
[mk-app] #350 not #353
[mk-app] #351 or #350 #345
[mk-app] #354 => #353 #345
[inst-discovered] theory-solving 0 basic# ; #354
[mk-app] #355 = #354 #351
[instance] 0 #355
[attach-enode] #355 0
[end-of-instance]
[mk-app] #354 and #352 #351
[mk-quant] #355 prelude_nat_clip 1 #348 #354
[attach-var-names] #355 (|i| ; |Int|)
[mk-app] #356 uClip #224 #127
[mk-app] #357 <= #341 #356
[mk-app] #358 uHi #224
[mk-app] #359 < #356 #358
[mk-app] #360 < #127 #358
[mk-app] #361 and #344 #360
[mk-app] #362 = #127 #356
[mk-app] #363 => #361 #362
[mk-app] #364 and #357 #359 #363
[mk-app] #365 pattern #356
[mk-quant] #366 prelude_u_clip 2 #365 #364
[attach-var-names] #366 (|i| ; |Int|) (|bits| ; |Int|)
[mk-app] #367 Int
[attach-meaning] #367 arith (- 1)
[mk-app] #368 * #367 #356
[mk-app] #369 >= #356 #341
[inst-discovered] theory-solving 0 arith# ; #357
[mk-app] #367 = #357 #369
[instance] 0 #367
[attach-enode] #367 0
[end-of-instance]
[mk-app] #367 <= #358 #356
[mk-app] #368 not #367
[inst-discovered] theory-solving 0 arith# ; #359
[mk-app] #370 = #359 #368
[instance] 0 #370
[attach-enode] #370 0
[end-of-instance]
[mk-app] #370 Int
[attach-meaning] #370 arith (- 1)
[mk-app] #371 * #370 #356
[mk-app] #372 + #371 #358
[attach-meaning] #370 arith (- 1)
[mk-app] #373 * #370 #358
[mk-app] #374 + #356 #373
[mk-app] #371 >= #374 #341
[inst-discovered] theory-solving 0 arith# ; #367
[mk-app] #372 = #367 #371
[instance] 0 #372
[attach-enode] #372 0
[end-of-instance]
[mk-app] #372 not #371
[attach-meaning] #370 arith (- 1)
[mk-app] #375 * #370 #127
[inst-discovered] theory-solving 0 arith# ; #344
[mk-app] #375 = #344 #353
[instance] 0 #375
[attach-enode] #375 0
[end-of-instance]
[mk-app] #375 <= #358 #127
[mk-app] #376 not #375
[inst-discovered] theory-solving 0 arith# ; #360
[mk-app] #377 = #360 #376
[instance] 0 #377
[attach-enode] #377 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #377 * #370 #127
[mk-app] #378 + #377 #358
[attach-meaning] #370 arith (- 1)
[mk-app] #379 + #127 #373
[mk-app] #377 >= #379 #341
[inst-discovered] theory-solving 0 arith# ; #375
[mk-app] #378 = #375 #377
[instance] 0 #378
[attach-enode] #378 0
[end-of-instance]
[mk-app] #378 not #377
[mk-app] #380 and #353 #378
[mk-app] #381 not #380
[mk-app] #382 or #381 #362
[mk-app] #383 => #380 #362
[inst-discovered] theory-solving 0 basic# ; #383
[mk-app] #384 = #383 #382
[instance] 0 #384
[attach-enode] #384 0
[end-of-instance]
[mk-app] #383 and #369 #372 #382
[mk-quant] #384 prelude_u_clip 2 #365 #383
[attach-var-names] #384 (|i| ; |Int|) (|bits| ; |Int|)
[mk-app] #367 iLo #224
[mk-app] #368 iClip #224 #127
[mk-app] #375 <= #367 #368
[mk-app] #376 iHi #224
[mk-app] #385 < #368 #376
[mk-app] #386 <= #367 #127
[mk-app] #387 < #127 #376
[mk-app] #388 and #386 #387
[mk-app] #389 = #127 #368
[mk-app] #390 => #388 #389
[mk-app] #391 and #375 #385 #390
[mk-app] #392 pattern #368
[mk-quant] #393 prelude_i_clip 2 #392 #391
[attach-var-names] #393 (|i| ; |Int|) (|bits| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #394 * #370 #368
[mk-app] #395 + #367 #394
[mk-app] #396 <= #395 #341
[inst-discovered] theory-solving 0 arith# ; #375
[mk-app] #397 = #375 #396
[instance] 0 #397
[attach-enode] #397 0
[end-of-instance]
[mk-app] #397 <= #376 #368
[mk-app] #398 not #397
[inst-discovered] theory-solving 0 arith# ; #385
[mk-app] #399 = #385 #398
[instance] 0 #399
[attach-enode] #399 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #399 + #394 #376
[attach-meaning] #370 arith (- 1)
[mk-app] #400 * #370 #376
[mk-app] #401 + #368 #400
[mk-app] #399 >= #401 #341
[inst-discovered] theory-solving 0 arith# ; #397
[mk-app] #402 = #397 #399
[instance] 0 #402
[attach-enode] #402 0
[end-of-instance]
[mk-app] #402 not #399
[attach-meaning] #370 arith (- 1)
[mk-app] #403 * #370 #127
[mk-app] #404 + #403 #367
[attach-meaning] #370 arith (- 1)
[mk-app] #405 * #370 #367
[mk-app] #406 + #127 #405
[mk-app] #403 >= #406 #341
[inst-discovered] theory-solving 0 arith# ; #386
[mk-app] #404 = #386 #403
[instance] 0 #404
[attach-enode] #404 0
[end-of-instance]
[mk-app] #404 <= #376 #127
[mk-app] #407 not #404
[inst-discovered] theory-solving 0 arith# ; #387
[mk-app] #408 = #387 #407
[instance] 0 #408
[attach-enode] #408 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #408 * #370 #127
[mk-app] #409 + #408 #376
[attach-meaning] #370 arith (- 1)
[mk-app] #410 + #127 #400
[mk-app] #408 >= #410 #341
[inst-discovered] theory-solving 0 arith# ; #404
[mk-app] #409 = #404 #408
[instance] 0 #409
[attach-enode] #409 0
[end-of-instance]
[mk-app] #409 not #408
[mk-app] #411 and #403 #409
[mk-app] #412 not #411
[mk-app] #413 or #412 #389
[mk-app] #414 => #411 #389
[inst-discovered] theory-solving 0 basic# ; #414
[mk-app] #415 = #414 #413
[instance] 0 #415
[attach-enode] #415 0
[end-of-instance]
[mk-app] #414 and #396 #402 #413
[mk-quant] #415 prelude_i_clip 2 #392 #414
[attach-var-names] #415 (|i| ; |Int|) (|bits| ; |Int|)
[mk-app] #404 charClip #127
[mk-app] #407 <= #341 #404
[mk-app] #397 Int
[attach-meaning] #397 arith 55295
[mk-app] #398 <= #404 #397
[mk-app] #416 and #407 #398
[mk-app] #417 Int
[attach-meaning] #417 arith 57344
[mk-app] #418 <= #417 #404
[mk-app] #419 Int
[attach-meaning] #419 arith 1114111
[mk-app] #420 <= #404 #419
[mk-app] #421 and #418 #420
[mk-app] #422 or #416 #421
[attach-meaning] #397 arith 55295
[mk-app] #423 <= #127 #397
[mk-app] #424 and #344 #423
[attach-meaning] #417 arith 57344
[mk-app] #425 <= #417 #127
[attach-meaning] #419 arith 1114111
[mk-app] #426 <= #127 #419
[mk-app] #427 and #425 #426
[mk-app] #428 or #424 #427
[mk-app] #429 = #127 #404
[mk-app] #430 => #428 #429
[mk-app] #431 and #422 #430
[mk-app] #432 pattern #404
[mk-quant] #433 prelude_char_clip 1 #432 #431
[attach-var-names] #433 (|i| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #434 * #370 #404
[mk-app] #435 >= #404 #341
[inst-discovered] theory-solving 0 arith# ; #407
[mk-app] #434 = #407 #435
[instance] 0 #434
[attach-enode] #434 0
[end-of-instance]
[mk-app] #434 and #435 #398
[attach-meaning] #370 arith (- 1)
[mk-app] #436 * #370 #404
[mk-app] #437 Int
[attach-meaning] #437 arith (- 57344)
[attach-meaning] #417 arith 57344
[mk-app] #438 >= #404 #417
[inst-discovered] theory-solving 0 arith# ; #418
[mk-app] #436 = #418 #438
[instance] 0 #436
[attach-enode] #436 0
[end-of-instance]
[mk-app] #436 and #438 #420
[mk-app] #437 or #434 #436
[attach-meaning] #370 arith (- 1)
[mk-app] #439 * #370 #127
[inst-discovered] theory-solving 0 arith# ; #344
[mk-app] #439 = #344 #353
[instance] 0 #439
[attach-enode] #439 0
[end-of-instance]
[mk-app] #439 and #353 #423
[attach-meaning] #370 arith (- 1)
[mk-app] #440 * #370 #127
[mk-app] #441 Int
[attach-meaning] #441 arith (- 57344)
[attach-meaning] #417 arith 57344
[mk-app] #442 >= #127 #417
[inst-discovered] theory-solving 0 arith# ; #425
[mk-app] #440 = #425 #442
[instance] 0 #440
[attach-enode] #440 0
[end-of-instance]
[mk-app] #440 and #442 #426
[mk-app] #441 or #439 #440
[mk-app] #443 not #441
[mk-app] #444 or #443 #429
[mk-app] #445 => #441 #429
[inst-discovered] theory-solving 0 basic# ; #445
[mk-app] #446 = #445 #444
[instance] 0 #446
[attach-enode] #446 0
[end-of-instance]
[mk-app] #445 and #437 #444
[mk-quant] #446 prelude_char_clip 1 #432 #445
[attach-var-names] #446 (|i| ; |Int|)
[mk-app] #447 uInv #224 #127
[mk-app] #448 = #447 #361
[mk-app] #449 pattern #447
[mk-quant] #450 prelude_u_inv 2 #449 #448
[attach-var-names] #450 (|i| ; |Int|) (|bits| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #451 * #370 #127
[inst-discovered] theory-solving 0 arith# ; #344
[mk-app] #451 = #344 #353
[instance] 0 #451
[attach-enode] #451 0
[end-of-instance]
[mk-app] #451 <= #358 #127
[mk-app] #452 not #451
[inst-discovered] theory-solving 0 arith# ; #360
[mk-app] #453 = #360 #452
[instance] 0 #453
[attach-enode] #453 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #453 * #370 #127
[mk-app] #454 + #453 #358
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #451
[mk-app] #453 = #451 #377
[instance] 0 #453
[attach-enode] #453 0
[end-of-instance]
[mk-app] #453 = #447 #380
[mk-quant] #454 prelude_u_inv 2 #449 #453
[attach-var-names] #454 (|i| ; |Int|) (|bits| ; |Int|)
[mk-app] #451 iInv #224 #127
[mk-app] #452 = #451 #388
[mk-app] #455 pattern #451
[mk-quant] #456 prelude_i_inv 2 #455 #452
[attach-var-names] #456 (|i| ; |Int|) (|bits| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #457 * #370 #127
[mk-app] #458 + #457 #367
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #386
[mk-app] #457 = #386 #403
[instance] 0 #457
[attach-enode] #457 0
[end-of-instance]
[mk-app] #457 <= #376 #127
[mk-app] #458 not #457
[inst-discovered] theory-solving 0 arith# ; #387
[mk-app] #459 = #387 #458
[instance] 0 #459
[attach-enode] #459 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #459 * #370 #127
[mk-app] #460 + #459 #376
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #457
[mk-app] #459 = #457 #408
[instance] 0 #459
[attach-enode] #459 0
[end-of-instance]
[mk-app] #459 = #451 #411
[mk-quant] #460 prelude_i_inv 2 #455 #459
[attach-var-names] #460 (|i| ; |Int|) (|bits| ; |Int|)
[mk-app] #457 charInv #127
[attach-meaning] #397 arith 55295
[attach-meaning] #417 arith 57344
[attach-meaning] #419 arith 1114111
[mk-app] #458 = #457 #428
[mk-app] #461 pattern #457
[mk-quant] #462 prelude_char_inv 1 #461 #458
[attach-var-names] #462 (|i| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #463 * #370 #127
[inst-discovered] theory-solving 0 arith# ; #344
[mk-app] #463 = #344 #353
[instance] 0 #463
[attach-enode] #463 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #463 * #370 #127
[mk-app] #464 Int
[attach-meaning] #464 arith (- 57344)
[attach-meaning] #417 arith 57344
[inst-discovered] theory-solving 0 arith# ; #425
[mk-app] #463 = #425 #442
[instance] 0 #463
[attach-enode] #463 0
[end-of-instance]
[mk-app] #463 = #457 #441
[mk-quant] #464 prelude_char_inv 1 #461 #463
[attach-var-names] #464 (|i| ; |Int|)
[mk-app] #465 has_type #170 #189
[mk-app] #466 pattern #465
[mk-quant] #467 prelude_has_type_int 1 #466 #465
[attach-var-names] #467 (|x| ; |Int|)
[mk-app] #468 has_type #170 #200
[mk-app] #469 => #344 #468
[mk-app] #470 pattern #468
[mk-quant] #471 prelude_has_type_nat 1 #470 #469
[attach-var-names] #471 (|x| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #472 * #370 #127
[inst-discovered] theory-solving 0 arith# ; #344
[mk-app] #472 = #344 #353
[instance] 0 #472
[attach-enode] #472 0
[end-of-instance]
[mk-app] #472 or #350 #468
[mk-app] #473 => #353 #468
[inst-discovered] theory-solving 0 basic# ; #473
[mk-app] #474 = #473 #472
[instance] 0 #474
[attach-enode] #474 0
[end-of-instance]
[mk-quant] #473 prelude_has_type_nat 1 #470 #472
[attach-var-names] #473 (|x| ; |Int|)
[mk-app] #474 uInv #274 #127
[mk-app] #475 has_type #170 #208
[mk-app] #476 => #474 #475
[mk-app] #477 pattern #475
[mk-quant] #478 prelude_has_type_usize 1 #477 #476
[attach-var-names] #478 (|x| ; |Int|)
[mk-app] #479 not #474
[mk-app] #480 or #479 #475
[inst-discovered] theory-solving 0 basic# ; #476
[mk-app] #481 = #476 #480
[instance] 0 #481
[attach-enode] #481 0
[end-of-instance]
[mk-quant] #481 prelude_has_type_usize 1 #477 #480
[attach-var-names] #481 (|x| ; |Int|)
[mk-app] #482 iInv #274 #127
[mk-app] #483 has_type #170 #216
[mk-app] #484 => #482 #483
[mk-app] #485 pattern #483
[mk-quant] #486 prelude_has_type_isize 1 #485 #484
[attach-var-names] #486 (|x| ; |Int|)
[mk-app] #487 not #482
[mk-app] #488 or #487 #483
[inst-discovered] theory-solving 0 basic# ; #484
[mk-app] #489 = #484 #488
[instance] 0 #489
[attach-enode] #489 0
[end-of-instance]
[mk-quant] #489 prelude_has_type_isize 1 #485 #488
[attach-var-names] #489 (|x| ; |Int|)
[mk-app] #490 has_type #170 #225
[mk-app] #491 => #447 #490
[mk-app] #492 pattern #490
[mk-quant] #493 prelude_has_type_uint 2 #492 #491
[attach-var-names] #493 (|x| ; |Int|) (|bits| ; |Int|)
[mk-app] #494 not #447
[mk-app] #495 or #494 #490
[inst-discovered] theory-solving 0 basic# ; #491
[mk-app] #496 = #491 #495
[instance] 0 #496
[attach-enode] #496 0
[end-of-instance]
[mk-quant] #496 prelude_has_type_uint 2 #492 #495
[attach-var-names] #496 (|x| ; |Int|) (|bits| ; |Int|)
[mk-app] #497 has_type #170 #233
[mk-app] #498 => #451 #497
[mk-app] #499 pattern #497
[mk-quant] #500 prelude_has_type_sint 2 #499 #498
[attach-var-names] #500 (|x| ; |Int|) (|bits| ; |Int|)
[mk-app] #501 not #451
[mk-app] #502 or #501 #497
[inst-discovered] theory-solving 0 basic# ; #498
[mk-app] #503 = #498 #502
[instance] 0 #503
[attach-enode] #503 0
[end-of-instance]
[mk-quant] #503 prelude_has_type_sint 2 #499 #502
[attach-var-names] #503 (|x| ; |Int|) (|bits| ; |Int|)
[mk-app] #504 has_type #170 #241
[mk-app] #505 => #447 #504
[mk-app] #506 pattern #504
[mk-quant] #507 prelude_has_type_sint 2 #506 #505
[attach-var-names] #507 (|x| ; |Int|) (|bits| ; |Int|)
[mk-app] #508 or #494 #504
[inst-discovered] theory-solving 0 basic# ; #505
[mk-app] #509 = #505 #508
[instance] 0 #509
[attach-enode] #509 0
[end-of-instance]
[mk-quant] #509 prelude_has_type_sint 2 #506 #508
[attach-var-names] #509 (|x| ; |Int|) (|bits| ; |Int|)
[mk-app] #510 has_type #170 #249
[mk-app] #511 => #457 #510
[mk-app] #512 pattern #510
[mk-quant] #513 prelude_has_type_char 1 #512 #511
[attach-var-names] #513 (|x| ; |Int|)
[mk-app] #514 not #457
[mk-app] #515 or #514 #510
[inst-discovered] theory-solving 0 basic# ; #511
[mk-app] #516 = #511 #515
[instance] 0 #516
[attach-enode] #516 0
[end-of-instance]
[mk-quant] #516 prelude_has_type_char 1 #512 #515
[attach-var-names] #516 (|x| ; |Int|)
[mk-app] #517 <= #341 #191
[mk-app] #518 => #201 #517
[mk-quant] #519 prelude_unbox_int 1 #203 #518
[attach-var-names] #519 (|x| ; |Poly|)
[attach-meaning] #370 arith (- 1)
[mk-app] #520 * #370 #191
[mk-app] #521 >= #191 #341
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #520 = #517 #521
[instance] 0 #520
[attach-enode] #520 0
[end-of-instance]
[mk-app] #520 or #205 #521
[mk-app] #522 => #201 #521
[inst-discovered] theory-solving 0 basic# ; #522
[mk-app] #523 = #522 #520
[instance] 0 #523
[attach-enode] #523 0
[end-of-instance]
[mk-quant] #522 prelude_unbox_int 1 #203 #520
[attach-var-names] #522 (|x| ; |Poly|)
[mk-app] #523 uInv #274 #191
[mk-app] #524 => #209 #523
[mk-quant] #525 prelude_unbox_usize 1 #211 #524
[attach-var-names] #525 (|x| ; |Poly|)
[mk-app] #526 or #213 #523
[inst-discovered] theory-solving 0 basic# ; #524
[mk-app] #527 = #524 #526
[instance] 0 #527
[attach-enode] #527 0
[end-of-instance]
[mk-quant] #527 prelude_unbox_usize 1 #211 #526
[attach-var-names] #527 (|x| ; |Poly|)
[mk-app] #528 iInv #274 #191
[mk-app] #529 => #217 #528
[mk-quant] #530 prelude_unbox_isize 1 #219 #529
[attach-var-names] #530 (|x| ; |Poly|)
[mk-app] #531 or #221 #528
[inst-discovered] theory-solving 0 basic# ; #529
[mk-app] #532 = #529 #531
[instance] 0 #532
[attach-enode] #532 0
[end-of-instance]
[mk-quant] #532 prelude_unbox_isize 1 #219 #531
[attach-var-names] #532 (|x| ; |Poly|)
[mk-app] #533 uInv #224 #191
[mk-app] #534 => #226 #533
[mk-quant] #535 prelude_unbox_uint 2 #228 #534
[attach-var-names] #535 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #536 or #230 #533
[inst-discovered] theory-solving 0 basic# ; #534
[mk-app] #537 = #534 #536
[instance] 0 #537
[attach-enode] #537 0
[end-of-instance]
[mk-quant] #537 prelude_unbox_uint 2 #228 #536
[attach-var-names] #537 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #538 iInv #224 #191
[mk-app] #539 => #234 #538
[mk-quant] #540 prelude_unbox_sint 2 #236 #539
[attach-var-names] #540 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #541 or #238 #538
[inst-discovered] theory-solving 0 basic# ; #539
[mk-app] #542 = #539 #541
[instance] 0 #542
[attach-enode] #542 0
[end-of-instance]
[mk-quant] #542 prelude_unbox_sint 2 #236 #541
[attach-var-names] #542 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #543 => #242 #533
[mk-quant] #544 prelude_unbox_sint 2 #244 #543
[attach-var-names] #544 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #545 or #246 #533
[inst-discovered] theory-solving 0 basic# ; #543
[mk-app] #546 = #543 #545
[instance] 0 #546
[attach-enode] #546 0
[end-of-instance]
[mk-quant] #546 prelude_unbox_sint 2 #244 #545
[attach-var-names] #546 (|x| ; |Poly|) (|bits| ; |Int|)
[mk-app] #547 Add #224 #127
[mk-app] #548 + #224 #127
[mk-app] #549 = #547 #548
[mk-app] #550 pattern #547
[mk-quant] #551 prelude_add 2 #550 #549
[attach-var-names] #551 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #552 + #127 #224
[inst-discovered] theory-solving 0 arith# ; #548
[mk-app] #553 = #548 #552
[instance] 0 #553
[attach-enode] #553 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #553 * #370 #127
[attach-meaning] #370 arith (- 1)
[mk-app] #554 * #370 #224
[mk-app] #555 + #553 #554 #547
[attach-meaning] #370 arith (- 1)
[mk-app] #556 * #370 #547
[mk-app] #557 + #127 #224 #556
[mk-app] #553 = #557 #341
[mk-app] #554 = #547 #552
[inst-discovered] theory-solving 0 arith# ; #554
[mk-app] #555 = #554 #553
[instance] 0 #555
[attach-enode] #555 0
[end-of-instance]
[mk-quant] #552 prelude_add 2 #550 #553
[attach-var-names] #552 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #554 Sub #224 #127
[mk-app] #555 - #224 #127
[mk-app] #558 = #554 #555
[mk-app] #559 pattern #554
[mk-quant] #560 prelude_sub 2 #559 #558
[attach-var-names] #560 (|y| ; |Int|) (|x| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #561 * #370 #127
[mk-app] #562 + #224 #561
[inst-discovered] theory-solving 0 arith# ; #555
[mk-app] #563 = #555 #562
[instance] 0 #563
[attach-enode] #563 0
[end-of-instance]
[mk-app] #563 + #561 #224
[inst-discovered] theory-solving 0 arith# ; #562
[mk-app] #564 = #562 #563
[instance] 0 #564
[attach-enode] #564 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #564 * #370 #224
[mk-app] #565 + #127 #564 #554
[mk-app] #566 = #565 #341
[mk-app] #567 = #554 #563
[inst-discovered] theory-solving 0 arith# ; #567
[mk-app] #568 = #567 #566
[instance] 0 #568
[attach-enode] #568 0
[end-of-instance]
[mk-quant] #567 prelude_sub 2 #559 #566
[attach-var-names] #567 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #561 Mul #224 #127
[mk-app] #563 * #224 #127
[mk-app] #562 = #561 #563
[mk-app] #568 pattern #561
[mk-quant] #569 prelude_mul 2 #568 #562
[attach-var-names] #569 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #570 * #127 #224
[inst-discovered] theory-solving 0 arith# ; #563
[mk-app] #571 = #563 #570
[instance] 0 #571
[attach-enode] #571 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #571 * #370 #570
[mk-app] #572 + #561 #571
[mk-app] #573 = #572 #341
[mk-app] #574 = #561 #570
[inst-discovered] theory-solving 0 arith# ; #574
[mk-app] #575 = #574 #573
[instance] 0 #575
[attach-enode] #575 0
[end-of-instance]
[mk-quant] #574 prelude_mul 2 #568 #573
[attach-var-names] #574 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #575 EucDiv #224 #127
[mk-app] #576 div #224 #127
[mk-app] #577 = #575 #576
[mk-app] #578 pattern #575
[mk-quant] #579 prelude_eucdiv 2 #578 #577
[attach-var-names] #579 (|y| ; |Int|) (|x| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #580 * #370 #576
[mk-app] #581 + #575 #580
[mk-app] #582 = #581 #341
[inst-discovered] theory-solving 0 arith# ; #577
[mk-app] #583 = #577 #582
[instance] 0 #583
[attach-enode] #583 0
[end-of-instance]
[mk-quant] #583 prelude_eucdiv 2 #578 #582
[attach-var-names] #583 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #584 EucMod #224 #127
[mk-app] #585 mod #224 #127
[mk-app] #586 = #584 #585
[mk-app] #587 pattern #584
[mk-quant] #588 prelude_eucmod 2 #587 #586
[attach-var-names] #588 (|y| ; |Int|) (|x| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #589 * #370 #585
[mk-app] #590 + #584 #589
[mk-app] #591 = #590 #341
[inst-discovered] theory-solving 0 arith# ; #586
[mk-app] #592 = #586 #591
[instance] 0 #592
[attach-enode] #592 0
[end-of-instance]
[mk-quant] #592 prelude_eucmod 2 #587 #591
[attach-var-names] #592 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #593 RAdd #8 #7
[mk-app] #594 + #8 #7
[mk-app] #595 = #593 #594
[mk-app] #596 pattern #593
[mk-quant] #597 prelude_radd 2 #596 #595
[attach-var-names] #597 (|y| ; |Real|) (|x| ; |Real|)
[mk-app] #598 + #7 #8
[inst-discovered] theory-solving 0 arith# ; #594
[mk-app] #599 = #594 #598
[instance] 0 #599
[attach-enode] #599 0
[end-of-instance]
[mk-app] #599 Real
[attach-meaning] #599 arith (- 1)
[mk-app] #600 * #599 #7
[attach-meaning] #599 arith (- 1)
[mk-app] #601 * #599 #8
[mk-app] #602 Real
[attach-meaning] #602 arith 0
[mk-app] #603 + #600 #601 #593
[attach-meaning] #599 arith (- 1)
[mk-app] #604 * #599 #593
[mk-app] #605 + #7 #8 #604
[mk-app] #600 = #605 #602
[mk-app] #601 = #593 #598
[inst-discovered] theory-solving 0 arith# ; #601
[mk-app] #603 = #601 #600
[instance] 0 #603
[attach-enode] #603 0
[end-of-instance]
[mk-quant] #598 prelude_radd 2 #596 #600
[attach-var-names] #598 (|y| ; |Real|) (|x| ; |Real|)
[mk-app] #601 RSub #8 #7
[mk-app] #603 - #8 #7
[mk-app] #606 = #601 #603
[mk-app] #607 pattern #601
[mk-quant] #608 prelude_rsub 2 #607 #606
[attach-var-names] #608 (|y| ; |Real|) (|x| ; |Real|)
[attach-meaning] #599 arith (- 1)
[mk-app] #609 * #599 #7
[mk-app] #610 + #8 #609
[inst-discovered] theory-solving 0 arith# ; #603
[mk-app] #611 = #603 #610
[instance] 0 #611
[attach-enode] #611 0
[end-of-instance]
[mk-app] #611 + #609 #8
[inst-discovered] theory-solving 0 arith# ; #610
[mk-app] #612 = #610 #611
[instance] 0 #612
[attach-enode] #612 0
[end-of-instance]
[attach-meaning] #599 arith (- 1)
[mk-app] #612 * #599 #8
[mk-app] #613 + #7 #612 #601
[mk-app] #614 = #613 #602
[mk-app] #615 = #601 #611
[inst-discovered] theory-solving 0 arith# ; #615
[mk-app] #616 = #615 #614
[instance] 0 #616
[attach-enode] #616 0
[end-of-instance]
[mk-quant] #615 prelude_rsub 2 #607 #614
[attach-var-names] #615 (|y| ; |Real|) (|x| ; |Real|)
[mk-app] #609 RMul #8 #7
[mk-app] #611 * #8 #7
[mk-app] #610 = #609 #611
[mk-app] #616 pattern #609
[mk-quant] #617 prelude_rmul 2 #616 #610
[attach-var-names] #617 (|y| ; |Real|) (|x| ; |Real|)
[mk-app] #618 * #7 #8
[inst-discovered] theory-solving 0 arith# ; #611
[mk-app] #619 = #611 #618
[instance] 0 #619
[attach-enode] #619 0
[end-of-instance]
[attach-meaning] #599 arith (- 1)
[mk-app] #619 * #599 #618
[mk-app] #620 + #609 #619
[mk-app] #621 = #620 #602
[mk-app] #622 = #609 #618
[inst-discovered] theory-solving 0 arith# ; #622
[mk-app] #623 = #622 #621
[instance] 0 #623
[attach-enode] #623 0
[end-of-instance]
[mk-quant] #622 prelude_rmul 2 #616 #621
[attach-var-names] #622 (|y| ; |Real|) (|x| ; |Real|)
[mk-app] #623 RDiv #8 #7
[mk-app] #624 / #8 #7
[mk-app] #625 = #623 #624
[mk-app] #626 pattern #623
[mk-quant] #627 prelude_rdiv 2 #626 #625
[attach-var-names] #627 (|y| ; |Real|) (|x| ; |Real|)
[attach-meaning] #599 arith (- 1)
[mk-app] #628 * #599 #624
[mk-app] #629 + #623 #628
[mk-app] #630 = #629 #602
[inst-discovered] theory-solving 0 arith# ; #625
[mk-app] #631 = #625 #630
[instance] 0 #631
[attach-enode] #631 0
[end-of-instance]
[mk-quant] #631 prelude_rdiv 2 #626 #630
[attach-var-names] #631 (|y| ; |Real|) (|x| ; |Real|)
[mk-app] #632 <= #341 #224
[mk-app] #633 and #632 #344
[mk-app] #634 <= #341 #561
[mk-app] #635 => #633 #634
[mk-quant] #636 prelude_mul_nats 2 #568 #635
[attach-var-names] #636 (|y| ; |Int|) (|x| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #637 >= #224 #341
[inst-discovered] theory-solving 0 arith# ; #632
[mk-app] #638 = #632 #637
[instance] 0 #638
[attach-enode] #638 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #638 * #370 #127
[inst-discovered] theory-solving 0 arith# ; #344
[mk-app] #638 = #344 #353
[instance] 0 #638
[attach-enode] #638 0
[end-of-instance]
[mk-app] #638 and #637 #353
[attach-meaning] #370 arith (- 1)
[mk-app] #639 * #370 #561
[mk-app] #640 >= #561 #341
[inst-discovered] theory-solving 0 arith# ; #634
[mk-app] #639 = #634 #640
[instance] 0 #639
[attach-enode] #639 0
[end-of-instance]
[mk-app] #639 not #638
[mk-app] #641 or #639 #640
[mk-app] #642 => #638 #640
[inst-discovered] theory-solving 0 basic# ; #642
[mk-app] #643 = #642 #641
[instance] 0 #643
[attach-enode] #643 0
[end-of-instance]
[mk-quant] #642 prelude_mul_nats 2 #568 #641
[attach-var-names] #642 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #643 < #341 #127
[mk-app] #644 and #632 #643
[mk-app] #645 <= #341 #575
[mk-app] #646 <= #575 #224
[mk-app] #647 and #645 #646
[mk-app] #648 => #644 #647
[mk-quant] #649 prelude_div_unsigned_in_bounds 2 #578 #648
[attach-var-names] #649 (|y| ; |Int|) (|x| ; |Int|)
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #632
[mk-app] #650 = #632 #637
[instance] 0 #650
[attach-enode] #650 0
[end-of-instance]
[mk-app] #650 <= #127 #341
[mk-app] #651 not #650
[inst-discovered] theory-solving 0 arith# ; #643
[mk-app] #652 = #643 #651
[instance] 0 #652
[attach-enode] #652 0
[end-of-instance]
[mk-app] #652 and #637 #651
[attach-meaning] #370 arith (- 1)
[mk-app] #653 * #370 #575
[mk-app] #654 >= #575 #341
[inst-discovered] theory-solving 0 arith# ; #645
[mk-app] #653 = #645 #654
[instance] 0 #653
[attach-enode] #653 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #653 + #564 #575
[attach-meaning] #370 arith (- 1)
[mk-app] #655 * #370 #575
[mk-app] #656 + #224 #655
[mk-app] #653 >= #656 #341
[inst-discovered] theory-solving 0 arith# ; #646
[mk-app] #657 = #646 #653
[instance] 0 #657
[attach-enode] #657 0
[end-of-instance]
[mk-app] #657 and #654 #653
[mk-app] #658 not #652
[mk-app] #659 or #658 #657
[mk-app] #660 => #652 #657
[inst-discovered] theory-solving 0 basic# ; #660
[mk-app] #661 = #660 #659
[instance] 0 #661
[attach-enode] #661 0
[end-of-instance]
[mk-quant] #660 prelude_div_unsigned_in_bounds 2 #578 #659
[attach-var-names] #660 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #661 <= #341 #584
[mk-app] #662 < #584 #127
[mk-app] #663 and #661 #662
[mk-app] #664 => #644 #663
[mk-quant] #665 prelude_mod_unsigned_in_bounds 2 #587 #664
[attach-var-names] #665 (|y| ; |Int|) (|x| ; |Int|)
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #632
[mk-app] #666 = #632 #637
[instance] 0 #666
[attach-enode] #666 0
[end-of-instance]
[inst-discovered] theory-solving 0 arith# ; #643
[mk-app] #666 = #643 #651
[instance] 0 #666
[attach-enode] #666 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #666 * #370 #584
[mk-app] #667 >= #584 #341
[inst-discovered] theory-solving 0 arith# ; #661
[mk-app] #666 = #661 #667
[instance] 0 #666
[attach-enode] #666 0
[end-of-instance]
[mk-app] #666 <= #127 #584
[mk-app] #668 not #666
[inst-discovered] theory-solving 0 arith# ; #662
[mk-app] #669 = #662 #668
[instance] 0 #669
[attach-enode] #669 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #669 * #370 #584
[mk-app] #670 + #127 #669
[mk-app] #671 <= #670 #341
[inst-discovered] theory-solving 0 arith# ; #666
[mk-app] #672 = #666 #671
[instance] 0 #672
[attach-enode] #672 0
[end-of-instance]
[mk-app] #672 not #671
[mk-app] #673 and #667 #672
[mk-app] #674 or #658 #673
[mk-app] #675 => #652 #673
[inst-discovered] theory-solving 0 basic# ; #675
[mk-app] #676 = #675 #674
[instance] 0 #676
[attach-enode] #676 0
[end-of-instance]
[mk-quant] #675 prelude_mod_unsigned_in_bounds 2 #587 #674
[attach-var-names] #675 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #666 %I #44
[mk-app] #668 uInv #127 #666
[mk-app] #676 %I #33
[mk-app] #677 uInv #127 #676
[mk-app] #678 and #668 #677
[mk-app] #679 bitxor #44 #33
[mk-app] #680 uInv #127 #679
[mk-app] #681 => #678 #680
[mk-app] #682 uClip #127 #679
[mk-app] #683 pattern #682
[mk-quant] #684 prelude_bit_xor_u_inv 3 #683 #681
[attach-var-names] #684 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #685 not #678
[mk-app] #686 or #685 #680
[inst-discovered] theory-solving 0 basic# ; #681
[mk-app] #687 = #681 #686
[instance] 0 #687
[attach-enode] #687 0
[end-of-instance]
[mk-quant] #687 prelude_bit_xor_u_inv 3 #683 #686
[attach-var-names] #687 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #688 iInv #127 #666
[mk-app] #689 iInv #127 #676
[mk-app] #690 and #688 #689
[mk-app] #691 iInv #127 #679
[mk-app] #692 => #690 #691
[mk-app] #693 iClip #127 #679
[mk-app] #694 pattern #693
[mk-quant] #695 prelude_bit_xor_i_inv 3 #694 #692
[attach-var-names] #695 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #696 not #690
[mk-app] #697 or #696 #691
[inst-discovered] theory-solving 0 basic# ; #692
[mk-app] #698 = #692 #697
[instance] 0 #698
[attach-enode] #698 0
[end-of-instance]
[mk-quant] #698 prelude_bit_xor_i_inv 3 #694 #697
[attach-var-names] #698 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #699 bitor #44 #33
[mk-app] #700 uInv #127 #699
[mk-app] #701 => #678 #700
[mk-app] #702 uClip #127 #699
[mk-app] #703 pattern #702
[mk-quant] #704 prelude_bit_or_u_inv 3 #703 #701
[attach-var-names] #704 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #705 or #685 #700
[inst-discovered] theory-solving 0 basic# ; #701
[mk-app] #706 = #701 #705
[instance] 0 #706
[attach-enode] #706 0
[end-of-instance]
[mk-quant] #706 prelude_bit_or_u_inv 3 #703 #705
[attach-var-names] #706 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #707 iInv #127 #699
[mk-app] #708 => #690 #707
[mk-app] #709 iClip #127 #699
[mk-app] #710 pattern #709
[mk-quant] #711 prelude_bit_or_i_inv 3 #710 #708
[attach-var-names] #711 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #712 or #696 #707
[inst-discovered] theory-solving 0 basic# ; #708
[mk-app] #713 = #708 #712
[instance] 0 #713
[attach-enode] #713 0
[end-of-instance]
[mk-quant] #713 prelude_bit_or_i_inv 3 #710 #712
[attach-var-names] #713 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #714 bitand #44 #33
[mk-app] #715 uInv #127 #714
[mk-app] #716 => #678 #715
[mk-app] #717 uClip #127 #714
[mk-app] #718 pattern #717
[mk-quant] #719 prelude_bit_and_u_inv 3 #718 #716
[attach-var-names] #719 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #720 or #685 #715
[inst-discovered] theory-solving 0 basic# ; #716
[mk-app] #721 = #716 #720
[instance] 0 #721
[attach-enode] #721 0
[end-of-instance]
[mk-quant] #721 prelude_bit_and_u_inv 3 #718 #720
[attach-var-names] #721 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #722 iInv #127 #714
[mk-app] #723 => #690 #722
[mk-app] #724 iClip #127 #714
[mk-app] #725 pattern #724
[mk-quant] #726 prelude_bit_and_i_inv 3 #725 #723
[attach-var-names] #726 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #727 or #696 #722
[inst-discovered] theory-solving 0 basic# ; #723
[mk-app] #728 = #723 #727
[instance] 0 #728
[attach-enode] #728 0
[end-of-instance]
[mk-quant] #728 prelude_bit_and_i_inv 3 #725 #727
[attach-var-names] #728 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #729 <= #341 #676
[mk-app] #730 and #668 #729
[mk-app] #731 bitshr #44 #33
[mk-app] #732 uInv #127 #731
[mk-app] #733 => #730 #732
[mk-app] #734 uClip #127 #731
[mk-app] #735 pattern #734
[mk-quant] #736 prelude_bit_shr_u_inv 3 #735 #733
[attach-var-names] #736 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[attach-meaning] #370 arith (- 1)
[mk-app] #737 * #370 #676
[mk-app] #738 >= #676 #341
[inst-discovered] theory-solving 0 arith# ; #729
[mk-app] #737 = #729 #738
[instance] 0 #737
[attach-enode] #737 0
[end-of-instance]
[mk-app] #737 and #668 #738
[mk-app] #739 not #737
[mk-app] #740 or #739 #732
[mk-app] #741 => #737 #732
[inst-discovered] theory-solving 0 basic# ; #741
[mk-app] #742 = #741 #740
[instance] 0 #742
[attach-enode] #742 0
[end-of-instance]
[mk-quant] #741 prelude_bit_shr_u_inv 3 #735 #740
[attach-var-names] #741 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #742 and #688 #729
[mk-app] #743 iInv #127 #731
[mk-app] #744 => #742 #743
[mk-app] #745 iClip #127 #731
[mk-app] #746 pattern #745
[mk-quant] #747 prelude_bit_shr_i_inv 3 #746 #744
[attach-var-names] #747 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[attach-meaning] #370 arith (- 1)
[mk-app] #748 * #370 #676
[inst-discovered] theory-solving 0 arith# ; #729
[mk-app] #748 = #729 #738
[instance] 0 #748
[attach-enode] #748 0
[end-of-instance]
[mk-app] #748 and #688 #738
[mk-app] #749 not #748
[mk-app] #750 or #749 #743
[mk-app] #751 => #748 #743
[inst-discovered] theory-solving 0 basic# ; #751
[mk-app] #752 = #751 #750
[instance] 0 #752
[attach-enode] #752 0
[end-of-instance]
[mk-quant] #751 prelude_bit_shr_i_inv 3 #746 #750
[attach-var-names] #751 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #752 = #127 #341
[mk-app] #753 not #752
[mk-app] #754 singular_mod #224 #127
[mk-app] #755 = #584 #754
[mk-app] #756 => #753 #755
[mk-app] #757 pattern #754
[mk-quant] #758 prelude_singularmod 2 #757 #756
[attach-var-names] #758 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #759 or #752 #755
[inst-discovered] theory-solving 0 basic# ; #756
[mk-app] #760 = #756 #759
[instance] 0 #760
[attach-enode] #760 0
[end-of-instance]
[mk-quant] #760 prelude_singularmod 2 #757 #759
[attach-var-names] #760 (|y| ; |Int|) (|x| ; |Int|)
[mk-var] #761 2
[mk-app] #762 check_decrease_int #761 #224 #133
[mk-app] #763 <= #341 #761
[mk-app] #764 < #761 #224
[mk-app] #765 and #763 #764
[mk-app] #766 = #761 #224
[mk-app] #767 and #766 #133
[mk-app] #768 or #765 #767
[mk-app] #769 = #762 #768
[mk-app] #770 pattern #762
[mk-quant] #771 prelude_check_decrease_int 3 #770 #769
[attach-var-names] #771 (|otherwise| ; |Bool|) (|prev| ; |Int|) (|cur| ; |Int|)
[attach-meaning] #370 arith (- 1)
[mk-app] #772 * #370 #761
[mk-app] #773 >= #761 #341
[inst-discovered] theory-solving 0 arith# ; #763
[mk-app] #772 = #763 #773
[instance] 0 #772
[attach-enode] #772 0
[end-of-instance]
[mk-app] #772 <= #224 #761
[mk-app] #774 not #772
[inst-discovered] theory-solving 0 arith# ; #764
[mk-app] #775 = #764 #774
[instance] 0 #775
[attach-enode] #775 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #775 * #370 #761
[mk-app] #776 + #224 #775
[mk-app] #777 <= #776 #341
[inst-discovered] theory-solving 0 arith# ; #772
[mk-app] #778 = #772 #777
[instance] 0 #778
[attach-enode] #778 0
[end-of-instance]
[mk-app] #778 not #777
[mk-app] #779 and #773 #778
[mk-app] #780 or #779 #767
[inst-discovered] theory-solving 0 basic# ; #780
[mk-app] #781 = #780 #780
[instance] 0 #781
[attach-enode] #781 0
[end-of-instance]
[mk-app] #781 = #762 #780
[mk-quant] #782 prelude_check_decrease_int 3 #770 #781
[attach-var-names] #782 (|otherwise| ; |Bool|) (|prev| ; |Int|) (|cur| ; |Int|)
[mk-app] #772 check_decrease_height #44 #33 #133
[mk-app] #774 height #44
[mk-app] #783 height #33
[mk-app] #784 height_lt #774 #783
[mk-app] #785 = #774 #783
[mk-app] #786 and #785 #133
[mk-app] #787 or #784 #786
[mk-app] #788 = #772 #787
[mk-app] #789 pattern #772
[mk-quant] #790 prelude_check_decrease_height 3 #789 #788
[attach-var-names] #790 (|otherwise| ; |Bool|) (|prev| ; |Poly|) (|cur| ; |Poly|)
[mk-var] #791 1
[mk-var] #792 0
[mk-app] #793 height_lt #791 #792
[mk-app] #794 partial-order #791 #792
[mk-app] #795 = #791 #792
[mk-app] #796 not #795
[mk-app] #797 and #794 #796
[mk-app] #798 = #793 #797
[mk-app] #799 pattern #793
[mk-quant] #800 prelude_height_lt 2 #799 #798
[attach-var-names] #800 (|y| ; |Height|) (|x| ; |Height|)
[mk-app] #801 fuel%vstd!std_specs.option.impl&%0.arrow_0.
[mk-app] #802 fuel%vstd!std_specs.option.is_some.
[mk-app] #803 fuel%vstd!std_specs.option.is_none.
[mk-app] #804 fuel%vstd!std_specs.option.spec_unwrap.
[mk-app] #805 fuel%vstd!std_specs.vec.impl&%0.spec_index.
[mk-app] #806 fuel%vstd!std_specs.vec.axiom_spec_len.
[mk-app] #807 fuel%vstd!std_specs.vec.axiom_vec_index_decreases.
[mk-app] #808 fuel%vstd!std_specs.vec.axiom_vec_has_resolved.
[mk-app] #809 fuel%vstd!std_specs.vec.axiom_vec_decreases_to_view.
[mk-app] #810 fuel%vstd!seq.impl&%0.spec_index.
[mk-app] #811 fuel%vstd!seq.axiom_seq_index_decreases.
[mk-app] #812 fuel%vstd!seq.axiom_seq_empty.
[mk-app] #813 fuel%vstd!seq.axiom_seq_new_len.
[mk-app] #814 fuel%vstd!seq.axiom_seq_new_index.
[mk-app] #815 fuel%vstd!seq.axiom_seq_push_len.
[mk-app] #816 fuel%vstd!seq.axiom_seq_push_index_same.
[mk-app] #817 fuel%vstd!seq.axiom_seq_push_index_different.
[mk-app] #818 fuel%vstd!seq.axiom_seq_ext_equal.
[mk-app] #819 fuel%vstd!seq.axiom_seq_ext_equal_deep.
[mk-app] #820 fuel%vstd!seq_lib.impl&%0.map.
[mk-app] #821 fuel%vstd!seq_lib.lemma_seq_empty_equality.
[mk-app] #822 fuel%vstd!view.impl&%0.view.
[mk-app] #823 fuel%vstd!view.impl&%2.view.
[mk-app] #824 fuel%vstd!view.impl&%4.view.
[mk-app] #825 fuel%vstd!view.impl&%6.view.
[mk-app] #826 fuel%vstd!view.impl&%14.view.
[mk-app] #827 fuel%vstd!view.impl&%16.view.
[mk-app] #828 fuel%vstd!view.impl&%18.view.
[mk-app] #829 fuel%vstd!view.impl&%30.view.
[mk-app] #830 fuel%vstd!view.impl&%36.view.
[mk-app] #831 fuel%lib!Chap19.ArraySeqStEph.ArraySeqStEph.impl&%2.view.
[mk-app] #832 fuel%lib!Chap19.ArraySeqStEph.ArraySeqStEph.impl&%3.spec_arrayseqsteph_wf.
[mk-app] #833 fuel%lib!Chap19.ArraySeqStEph.ArraySeqStEph.impl&%3.spec_len.
[mk-app] #834 fuel%lib!Chap19.ArraySeqStEph.ArraySeqStEph.impl&%3.spec_index.
[mk-app] #835 fuel%lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.
[mk-app] #836 fuel%lib!Chap28.MCSSSpec.MCSSSpec.is_mcss_of.
[mk-app] #837 fuel%lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.
[mk-app] #838 fuel%lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.
[mk-app] #839 fuel%lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.
[mk-app] #840 fuel%vstd!array.group_array_axioms.
[mk-app] #841 fuel%vstd!function.group_function_axioms.
[mk-app] #842 fuel%vstd!laws_cmp.group_laws_cmp.
[mk-app] #843 fuel%vstd!laws_eq.bool_laws.group_laws_eq.
[mk-app] #844 fuel%vstd!laws_eq.u8_laws.group_laws_eq.
[mk-app] #845 fuel%vstd!laws_eq.i8_laws.group_laws_eq.
[mk-app] #846 fuel%vstd!laws_eq.u16_laws.group_laws_eq.
[mk-app] #847 fuel%vstd!laws_eq.i16_laws.group_laws_eq.
[mk-app] #848 fuel%vstd!laws_eq.u32_laws.group_laws_eq.
[mk-app] #849 fuel%vstd!laws_eq.i32_laws.group_laws_eq.
[mk-app] #850 fuel%vstd!laws_eq.u64_laws.group_laws_eq.
[mk-app] #851 fuel%vstd!laws_eq.i64_laws.group_laws_eq.
[mk-app] #852 fuel%vstd!laws_eq.u128_laws.group_laws_eq.
[mk-app] #853 fuel%vstd!laws_eq.i128_laws.group_laws_eq.
[mk-app] #854 fuel%vstd!laws_eq.usize_laws.group_laws_eq.
[mk-app] #855 fuel%vstd!laws_eq.isize_laws.group_laws_eq.
[mk-app] #856 fuel%vstd!laws_eq.group_laws_eq.
[mk-app] #857 fuel%vstd!layout.group_align_properties.
[mk-app] #858 fuel%vstd!layout.group_layout_axioms.
[mk-app] #859 fuel%vstd!map.group_map_axioms.
[mk-app] #860 fuel%vstd!multiset.group_multiset_axioms.
[mk-app] #861 fuel%vstd!raw_ptr.group_raw_ptr_axioms.
[mk-app] #862 fuel%vstd!seq.group_seq_axioms.
[mk-app] #863 fuel%vstd!seq_lib.group_filter_ensures.
[mk-app] #864 fuel%vstd!seq_lib.group_seq_lib_default.
[mk-app] #865 fuel%vstd!seq_lib.group_to_multiset_ensures.
[mk-app] #866 fuel%vstd!seq_lib.group_seq_properties.
[mk-app] #867 fuel%vstd!set.group_set_axioms.
[mk-app] #868 fuel%vstd!set_lib.group_set_lib_default.
[mk-app] #869 fuel%vstd!slice.group_slice_axioms.
[mk-app] #870 fuel%vstd!string.group_string_axioms.
[mk-app] #871 fuel%vstd!std_specs.bits.group_bits_axioms.
[mk-app] #872 fuel%vstd!std_specs.control_flow.group_control_flow_axioms.
[mk-app] #873 fuel%vstd!std_specs.manually_drop.group_manually_drop_axioms.
[mk-app] #874 fuel%vstd!std_specs.btree.group_btree_axioms.
[mk-app] #875 fuel%vstd!std_specs.hash.group_hash_axioms.
[mk-app] #876 fuel%vstd!std_specs.range.group_range_axioms.
[mk-app] #877 fuel%vstd!std_specs.slice.group_slice_axioms.
[mk-app] #878 fuel%vstd!std_specs.vec.group_vec_axioms.
[mk-app] #879 fuel%vstd!std_specs.vecdeque.group_vec_dequeue_axioms.
[mk-app] #880 fuel%vstd!group_vstd_default.
[mk-app] #881 fuel%lib!vstdplus.feq.feq.group_feq_axioms.
[mk-app] #882 distinct #801 #802 #803 #804 #805 #806 #807 #808 #809 #810 #811 #812 #813 #814 #815 #816 #817 #818 #819 #820 #821 #822 #823 #824 #825 #826 #827 #828 #829 #830 #831 #832 #833 #834 #835 #836 #837 #838 #839 #840 #841 #842 #843 #844 #845 #846 #847 #848 #849 #850 #851 #852 #853 #854 #855 #856 #857 #858 #859 #860 #861 #862 #863 #864 #865 #866 #867 #868 #869 #870 #871 #872 #873 #874 #875 #876 #877 #878 #879 #880 #881
[mk-app] #883 fuel_bool_default #856
[mk-app] #884 fuel_bool_default #843
[mk-app] #885 fuel_bool_default #844
[mk-app] #886 fuel_bool_default #845
[mk-app] #887 fuel_bool_default #846
[mk-app] #888 fuel_bool_default #847
[mk-app] #889 fuel_bool_default #848
[mk-app] #890 fuel_bool_default #849
[mk-app] #891 fuel_bool_default #850
[mk-app] #892 fuel_bool_default #851
[mk-app] #893 fuel_bool_default #852
[mk-app] #894 fuel_bool_default #853
[mk-app] #895 fuel_bool_default #854
[mk-app] #896 fuel_bool_default #855
[mk-app] #897 and #884 #885 #886 #887 #888 #889 #890 #891 #892 #893 #894 #895 #896
[mk-app] #898 => #883 #897
[mk-app] #899 not #883
[mk-app] #900 or #899 #897
[inst-discovered] theory-solving 0 basic# ; #898
[mk-app] #901 = #898 #900
[instance] 0 #901
[attach-enode] #901 0
[end-of-instance]
[mk-app] #901 fuel_bool_default #858
[mk-app] #902 fuel_bool_default #857
[mk-app] #903 => #901 #902
[mk-app] #904 not #901
[mk-app] #905 or #904 #902
[inst-discovered] theory-solving 0 basic# ; #903
[mk-app] #906 = #903 #905
[instance] 0 #906
[attach-enode] #906 0
[end-of-instance]
[mk-app] #906 fuel_bool_default #862
[mk-app] #907 fuel_bool_default #811
[mk-app] #908 fuel_bool_default #812
[mk-app] #909 fuel_bool_default #813
[mk-app] #910 fuel_bool_default #814
[mk-app] #911 fuel_bool_default #815
[mk-app] #912 fuel_bool_default #816
[mk-app] #913 fuel_bool_default #817
[mk-app] #914 fuel_bool_default #818
[mk-app] #915 fuel_bool_default #819
[mk-app] #916 and #907 #908 #909 #910 #911 #912 #913 #914 #915
[mk-app] #917 => #906 #916
[mk-app] #918 not #906
[mk-app] #919 or #918 #916
[inst-discovered] theory-solving 0 basic# ; #917
[mk-app] #920 = #917 #919
[instance] 0 #920
[attach-enode] #920 0
[end-of-instance]
[mk-app] #920 fuel_bool_default #864
[mk-app] #921 fuel_bool_default #863
[mk-app] #922 => #920 #921
[mk-app] #923 not #920
[mk-app] #924 or #923 #921
[inst-discovered] theory-solving 0 basic# ; #922
[mk-app] #925 = #922 #924
[instance] 0 #925
[attach-enode] #925 0
[end-of-instance]
[mk-app] #925 fuel_bool_default #866
[mk-app] #926 fuel_bool_default #821
[mk-app] #927 fuel_bool_default #865
[mk-app] #928 and #926 #927
[mk-app] #929 => #925 #928
[mk-app] #930 not #925
[mk-app] #931 or #930 #928
[inst-discovered] theory-solving 0 basic# ; #929
[mk-app] #932 = #929 #931
[instance] 0 #932
[attach-enode] #932 0
[end-of-instance]
[mk-app] #932 fuel_bool_default #878
[mk-app] #933 fuel_bool_default #806
[mk-app] #934 fuel_bool_default #807
[mk-app] #935 fuel_bool_default #808
[mk-app] #936 fuel_bool_default #809
[mk-app] #937 and #933 #934 #935 #936
[mk-app] #938 => #932 #937
[mk-app] #939 not #932
[mk-app] #940 or #939 #937
[inst-discovered] theory-solving 0 basic# ; #938
[mk-app] #941 = #938 #940
[instance] 0 #941
[attach-enode] #941 0
[end-of-instance]
[mk-app] #941 fuel_bool_default #880
[mk-app] #942 fuel_bool_default #859
[mk-app] #943 fuel_bool_default #867
[mk-app] #944 fuel_bool_default #868
[mk-app] #945 fuel_bool_default #860
[mk-app] #946 fuel_bool_default #841
[mk-app] #947 fuel_bool_default #842
[mk-app] #948 fuel_bool_default #869
[mk-app] #949 fuel_bool_default #840
[mk-app] #950 fuel_bool_default #870
[mk-app] #951 fuel_bool_default #861
[mk-app] #952 fuel_bool_default #876
[mk-app] #953 fuel_bool_default #871
[mk-app] #954 fuel_bool_default #872
[mk-app] #955 fuel_bool_default #877
[mk-app] #956 fuel_bool_default #873
[mk-app] #957 fuel_bool_default #879
[mk-app] #958 fuel_bool_default #875
[mk-app] #959 fuel_bool_default #874
[mk-app] #960 and #906 #920 #942 #943 #944 #945 #946 #883 #947 #948 #949 #950 #951 #901 #952 #953 #954 #955 #956 #932 #957 #958 #959
[mk-app] #961 => #941 #960
[mk-app] #962 not #941
[mk-app] #963 or #962 #960
[inst-discovered] theory-solving 0 basic# ; #961
[mk-app] #964 = #961 #963
[instance] 0 #964
[attach-enode] #964 0
[end-of-instance]
[mk-app] #964 fuel_bool_default #881
[mk-app] #965 and #964 #906 #925 #927
[mk-var] datatype#10 0
[mk-app] datatype#11 core!option.Option./Some datatype#10
[mk-app] datatype#12 pattern datatype#11
[mk-app] datatype#13 core!option.Option./Some/?0 datatype#11
[mk-app] datatype#14 = datatype#10 datatype#13
[mk-quant] datatype#15 constructor_accessor_axiom 1 datatype#12 datatype#14
[attach-var-names] datatype#15 (;Poly)
[mk-var] datatype#16 0
[mk-app] datatype#17 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS datatype#16
[mk-app] datatype#18 pattern datatype#17
[mk-app] datatype#19 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq datatype#17
[mk-app] datatype#20 = datatype#16 datatype#19
[mk-quant] datatype#21 constructor_accessor_axiom 1 datatype#18 datatype#20
[attach-var-names] datatype#21 (;Poly)
[mk-app] #966 Poly%fun%1. #161
[mk-app] #967 %Poly%fun%1. #966
[mk-app] #968 = #161 #967
[mk-app] #969 pattern #966
[mk-quant] #970 internal_crate__fun__1_box_axiom_definition 1 #969 #968
[attach-var-names] #970 (|x| ; |%%Function%%|)
[mk-var] #971 4
[mk-var] #972 3
[mk-app] #973 TYPE%fun%1. #971 #972 #65 #66
[mk-app] #974 has_type #34 #973
[mk-app] #975 %Poly%fun%1. #34
[mk-app] #976 Poly%fun%1. #975
[mk-app] #977 = #34 #976
[mk-app] #978 => #974 #977
[mk-app] #979 pattern #974
[mk-quant] #980 internal_crate__fun__1_unbox_axiom_definition 5 #979 #978
[attach-var-names] #980 (|x| ; |Poly|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #981 not #974
[mk-app] #982 or #981 #977
[inst-discovered] theory-solving 0 basic# ; #978
[mk-app] #983 = #978 #982
[instance] 0 #983
[attach-enode] #983 0
[end-of-instance]
[mk-quant] #983 internal_crate__fun__1_unbox_axiom_definition 5 #979 #982
[attach-var-names] #983 (|x| ; |Poly|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-var] #984 4
[mk-app] #985 has_type #34 #984
[mk-var] #986 1
[mk-app] #987 %%apply%%0 #986 #34
[mk-app] #988 has_type #987 #268
[mk-app] #989 => #985 #988
[mk-app] #990 pattern #988
[mk-quant] #991 internal_crate__fun__1_constructor_inner_definition 1 #990 #989
[attach-var-names] #991 (|T%0| ; |Poly|)
[mk-app] #992 Poly%fun%1. #162
[mk-app] #993 has_type #992 #973
[mk-app] #994 => #991 #993
[mk-app] #995 pattern #993
[mk-quant] #996 internal_crate__fun__1_constructor_definition 5 #995 #994
[attach-var-names] #996 (|x| ; |%%Function%%|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #997 not #985
[mk-app] #998 or #997 #988
[inst-discovered] theory-solving 0 basic# ; #989
[mk-app] #999 = #989 #998
[instance] 0 #999
[attach-enode] #999 0
[end-of-instance]
[mk-quant] #999 internal_crate__fun__1_constructor_inner_definition 1 #990 #998
[attach-var-names] #999 (|T%0| ; |Poly|)
[mk-app] #1000 not #999
[mk-app] #1001 or #1000 #993
[mk-app] #1002 => #999 #993
[inst-discovered] theory-solving 0 basic# ; #1002
[mk-app] #1003 = #1002 #1001
[instance] 0 #1003
[attach-enode] #1003 0
[end-of-instance]
[mk-quant] #1002 internal_crate__fun__1_constructor_definition 5 #995 #1001
[attach-var-names] #1002 (|x| ; |%%Function%%|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-var] #1003 5
[mk-var] #1004 3
[mk-app] #1005 TYPE%fun%1. #1003 #984 #1004 #268
[mk-app] #1006 has_type #966 #1005
[mk-app] #1007 has_type #33 #984
[mk-app] #1008 and #1006 #1007
[mk-app] #1009 %%apply%%0 #161 #33
[mk-app] #1010 has_type #1009 #268
[mk-app] #1011 => #1008 #1010
[mk-app] #1012 pattern #1009 #1006
[mk-quant] #1013 internal_crate__fun__1_apply_definition 6 #1012 #1011
[attach-var-names] #1013 (|x| ; |%%Function%%|) (|T%0| ; |Poly|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1014 not #1008
[mk-app] #1015 or #1014 #1010
[inst-discovered] theory-solving 0 basic# ; #1011
[mk-app] #1016 = #1011 #1015
[instance] 0 #1016
[attach-enode] #1016 0
[end-of-instance]
[mk-quant] #1016 internal_crate__fun__1_apply_definition 6 #1012 #1015
[attach-var-names] #1016 (|x| ; |%%Function%%|) (|T%0| ; |Poly|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1017 height #1009
[mk-app] #1018 fun_from_recursive_field #992
[mk-app] #1019 height #1018
[mk-app] #1020 height_lt #1017 #1019
[mk-app] #1021 => #1008 #1020
[mk-app] #1022 pattern #1017 #1006
[mk-quant] #1023 internal_crate__fun__1_height_apply_definition 6 #1022 #1021
[attach-var-names] #1023 (|x| ; |%%Function%%|) (|T%0| ; |Poly|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1024 or #1014 #1020
[inst-discovered] theory-solving 0 basic# ; #1021
[mk-app] #1025 = #1021 #1024
[instance] 0 #1025
[attach-enode] #1025 0
[end-of-instance]
[mk-quant] #1025 internal_crate__fun__1_height_apply_definition 6 #1022 #1024
[attach-var-names] #1025 (|x| ; |%%Function%%|) (|T%0| ; |Poly|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-var] #1026 6
[mk-var] #1027 5
[mk-var] #1028 2
[mk-app] #1029 TYPE%fun%1. #1026 #1027 #971 #972
[mk-app] #1030 has_type #33 #1029
[mk-app] #1031 has_type #34 #1029
[mk-var] #1032 6
[mk-app] #1033 has_type #34 #1032
[mk-app] #1034 %Poly%fun%1. #44
[mk-app] #1035 %%apply%%0 #1034 #34
[mk-app] #1036 %Poly%fun%1. #33
[mk-app] #1037 %%apply%%0 #1036 #34
[mk-app] #1038 ext_eq #267 #984 #1035 #1037
[mk-app] #1039 => #1033 #1038
[mk-app] #1040 pattern #1038
[mk-quant] #1041 internal_crate__fun__1_inner_ext_equal_definition 1 #1040 #1039
[attach-var-names] #1041 (|T%0| ; |Poly|)
[mk-app] #1042 and #1030 #1031 #1041
[mk-app] #1043 ext_eq #1028 #1029 #33 #34
[mk-app] #1044 => #1042 #1043
[mk-app] #1045 pattern #1043
[mk-quant] #1046 internal_crate__fun__1_ext_equal_definition 7 #1045 #1044
[attach-var-names] #1046 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1047 not #1033
[mk-app] #1048 or #1047 #1038
[inst-discovered] theory-solving 0 basic# ; #1039
[mk-app] #1049 = #1039 #1048
[instance] 0 #1049
[attach-enode] #1049 0
[end-of-instance]
[mk-quant] #1049 internal_crate__fun__1_inner_ext_equal_definition 1 #1040 #1048
[attach-var-names] #1049 (|T%0| ; |Poly|)
[mk-app] #1050 and #1030 #1031 #1049
[mk-app] #1051 not #1050
[mk-app] #1052 or #1051 #1043
[mk-app] #1053 => #1050 #1043
[inst-discovered] theory-solving 0 basic# ; #1053
[mk-app] #1054 = #1053 #1052
[instance] 0 #1054
[attach-enode] #1054 0
[end-of-instance]
[mk-quant] #1053 internal_crate__fun__1_ext_equal_definition 7 #1045 #1052
[attach-var-names] #1053 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1054 Poly%fun%2. #161
[mk-app] #1055 %Poly%fun%2. #1054
[mk-app] #1056 = #161 #1055
[mk-app] #1057 pattern #1054
[mk-quant] #1058 internal_crate__fun__2_box_axiom_definition 1 #1057 #1056
[attach-var-names] #1058 (|x| ; |%%Function%%|)
[mk-app] #1059 TYPE%fun%2. #1026 #1027 #971 #972 #65 #66
[mk-app] #1060 has_type #34 #1059
[mk-app] #1061 %Poly%fun%2. #34
[mk-app] #1062 Poly%fun%2. #1061
[mk-app] #1063 = #34 #1062
[mk-app] #1064 => #1060 #1063
[mk-app] #1065 pattern #1060
[mk-quant] #1066 internal_crate__fun__2_unbox_axiom_definition 7 #1065 #1064
[attach-var-names] #1066 (|x| ; |Poly|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1067 not #1060
[mk-app] #1068 or #1067 #1063
[inst-discovered] theory-solving 0 basic# ; #1064
[mk-app] #1069 = #1064 #1068
[instance] 0 #1069
[attach-enode] #1069 0
[end-of-instance]
[mk-quant] #1069 internal_crate__fun__2_unbox_axiom_definition 7 #1065 #1068
[attach-var-names] #1069 (|x| ; |Poly|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-var] #1070 7
[mk-app] #1071 has_type #33 #1070
[mk-app] #1072 has_type #34 #1027
[mk-app] #1073 and #1071 #1072
[mk-var] #1074 2
[mk-app] #1075 %%apply%%1 #1074 #33 #34
[mk-app] #1076 has_type #1075 #972
[mk-app] #1077 => #1073 #1076
[mk-app] #1078 pattern #1076
[mk-quant] #1079 internal_crate__fun__2_constructor_inner_definition 2 #1078 #1077
[attach-var-names] #1079 (|T%1| ; |Poly|) (|T%0| ; |Poly|)
[mk-app] #1080 Poly%fun%2. #162
[mk-app] #1081 has_type #1080 #1059
[mk-app] #1082 => #1079 #1081
[mk-app] #1083 pattern #1081
[mk-quant] #1084 internal_crate__fun__2_constructor_definition 7 #1083 #1082
[attach-var-names] #1084 (|x| ; |%%Function%%|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1085 not #1073
[mk-app] #1086 or #1085 #1076
[inst-discovered] theory-solving 0 basic# ; #1077
[mk-app] #1087 = #1077 #1086
[instance] 0 #1087
[attach-enode] #1087 0
[end-of-instance]
[mk-quant] #1087 internal_crate__fun__2_constructor_inner_definition 2 #1078 #1086
[attach-var-names] #1087 (|T%1| ; |Poly|) (|T%0| ; |Poly|)
[mk-app] #1088 not #1087
[mk-app] #1089 or #1088 #1081
[mk-app] #1090 => #1087 #1081
[inst-discovered] theory-solving 0 basic# ; #1090
[mk-app] #1091 = #1090 #1089
[instance] 0 #1091
[attach-enode] #1091 0
[end-of-instance]
[mk-quant] #1090 internal_crate__fun__2_constructor_definition 7 #1083 #1089
[attach-var-names] #1090 (|x| ; |%%Function%%|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-var] #1091 8
[mk-app] #1092 TYPE%fun%2. #1091 #1070 #1026 #1027 #971 #972
[mk-app] #1093 has_type #1054 #1092
[mk-app] #1094 has_type #44 #1070
[mk-app] #1095 has_type #33 #1027
[mk-app] #1096 and #1093 #1094 #1095
[mk-app] #1097 %%apply%%1 #161 #44 #33
[mk-app] #1098 has_type #1097 #972
[mk-app] #1099 => #1096 #1098
[mk-app] #1100 pattern #1097 #1093
[mk-quant] #1101 internal_crate__fun__2_apply_definition 9 #1100 #1099
[attach-var-names] #1101 (|x| ; |%%Function%%|) (|T%1| ; |Poly|) (|T%0| ; |Poly|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1102 not #1096
[mk-app] #1103 or #1102 #1098
[inst-discovered] theory-solving 0 basic# ; #1099
[mk-app] #1104 = #1099 #1103
[instance] 0 #1104
[attach-enode] #1104 0
[end-of-instance]
[mk-quant] #1104 internal_crate__fun__2_apply_definition 9 #1100 #1103
[attach-var-names] #1104 (|x| ; |%%Function%%|) (|T%1| ; |Poly|) (|T%0| ; |Poly|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1105 height #1097
[mk-app] #1106 fun_from_recursive_field #1080
[mk-app] #1107 height #1106
[mk-app] #1108 height_lt #1105 #1107
[mk-app] #1109 => #1096 #1108
[mk-app] #1110 pattern #1105 #1093
[mk-quant] #1111 internal_crate__fun__2_height_apply_definition 9 #1110 #1109
[attach-var-names] #1111 (|x| ; |%%Function%%|) (|T%1| ; |Poly|) (|T%0| ; |Poly|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1112 or #1102 #1108
[inst-discovered] theory-solving 0 basic# ; #1109
[mk-app] #1113 = #1109 #1112
[instance] 0 #1113
[attach-enode] #1113 0
[end-of-instance]
[mk-quant] #1113 internal_crate__fun__2_height_apply_definition 9 #1110 #1112
[attach-var-names] #1113 (|x| ; |%%Function%%|) (|T%1| ; |Poly|) (|T%0| ; |Poly|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1114 has_type #33 #1092
[mk-app] #1115 has_type #34 #1092
[mk-var] #1116 9
[mk-app] #1117 has_type #33 #1116
[mk-app] #1118 has_type #34 #1070
[mk-app] #1119 and #1117 #1118
[mk-var] #1120 4
[mk-app] #1121 %Poly%fun%2. #64
[mk-app] #1122 %%apply%%1 #1121 #33 #34
[mk-app] #1123 %Poly%fun%2. #44
[mk-app] #1124 %%apply%%1 #1123 #33 #34
[mk-app] #1125 ext_eq #1120 #1027 #1122 #1124
[mk-app] #1126 => #1119 #1125
[mk-app] #1127 pattern #1125
[mk-quant] #1128 internal_crate__fun__2_inner_ext_equal_definition 2 #1127 #1126
[attach-var-names] #1128 (|T%1| ; |Poly|) (|T%0| ; |Poly|)
[mk-app] #1129 and #1114 #1115 #1128
[mk-app] #1130 ext_eq #1028 #1092 #33 #34
[mk-app] #1131 => #1129 #1130
[mk-app] #1132 pattern #1130
[mk-quant] #1133 internal_crate__fun__2_ext_equal_definition 9 #1132 #1131
[attach-var-names] #1133 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #1134 not #1119
[mk-app] #1135 or #1134 #1125
[inst-discovered] theory-solving 0 basic# ; #1126
[mk-app] #1136 = #1126 #1135
[instance] 0 #1136
[attach-enode] #1136 0
[end-of-instance]
[mk-quant] #1136 internal_crate__fun__2_inner_ext_equal_definition 2 #1127 #1135
[attach-var-names] #1136 (|T%1| ; |Poly|) (|T%0| ; |Poly|)
[mk-app] #1137 and #1114 #1115 #1136
[mk-app] #1138 not #1137
[mk-app] #1139 or #1138 #1130
[mk-app] #1140 => #1137 #1130
[inst-discovered] theory-solving 0 basic# ; #1140
[mk-app] #1141 = #1140 #1139
[instance] 0 #1141
[attach-enode] #1141 0
[end-of-instance]
[mk-quant] #1140 internal_crate__fun__2_ext_equal_definition 9 #1132 #1139
[attach-var-names] #1140 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-var] #1141 0
[mk-app] #1142 Poly%alloc!alloc.Global. #1141
[mk-app] #1143 %Poly%alloc!alloc.Global. #1142
[mk-app] #1144 = #1141 #1143
[mk-app] #1145 pattern #1142
[mk-quant] #1146 internal_alloc__alloc__Global_box_axiom_definition 1 #1145 #1144
[attach-var-names] #1146 (|x| ; |alloc!alloc.Global.|)
[mk-app] #1147 TYPE%alloc!alloc.Global.
[mk-app] #1148 has_type #34 #1147
[mk-app] #1149 %Poly%alloc!alloc.Global. #34
[mk-app] #1150 Poly%alloc!alloc.Global. #1149
[mk-app] #1151 = #34 #1150
[mk-app] #1152 => #1148 #1151
[mk-app] #1153 pattern #1148
[mk-quant] #1154 internal_alloc__alloc__Global_unbox_axiom_definition 1 #1153 #1152
[attach-var-names] #1154 (|x| ; |Poly|)
[mk-app] #1155 not #1148
[mk-app] #1156 or #1155 #1151
[inst-discovered] theory-solving 0 basic# ; #1152
[mk-app] #1157 = #1152 #1156
[instance] 0 #1157
[attach-enode] #1157 0
[end-of-instance]
[mk-quant] #1157 internal_alloc__alloc__Global_unbox_axiom_definition 1 #1153 #1156
[attach-var-names] #1157 (|x| ; |Poly|)
[mk-app] #1158 has_type #1142 #1147
[mk-app] #1159 pattern #1158
[mk-quant] #1160 internal_alloc__alloc__Global_has_type_always_definition 1 #1159 #1158
[attach-var-names] #1160 (|x| ; |alloc!alloc.Global.|)
[mk-var] #1161 0
[mk-app] #1162 Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #1161
[mk-app] #1163 %Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #1162
[mk-app] #1164 = #1161 #1163
[mk-app] #1165 pattern #1162
[mk-quant] #1166 internal_alloc__vec__Vec<i32./alloc!alloc.Global.>_box_axiom_definition 1 #1165 #1164
[attach-var-names] #1166 (|x| ; |alloc!vec.Vec<i32./alloc!alloc.Global.>.|)
[attach-meaning] #275 arith 32
[mk-app] #1167 SINT #275
[mk-app] #1168 TYPE%alloc!vec.Vec. #125 #1167 #125 #1147
[mk-app] #1169 has_type #34 #1168
[mk-app] #1170 %Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #34
[mk-app] #1171 Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #1170
[mk-app] #1172 = #34 #1171
[mk-app] #1173 => #1169 #1172
[attach-meaning] #275 arith 32
[mk-app] #1174 pattern #1169
[mk-quant] #1175 internal_alloc__vec__Vec<i32./alloc!alloc.Global.>_unbox_axiom_definition 1 #1174 #1173
[attach-var-names] #1175 (|x| ; |Poly|)
[mk-app] #1176 not #1169
[mk-app] #1177 or #1176 #1172
[inst-discovered] theory-solving 0 basic# ; #1173
[mk-app] #1178 = #1173 #1177
[instance] 0 #1178
[attach-enode] #1178 0
[end-of-instance]
[mk-quant] #1178 internal_alloc__vec__Vec<i32./alloc!alloc.Global.>_unbox_axiom_definition 1 #1174 #1177
[attach-var-names] #1178 (|x| ; |Poly|)
[attach-meaning] #275 arith 32
[mk-app] #1179 has_type #1162 #1168
[attach-meaning] #275 arith 32
[mk-app] #1180 pattern #1179
[mk-quant] #1181 internal_alloc__vec__Vec<i32./alloc!alloc.Global.>_has_type_always_definition 1 #1180 #1179
[attach-var-names] #1181 (|x| ; |alloc!vec.Vec<i32./alloc!alloc.Global.>.|)
[mk-var] #1182 0
[mk-app] #1183 Poly%vstd!seq.Seq<i32.>. #1182
[mk-app] #1184 %Poly%vstd!seq.Seq<i32.>. #1183
[mk-app] #1185 = #1182 #1184
[mk-app] #1186 pattern #1183
[mk-quant] #1187 internal_vstd__seq__Seq<i32.>_box_axiom_definition 1 #1186 #1185
[attach-var-names] #1187 (|x| ; |vstd!seq.Seq<i32.>.|)
[attach-meaning] #275 arith 32
[mk-app] #1188 TYPE%vstd!seq.Seq. #125 #1167
[mk-app] #1189 has_type #34 #1188
[mk-app] #1190 %Poly%vstd!seq.Seq<i32.>. #34
[mk-app] #1191 Poly%vstd!seq.Seq<i32.>. #1190
[mk-app] #1192 = #34 #1191
[mk-app] #1193 => #1189 #1192
[attach-meaning] #275 arith 32
[mk-app] #1194 pattern #1189
[mk-quant] #1195 internal_vstd__seq__Seq<i32.>_unbox_axiom_definition 1 #1194 #1193
[attach-var-names] #1195 (|x| ; |Poly|)
[mk-app] #1196 not #1189
[mk-app] #1197 or #1196 #1192
[inst-discovered] theory-solving 0 basic# ; #1193
[mk-app] #1198 = #1193 #1197
[instance] 0 #1198
[attach-enode] #1198 0
[end-of-instance]
[mk-quant] #1198 internal_vstd__seq__Seq<i32.>_unbox_axiom_definition 1 #1194 #1197
[attach-var-names] #1198 (|x| ; |Poly|)
[attach-meaning] #275 arith 32
[mk-app] #1199 has_type #1183 #1188
[attach-meaning] #275 arith 32
[mk-app] #1200 pattern #1199
[mk-quant] #1201 internal_vstd__seq__Seq<i32.>_has_type_always_definition 1 #1200 #1199
[attach-var-names] #1201 (|x| ; |vstd!seq.Seq<i32.>.|)
[mk-var] #1202 0
[mk-app] #1203 Poly%core!option.Option. #1202
[mk-app] #1204 %Poly%core!option.Option. #1203
[mk-app] #1205 = #1202 #1204
[mk-app] #1206 pattern #1203
[mk-quant] #1207 internal_core__option__Option_box_axiom_definition 1 #1206 #1205
[attach-var-names] #1207 (|x| ; |core!option.Option.|)
[mk-app] #1208 TYPE%core!option.Option. #65 #66
[mk-app] #1209 has_type #34 #1208
[mk-app] #1210 %Poly%core!option.Option. #34
[mk-app] #1211 Poly%core!option.Option. #1210
[mk-app] #1212 = #34 #1211
[mk-app] #1213 => #1209 #1212
[mk-app] #1214 pattern #1209
[mk-quant] #1215 internal_core__option__Option_unbox_axiom_definition 3 #1214 #1213
[attach-var-names] #1215 (|x| ; |Poly|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1216 not #1209
[mk-app] #1217 or #1216 #1212
[inst-discovered] theory-solving 0 basic# ; #1213
[mk-app] #1218 = #1213 #1217
[instance] 0 #1218
[attach-enode] #1218 0
[end-of-instance]
[mk-quant] #1218 internal_core__option__Option_unbox_axiom_definition 3 #1214 #1217
[attach-var-names] #1218 (|x| ; |Poly|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1219 core!option.Option./None
[mk-app] #1220 Poly%core!option.Option. #1219
[mk-app] #1221 TYPE%core!option.Option. #45 #46
[mk-app] #1222 has_type #1220 #1221
[mk-app] #1223 pattern #1222
[mk-quant] #1224 internal_core!option.Option./None_constructor_definition 2 #1223 #1222
[attach-var-names] #1224 (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1225 core!option.Option./Some #34
[mk-app] #1226 Poly%core!option.Option. #1225
[mk-app] #1227 has_type #1226 #1208
[mk-app] #1228 => #69 #1227
[mk-app] #1229 pattern #1227
[mk-quant] #1230 internal_core!option.Option./Some_constructor_definition 3 #1229 #1228
[attach-var-names] #1230 (|_0!| ; |Poly|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1231 not #69
[mk-app] #1232 or #1231 #1227
[inst-discovered] theory-solving 0 basic# ; #1228
[mk-app] #1233 = #1228 #1232
[instance] 0 #1233
[attach-enode] #1233 0
[end-of-instance]
[mk-quant] #1233 internal_core!option.Option./Some_constructor_definition 3 #1229 #1232
[attach-var-names] #1233 (|_0!| ; |Poly|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1234 is-core!option.Option./Some #1202
[mk-app] #1235 core!option.Option./Some/0 #65 #66 #1202
[mk-app] #1236 core!option.Option./Some/?0 #1202
[mk-app] #1237 = #1235 #1236
[mk-app] #1238 => #1234 #1237
[mk-app] #1239 pattern #1235
[mk-quant] #1240 internal_core!option.Option./Some/0_accessor_definition 3 #1239 #1238
[attach-var-names] #1240 (|x| ; |core!option.Option.|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1241 is #1202
[inst-discovered] theory-solving 0 datatype# ; #1234
[mk-app] #1242 = #1234 #1241
[instance] 0 #1242
[attach-enode] #1242 0
[end-of-instance]
[mk-app] #1242 not #1241
[mk-app] #1243 or #1242 #1237
[mk-app] #1244 => #1241 #1237
[inst-discovered] theory-solving 0 basic# ; #1244
[mk-app] #1245 = #1244 #1243
[instance] 0 #1245
[attach-enode] #1245 0
[end-of-instance]
[mk-quant] #1244 internal_core!option.Option./Some/0_accessor_definition 3 #1239 #1243
[attach-var-names] #1244 (|x| ; |core!option.Option.|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1245 core!option.Option./Some/0 #65 #66 #1210
[mk-app] #1246 has_type #1245 #66
[mk-app] #1247 => #1209 #1246
[mk-app] #1248 pattern #1245 #1209
[mk-quant] #1249 internal_core!option.Option./Some/0_invariant_definition 3 #1248 #1247
[attach-var-names] #1249 (|x| ; |Poly|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1250 or #1216 #1246
[inst-discovered] theory-solving 0 basic# ; #1247
[mk-app] #1251 = #1247 #1250
[instance] 0 #1251
[attach-enode] #1251 0
[end-of-instance]
[mk-quant] #1251 internal_core!option.Option./Some/0_invariant_definition 3 #1248 #1250
[attach-var-names] #1251 (|x| ; |Poly|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1252 height #1235
[mk-app] #1253 height #1203
[mk-app] #1254 height_lt #1252 #1253
[mk-app] #1255 => #1234 #1254
[mk-app] #1256 pattern #1252
[mk-quant] #1257 prelude_datatype_height_core!option.Option./Some/0 3 #1256 #1255
[attach-var-names] #1257 (|x| ; |core!option.Option.|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[inst-discovered] theory-solving 0 datatype# ; #1234
[mk-app] #1258 = #1234 #1241
[instance] 0 #1258
[attach-enode] #1258 0
[end-of-instance]
[mk-app] #1258 or #1242 #1254
[mk-app] #1259 => #1241 #1254
[inst-discovered] theory-solving 0 basic# ; #1259
[mk-app] #1260 = #1259 #1258
[instance] 0 #1260
[attach-enode] #1260 0
[end-of-instance]
[mk-quant] #1259 prelude_datatype_height_core!option.Option./Some/0 3 #1256 #1258
[attach-var-names] #1259 (|x| ; |core!option.Option.|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1260 TYPE%core!option.Option. #971 #972
[mk-app] #1261 has_type #33 #1260
[mk-app] #1262 has_type #34 #1260
[mk-app] #1263 %Poly%core!option.Option. #33
[mk-app] #1264 is-core!option.Option./None #1263
[mk-app] #1265 is-core!option.Option./None #1210
[mk-app] #1266 and #1261 #1262 #1264 #1265
[mk-app] #1267 ext_eq #1028 #1260 #33 #34
[mk-app] #1268 => #1266 #1267
[mk-app] #1269 pattern #1267
[mk-quant] #1270 internal_core!option.Option./None_ext_equal_definition 5 #1269 #1268
[attach-var-names] #1270 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1271 is #1263
[inst-discovered] theory-solving 0 datatype# ; #1264
[mk-app] #1272 = #1264 #1271
[instance] 0 #1272
[attach-enode] #1272 0
[end-of-instance]
[mk-app] #1272 is #1210
[inst-discovered] theory-solving 0 datatype# ; #1265
[mk-app] #1273 = #1265 #1272
[instance] 0 #1273
[attach-enode] #1273 0
[end-of-instance]
[mk-app] #1273 and #1261 #1262 #1271 #1272
[mk-app] #1274 not #1273
[mk-app] #1275 or #1274 #1267
[mk-app] #1276 => #1273 #1267
[inst-discovered] theory-solving 0 basic# ; #1276
[mk-app] #1277 = #1276 #1275
[instance] 0 #1277
[attach-enode] #1277 0
[end-of-instance]
[mk-quant] #1276 internal_core!option.Option./None_ext_equal_definition 5 #1269 #1275
[attach-var-names] #1276 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1277 is-core!option.Option./Some #1263
[mk-app] #1278 is-core!option.Option./Some #1210
[mk-app] #1279 core!option.Option./Some/0 #971 #972 #1263
[mk-app] #1280 core!option.Option./Some/0 #971 #972 #1210
[mk-app] #1281 ext_eq #1028 #972 #1279 #1280
[mk-app] #1282 and #1261 #1262 #1277 #1278 #1281
[mk-app] #1283 => #1282 #1267
[mk-quant] #1284 internal_core!option.Option./Some_ext_equal_definition 5 #1269 #1283
[attach-var-names] #1284 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #1285 is #1263
[inst-discovered] theory-solving 0 datatype# ; #1277
[mk-app] #1286 = #1277 #1285
[instance] 0 #1286
[attach-enode] #1286 0
[end-of-instance]
[mk-app] #1286 is #1210
[inst-discovered] theory-solving 0 datatype# ; #1278
[mk-app] #1287 = #1278 #1286
[instance] 0 #1287
[attach-enode] #1287 0
[end-of-instance]
[mk-app] #1287 and #1261 #1262 #1285 #1286 #1281
[mk-app] #1288 not #1287
[mk-app] #1289 or #1288 #1267
[mk-app] #1290 => #1287 #1267
[inst-discovered] theory-solving 0 basic# ; #1290
[mk-app] #1291 = #1290 #1289
[instance] 0 #1291
[attach-enode] #1291 0
[end-of-instance]
[mk-quant] #1290 internal_core!option.Option./Some_ext_equal_definition 5 #1269 #1289
[attach-var-names] #1290 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-var] #1291 0
[mk-app] #1292 Poly%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #1291
[mk-app] #1293 %Poly%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #1292
[mk-app] #1294 = #1291 #1293
[mk-app] #1295 pattern #1292
[mk-quant] #1296 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS_box_axiom_definition 1 #1295 #1294
[attach-var-names] #1296 (|x| ; |lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS.|)
[mk-app] #1297 TYPE%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #65 #66
[mk-app] #1298 has_type #34 #1297
[mk-app] #1299 %Poly%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #34
[mk-app] #1300 Poly%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #1299
[mk-app] #1301 = #34 #1300
[mk-app] #1302 => #1298 #1301
[mk-app] #1303 pattern #1298
[mk-quant] #1304 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS_unbox_axiom_definition 3 #1303 #1302
[attach-var-names] #1304 (|x| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1305 not #1298
[mk-app] #1306 or #1305 #1301
[inst-discovered] theory-solving 0 basic# ; #1302
[mk-app] #1307 = #1302 #1306
[instance] 0 #1307
[attach-enode] #1307 0
[end-of-instance]
[mk-quant] #1307 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS_unbox_axiom_definition 3 #1303 #1306
[attach-var-names] #1307 (|x| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1308 TYPE%alloc!vec.Vec. #65 #66 #125 #1147
[mk-app] #1309 has_type #34 #1308
[mk-app] #1310 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS #34
[mk-app] #1311 Poly%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #1310
[mk-app] #1312 has_type #1311 #1297
[mk-app] #1313 => #1309 #1312
[mk-app] #1314 pattern #1312
[mk-quant] #1315 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS_constructor_definition 3 #1314 #1313
[attach-var-names] #1315 (|_seq!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1316 not #1309
[mk-app] #1317 or #1316 #1312
[inst-discovered] theory-solving 0 basic# ; #1313
[mk-app] #1318 = #1313 #1317
[instance] 0 #1318
[attach-enode] #1318 0
[end-of-instance]
[mk-quant] #1318 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS_constructor_definition 3 #1314 #1317
[attach-var-names] #1318 (|_seq!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1319 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq #1291
[mk-app] #1320 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #1291
[mk-app] #1321 = #1319 #1320
[mk-app] #1322 pattern #1319
[mk-quant] #1323 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq_accessor_definition 1 #1322 #1321
[attach-var-names] #1323 (|x| ; |lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS.|)
[mk-app] #1324 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq #1299
[mk-app] #1325 has_type #1324 #1308
[mk-app] #1326 => #1298 #1325
[mk-app] #1327 pattern #1324 #1298
[mk-quant] #1328 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq_invariant_definition 3 #1327 #1326
[attach-var-names] #1328 (|x| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1329 or #1305 #1325
[inst-discovered] theory-solving 0 basic# ; #1326
[mk-app] #1330 = #1326 #1329
[instance] 0 #1330
[attach-enode] #1330 0
[end-of-instance]
[mk-quant] #1330 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq_invariant_definition 3 #1327 #1329
[attach-var-names] #1330 (|x| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1331 is-lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS #1291
[mk-app] #1332 height #1319
[mk-app] #1333 height #1292
[mk-app] #1334 height_lt #1332 #1333
[mk-app] #1335 => #1331 #1334
[mk-app] #1336 pattern #1332
[mk-quant] #1337 prelude_datatype_height_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq 1 #1336 #1335
[attach-var-names] #1337 (|x| ; |lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS.|)
[mk-app] #1338 is #1291
[inst-discovered] theory-solving 0 datatype# ; #1331
[mk-app] #1339 = #1331 #1338
[instance] 0 #1339
[attach-enode] #1339 0
[end-of-instance]
[inst-discovered] theory-solving 0 datatype# ; #1338
[mk-app] #1339 = #1338 #1
[instance] 0 #1339
[attach-enode] #1339 0
[end-of-instance]
[mk-app] #1339 => #1 #1334
[inst-discovered] theory-solving 0 basic# ; #1339
[mk-app] #1340 = #1339 #1334
[instance] 0 #1340
[attach-enode] #1340 0
[end-of-instance]
[mk-quant] #1339 prelude_datatype_height_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq 1 #1336 #1334
[attach-var-names] #1339 (|x| ; |lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS.|)
[mk-var] #1338 0
[mk-app] #1340 Poly%tuple%0. #1338
[mk-app] #1341 %Poly%tuple%0. #1340
[mk-app] #1342 = #1338 #1341
[mk-app] #1343 pattern #1340
[mk-quant] #1344 internal_crate__tuple__0_box_axiom_definition 1 #1343 #1342
[attach-var-names] #1344 (|x| ; |tuple%0.|)
[mk-app] #1345 TYPE%tuple%0.
[mk-app] #1346 has_type #34 #1345
[mk-app] #1347 %Poly%tuple%0. #34
[mk-app] #1348 Poly%tuple%0. #1347
[mk-app] #1349 = #34 #1348
[mk-app] #1350 => #1346 #1349
[mk-app] #1351 pattern #1346
[mk-quant] #1352 internal_crate__tuple__0_unbox_axiom_definition 1 #1351 #1350
[attach-var-names] #1352 (|x| ; |Poly|)
[mk-app] #1353 not #1346
[mk-app] #1354 or #1353 #1349
[inst-discovered] theory-solving 0 basic# ; #1350
[mk-app] #1355 = #1350 #1354
[instance] 0 #1355
[attach-enode] #1355 0
[end-of-instance]
[mk-quant] #1355 internal_crate__tuple__0_unbox_axiom_definition 1 #1351 #1354
[attach-var-names] #1355 (|x| ; |Poly|)
[mk-app] #1356 has_type #1340 #1345
[mk-app] #1357 pattern #1356
[mk-quant] #1358 internal_crate__tuple__0_has_type_always_definition 1 #1357 #1356
[attach-var-names] #1358 (|x| ; |tuple%0.|)
[mk-app] #1359 tr_bound%vstd!view.View. #45 #46
[mk-app] #1360 proj%%vstd!view.View./V #45 #46
[mk-app] #1361 sized #1360
[mk-app] #1362 => #1359 #1361
[mk-app] #1363 pattern #1359
[mk-quant] #1364 internal_vstd__view__View_trait_type_bounds_definition 2 #1363 #1362
[attach-var-names] #1364 (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1365 not #1359
[mk-app] #1366 or #1365 #1361
[inst-discovered] theory-solving 0 basic# ; #1362
[mk-app] #1367 = #1362 #1366
[instance] 0 #1367
[attach-enode] #1367 0
[end-of-instance]
[mk-quant] #1367 internal_vstd__view__View_trait_type_bounds_definition 2 #1363 #1366
[attach-var-names] #1367 (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1368 tr_bound%core!alloc.Allocator. #45 #46
[mk-app] #1369 pattern #1368
[mk-quant] #1370 internal_core__alloc__Allocator_trait_type_bounds_definition 2 #1369 #1
[attach-var-names] #1370 (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1371 tr_bound%vstd!std_specs.option.OptionAdditionalFns. #1004 #268 #45 #46
[mk-app] #1372 sized #1004
[mk-app] #1373 sized #45
[mk-app] #1374 and #1372 #1373
[mk-app] #1375 => #1371 #1374
[mk-app] #1376 pattern #1371
[mk-quant] #1377 internal_vstd__std_specs__option__OptionAdditionalFns_trait_type_bounds_definition 4 #1376 #1375
[attach-var-names] #1377 (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1378 not #1371
[mk-app] #1379 or #1378 #1374
[inst-discovered] theory-solving 0 basic# ; #1375
[mk-app] #1380 = #1375 #1379
[instance] 0 #1380
[attach-enode] #1380 0
[end-of-instance]
[mk-quant] #1380 internal_vstd__std_specs__option__OptionAdditionalFns_trait_type_bounds_definition 4 #1376 #1379
[attach-var-names] #1380 (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1381 tr_bound%vstd!std_specs.vec.VecAdditionalSpecFns. #1004 #268 #45 #46
[mk-app] #1382 tr_bound%vstd!view.View. #1004 #268
[mk-app] #1383 proj%%vstd!view.View./V #1004 #268
[mk-app] #1384 = #125 #1383
[mk-app] #1385 TYPE%vstd!seq.Seq. #45 #46
[mk-app] #1386 proj%vstd!view.View./V #1004 #268
[mk-app] #1387 = #1385 #1386
[mk-app] #1388 and #1384 #1387
[mk-app] #1389 and #1382 #1388 #1373
[mk-app] #1390 => #1381 #1389
[mk-app] #1391 pattern #1381
[mk-quant] #1392 internal_vstd__std_specs__vec__VecAdditionalSpecFns_trait_type_bounds_definition 4 #1391 #1390
[attach-var-names] #1392 (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1393 and #1382 #1384 #1387 #1373
[mk-app] #1394 not #1381
[mk-app] #1395 or #1394 #1393
[mk-app] #1396 => #1381 #1393
[inst-discovered] theory-solving 0 basic# ; #1396
[mk-app] #1397 = #1396 #1395
[instance] 0 #1397
[attach-enode] #1397 0
[end-of-instance]
[mk-quant] #1396 internal_vstd__std_specs__vec__VecAdditionalSpecFns_trait_type_bounds_definition 4 #1391 #1395
[attach-var-names] #1396 (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1397 tr_bound%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait. #1004 #268 #45 #46
[mk-app] #1398 => #1397 #1374
[mk-app] #1399 pattern #1397
[mk-quant] #1400 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphTrait_trait_type_bounds_definition 4 #1399 #1398
[attach-var-names] #1400 (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1401 not #1397
[mk-app] #1402 or #1401 #1374
[inst-discovered] theory-solving 0 basic# ; #1398
[mk-app] #1403 = #1398 #1402
[instance] 0 #1403
[attach-enode] #1403 0
[end-of-instance]
[mk-quant] #1403 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphTrait_trait_type_bounds_definition 4 #1399 #1402
[attach-var-names] #1403 (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1404 tr_bound%lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.MaxContigSubSumOptTrait. #45 #46
[mk-app] #1405 pattern #1404
[mk-quant] #1406 internal_lib__Chap28__MaxContigSubSumOptStEph__MaxContigSubSumOptStEph__MaxContigSubSumOptTrait_trait_type_bounds_definition 2 #1405 #1
[attach-var-names] #1406 (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1407 REF #45
[mk-app] #1408 proj%%vstd!view.View./V #1407 #46
[mk-app] #1409 = #1408 #1360
[mk-app] #1410 => #1359 #1409
[mk-app] #1411 pattern #1408
[mk-quant] #1412 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1411 #1410
[attach-var-names] #1412 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1413 or #1365 #1409
[inst-discovered] theory-solving 0 basic# ; #1410
[mk-app] #1414 = #1410 #1413
[instance] 0 #1414
[attach-enode] #1414 0
[end-of-instance]
[mk-quant] #1414 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1411 #1413
[attach-var-names] #1414 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1415 proj%vstd!view.View./V #1407 #46
[mk-app] #1416 proj%vstd!view.View./V #45 #46
[mk-app] #1417 = #1415 #1416
[mk-app] #1418 => #1359 #1417
[mk-app] #1419 pattern #1415
[mk-quant] #1420 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1419 #1418
[attach-var-names] #1420 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1421 or #1365 #1417
[inst-discovered] theory-solving 0 basic# ; #1418
[mk-app] #1422 = #1418 #1421
[instance] 0 #1422
[attach-enode] #1422 0
[end-of-instance]
[mk-quant] #1422 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1419 #1421
[attach-var-names] #1422 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1423 BOX #125 #1147 #45
[mk-app] #1424 proj%%vstd!view.View./V #1423 #46
[mk-app] #1425 = #1424 #1360
[mk-app] #1426 => #1359 #1425
[mk-app] #1427 pattern #1424
[mk-quant] #1428 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1427 #1426
[attach-var-names] #1428 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1429 or #1365 #1425
[inst-discovered] theory-solving 0 basic# ; #1426
[mk-app] #1430 = #1426 #1429
[instance] 0 #1430
[attach-enode] #1430 0
[end-of-instance]
[mk-quant] #1430 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1427 #1429
[attach-var-names] #1430 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1431 proj%vstd!view.View./V #1423 #46
[mk-app] #1432 = #1431 #1416
[mk-app] #1433 => #1359 #1432
[mk-app] #1434 pattern #1431
[mk-quant] #1435 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1434 #1433
[attach-var-names] #1435 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1436 or #1365 #1432
[inst-discovered] theory-solving 0 basic# ; #1433
[mk-app] #1437 = #1433 #1436
[instance] 0 #1437
[attach-enode] #1437 0
[end-of-instance]
[mk-quant] #1437 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1434 #1436
[attach-var-names] #1437 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1438 and #1373 #1359
[mk-app] #1439 RC #125 #1147 #45
[mk-app] #1440 proj%%vstd!view.View./V #1439 #46
[mk-app] #1441 = #1440 #1360
[mk-app] #1442 => #1438 #1441
[mk-app] #1443 pattern #1440
[mk-quant] #1444 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1443 #1442
[attach-var-names] #1444 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1445 not #1438
[mk-app] #1446 or #1445 #1441
[inst-discovered] theory-solving 0 basic# ; #1442
[mk-app] #1447 = #1442 #1446
[instance] 0 #1447
[attach-enode] #1447 0
[end-of-instance]
[mk-quant] #1447 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1443 #1446
[attach-var-names] #1447 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1448 proj%vstd!view.View./V #1439 #46
[mk-app] #1449 = #1448 #1416
[mk-app] #1450 => #1438 #1449
[mk-app] #1451 pattern #1448
[mk-quant] #1452 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1451 #1450
[attach-var-names] #1452 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1453 or #1445 #1449
[inst-discovered] theory-solving 0 basic# ; #1450
[mk-app] #1454 = #1450 #1453
[instance] 0 #1454
[attach-enode] #1454 0
[end-of-instance]
[mk-quant] #1454 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1451 #1453
[attach-var-names] #1454 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1455 ARC #125 #1147 #45
[mk-app] #1456 proj%%vstd!view.View./V #1455 #46
[mk-app] #1457 = #1456 #1360
[mk-app] #1458 => #1438 #1457
[mk-app] #1459 pattern #1456
[mk-quant] #1460 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1459 #1458
[attach-var-names] #1460 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1461 or #1445 #1457
[inst-discovered] theory-solving 0 basic# ; #1458
[mk-app] #1462 = #1458 #1461
[instance] 0 #1462
[attach-enode] #1462 0
[end-of-instance]
[mk-quant] #1462 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1459 #1461
[attach-var-names] #1462 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1463 proj%vstd!view.View./V #1455 #46
[mk-app] #1464 = #1463 #1416
[mk-app] #1465 => #1438 #1464
[mk-app] #1466 pattern #1463
[mk-quant] #1467 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1466 #1465
[attach-var-names] #1467 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1468 or #1445 #1464
[inst-discovered] theory-solving 0 basic# ; #1465
[mk-app] #1469 = #1465 #1468
[instance] 0 #1469
[attach-enode] #1469 0
[end-of-instance]
[mk-quant] #1469 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1466 #1468
[attach-var-names] #1469 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1470 and #1372 #1373 #1368
[mk-app] #1471 TYPE%alloc!vec.Vec. #1004 #268 #45 #46
[mk-app] #1472 proj%%vstd!view.View./V #125 #1471
[mk-app] #1473 = #1472 #125
[mk-app] #1474 => #1470 #1473
[mk-app] #1475 pattern #1472
[mk-quant] #1476 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 4 #1475 #1474
[attach-var-names] #1476 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1477 not #1470
[mk-app] #1478 or #1477 #1473
[inst-discovered] theory-solving 0 basic# ; #1474
[mk-app] #1479 = #1474 #1478
[instance] 0 #1479
[attach-enode] #1479 0
[end-of-instance]
[mk-quant] #1479 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 4 #1475 #1478
[attach-var-names] #1479 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1480 proj%vstd!view.View./V #125 #1471
[mk-app] #1481 TYPE%vstd!seq.Seq. #1004 #268
[mk-app] #1482 = #1480 #1481
[mk-app] #1483 => #1470 #1482
[mk-app] #1484 pattern #1480
[mk-quant] #1485 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 4 #1484 #1483
[attach-var-names] #1485 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1486 or #1477 #1482
[inst-discovered] theory-solving 0 basic# ; #1483
[mk-app] #1487 = #1483 #1486
[instance] 0 #1487
[attach-enode] #1487 0
[end-of-instance]
[mk-quant] #1487 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 4 #1484 #1486
[attach-var-names] #1487 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1488 proj%%vstd!view.View./V #125 #1221
[mk-app] #1489 = #1488 #125
[mk-app] #1490 => #1373 #1489
[mk-app] #1491 pattern #1488
[mk-quant] #1492 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1491 #1490
[attach-var-names] #1492 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1493 not #1373
[mk-app] #1494 or #1493 #1489
[inst-discovered] theory-solving 0 basic# ; #1490
[mk-app] #1495 = #1490 #1494
[instance] 0 #1495
[attach-enode] #1495 0
[end-of-instance]
[mk-quant] #1495 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1491 #1494
[attach-var-names] #1495 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1496 proj%vstd!view.View./V #125 #1221
[mk-app] #1497 = #1496 #1221
[mk-app] #1498 => #1373 #1497
[mk-app] #1499 pattern #1496
[mk-quant] #1500 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1499 #1498
[attach-var-names] #1500 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1501 or #1493 #1497
[inst-discovered] theory-solving 0 basic# ; #1498
[mk-app] #1502 = #1498 #1501
[instance] 0 #1502
[attach-enode] #1502 0
[end-of-instance]
[mk-quant] #1502 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1499 #1501
[attach-var-names] #1502 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1503 proj%%vstd!view.View./V #125 #1345
[mk-app] #1504 = #1503 #125
[mk-app] #1505 proj%vstd!view.View./V #125 #1345
[mk-app] #1506 = #1505 #1345
[mk-app] #1507 proj%%vstd!view.View./V #125 #140
[mk-app] #1508 = #1507 #125
[mk-app] #1509 proj%vstd!view.View./V #125 #140
[mk-app] #1510 = #1509 #140
[mk-app] #1511 proj%%vstd!view.View./V #125 #208
[mk-app] #1512 = #1511 #125
[mk-app] #1513 proj%vstd!view.View./V #125 #208
[mk-app] #1514 = #1513 #208
[attach-meaning] #275 arith 32
[mk-app] #1515 proj%%vstd!view.View./V #125 #1167
[mk-app] #1516 = #1515 #125
[attach-meaning] #275 arith 32
[mk-app] #1517 proj%vstd!view.View./V #125 #1167
[attach-meaning] #275 arith 32
[mk-app] #1518 = #1517 #1167
[mk-app] #1519 TYPE%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #45 #46
[mk-app] #1520 proj%%vstd!view.View./V #125 #1519
[mk-app] #1521 = #1520 #125
[mk-app] #1522 => #1438 #1521
[mk-app] #1523 pattern #1520
[mk-quant] #1524 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1523 #1522
[attach-var-names] #1524 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1525 or #1445 #1521
[inst-discovered] theory-solving 0 basic# ; #1522
[mk-app] #1526 = #1522 #1525
[instance] 0 #1526
[attach-enode] #1526 0
[end-of-instance]
[mk-quant] #1526 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1523 #1525
[attach-var-names] #1526 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1527 proj%vstd!view.View./V #125 #1519
[mk-app] #1528 TYPE%vstd!seq.Seq. #1360 #1416
[mk-app] #1529 = #1527 #1528
[mk-app] #1530 => #1438 #1529
[mk-app] #1531 pattern #1527
[mk-quant] #1532 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1531 #1530
[attach-var-names] #1532 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1533 or #1445 #1529
[inst-discovered] theory-solving 0 basic# ; #1530
[mk-app] #1534 = #1530 #1533
[instance] 0 #1534
[attach-enode] #1534 0
[end-of-instance]
[mk-quant] #1534 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1531 #1533
[attach-var-names] #1534 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1535 TYPE%vstd!seq.Seq. #65 #66
[mk-app] #1536 has_type #34 #1535
[mk-app] #1537 vstd!seq.Seq.len.? #65 #66 #34
[mk-app] #1538 <= #341 #1537
[mk-app] #1539 => #1536 #1538
[mk-app] #1540 pattern #1537
[mk-quant] #1541 internal_vstd!seq.Seq.len.?_pre_post_definition 3 #1540 #1539
[attach-var-names] #1541 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[attach-meaning] #370 arith (- 1)
[mk-app] #1542 * #370 #1537
[mk-app] #1543 >= #1537 #341
[inst-discovered] theory-solving 0 arith# ; #1538
[mk-app] #1542 = #1538 #1543
[instance] 0 #1542
[attach-enode] #1542 0
[end-of-instance]
[mk-app] #1542 not #1536
[mk-app] #1544 or #1542 #1543
[mk-app] #1545 => #1536 #1543
[inst-discovered] theory-solving 0 basic# ; #1545
[mk-app] #1546 = #1545 #1544
[instance] 0 #1546
[attach-enode] #1546 0
[end-of-instance]
[mk-quant] #1545 internal_vstd!seq.Seq.len.?_pre_post_definition 3 #1540 #1544
[attach-var-names] #1545 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1546 req%vstd!seq.Seq.index. #1004 #268 #33 #34
[mk-app] #1547 %%global_location_label%%0
[mk-app] #1548 vstd!seq.Seq.len.? #1004 #268 #33
[mk-app] #1549 < #191 #1548
[mk-app] #1550 and #517 #1549
[mk-app] #1551 => #1547 #1550
[mk-app] #1552 = #1546 #1551
[mk-app] #1553 pattern #1546
[mk-quant] #1554 internal_req__vstd!seq.Seq.index._definition 4 #1553 #1552
[attach-var-names] #1554 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[attach-meaning] #370 arith (- 1)
[mk-app] #1555 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #1555 = #517 #521
[instance] 0 #1555
[attach-enode] #1555 0
[end-of-instance]
[mk-app] #1555 <= #1548 #191
[mk-app] #1556 not #1555
[inst-discovered] theory-solving 0 arith# ; #1549
[mk-app] #1557 = #1549 #1556
[instance] 0 #1557
[attach-enode] #1557 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #1557 * #370 #191
[mk-app] #1558 + #1557 #1548
[attach-meaning] #370 arith (- 1)
[mk-app] #1559 * #370 #1548
[mk-app] #1560 + #191 #1559
[mk-app] #1557 >= #1560 #341
[inst-discovered] theory-solving 0 arith# ; #1555
[mk-app] #1558 = #1555 #1557
[instance] 0 #1558
[attach-enode] #1558 0
[end-of-instance]
[mk-app] #1558 not #1557
[mk-app] #1561 and #521 #1558
[mk-app] #1562 not #1547
[mk-app] #1563 or #1562 #1561
[mk-app] #1564 => #1547 #1561
[inst-discovered] theory-solving 0 basic# ; #1564
[mk-app] #1565 = #1564 #1563
[instance] 0 #1565
[attach-enode] #1565 0
[end-of-instance]
[mk-app] #1564 = #1546 #1563
[mk-quant] #1565 internal_req__vstd!seq.Seq.index._definition 4 #1553 #1564
[attach-var-names] #1565 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1555 has_type #33 #1481
[mk-app] #1556 and #1555 #190
[mk-app] #1566 vstd!seq.Seq.index.? #1004 #268 #33 #34
[mk-app] #1567 has_type #1566 #268
[mk-app] #1568 => #1556 #1567
[mk-app] #1569 pattern #1566
[mk-quant] #1570 internal_vstd!seq.Seq.index.?_pre_post_definition 4 #1569 #1568
[attach-var-names] #1570 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1571 not #1556
[mk-app] #1572 or #1571 #1567
[inst-discovered] theory-solving 0 basic# ; #1568
[mk-app] #1573 = #1568 #1572
[instance] 0 #1573
[attach-enode] #1573 0
[end-of-instance]
[mk-quant] #1573 internal_vstd!seq.Seq.index.?_pre_post_definition 4 #1569 #1572
[attach-var-names] #1573 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1574 req%vstd!seq.impl&%0.spec_index. #1004 #268 #33 #34
[mk-app] #1575 %%global_location_label%%1
[mk-app] #1576 => #1575 #1550
[mk-app] #1577 = #1574 #1576
[mk-app] #1578 pattern #1574
[mk-quant] #1579 internal_req__vstd!seq.impl&__0.spec_index._definition 4 #1578 #1577
[attach-var-names] #1579 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[attach-meaning] #370 arith (- 1)
[mk-app] #1580 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #1580 = #517 #521
[instance] 0 #1580
[attach-enode] #1580 0
[end-of-instance]
[mk-app] #1580 <= #1548 #191
[mk-app] #1581 not #1580
[inst-discovered] theory-solving 0 arith# ; #1549
[mk-app] #1582 = #1549 #1581
[instance] 0 #1582
[attach-enode] #1582 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #1582 * #370 #191
[mk-app] #1583 + #1582 #1548
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #1580
[mk-app] #1582 = #1580 #1557
[instance] 0 #1582
[attach-enode] #1582 0
[end-of-instance]
[mk-app] #1582 not #1575
[mk-app] #1583 or #1582 #1561
[mk-app] #1584 => #1575 #1561
[inst-discovered] theory-solving 0 basic# ; #1584
[mk-app] #1585 = #1584 #1583
[instance] 0 #1585
[attach-enode] #1585 0
[end-of-instance]
[mk-app] #1584 = #1574 #1583
[mk-quant] #1585 internal_req__vstd!seq.impl&__0.spec_index._definition 4 #1578 #1584
[attach-var-names] #1585 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1580 fuel_bool_default #810
[mk-app] #1581 fuel_bool #810
[mk-app] #1586 vstd!seq.impl&%0.spec_index.? #1004 #268 #33 #34
[mk-app] #1587 = #1586 #1566
[mk-app] #1588 pattern #1586
[mk-quant] #1589 internal_vstd!seq.impl&__0.spec_index.?_definition 4 #1588 #1587
[attach-var-names] #1589 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1590 => #1581 #1589
[mk-app] #1591 not #1581
[mk-app] #1592 or #1591 #1589
[inst-discovered] theory-solving 0 basic# ; #1590
[mk-app] #1593 = #1590 #1592
[instance] 0 #1593
[attach-enode] #1593 0
[end-of-instance]
[mk-app] #1593 has_type #1586 #268
[mk-app] #1594 => #1556 #1593
[mk-quant] #1595 internal_vstd!seq.impl&__0.spec_index.?_pre_post_definition 4 #1588 #1594
[attach-var-names] #1595 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1596 or #1571 #1593
[inst-discovered] theory-solving 0 basic# ; #1594
[mk-app] #1597 = #1594 #1596
[instance] 0 #1597
[attach-enode] #1597 0
[end-of-instance]
[mk-quant] #1597 internal_vstd!seq.impl&__0.spec_index.?_pre_post_definition 4 #1588 #1596
[attach-var-names] #1597 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1598 fuel_bool #811
[mk-app] #1599 and #1372 #1550
[mk-app] #1600 height #1566
[mk-app] #1601 height_lt #1600 #783
[mk-app] #1602 => #1599 #1601
[mk-app] #1603 => #1556 #1602
[mk-app] #1604 pattern #1600
[mk-quant] #1605 user_vstd__seq__axiom_seq_index_decreases_0 4 #1604 #1603
[attach-var-names] #1605 (|i!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1606 => #1598 #1605
[attach-meaning] #370 arith (- 1)
[mk-app] #1607 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #1607 = #517 #521
[instance] 0 #1607
[attach-enode] #1607 0
[end-of-instance]
[mk-app] #1607 <= #1548 #191
[mk-app] #1608 not #1607
[inst-discovered] theory-solving 0 arith# ; #1549
[mk-app] #1609 = #1549 #1608
[instance] 0 #1609
[attach-enode] #1609 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #1609 * #370 #191
[mk-app] #1610 + #1609 #1548
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #1607
[mk-app] #1609 = #1607 #1557
[instance] 0 #1609
[attach-enode] #1609 0
[end-of-instance]
[mk-app] #1609 and #1372 #521 #1558
[mk-app] #1610 and #1372 #1561
[inst-discovered] theory-solving 0 basic# ; #1610
[mk-app] #1611 = #1610 #1609
[instance] 0 #1611
[attach-enode] #1611 0
[end-of-instance]
[mk-app] #1610 not #1609
[mk-app] #1611 or #1610 #1601
[mk-app] #1612 => #1609 #1601
[inst-discovered] theory-solving 0 basic# ; #1612
[mk-app] #1613 = #1612 #1611
[instance] 0 #1613
[attach-enode] #1613 0
[end-of-instance]
[mk-app] #1612 or #1571 #1610 #1601
[mk-app] #1613 => #1556 #1611
[inst-discovered] theory-solving 0 basic# ; #1613
[mk-app] #1614 = #1613 #1612
[instance] 0 #1614
[attach-enode] #1614 0
[end-of-instance]
[mk-quant] #1611 user_vstd__seq__axiom_seq_index_decreases_0 4 #1604 #1612
[attach-var-names] #1611 (|i!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1607 not #1598
[mk-app] #1608 or #1607 #1611
[mk-app] #1613 => #1598 #1611
[inst-discovered] theory-solving 0 basic# ; #1613
[mk-app] #1614 = #1613 #1608
[instance] 0 #1614
[attach-enode] #1614 0
[end-of-instance]
[mk-app] #1613 vstd!seq.Seq.empty.? #45 #46
[mk-app] #1614 has_type #1613 #1385
[mk-app] #1615 pattern #1613
[mk-quant] #1616 internal_vstd!seq.Seq.empty.?_pre_post_definition 2 #1615 #1614
[attach-var-names] #1616 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1617 fuel_bool #812
[mk-app] #1618 vstd!seq.Seq.len.? #45 #46 #1613
[mk-app] #1619 = #1618 #341
[mk-app] #1620 => #1373 #1619
[mk-app] #1621 pattern #1618
[mk-quant] #1622 user_vstd__seq__axiom_seq_empty_1 2 #1621 #1620
[attach-var-names] #1622 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1623 => #1617 #1622
[mk-app] #1624 or #1493 #1619
[inst-discovered] theory-solving 0 basic# ; #1620
[mk-app] #1625 = #1620 #1624
[instance] 0 #1625
[attach-enode] #1625 0
[end-of-instance]
[mk-quant] #1625 user_vstd__seq__axiom_seq_empty_1 2 #1621 #1624
[attach-var-names] #1625 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1626 not #1617
[mk-app] #1627 or #1626 #1625
[mk-app] #1628 => #1617 #1625
[inst-discovered] theory-solving 0 basic# ; #1628
[mk-app] #1629 = #1628 #1627
[instance] 0 #1629
[attach-enode] #1629 0
[end-of-instance]
[mk-app] #1628 has_type #33 #200
[mk-app] #1629 has_type #34 #268
[mk-app] #1630 and #1628 #1629
[mk-app] #1631 vstd!seq.Seq.new.? #1003 #984 #1004 #268 #33 #34
[mk-app] #1632 TYPE%vstd!seq.Seq. #1003 #984
[mk-app] #1633 has_type #1631 #1632
[mk-app] #1634 => #1630 #1633
[mk-app] #1635 pattern #1631
[mk-quant] #1636 internal_vstd!seq.Seq.new.?_pre_post_definition 6 #1635 #1634
[attach-var-names] #1636 (|f!| ; |Poly|) (|len!| ; |Poly|) (|impl%1&| ; |Type|) (|impl%1&.| ; |Dcr|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1637 not #1630
[mk-app] #1638 or #1637 #1633
[inst-discovered] theory-solving 0 basic# ; #1634
[mk-app] #1639 = #1634 #1638
[instance] 0 #1639
[attach-enode] #1639 0
[end-of-instance]
[mk-quant] #1639 internal_vstd!seq.Seq.new.?_pre_post_definition 6 #1635 #1638
[attach-var-names] #1639 (|f!| ; |Poly|) (|len!| ; |Poly|) (|impl%1&| ; |Type|) (|impl%1&.| ; |Dcr|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1640 fuel_bool #813
[mk-app] #1641 TYPE%fun%1. #125 #189 #1004 #268
[mk-app] #1642 has_type #34 #1641
[mk-app] #1643 and #1628 #1642
[mk-app] #1644 vstd!seq.Seq.new.? #1004 #268 #125 #1641 #33 #34
[mk-app] #1645 vstd!seq.Seq.len.? #1004 #268 #1644
[mk-app] #1646 = #1645 #676
[mk-app] #1647 => #1372 #1646
[mk-app] #1648 => #1643 #1647
[mk-app] #1649 pattern #1645
[mk-quant] #1650 user_vstd__seq__axiom_seq_new_len_2 4 #1649 #1648
[attach-var-names] #1650 (|f!| ; |Poly|) (|len!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1651 => #1640 #1650
[mk-app] #1652 not #1372
[mk-app] #1653 or #1652 #1646
[inst-discovered] theory-solving 0 basic# ; #1647
[mk-app] #1654 = #1647 #1653
[instance] 0 #1654
[attach-enode] #1654 0
[end-of-instance]
[mk-app] #1654 not #1643
[mk-app] #1655 or #1654 #1652 #1646
[mk-app] #1656 => #1643 #1653
[inst-discovered] theory-solving 0 basic# ; #1656
[mk-app] #1657 = #1656 #1655
[instance] 0 #1657
[attach-enode] #1657 0
[end-of-instance]
[mk-quant] #1653 user_vstd__seq__axiom_seq_new_len_2 4 #1649 #1655
[attach-var-names] #1653 (|f!| ; |Poly|) (|len!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1656 not #1640
[mk-app] #1657 or #1656 #1653
[mk-app] #1658 => #1640 #1653
[inst-discovered] theory-solving 0 basic# ; #1658
[mk-app] #1659 = #1658 #1657
[instance] 0 #1659
[attach-enode] #1659 0
[end-of-instance]
[mk-app] #1658 fuel_bool #814
[mk-app] #1659 has_type #44 #200
[mk-app] #1660 TYPE%fun%1. #125 #189 #971 #972
[mk-app] #1661 has_type #33 #1660
[mk-app] #1662 and #1659 #1661 #190
[mk-app] #1663 sized #971
[mk-app] #1664 < #191 #666
[mk-app] #1665 and #517 #1664
[mk-app] #1666 and #1663 #1665
[mk-app] #1667 vstd!seq.Seq.new.? #971 #972 #125 #1660 #44 #33
[mk-app] #1668 vstd!seq.Seq.index.? #971 #972 #1667 #34
[mk-app] #1669 = #1668 #1037
[mk-app] #1670 => #1666 #1669
[mk-app] #1671 => #1662 #1670
[mk-app] #1672 pattern #1668
[mk-quant] #1673 user_vstd__seq__axiom_seq_new_index_3 5 #1672 #1671
[attach-var-names] #1673 (|i!| ; |Poly|) (|f!| ; |Poly|) (|len!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1674 => #1658 #1673
[attach-meaning] #370 arith (- 1)
[mk-app] #1675 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #1675 = #517 #521
[instance] 0 #1675
[attach-enode] #1675 0
[end-of-instance]
[mk-app] #1675 <= #666 #191
[mk-app] #1676 not #1675
[inst-discovered] theory-solving 0 arith# ; #1664
[mk-app] #1677 = #1664 #1676
[instance] 0 #1677
[attach-enode] #1677 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #1677 * #370 #191
[mk-app] #1678 + #1677 #666
[attach-meaning] #370 arith (- 1)
[mk-app] #1679 * #370 #666
[mk-app] #1680 + #191 #1679
[mk-app] #1677 >= #1680 #341
[inst-discovered] theory-solving 0 arith# ; #1675
[mk-app] #1678 = #1675 #1677
[instance] 0 #1678
[attach-enode] #1678 0
[end-of-instance]
[mk-app] #1678 not #1677
[mk-app] #1681 and #1663 #521 #1678
[mk-app] #1682 not #1681
[mk-app] #1683 or #1682 #1669
[mk-app] #1684 => #1681 #1669
[inst-discovered] theory-solving 0 basic# ; #1684
[mk-app] #1685 = #1684 #1683
[instance] 0 #1685
[attach-enode] #1685 0
[end-of-instance]
[mk-app] #1684 not #1662
[mk-app] #1685 or #1684 #1682 #1669
[mk-app] #1686 => #1662 #1683
[inst-discovered] theory-solving 0 basic# ; #1686
[mk-app] #1687 = #1686 #1685
[instance] 0 #1687
[attach-enode] #1687 0
[end-of-instance]
[mk-quant] #1683 user_vstd__seq__axiom_seq_new_index_3 5 #1672 #1685
[attach-var-names] #1683 (|i!| ; |Poly|) (|f!| ; |Poly|) (|len!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1675 not #1658
[mk-app] #1676 or #1675 #1683
[mk-app] #1686 => #1658 #1683
[inst-discovered] theory-solving 0 basic# ; #1686
[mk-app] #1687 = #1686 #1676
[instance] 0 #1687
[attach-enode] #1687 0
[end-of-instance]
[mk-app] #1686 and #1555 #1629
[mk-app] #1687 vstd!seq.Seq.push.? #1004 #268 #33 #34
[mk-app] #1688 has_type #1687 #1481
[mk-app] #1689 => #1686 #1688
[mk-app] #1690 pattern #1687
[mk-quant] #1691 internal_vstd!seq.Seq.push.?_pre_post_definition 4 #1690 #1689
[attach-var-names] #1691 (|a!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1692 not #1686
[mk-app] #1693 or #1692 #1688
[inst-discovered] theory-solving 0 basic# ; #1689
[mk-app] #1694 = #1689 #1693
[instance] 0 #1694
[attach-enode] #1694 0
[end-of-instance]
[mk-quant] #1694 internal_vstd!seq.Seq.push.?_pre_post_definition 4 #1690 #1693
[attach-var-names] #1694 (|a!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1695 fuel_bool #815
[mk-app] #1696 vstd!seq.Seq.len.? #1004 #268 #1687
[mk-app] #1697 Add #1548 #296
[mk-app] #1698 nClip #1697
[mk-app] #1699 = #1696 #1698
[mk-app] #1700 => #1372 #1699
[mk-app] #1701 => #1686 #1700
[mk-app] #1702 pattern #1696
[mk-quant] #1703 user_vstd__seq__axiom_seq_push_len_4 4 #1702 #1701
[attach-var-names] #1703 (|a!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1704 => #1695 #1703
[mk-app] #1705 or #1652 #1699
[inst-discovered] theory-solving 0 basic# ; #1700
[mk-app] #1706 = #1700 #1705
[instance] 0 #1706
[attach-enode] #1706 0
[end-of-instance]
[mk-app] #1706 or #1692 #1652 #1699
[mk-app] #1707 => #1686 #1705
[inst-discovered] theory-solving 0 basic# ; #1707
[mk-app] #1708 = #1707 #1706
[instance] 0 #1708
[attach-enode] #1708 0
[end-of-instance]
[mk-quant] #1705 user_vstd__seq__axiom_seq_push_len_4 4 #1702 #1706
[attach-var-names] #1705 (|a!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1707 not #1695
[mk-app] #1708 or #1707 #1705
[mk-app] #1709 => #1695 #1705
[inst-discovered] theory-solving 0 basic# ; #1709
[mk-app] #1710 = #1709 #1708
[instance] 0 #1710
[attach-enode] #1710 0
[end-of-instance]
[mk-app] #1709 fuel_bool #816
[mk-app] #1710 TYPE%vstd!seq.Seq. #971 #972
[mk-app] #1711 has_type #44 #1710
[mk-app] #1712 has_type #33 #972
[mk-app] #1713 and #1711 #1712 #190
[mk-app] #1714 vstd!seq.Seq.len.? #971 #972 #44
[mk-app] #1715 = #191 #1714
[mk-app] #1716 and #1663 #1715
[mk-app] #1717 vstd!seq.Seq.push.? #971 #972 #44 #33
[mk-app] #1718 vstd!seq.Seq.index.? #971 #972 #1717 #34
[mk-app] #1719 = #1718 #33
[mk-app] #1720 => #1716 #1719
[mk-app] #1721 => #1713 #1720
[mk-app] #1722 pattern #1718
[mk-quant] #1723 user_vstd__seq__axiom_seq_push_index_same_5 5 #1722 #1721
[attach-var-names] #1723 (|i!| ; |Poly|) (|a!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1724 => #1709 #1723
[mk-app] #1725 not #1716
[mk-app] #1726 or #1725 #1719
[inst-discovered] theory-solving 0 basic# ; #1720
[mk-app] #1727 = #1720 #1726
[instance] 0 #1727
[attach-enode] #1727 0
[end-of-instance]
[mk-app] #1727 not #1713
[mk-app] #1728 or #1727 #1725 #1719
[mk-app] #1729 => #1713 #1726
[inst-discovered] theory-solving 0 basic# ; #1729
[mk-app] #1730 = #1729 #1728
[instance] 0 #1730
[attach-enode] #1730 0
[end-of-instance]
[mk-quant] #1726 user_vstd__seq__axiom_seq_push_index_same_5 5 #1722 #1728
[attach-var-names] #1726 (|i!| ; |Poly|) (|a!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1729 not #1709
[mk-app] #1730 or #1729 #1726
[mk-app] #1731 => #1709 #1726
[inst-discovered] theory-solving 0 basic# ; #1731
[mk-app] #1732 = #1731 #1730
[instance] 0 #1732
[attach-enode] #1732 0
[end-of-instance]
[mk-app] #1731 fuel_bool #817
[mk-app] #1732 < #191 #1714
[mk-app] #1733 and #517 #1732
[mk-app] #1734 and #1663 #1733
[mk-app] #1735 vstd!seq.Seq.index.? #971 #972 #44 #34
[mk-app] #1736 = #1718 #1735
[mk-app] #1737 => #1734 #1736
[mk-app] #1738 => #1713 #1737
[mk-quant] #1739 user_vstd__seq__axiom_seq_push_index_different_6 5 #1722 #1738
[attach-var-names] #1739 (|i!| ; |Poly|) (|a!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1740 => #1731 #1739
[attach-meaning] #370 arith (- 1)
[mk-app] #1741 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #1741 = #517 #521
[instance] 0 #1741
[attach-enode] #1741 0
[end-of-instance]
[mk-app] #1741 <= #1714 #191
[mk-app] #1742 not #1741
[inst-discovered] theory-solving 0 arith# ; #1732
[mk-app] #1743 = #1732 #1742
[instance] 0 #1743
[attach-enode] #1743 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #1743 * #370 #191
[mk-app] #1744 + #1743 #1714
[attach-meaning] #370 arith (- 1)
[mk-app] #1745 * #370 #1714
[mk-app] #1746 + #191 #1745
[mk-app] #1743 >= #1746 #341
[inst-discovered] theory-solving 0 arith# ; #1741
[mk-app] #1744 = #1741 #1743
[instance] 0 #1744
[attach-enode] #1744 0
[end-of-instance]
[mk-app] #1744 not #1743
[mk-app] #1747 and #1663 #521 #1744
[mk-app] #1748 not #1747
[mk-app] #1749 or #1748 #1736
[mk-app] #1750 => #1747 #1736
[inst-discovered] theory-solving 0 basic# ; #1750
[mk-app] #1751 = #1750 #1749
[instance] 0 #1751
[attach-enode] #1751 0
[end-of-instance]
[mk-app] #1750 or #1727 #1748 #1736
[mk-app] #1751 => #1713 #1749
[inst-discovered] theory-solving 0 basic# ; #1751
[mk-app] #1752 = #1751 #1750
[instance] 0 #1752
[attach-enode] #1752 0
[end-of-instance]
[mk-quant] #1749 user_vstd__seq__axiom_seq_push_index_different_6 5 #1722 #1750
[attach-var-names] #1749 (|i!| ; |Poly|) (|a!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1741 not #1731
[mk-app] #1742 or #1741 #1749
[mk-app] #1751 => #1731 #1749
[inst-discovered] theory-solving 0 basic# ; #1751
[mk-app] #1752 = #1751 #1742
[instance] 0 #1752
[attach-enode] #1752 0
[end-of-instance]
[mk-app] #1751 fuel_bool #818
[mk-app] #1752 has_type #34 #1481
[mk-app] #1753 and #1555 #1752
[mk-app] #1754 ext_eq #2 #1481 #33 #34
[mk-app] #1755 vstd!seq.Seq.len.? #1004 #268 #34
[mk-app] #1756 = #1548 #1755
[mk-app] #1757 vstd!seq.Seq.index.? #971 #972 #33 #34
[mk-app] #1758 = #1735 #1757
[mk-app] #1759 => #1733 #1758
[mk-app] #1760 => #190 #1759
[mk-app] #1761 pattern #1735
[mk-app] #1762 pattern #1757
[mk-quant] #1763 user_vstd__seq__axiom_seq_ext_equal_7 1 #1761 #1762 #1760
[attach-var-names] #1763 (|i$| ; |Poly|)
[mk-app] #1764 and #1756 #1763
[mk-app] #1765 = #1754 #1764
[mk-app] #1766 => #1372 #1765
[mk-app] #1767 => #1753 #1766
[mk-app] #1768 pattern #1754
[mk-quant] #1769 user_vstd__seq__axiom_seq_ext_equal_8 4 #1768 #1767
[attach-var-names] #1769 (|s2!| ; |Poly|) (|s1!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1770 => #1751 #1769
[attach-meaning] #370 arith (- 1)
[mk-app] #1771 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #1771 = #517 #521
[instance] 0 #1771
[attach-enode] #1771 0
[end-of-instance]
[mk-app] #1771 <= #1714 #191
[mk-app] #1772 not #1771
[inst-discovered] theory-solving 0 arith# ; #1732
[mk-app] #1773 = #1732 #1772
[instance] 0 #1773
[attach-enode] #1773 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #1773 * #370 #191
[mk-app] #1774 + #1773 #1714
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #1771
[mk-app] #1773 = #1771 #1743
[instance] 0 #1773
[attach-enode] #1773 0
[end-of-instance]
[mk-app] #1773 and #521 #1744
[mk-app] #1774 not #1773
[mk-app] #1775 or #1774 #1758
[mk-app] #1776 => #1773 #1758
[inst-discovered] theory-solving 0 basic# ; #1776
[mk-app] #1777 = #1776 #1775
[instance] 0 #1777
[attach-enode] #1777 0
[end-of-instance]
[mk-app] #1776 or #197 #1774 #1758
[mk-app] #1777 => #190 #1775
[inst-discovered] theory-solving 0 basic# ; #1777
[mk-app] #1778 = #1777 #1776
[instance] 0 #1778
[attach-enode] #1778 0
[end-of-instance]
[mk-quant] #1775 user_vstd__seq__axiom_seq_ext_equal_7 1 #1761 #1762 #1776
[attach-var-names] #1775 (|i$| ; |Poly|)
[mk-app] #1771 and #1756 #1775
[mk-app] #1772 = #1754 #1771
[mk-app] #1777 or #1652 #1772
[mk-app] #1778 => #1372 #1772
[inst-discovered] theory-solving 0 basic# ; #1778
[mk-app] #1779 = #1778 #1777
[instance] 0 #1779
[attach-enode] #1779 0
[end-of-instance]
[mk-app] #1778 not #1753
[mk-app] #1779 or #1778 #1652 #1772
[mk-app] #1780 => #1753 #1777
[inst-discovered] theory-solving 0 basic# ; #1780
[mk-app] #1781 = #1780 #1779
[instance] 0 #1781
[attach-enode] #1781 0
[end-of-instance]
[mk-quant] #1777 user_vstd__seq__axiom_seq_ext_equal_8 4 #1768 #1779
[attach-var-names] #1777 (|s2!| ; |Poly|) (|s1!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1780 not #1751
[mk-app] #1781 or #1780 #1777
[mk-app] #1782 => #1751 #1777
[inst-discovered] theory-solving 0 basic# ; #1782
[mk-app] #1783 = #1782 #1781
[instance] 0 #1783
[attach-enode] #1783 0
[end-of-instance]
[mk-app] #1782 fuel_bool #819
[mk-app] #1783 ext_eq #1 #1481 #33 #34
[mk-app] #1784 ext_eq #1 #972 #1735 #1757
[mk-app] #1785 => #1733 #1784
[mk-app] #1786 => #190 #1785
[mk-quant] #1787 user_vstd__seq__axiom_seq_ext_equal_deep_9 1 #1761 #1762 #1786
[attach-var-names] #1787 (|i$| ; |Poly|)
[mk-app] #1788 and #1756 #1787
[mk-app] #1789 = #1783 #1788
[mk-app] #1790 => #1372 #1789
[mk-app] #1791 => #1753 #1790
[mk-app] #1792 pattern #1783
[mk-quant] #1793 user_vstd__seq__axiom_seq_ext_equal_deep_10 4 #1792 #1791
[attach-var-names] #1793 (|s2!| ; |Poly|) (|s1!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1794 => #1782 #1793
[attach-meaning] #370 arith (- 1)
[mk-app] #1795 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #1795 = #517 #521
[instance] 0 #1795
[attach-enode] #1795 0
[end-of-instance]
[mk-app] #1795 <= #1714 #191
[mk-app] #1796 not #1795
[inst-discovered] theory-solving 0 arith# ; #1732
[mk-app] #1797 = #1732 #1796
[instance] 0 #1797
[attach-enode] #1797 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #1797 * #370 #191
[mk-app] #1798 + #1797 #1714
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #1795
[mk-app] #1797 = #1795 #1743
[instance] 0 #1797
[attach-enode] #1797 0
[end-of-instance]
[mk-app] #1797 or #1774 #1784
[mk-app] #1798 => #1773 #1784
[inst-discovered] theory-solving 0 basic# ; #1798
[mk-app] #1799 = #1798 #1797
[instance] 0 #1799
[attach-enode] #1799 0
[end-of-instance]
[mk-app] #1798 or #197 #1774 #1784
[mk-app] #1799 => #190 #1797
[inst-discovered] theory-solving 0 basic# ; #1799
[mk-app] #1800 = #1799 #1798
[instance] 0 #1800
[attach-enode] #1800 0
[end-of-instance]
[mk-quant] #1797 user_vstd__seq__axiom_seq_ext_equal_deep_9 1 #1761 #1762 #1798
[attach-var-names] #1797 (|i$| ; |Poly|)
[mk-app] #1795 and #1756 #1797
[mk-app] #1796 = #1783 #1795
[mk-app] #1799 or #1652 #1796
[mk-app] #1800 => #1372 #1796
[inst-discovered] theory-solving 0 basic# ; #1800
[mk-app] #1801 = #1800 #1799
[instance] 0 #1801
[attach-enode] #1801 0
[end-of-instance]
[mk-app] #1800 or #1778 #1652 #1796
[mk-app] #1801 => #1753 #1799
[inst-discovered] theory-solving 0 basic# ; #1801
[mk-app] #1802 = #1801 #1800
[instance] 0 #1802
[attach-enode] #1802 0
[end-of-instance]
[mk-quant] #1799 user_vstd__seq__axiom_seq_ext_equal_deep_10 4 #1792 #1800
[attach-var-names] #1799 (|s2!| ; |Poly|) (|s1!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1801 not #1782
[mk-app] #1802 or #1801 #1799
[mk-app] #1803 => #1782 #1799
[inst-discovered] theory-solving 0 basic# ; #1803
[mk-app] #1804 = #1803 #1802
[instance] 0 #1804
[attach-enode] #1804 0
[end-of-instance]
[mk-app] #1803 vstd!view.View.view.? #65 #66 #34
[mk-app] #1804 proj%vstd!view.View./V #65 #66
[mk-app] #1805 has_type #1803 #1804
[mk-app] #1806 => #69 #1805
[mk-app] #1807 pattern #1803
[mk-quant] #1808 internal_vstd!view.View.view.?_pre_post_definition 3 #1807 #1806
[attach-var-names] #1808 (|self!| ; |Poly|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1809 or #1231 #1805
[inst-discovered] theory-solving 0 basic# ; #1806
[mk-app] #1810 = #1806 #1809
[instance] 0 #1810
[attach-enode] #1810 0
[end-of-instance]
[mk-quant] #1810 internal_vstd!view.View.view.?_pre_post_definition 3 #1807 #1809
[attach-var-names] #1810 (|self!| ; |Poly|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1811 fuel_bool_default #828
[mk-app] #1812 fuel_bool #828
[mk-app] #1813 vstd!view.View.view.? #125 #140 #34
[mk-app] #1814 = #1813 #34
[mk-app] #1815 pattern #1813
[mk-quant] #1816 internal_vstd!view.View.view.?_definition 1 #1815 #1814
[attach-var-names] #1816 (|self!| ; |Poly|)
[mk-app] #1817 => #1812 #1816
[mk-app] #1818 not #1812
[mk-app] #1819 or #1818 #1816
[inst-discovered] theory-solving 0 basic# ; #1817
[mk-app] #1820 = #1817 #1819
[instance] 0 #1820
[attach-enode] #1820 0
[end-of-instance]
[mk-app] #1820 tr_bound%vstd!view.View. #125 #140
[mk-app] #1821 fuel_bool_default #830
[mk-app] #1822 fuel_bool #830
[attach-meaning] #275 arith 32
[mk-app] #1823 vstd!view.View.view.? #125 #1167 #34
[mk-app] #1824 = #1823 #34
[attach-meaning] #275 arith 32
[mk-app] #1825 pattern #1823
[mk-quant] #1826 internal_vstd!view.View.view.?_definition 1 #1825 #1824
[attach-var-names] #1826 (|self!| ; |Poly|)
[mk-app] #1827 => #1822 #1826
[mk-app] #1828 not #1822
[mk-app] #1829 or #1828 #1826
[inst-discovered] theory-solving 0 basic# ; #1827
[mk-app] #1830 = #1827 #1829
[instance] 0 #1830
[attach-enode] #1830 0
[end-of-instance]
[attach-meaning] #275 arith 32
[mk-app] #1830 tr_bound%vstd!view.View. #125 #1167
[mk-app] #1831 fuel_bool_default #829
[mk-app] #1832 fuel_bool #829
[mk-app] #1833 vstd!view.View.view.? #125 #208 #34
[mk-app] #1834 = #1833 #34
[mk-app] #1835 pattern #1833
[mk-quant] #1836 internal_vstd!view.View.view.?_definition 1 #1835 #1834
[attach-var-names] #1836 (|self!| ; |Poly|)
[mk-app] #1837 => #1832 #1836
[mk-app] #1838 not #1832
[mk-app] #1839 or #1838 #1836
[inst-discovered] theory-solving 0 basic# ; #1837
[mk-app] #1840 = #1837 #1839
[instance] 0 #1840
[attach-enode] #1840 0
[end-of-instance]
[mk-app] #1840 tr_bound%vstd!view.View. #125 #208
[mk-app] #1841 fuel_bool_default #826
[mk-app] #1842 fuel_bool #826
[mk-app] #1843 sized #65
[mk-app] #1844 vstd!view.View.view.? #125 #1208 #34
[mk-app] #1845 = #1844 #34
[mk-app] #1846 => #1843 #1845
[mk-app] #1847 pattern #1844
[mk-quant] #1848 internal_vstd!view.View.view.?_definition 3 #1847 #1846
[attach-var-names] #1848 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1849 => #1842 #1848
[mk-app] #1850 not #1843
[mk-app] #1851 or #1850 #1845
[inst-discovered] theory-solving 0 basic# ; #1846
[mk-app] #1852 = #1846 #1851
[instance] 0 #1852
[attach-enode] #1852 0
[end-of-instance]
[mk-quant] #1852 internal_vstd!view.View.view.?_definition 3 #1847 #1851
[attach-var-names] #1852 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1853 not #1842
[mk-app] #1854 or #1853 #1852
[mk-app] #1855 => #1842 #1852
[inst-discovered] theory-solving 0 basic# ; #1855
[mk-app] #1856 = #1855 #1854
[instance] 0 #1856
[attach-enode] #1856 0
[end-of-instance]
[mk-app] #1855 tr_bound%vstd!view.View. #125 #1221
[mk-app] #1856 => #1373 #1855
[mk-app] #1857 pattern #1855
[mk-quant] #1858 internal_vstd__view__impl&__14_trait_impl_definition 2 #1857 #1856
[attach-var-names] #1858 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1859 or #1493 #1855
[inst-discovered] theory-solving 0 basic# ; #1856
[mk-app] #1860 = #1856 #1859
[instance] 0 #1860
[attach-enode] #1860 0
[end-of-instance]
[mk-quant] #1860 internal_vstd__view__impl&__14_trait_impl_definition 2 #1857 #1859
[attach-var-names] #1860 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1861 TYPE%alloc!vec.Vec. #971 #972 #65 #66
[mk-app] #1862 has_type #34 #1861
[mk-app] #1863 vstd!std_specs.vec.spec_vec_len.? #971 #972 #65 #66 #34
[mk-app] #1864 uInv #274 #1863
[mk-app] #1865 => #1862 #1864
[mk-app] #1866 pattern #1863
[mk-quant] #1867 internal_vstd!std_specs.vec.spec_vec_len.?_pre_post_definition 5 #1866 #1865
[attach-var-names] #1867 (|v!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1868 not #1862
[mk-app] #1869 or #1868 #1864
[inst-discovered] theory-solving 0 basic# ; #1865
[mk-app] #1870 = #1865 #1869
[instance] 0 #1870
[attach-enode] #1870 0
[end-of-instance]
[mk-quant] #1870 internal_vstd!std_specs.vec.spec_vec_len.?_pre_post_definition 5 #1866 #1869
[attach-var-names] #1870 (|v!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1871 tr_bound%vstd!view.View. #125 #1471
[mk-app] #1872 => #1470 #1871
[mk-app] #1873 pattern #1871
[mk-quant] #1874 internal_vstd__view__impl&__8_trait_impl_definition 4 #1873 #1872
[attach-var-names] #1874 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1875 or #1477 #1871
[inst-discovered] theory-solving 0 basic# ; #1872
[mk-app] #1876 = #1872 #1875
[instance] 0 #1876
[attach-enode] #1876 0
[end-of-instance]
[mk-quant] #1876 internal_vstd__view__impl&__8_trait_impl_definition 4 #1873 #1875
[attach-var-names] #1876 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1877 fuel_bool #806
[mk-app] #1878 and #1663 #1843
[mk-app] #1879 tr_bound%core!alloc.Allocator. #65 #66
[mk-app] #1880 and #1878 #1879
[mk-app] #1881 vstd!view.View.view.? #125 #1861 #34
[mk-app] #1882 vstd!seq.Seq.len.? #971 #972 #1881
[mk-app] #1883 = #1863 #1882
[mk-app] #1884 => #1880 #1883
[mk-app] #1885 => #1862 #1884
[mk-quant] #1886 user_vstd__std_specs__vec__axiom_spec_len_11 5 #1866 #1885
[attach-var-names] #1886 (|v!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1887 => #1877 #1886
[mk-app] #1888 and #1663 #1843 #1879
[mk-app] #1889 not #1888
[mk-app] #1890 or #1889 #1883
[mk-app] #1891 => #1888 #1883
[inst-discovered] theory-solving 0 basic# ; #1891
[mk-app] #1892 = #1891 #1890
[instance] 0 #1892
[attach-enode] #1892 0
[end-of-instance]
[mk-app] #1891 or #1868 #1889 #1883
[mk-app] #1892 => #1862 #1890
[inst-discovered] theory-solving 0 basic# ; #1892
[mk-app] #1893 = #1892 #1891
[instance] 0 #1893
[attach-enode] #1893 0
[end-of-instance]
[mk-quant] #1890 user_vstd__std_specs__vec__axiom_spec_len_11 5 #1866 #1891
[attach-var-names] #1890 (|v!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1892 not #1877
[mk-app] #1893 or #1892 #1890
[mk-app] #1894 => #1877 #1890
[inst-discovered] theory-solving 0 basic# ; #1894
[mk-app] #1895 = #1894 #1893
[instance] 0 #1895
[attach-enode] #1895 0
[end-of-instance]
[mk-app] #1894 tr_bound%core!alloc.Allocator. #125 #1147
[mk-app] #1895 req%vstd!std_specs.vec.VecAdditionalSpecFns.spec_index. #1003 #984 #1004 #268 #33 #34
[mk-app] #1896 %%global_location_label%%2
[mk-app] #1897 vstd!view.View.view.? #1003 #984 #33
[mk-app] #1898 vstd!seq.Seq.len.? #1004 #268 #1897
[mk-app] #1899 < #191 #1898
[mk-app] #1900 and #517 #1899
[mk-app] #1901 => #1896 #1900
[mk-app] #1902 = #1895 #1901
[mk-app] #1903 pattern #1895
[mk-quant] #1904 internal_req__vstd!std_specs.vec.VecAdditionalSpecFns.spec_index._definition 6 #1903 #1902
[attach-var-names] #1904 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[attach-meaning] #370 arith (- 1)
[mk-app] #1905 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #1905 = #517 #521
[instance] 0 #1905
[attach-enode] #1905 0
[end-of-instance]
[mk-app] #1905 <= #1898 #191
[mk-app] #1906 not #1905
[inst-discovered] theory-solving 0 arith# ; #1899
[mk-app] #1907 = #1899 #1906
[instance] 0 #1907
[attach-enode] #1907 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #1907 * #370 #191
[mk-app] #1908 + #1907 #1898
[attach-meaning] #370 arith (- 1)
[mk-app] #1909 * #370 #1898
[mk-app] #1910 + #191 #1909
[mk-app] #1907 >= #1910 #341
[inst-discovered] theory-solving 0 arith# ; #1905
[mk-app] #1908 = #1905 #1907
[instance] 0 #1908
[attach-enode] #1908 0
[end-of-instance]
[mk-app] #1908 not #1907
[mk-app] #1911 and #521 #1908
[mk-app] #1912 not #1896
[mk-app] #1913 or #1912 #1911
[mk-app] #1914 => #1896 #1911
[inst-discovered] theory-solving 0 basic# ; #1914
[mk-app] #1915 = #1914 #1913
[instance] 0 #1915
[attach-enode] #1915 0
[end-of-instance]
[mk-app] #1914 = #1895 #1913
[mk-quant] #1915 internal_req__vstd!std_specs.vec.VecAdditionalSpecFns.spec_index._definition 6 #1903 #1914
[attach-var-names] #1915 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1905 and #1007 #190
[mk-app] #1906 vstd!std_specs.vec.VecAdditionalSpecFns.spec_index.? #1003 #984 #1004 #268 #33 #34
[mk-app] #1916 has_type #1906 #268
[mk-app] #1917 => #1905 #1916
[mk-app] #1918 pattern #1906
[mk-quant] #1919 internal_vstd!std_specs.vec.VecAdditionalSpecFns.spec_index.?_pre_post_definition 6 #1918 #1917
[attach-var-names] #1919 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1920 not #1905
[mk-app] #1921 or #1920 #1916
[inst-discovered] theory-solving 0 basic# ; #1917
[mk-app] #1922 = #1917 #1921
[instance] 0 #1922
[attach-enode] #1922 0
[end-of-instance]
[mk-quant] #1922 internal_vstd!std_specs.vec.VecAdditionalSpecFns.spec_index.?_pre_post_definition 6 #1918 #1921
[attach-var-names] #1922 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #1923 fuel_bool_default #805
[mk-app] #1924 fuel_bool #805
[mk-app] #1925 sized #1003
[mk-app] #1926 tr_bound%core!alloc.Allocator. #1004 #268
[mk-app] #1927 and #1925 #1372 #1926
[mk-app] #1928 TYPE%alloc!vec.Vec. #1003 #984 #1004 #268
[mk-app] #1929 vstd!std_specs.vec.VecAdditionalSpecFns.spec_index.? #125 #1928 #1003 #984 #33 #34
[mk-app] #1930 vstd!view.View.view.? #125 #1928 #33
[mk-app] #1931 vstd!seq.Seq.index.? #1003 #984 #1930 #34
[mk-app] #1932 = #1929 #1931
[mk-app] #1933 => #1927 #1932
[mk-app] #1934 pattern #1929
[mk-quant] #1935 internal_vstd!std_specs.vec.VecAdditionalSpecFns.spec_index.?_definition 6 #1934 #1933
[attach-var-names] #1935 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1936 => #1924 #1935
[mk-app] #1937 not #1927
[mk-app] #1938 or #1937 #1932
[inst-discovered] theory-solving 0 basic# ; #1933
[mk-app] #1939 = #1933 #1938
[instance] 0 #1939
[attach-enode] #1939 0
[end-of-instance]
[mk-quant] #1939 internal_vstd!std_specs.vec.VecAdditionalSpecFns.spec_index.?_definition 6 #1934 #1938
[attach-var-names] #1939 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1940 not #1924
[mk-app] #1941 or #1940 #1939
[mk-app] #1942 => #1924 #1939
[inst-discovered] theory-solving 0 basic# ; #1942
[mk-app] #1943 = #1942 #1941
[instance] 0 #1943
[attach-enode] #1943 0
[end-of-instance]
[mk-app] #1942 tr_bound%vstd!std_specs.vec.VecAdditionalSpecFns. #125 #1471 #1004 #268
[mk-app] #1943 => #1470 #1942
[mk-app] #1944 pattern #1942
[mk-quant] #1945 internal_vstd__std_specs__vec__impl&__0_trait_impl_definition 4 #1944 #1943
[attach-var-names] #1945 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1946 or #1477 #1942
[inst-discovered] theory-solving 0 basic# ; #1943
[mk-app] #1947 = #1943 #1946
[instance] 0 #1947
[attach-enode] #1947 0
[end-of-instance]
[mk-quant] #1947 internal_vstd__std_specs__vec__impl&__0_trait_impl_definition 4 #1944 #1946
[attach-var-names] #1947 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1948 fuel_bool #807
[mk-app] #1949 TYPE%alloc!vec.Vec. #1004 #268 #125 #1147
[mk-app] #1950 has_type #33 #1949
[mk-app] #1951 and #1950 #190
[mk-app] #1952 vstd!std_specs.vec.spec_vec_len.? #1004 #268 #125 #1147 #33
[mk-app] #1953 < #191 #1952
[mk-app] #1954 and #517 #1953
[mk-app] #1955 and #1372 #1954
[mk-app] #1956 vstd!view.View.view.? #125 #1949 #33
[mk-app] #1957 vstd!seq.Seq.index.? #1004 #268 #1956 #34
[mk-app] #1958 height #1957
[mk-app] #1959 height_lt #1958 #783
[mk-app] #1960 => #1955 #1959
[mk-app] #1961 => #1951 #1960
[mk-app] #1962 pattern #1958
[mk-quant] #1963 user_vstd__std_specs__vec__axiom_vec_index_decreases_12 4 #1962 #1961
[attach-var-names] #1963 (|i!| ; |Poly|) (|v!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1964 => #1948 #1963
[attach-meaning] #370 arith (- 1)
[mk-app] #1965 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #1965 = #517 #521
[instance] 0 #1965
[attach-enode] #1965 0
[end-of-instance]
[mk-app] #1965 <= #1952 #191
[mk-app] #1966 not #1965
[inst-discovered] theory-solving 0 arith# ; #1953
[mk-app] #1967 = #1953 #1966
[instance] 0 #1967
[attach-enode] #1967 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #1967 * #370 #191
[mk-app] #1968 + #1967 #1952
[attach-meaning] #370 arith (- 1)
[mk-app] #1969 * #370 #1952
[mk-app] #1970 + #191 #1969
[mk-app] #1967 >= #1970 #341
[inst-discovered] theory-solving 0 arith# ; #1965
[mk-app] #1968 = #1965 #1967
[instance] 0 #1968
[attach-enode] #1968 0
[end-of-instance]
[mk-app] #1968 not #1967
[mk-app] #1971 and #1372 #521 #1968
[mk-app] #1972 not #1971
[mk-app] #1973 or #1972 #1959
[mk-app] #1974 => #1971 #1959
[inst-discovered] theory-solving 0 basic# ; #1974
[mk-app] #1975 = #1974 #1973
[instance] 0 #1975
[attach-enode] #1975 0
[end-of-instance]
[mk-app] #1974 not #1951
[mk-app] #1975 or #1974 #1972 #1959
[mk-app] #1976 => #1951 #1973
[inst-discovered] theory-solving 0 basic# ; #1976
[mk-app] #1977 = #1976 #1975
[instance] 0 #1977
[attach-enode] #1977 0
[end-of-instance]
[mk-quant] #1973 user_vstd__std_specs__vec__axiom_vec_index_decreases_12 4 #1962 #1975
[attach-var-names] #1973 (|i!| ; |Poly|) (|v!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #1965 not #1948
[mk-app] #1966 or #1965 #1973
[mk-app] #1976 => #1948 #1973
[inst-discovered] theory-solving 0 basic# ; #1976
[mk-app] #1977 = #1976 #1966
[instance] 0 #1977
[attach-enode] #1977 0
[end-of-instance]
[mk-app] #1976 fuel_bool #808
[mk-app] #1977 has_resolved #125 #1949 #33
[mk-app] #1978 has_resolved #1004 #268 #1957
[mk-app] #1979 => #1977 #1978
[mk-app] #1980 => #1954 #1979
[mk-app] #1981 => #1372 #1980
[mk-app] #1982 => #1951 #1981
[mk-app] #1983 pattern #1977 #1957
[mk-quant] #1984 user_vstd__std_specs__vec__axiom_vec_has_resolved_13 4 #1983 #1982
[attach-var-names] #1984 (|i!| ; |Poly|) (|vec!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1985 => #1976 #1984
[attach-meaning] #370 arith (- 1)
[mk-app] #1986 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #1986 = #517 #521
[instance] 0 #1986
[attach-enode] #1986 0
[end-of-instance]
[mk-app] #1986 <= #1952 #191
[mk-app] #1987 not #1986
[inst-discovered] theory-solving 0 arith# ; #1953
[mk-app] #1988 = #1953 #1987
[instance] 0 #1988
[attach-enode] #1988 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #1988 * #370 #191
[mk-app] #1989 + #1988 #1952
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #1986
[mk-app] #1988 = #1986 #1967
[instance] 0 #1988
[attach-enode] #1988 0
[end-of-instance]
[mk-app] #1988 and #521 #1968
[mk-app] #1989 not #1977
[mk-app] #1990 or #1989 #1978
[inst-discovered] theory-solving 0 basic# ; #1979
[mk-app] #1991 = #1979 #1990
[instance] 0 #1991
[attach-enode] #1991 0
[end-of-instance]
[mk-app] #1991 not #1988
[mk-app] #1992 or #1991 #1989 #1978
[mk-app] #1993 => #1988 #1990
[inst-discovered] theory-solving 0 basic# ; #1993
[mk-app] #1994 = #1993 #1992
[instance] 0 #1994
[attach-enode] #1994 0
[end-of-instance]
[mk-app] #1990 or #1652 #1991 #1989 #1978
[mk-app] #1993 => #1372 #1992
[inst-discovered] theory-solving 0 basic# ; #1993
[mk-app] #1994 = #1993 #1990
[instance] 0 #1994
[attach-enode] #1994 0
[end-of-instance]
[mk-app] #1992 or #1974 #1652 #1991 #1989 #1978
[mk-app] #1993 => #1951 #1990
[inst-discovered] theory-solving 0 basic# ; #1993
[mk-app] #1994 = #1993 #1992
[instance] 0 #1994
[attach-enode] #1994 0
[end-of-instance]
[mk-quant] #1990 user_vstd__std_specs__vec__axiom_vec_has_resolved_13 4 #1983 #1992
[attach-var-names] #1990 (|i!| ; |Poly|) (|vec!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #1986 not #1976
[mk-app] #1987 or #1986 #1990
[mk-app] #1993 => #1976 #1990
[inst-discovered] theory-solving 0 basic# ; #1993
[mk-app] #1994 = #1993 #1987
[instance] 0 #1994
[attach-enode] #1994 0
[end-of-instance]
[mk-app] #1993 fuel_bool #809
[mk-app] #1994 vstd!view.View.view.? #125 #1308 #34
[mk-app] #1995 height #1994
[mk-app] #1996 height #34
[mk-app] #1997 height_lt #1995 #1996
[mk-app] #1998 => #1843 #1997
[mk-app] #1999 => #1309 #1998
[mk-app] #2000 pattern #1995
[mk-quant] #2001 user_vstd__std_specs__vec__axiom_vec_decreases_to_view_14 3 #2000 #1999
[attach-var-names] #2001 (|v!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2002 => #1993 #2001
[mk-app] #2003 or #1850 #1997
[inst-discovered] theory-solving 0 basic# ; #1998
[mk-app] #2004 = #1998 #2003
[instance] 0 #2004
[attach-enode] #2004 0
[end-of-instance]
[mk-app] #2004 or #1316 #1850 #1997
[mk-app] #2005 => #1309 #2003
[inst-discovered] theory-solving 0 basic# ; #2005
[mk-app] #2006 = #2005 #2004
[instance] 0 #2006
[attach-enode] #2006 0
[end-of-instance]
[mk-quant] #2003 user_vstd__std_specs__vec__axiom_vec_decreases_to_view_14 3 #2000 #2004
[attach-var-names] #2003 (|v!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2005 not #1993
[mk-app] #2006 or #2005 #2003
[mk-app] #2007 => #1993 #2003
[inst-discovered] theory-solving 0 basic# ; #2007
[mk-app] #2008 = #2007 #2006
[instance] 0 #2008
[attach-enode] #2008 0
[end-of-instance]
[mk-app] #2007 fuel_bool #821
[mk-app] #2008 = #1537 #341
[mk-app] #2009 vstd!seq.Seq.empty.? #65 #66
[mk-app] #2010 ext_eq #2 #1535 #34 #2009
[mk-app] #2011 => #2008 #2010
[mk-app] #2012 => #1843 #2011
[mk-app] #2013 => #1536 #2012
[mk-quant] #2014 user_vstd__seq_lib__lemma_seq_empty_equality_15 3 #1540 #2013
[attach-var-names] #2014 (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2015 => #2007 #2014
[mk-app] #2016 not #2008
[mk-app] #2017 or #2016 #2010
[inst-discovered] theory-solving 0 basic# ; #2011
[mk-app] #2018 = #2011 #2017
[instance] 0 #2018
[attach-enode] #2018 0
[end-of-instance]
[mk-app] #2018 or #1850 #2016 #2010
[mk-app] #2019 => #1843 #2017
[inst-discovered] theory-solving 0 basic# ; #2019
[mk-app] #2020 = #2019 #2018
[instance] 0 #2020
[attach-enode] #2020 0
[end-of-instance]
[mk-app] #2017 or #1542 #1850 #2016 #2010
[mk-app] #2019 => #1536 #2018
[inst-discovered] theory-solving 0 basic# ; #2019
[mk-app] #2020 = #2019 #2017
[instance] 0 #2020
[attach-enode] #2020 0
[end-of-instance]
[mk-quant] #2018 user_vstd__seq_lib__lemma_seq_empty_equality_15 3 #1540 #2017
[attach-var-names] #2018 (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2019 not #2007
[mk-app] #2020 or #2019 #2018
[mk-app] #2021 => #2007 #2018
[inst-discovered] theory-solving 0 basic# ; #2021
[mk-app] #2022 = #2021 #2020
[instance] 0 #2022
[attach-enode] #2022 0
[end-of-instance]
[mk-app] #2021 fuel_bool_default #803
[mk-app] #2022 fuel_bool #803
[mk-app] #2023 vstd!std_specs.option.is_none.? #65 #66 #34
[mk-app] #2024 = #2023 #1265
[mk-app] #2025 pattern #2023
[mk-quant] #2026 internal_vstd!std_specs.option.is_none.?_definition 3 #2025 #2024
[attach-var-names] #2026 (|option!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2027 => #2022 #2026
[inst-discovered] theory-solving 0 datatype# ; #1265
[mk-app] #2028 = #1265 #1272
[instance] 0 #2028
[attach-enode] #2028 0
[end-of-instance]
[mk-app] #2028 = #2023 #1272
[mk-quant] #2029 internal_vstd!std_specs.option.is_none.?_definition 3 #2025 #2028
[attach-var-names] #2029 (|option!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2030 not #2022
[mk-app] #2031 or #2030 #2029
[mk-app] #2032 => #2022 #2029
[inst-discovered] theory-solving 0 basic# ; #2032
[mk-app] #2033 = #2032 #2031
[instance] 0 #2033
[attach-enode] #2033 0
[end-of-instance]
[mk-app] #2032 fuel_bool_default #820
[mk-app] #2033 %%lambda%%0 #971 #972 #44 #986
[mk-app] #2034 %%apply%%0 #2033 #34
[mk-app] #2035 %%apply%%1 #986 #34 #1735
[mk-app] #2036 = #2034 #2035
[mk-app] #2037 pattern #2034
[mk-quant] #2038 k!2814 5 #2037 #2036
[attach-var-names] #2038 (|i$| ; |Poly|) (|%%hole%%3| ; |%%Function%%|) (|%%hole%%2| ; |Poly|) (|%%hole%%1| ; |Type|) (|%%hole%%0| ; |Dcr|)
[mk-app] #2039 fuel_bool #820
[mk-app] #2040 vstd!seq_lib.impl&%0.map.? #1003 #984 #1004 #268 #33 #34
[mk-app] #2041 vstd!seq.Seq.len.? #1003 #984 #33
[mk-app] #2042 I #2041
[mk-app] #2043 %%lambda%%0 #1003 #984 #33 #1061
[mk-app] #2044 mk_fun #2043
[mk-app] #2045 Poly%fun%1. #2044
[mk-app] #2046 vstd!seq.Seq.new.? #1004 #268 #125 #1641 #2042 #2045
[mk-app] #2047 = #2040 #2046
[mk-app] #2048 pattern #2040
[mk-quant] #2049 internal_vstd!seq_lib.impl&__0.map.?_definition 6 #2048 #2047
[attach-var-names] #2049 (|f!| ; |Poly|) (|self!| ; |Poly|) (|B&| ; |Type|) (|B&.| ; |Dcr|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2050 => #2039 #2049
[mk-app] #2051 not #2039
[mk-app] #2052 or #2051 #2049
[inst-discovered] theory-solving 0 basic# ; #2050
[mk-app] #2053 = #2050 #2052
[instance] 0 #2053
[attach-enode] #2053 0
[end-of-instance]
[mk-app] #2053 has_type #33 #1632
[mk-app] #2054 TYPE%fun%2. #125 #189 #1003 #984 #1004 #268
[mk-app] #2055 has_type #34 #2054
[mk-app] #2056 and #2053 #2055
[mk-app] #2057 has_type #2040 #1481
[mk-app] #2058 => #2056 #2057
[mk-quant] #2059 internal_vstd!seq_lib.impl&__0.map.?_pre_post_definition 6 #2048 #2058
[attach-var-names] #2059 (|f!| ; |Poly|) (|self!| ; |Poly|) (|B&| ; |Type|) (|B&.| ; |Dcr|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2060 not #2056
[mk-app] #2061 or #2060 #2057
[inst-discovered] theory-solving 0 basic# ; #2058
[mk-app] #2062 = #2058 #2061
[instance] 0 #2062
[attach-enode] #2062 0
[end-of-instance]
[mk-quant] #2062 internal_vstd!seq_lib.impl&__0.map.?_pre_post_definition 6 #2048 #2061
[attach-var-names] #2062 (|f!| ; |Poly|) (|self!| ; |Poly|) (|B&| ; |Type|) (|B&.| ; |Dcr|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2063 fuel_bool_default #802
[mk-app] #2064 fuel_bool #802
[mk-app] #2065 vstd!std_specs.option.is_some.? #65 #66 #34
[mk-app] #2066 = #2065 #1278
[mk-app] #2067 pattern #2065
[mk-quant] #2068 internal_vstd!std_specs.option.is_some.?_definition 3 #2067 #2066
[attach-var-names] #2068 (|option!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2069 => #2064 #2068
[inst-discovered] theory-solving 0 datatype# ; #1278
[mk-app] #2070 = #1278 #1286
[instance] 0 #2070
[attach-enode] #2070 0
[end-of-instance]
[mk-app] #2070 = #2065 #1286
[mk-quant] #2071 internal_vstd!std_specs.option.is_some.?_definition 3 #2067 #2070
[attach-var-names] #2071 (|option!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2072 not #2064
[mk-app] #2073 or #2072 #2071
[mk-app] #2074 => #2064 #2071
[inst-discovered] theory-solving 0 basic# ; #2074
[mk-app] #2075 = #2074 #2073
[instance] 0 #2075
[attach-enode] #2075 0
[end-of-instance]
[mk-app] #2074 has_type #34 #972
[mk-app] #2075 vstd!std_specs.option.OptionAdditionalFns.arrow_0.? #971 #972 #65 #66 #34
[mk-app] #2076 has_type #2075 #66
[mk-app] #2077 => #2074 #2076
[mk-app] #2078 pattern #2075
[mk-quant] #2079 internal_vstd!std_specs.option.OptionAdditionalFns.arrow_0.?_pre_post_definition 5 #2078 #2077
[attach-var-names] #2079 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2080 not #2074
[mk-app] #2081 or #2080 #2076
[inst-discovered] theory-solving 0 basic# ; #2077
[mk-app] #2082 = #2077 #2081
[instance] 0 #2082
[attach-enode] #2082 0
[end-of-instance]
[mk-quant] #2082 internal_vstd!std_specs.option.OptionAdditionalFns.arrow_0.?_pre_post_definition 5 #2078 #2081
[attach-var-names] #2082 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2083 fuel_bool_default #801
[mk-app] #2084 fuel_bool #801
[mk-app] #2085 vstd!std_specs.option.OptionAdditionalFns.arrow_0.? #125 #1208 #65 #66 #34
[mk-app] #2086 = #2085 #1245
[mk-app] #2087 => #1843 #2086
[mk-app] #2088 pattern #2085
[mk-quant] #2089 internal_vstd!std_specs.option.OptionAdditionalFns.arrow_0.?_definition 3 #2088 #2087
[attach-var-names] #2089 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2090 => #2084 #2089
[mk-app] #2091 or #1850 #2086
[inst-discovered] theory-solving 0 basic# ; #2087
[mk-app] #2092 = #2087 #2091
[instance] 0 #2092
[attach-enode] #2092 0
[end-of-instance]
[mk-quant] #2092 internal_vstd!std_specs.option.OptionAdditionalFns.arrow_0.?_definition 3 #2088 #2091
[attach-var-names] #2092 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2093 not #2084
[mk-app] #2094 or #2093 #2092
[mk-app] #2095 => #2084 #2092
[inst-discovered] theory-solving 0 basic# ; #2095
[mk-app] #2096 = #2095 #2094
[instance] 0 #2096
[attach-enode] #2096 0
[end-of-instance]
[mk-app] #2095 tr_bound%vstd!std_specs.option.OptionAdditionalFns. #125 #1221 #45 #46
[mk-app] #2096 => #1373 #2095
[mk-app] #2097 pattern #2095
[mk-quant] #2098 internal_vstd__std_specs__option__impl&__0_trait_impl_definition 2 #2097 #2096
[attach-var-names] #2098 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2099 or #1493 #2095
[inst-discovered] theory-solving 0 basic# ; #2096
[mk-app] #2100 = #2096 #2099
[instance] 0 #2100
[attach-enode] #2100 0
[end-of-instance]
[mk-quant] #2100 internal_vstd__std_specs__option__impl&__0_trait_impl_definition 2 #2097 #2099
[attach-var-names] #2100 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2101 req%vstd!std_specs.option.spec_unwrap. #65 #66 #34
[mk-app] #2102 %%global_location_label%%3
[mk-app] #2103 => #2102 #1278
[mk-app] #2104 = #2101 #2103
[mk-app] #2105 pattern #2101
[mk-quant] #2106 internal_req__vstd!std_specs.option.spec_unwrap._definition 3 #2105 #2104
[attach-var-names] #2106 (|option!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[inst-discovered] theory-solving 0 datatype# ; #1278
[mk-app] #2107 = #1278 #1286
[instance] 0 #2107
[attach-enode] #2107 0
[end-of-instance]
[mk-app] #2107 not #2102
[mk-app] #2108 or #2107 #1286
[mk-app] #2109 => #2102 #1286
[inst-discovered] theory-solving 0 basic# ; #2109
[mk-app] #2110 = #2109 #2108
[instance] 0 #2110
[attach-enode] #2110 0
[end-of-instance]
[mk-app] #2109 = #2101 #2108
[mk-quant] #2110 internal_req__vstd!std_specs.option.spec_unwrap._definition 3 #2105 #2109
[attach-var-names] #2110 (|option!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2111 fuel_bool_default #804
[mk-app] #2112 fuel_bool #804
[mk-app] #2113 vstd!std_specs.option.spec_unwrap.? #65 #66 #34
[mk-app] #2114 = #2113 #1245
[mk-app] #2115 pattern #2113
[mk-quant] #2116 internal_vstd!std_specs.option.spec_unwrap.?_definition 3 #2115 #2114
[attach-var-names] #2116 (|option!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2117 => #2112 #2116
[mk-app] #2118 not #2112
[mk-app] #2119 or #2118 #2116
[inst-discovered] theory-solving 0 basic# ; #2117
[mk-app] #2120 = #2117 #2119
[instance] 0 #2120
[attach-enode] #2120 0
[end-of-instance]
[mk-app] #2120 has_type #2113 #66
[mk-app] #2121 => #1209 #2120
[mk-quant] #2122 internal_vstd!std_specs.option.spec_unwrap.?_pre_post_definition 3 #2115 #2121
[attach-var-names] #2122 (|option!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2123 or #1216 #2120
[inst-discovered] theory-solving 0 basic# ; #2121
[mk-app] #2124 = #2121 #2123
[instance] 0 #2124
[attach-enode] #2124 0
[end-of-instance]
[mk-quant] #2124 internal_vstd!std_specs.option.spec_unwrap.?_pre_post_definition 3 #2115 #2123
[attach-var-names] #2124 (|option!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2125 ens%alloc!vec.impl&%43.push. #1026 #1027 #971 #972 #44 #33 #34
[mk-app] #2126 TYPE%alloc!vec.Vec. #1026 #1027 #971 #972
[mk-app] #2127 has_type #33 #2126
[mk-app] #2128 vstd!view.View.view.? #125 #2126 #33
[mk-app] #2129 vstd!view.View.view.? #125 #2126 #44
[mk-app] #2130 vstd!seq.Seq.push.? #1026 #1027 #2129 #34
[mk-app] #2131 = #2128 #2130
[mk-app] #2132 and #2127 #2131
[mk-app] #2133 = #2125 #2132
[mk-app] #2134 pattern #2125
[mk-quant] #2135 internal_ens__alloc!vec.impl&__43.push._definition 7 #2134 #2133
[attach-var-names] #2135 (|value!| ; |Poly|) (|vec!| ; |Poly|) (|pre%vec!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2136 ens%alloc!vec.impl&%0.with_capacity. #1004 #268 #224 #34
[mk-app] #2137 has_type #34 #1949
[mk-app] #2138 vstd!view.View.view.? #125 #1949 #34
[mk-app] #2139 vstd!seq.Seq.empty.? #1004 #268
[mk-app] #2140 = #2138 #2139
[mk-app] #2141 and #2137 #2140
[mk-app] #2142 = #2136 #2141
[mk-app] #2143 pattern #2136
[mk-quant] #2144 internal_ens__alloc!vec.impl&__0.with_capacity._definition 4 #2143 #2142
[attach-var-names] #2144 (|v!| ; |Poly|) (|capacity!| ; |Int|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2145 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_arrayseqsteph_wf.? #971 #972 #65 #66 #34
[mk-app] #2146 has_type #2145 #140
[mk-app] #2147 => #2074 #2146
[mk-app] #2148 pattern #2145
[mk-quant] #2149 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_arrayseqsteph_wf.?_pre_post_definition 5 #2148 #2147
[attach-var-names] #2149 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2150 or #2080 #2146
[inst-discovered] theory-solving 0 basic# ; #2147
[mk-app] #2151 = #2147 #2150
[instance] 0 #2151
[attach-enode] #2151 0
[end-of-instance]
[mk-quant] #2151 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_arrayseqsteph_wf.?_pre_post_definition 5 #2148 #2150
[attach-var-names] #2151 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2152 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_len.? #971 #972 #65 #66 #34
[mk-app] #2153 has_type #2152 #200
[mk-app] #2154 => #2074 #2153
[mk-app] #2155 pattern #2152
[mk-quant] #2156 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_len.?_pre_post_definition 5 #2155 #2154
[attach-var-names] #2156 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2157 or #2080 #2153
[inst-discovered] theory-solving 0 basic# ; #2154
[mk-app] #2158 = #2154 #2157
[instance] 0 #2158
[attach-enode] #2158 0
[end-of-instance]
[mk-quant] #2158 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_len.?_pre_post_definition 5 #2155 #2157
[attach-var-names] #2158 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2159 req%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index. #1003 #984 #1004 #268 #33 #34
[mk-app] #2160 %%global_location_label%%4
[mk-app] #2161 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_len.? #1003 #984 #1004 #268 #33
[mk-app] #2162 %I #2161
[mk-app] #2163 < #191 #2162
[mk-app] #2164 => #2160 #2163
[mk-app] #2165 = #2159 #2164
[mk-app] #2166 pattern #2159
[mk-quant] #2167 internal_req__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index._definition 6 #2166 #2165
[attach-var-names] #2167 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2168 <= #2162 #191
[mk-app] #2169 not #2168
[inst-discovered] theory-solving 0 arith# ; #2163
[mk-app] #2170 = #2163 #2169
[instance] 0 #2170
[attach-enode] #2170 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2170 * #370 #191
[mk-app] #2171 + #2170 #2162
[attach-meaning] #370 arith (- 1)
[mk-app] #2172 * #370 #2162
[mk-app] #2173 + #191 #2172
[mk-app] #2170 >= #2173 #341
[inst-discovered] theory-solving 0 arith# ; #2168
[mk-app] #2171 = #2168 #2170
[instance] 0 #2171
[attach-enode] #2171 0
[end-of-instance]
[mk-app] #2171 not #2170
[mk-app] #2174 not #2160
[mk-app] #2175 or #2174 #2171
[mk-app] #2176 => #2160 #2171
[inst-discovered] theory-solving 0 basic# ; #2176
[mk-app] #2177 = #2176 #2175
[instance] 0 #2177
[attach-enode] #2177 0
[end-of-instance]
[mk-app] #2176 = #2159 #2175
[mk-quant] #2177 internal_req__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index._definition 6 #2166 #2176
[attach-var-names] #2177 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2168 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.? #1003 #984 #1004 #268 #33 #34
[mk-app] #2169 has_type #2168 #268
[mk-app] #2178 => #1905 #2169
[mk-app] #2179 pattern #2168
[mk-quant] #2180 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.?_pre_post_definition 6 #2179 #2178
[attach-var-names] #2180 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2181 or #1920 #2169
[inst-discovered] theory-solving 0 basic# ; #2178
[mk-app] #2182 = #2178 #2181
[instance] 0 #2182
[attach-enode] #2182 0
[end-of-instance]
[mk-quant] #2182 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.?_pre_post_definition 6 #2179 #2181
[attach-var-names] #2182 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2183 ens%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.length. #1003 #984 #1004 #268 #33 #34
[mk-app] #2184 = #34 #2161
[mk-app] #2185 and #209 #2184
[mk-app] #2186 = #2183 #2185
[mk-app] #2187 pattern #2183
[mk-quant] #2188 internal_ens__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.length._definition 6 #2187 #2186
[attach-var-names] #2188 (|len!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2189 req%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth. #1003 #984 #1004 #268 #33 #34
[mk-app] #2190 %%global_location_label%%5
[mk-app] #2191 => #2190 #2163
[mk-app] #2192 = #2189 #2191
[mk-app] #2193 pattern #2189
[mk-quant] #2194 internal_req__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth._definition 6 #2193 #2192
[attach-var-names] #2194 (|index!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2195 <= #2162 #191
[mk-app] #2196 not #2195
[inst-discovered] theory-solving 0 arith# ; #2163
[mk-app] #2197 = #2163 #2196
[instance] 0 #2197
[attach-enode] #2197 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2197 * #370 #191
[mk-app] #2198 + #2197 #2162
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #2195
[mk-app] #2197 = #2195 #2170
[instance] 0 #2197
[attach-enode] #2197 0
[end-of-instance]
[mk-app] #2197 not #2190
[mk-app] #2198 or #2197 #2171
[mk-app] #2199 => #2190 #2171
[inst-discovered] theory-solving 0 basic# ; #2199
[mk-app] #2200 = #2199 #2198
[instance] 0 #2200
[attach-enode] #2200 0
[end-of-instance]
[mk-app] #2199 = #2189 #2198
[mk-quant] #2200 internal_req__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth._definition 6 #2193 #2199
[attach-var-names] #2200 (|index!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2195 ens%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth. #1026 #1027 #971 #972 #44 #33 #34
[mk-app] #2196 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.? #1026 #1027 #971 #972 #44 #33
[mk-app] #2201 = #34 #2196
[mk-app] #2202 and #2074 #2201
[mk-app] #2203 = #2195 #2202
[mk-app] #2204 pattern #2195
[mk-quant] #2205 internal_ens__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth._definition 7 #2204 #2203
[attach-var-names] #2205 (|nth_elem!| ; |Poly|) (|index!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2206 ens%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.from_vec. #1003 #984 #1004 #268 #33 #34
[mk-app] #2207 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_arrayseqsteph_wf.? #1003 #984 #1004 #268 #34
[mk-app] #2208 %B #2207
[mk-app] #2209 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_len.? #1003 #984 #1004 #268 #34
[mk-app] #2210 %I #2209
[mk-app] #2211 vstd!seq.Seq.len.? #1004 #268 #1956
[mk-app] #2212 = #2210 #2211
[mk-app] #2213 TYPE%alloc!vec.Vec. #971 #972 #125 #1147
[mk-app] #2214 vstd!view.View.view.? #125 #2213 #44
[mk-app] #2215 vstd!seq.Seq.len.? #971 #972 #2214
[mk-app] #2216 < #191 #2215
[mk-app] #2217 and #517 #2216
[mk-app] #2218 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.? #1026 #1027 #971 #972 #33 #34
[mk-app] #2219 vstd!seq.Seq.index.? #971 #972 #2214 #34
[mk-app] #2220 = #2218 #2219
[mk-app] #2221 => #2217 #2220
[mk-app] #2222 => #190 #2221
[mk-app] #2223 pattern #2218
[mk-quant] #2224 user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphTrait__from_vec_16 1 #2223 #2222
[attach-var-names] #2224 (|i$| ; |Poly|)
[mk-app] #2225 and #985 #2208 #2212 #2224
[mk-app] #2226 = #2206 #2225
[mk-app] #2227 pattern #2206
[mk-quant] #2228 internal_ens__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.from_vec._definition 6 #2227 #2226
[attach-var-names] #2228 (|seq!| ; |Poly|) (|elts!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[attach-meaning] #370 arith (- 1)
[mk-app] #2229 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #2229 = #517 #521
[instance] 0 #2229
[attach-enode] #2229 0
[end-of-instance]
[mk-app] #2229 <= #2215 #191
[mk-app] #2230 not #2229
[inst-discovered] theory-solving 0 arith# ; #2216
[mk-app] #2231 = #2216 #2230
[instance] 0 #2231
[attach-enode] #2231 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2231 * #370 #191
[mk-app] #2232 + #2231 #2215
[attach-meaning] #370 arith (- 1)
[mk-app] #2233 * #370 #2215
[mk-app] #2234 + #191 #2233
[mk-app] #2231 >= #2234 #341
[inst-discovered] theory-solving 0 arith# ; #2229
[mk-app] #2232 = #2229 #2231
[instance] 0 #2232
[attach-enode] #2232 0
[end-of-instance]
[mk-app] #2232 not #2231
[mk-app] #2235 and #521 #2232
[mk-app] #2236 not #2235
[mk-app] #2237 or #2236 #2220
[mk-app] #2238 => #2235 #2220
[inst-discovered] theory-solving 0 basic# ; #2238
[mk-app] #2239 = #2238 #2237
[instance] 0 #2239
[attach-enode] #2239 0
[end-of-instance]
[mk-app] #2238 or #197 #2236 #2220
[mk-app] #2239 => #190 #2237
[inst-discovered] theory-solving 0 basic# ; #2239
[mk-app] #2240 = #2239 #2238
[instance] 0 #2240
[attach-enode] #2240 0
[end-of-instance]
[mk-quant] #2237 user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphTrait__from_vec_16 1 #2223 #2238
[attach-var-names] #2237 (|i$| ; |Poly|)
[mk-app] #2229 and #985 #2208 #2212 #2237
[mk-app] #2230 = #2206 #2229
[mk-quant] #2239 internal_ens__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.from_vec._definition 6 #2227 #2230
[attach-var-names] #2239 (|seq!| ; |Poly|) (|elts!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2240 fuel_bool_default #833
[mk-app] #2241 fuel_bool #833
[mk-app] #2242 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_len.? #125 #1297 #65 #66 #34
[mk-app] #2243 vstd!view.View.view.? #125 #1308 #1324
[mk-app] #2244 vstd!seq.Seq.len.? #65 #66 #2243
[mk-app] #2245 I #2244
[mk-app] #2246 = #2242 #2245
[mk-app] #2247 => #1843 #2246
[mk-app] #2248 pattern #2242
[mk-quant] #2249 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_len.?_definition 3 #2248 #2247
[attach-var-names] #2249 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2250 => #2241 #2249
[mk-app] #2251 or #1850 #2246
[inst-discovered] theory-solving 0 basic# ; #2247
[mk-app] #2252 = #2247 #2251
[instance] 0 #2252
[attach-enode] #2252 0
[end-of-instance]
[mk-quant] #2252 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_len.?_definition 3 #2248 #2251
[attach-var-names] #2252 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2253 not #2241
[mk-app] #2254 or #2253 #2252
[mk-app] #2255 => #2241 #2252
[inst-discovered] theory-solving 0 basic# ; #2255
[mk-app] #2256 = #2255 #2254
[instance] 0 #2256
[attach-enode] #2256 0
[end-of-instance]
[mk-app] #2255 fuel_bool_default #834
[mk-app] #2256 fuel_bool #834
[mk-app] #2257 TYPE%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #1004 #268
[mk-app] #2258 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.? #125 #2257 #1004 #268 #33 #34
[mk-app] #2259 %Poly%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #33
[mk-app] #2260 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq #2259
[mk-app] #2261 vstd!view.View.view.? #125 #1949 #2260
[mk-app] #2262 vstd!seq.Seq.index.? #1004 #268 #2261 #34
[mk-app] #2263 = #2258 #2262
[mk-app] #2264 => #1372 #2263
[mk-app] #2265 pattern #2258
[mk-quant] #2266 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.?_definition 4 #2265 #2264
[attach-var-names] #2266 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2267 => #2256 #2266
[mk-app] #2268 or #1652 #2263
[inst-discovered] theory-solving 0 basic# ; #2264
[mk-app] #2269 = #2264 #2268
[instance] 0 #2269
[attach-enode] #2269 0
[end-of-instance]
[mk-quant] #2269 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.?_definition 4 #2265 #2268
[attach-var-names] #2269 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2270 not #2256
[mk-app] #2271 or #2270 #2269
[mk-app] #2272 => #2256 #2269
[inst-discovered] theory-solving 0 basic# ; #2272
[mk-app] #2273 = #2272 #2271
[instance] 0 #2273
[attach-enode] #2273 0
[end-of-instance]
[mk-app] #2272 fuel_bool_default #832
[mk-app] #2273 fuel_bool #832
[mk-app] #2274 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_arrayseqsteph_wf.? #125 #1297 #65 #66 #34
[mk-app] #2275 B #1
[mk-app] #2276 = #2274 #2275
[mk-app] #2277 => #1843 #2276
[mk-app] #2278 pattern #2274
[mk-quant] #2279 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_arrayseqsteph_wf.?_definition 3 #2278 #2277
[attach-var-names] #2279 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2280 => #2273 #2279
[mk-app] #2281 or #1850 #2276
[inst-discovered] theory-solving 0 basic# ; #2277
[mk-app] #2282 = #2277 #2281
[instance] 0 #2282
[attach-enode] #2282 0
[end-of-instance]
[mk-quant] #2282 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_arrayseqsteph_wf.?_definition 3 #2278 #2281
[attach-var-names] #2282 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2283 not #2273
[mk-app] #2284 or #2283 #2282
[mk-app] #2285 => #2273 #2282
[inst-discovered] theory-solving 0 basic# ; #2285
[mk-app] #2286 = #2285 #2284
[instance] 0 #2286
[attach-enode] #2286 0
[end-of-instance]
[mk-app] #2285 tr_bound%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait. #125 #1519 #45 #46
[mk-app] #2286 => #1373 #2285
[mk-app] #2287 pattern #2285
[mk-quant] #2288 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__impl&__3_trait_impl_definition 2 #2287 #2286
[attach-var-names] #2288 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2289 or #1493 #2285
[inst-discovered] theory-solving 0 basic# ; #2286
[mk-app] #2290 = #2286 #2289
[instance] 0 #2290
[attach-enode] #2290 0
[end-of-instance]
[mk-quant] #2290 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__impl&__3_trait_impl_definition 2 #2287 #2289
[attach-var-names] #2290 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2291 fuel_bool_default #825
[mk-app] #2292 fuel_bool #825
[mk-app] #2293 tr_bound%vstd!view.View. #65 #66
[mk-app] #2294 and #1843 #2293
[mk-app] #2295 ARC #125 #1147 #65
[mk-app] #2296 vstd!view.View.view.? #2295 #66 #34
[mk-app] #2297 = #2296 #1803
[mk-app] #2298 => #2294 #2297
[mk-app] #2299 pattern #2296
[mk-quant] #2300 internal_vstd!view.View.view.?_definition 3 #2299 #2298
[attach-var-names] #2300 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2301 => #2292 #2300
[mk-app] #2302 not #2294
[mk-app] #2303 or #2302 #2297
[inst-discovered] theory-solving 0 basic# ; #2298
[mk-app] #2304 = #2298 #2303
[instance] 0 #2304
[attach-enode] #2304 0
[end-of-instance]
[mk-quant] #2304 internal_vstd!view.View.view.?_definition 3 #2299 #2303
[attach-var-names] #2304 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2305 not #2292
[mk-app] #2306 or #2305 #2304
[mk-app] #2307 => #2292 #2304
[inst-discovered] theory-solving 0 basic# ; #2307
[mk-app] #2308 = #2307 #2306
[instance] 0 #2308
[attach-enode] #2308 0
[end-of-instance]
[mk-app] #2307 tr_bound%vstd!view.View. #1455 #46
[mk-app] #2308 => #1438 #2307
[mk-app] #2309 pattern #2307
[mk-quant] #2310 internal_vstd__view__impl&__6_trait_impl_definition 2 #2309 #2308
[attach-var-names] #2310 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2311 or #1445 #2307
[inst-discovered] theory-solving 0 basic# ; #2308
[mk-app] #2312 = #2308 #2311
[instance] 0 #2312
[attach-enode] #2312 0
[end-of-instance]
[mk-quant] #2312 internal_vstd__view__impl&__6_trait_impl_definition 2 #2309 #2311
[attach-var-names] #2312 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2313 fuel_bool_default #835
[mk-var] #2314 0
[mk-app] #2315 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #64 #44 #33 #2314
[mk-app] #2316 zero
[mk-app] #2317 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #64 #44 #33 #2316
[mk-app] #2318 = #2315 #2317
[mk-app] #2319 pattern #2315
[mk-quant] #2320 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum._fuel_to_zero_definition 4 #2319 #2318
[attach-var-names] #2320 (|fuel%| ; |Fuel|) (|hi!| ; |Poly|) (|lo!| ; |Poly|) (|s!| ; |Poly|)
[attach-meaning] #275 arith 32
[mk-app] #2321 has_type #64 #1188
[mk-app] #2322 has_type #44 #189
[mk-app] #2323 has_type #33 #189
[mk-app] #2324 and #2321 #2322 #2323
[mk-app] #2325 succ #2314
[mk-app] #2326 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #64 #44 #33 #2325
[mk-app] #2327 >= #666 #676
[attach-meaning] #275 arith 32
[mk-app] #2328 vstd!seq.Seq.index.? #125 #1167 #64 #44
[mk-app] #2329 %I #2328
[mk-app] #2330 Add #666 #296
[mk-app] #2331 I #2330
[mk-app] #2332 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #64 #2331 #33 #2314
[mk-app] #2333 Add #2329 #2332
[mk-app] #2334 if #2327 #341 #2333
[mk-app] #2335 = #2326 #2334
[mk-app] #2336 => #2324 #2335
[mk-app] #2337 pattern #2326
[mk-quant] #2338 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum._fuel_to_body_definition 4 #2337 #2336
[attach-var-names] #2338 (|fuel%| ; |Fuel|) (|hi!| ; |Poly|) (|lo!| ; |Poly|) (|s!| ; |Poly|)
[attach-meaning] #370 arith (- 1)
[mk-app] #2339 * #370 #676
[mk-app] #2340 + #666 #2339
[mk-app] #2341 >= #2340 #341
[inst-discovered] theory-solving 0 arith# ; #2327
[mk-app] #2342 = #2327 #2341
[instance] 0 #2342
[attach-enode] #2342 0
[end-of-instance]
[mk-app] #2342 if #2341 #341 #2333
[mk-app] #2343 = #2326 #2342
[mk-app] #2344 not #2324
[mk-app] #2345 or #2344 #2343
[mk-app] #2346 => #2324 #2343
[inst-discovered] theory-solving 0 basic# ; #2346
[mk-app] #2347 = #2346 #2345
[instance] 0 #2347
[attach-enode] #2347 0
[end-of-instance]
[mk-quant] #2346 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum._fuel_to_body_definition 4 #2337 #2345
[attach-var-names] #2346 (|fuel%| ; |Fuel|) (|hi!| ; |Poly|) (|lo!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #2347 fuel_bool #835
[attach-meaning] #275 arith 32
[mk-app] #2348 has_type #44 #1188
[mk-app] #2349 and #2348 #2323 #190
[mk-app] #2350 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #44 #33 #34
[mk-app] #2351 fuel_nat%lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.
[mk-app] #2352 succ #2351
[mk-app] #2353 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #44 #33 #34 #2352
[mk-app] #2354 = #2350 #2353
[mk-app] #2355 => #2349 #2354
[mk-app] #2356 pattern #2350
[mk-quant] #2357 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.?_definition 3 #2356 #2355
[attach-var-names] #2357 (|hi!| ; |Poly|) (|lo!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #2358 => #2347 #2357
[mk-app] #2359 not #2349
[mk-app] #2360 or #2359 #2354
[inst-discovered] theory-solving 0 basic# ; #2355
[mk-app] #2361 = #2355 #2360
[instance] 0 #2361
[attach-enode] #2361 0
[end-of-instance]
[mk-quant] #2361 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.?_definition 3 #2356 #2360
[attach-var-names] #2361 (|hi!| ; |Poly|) (|lo!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #2362 not #2347
[mk-app] #2363 or #2362 #2361
[mk-app] #2364 => #2347 #2361
[inst-discovered] theory-solving 0 basic# ; #2364
[mk-app] #2365 = #2364 #2363
[instance] 0 #2365
[attach-enode] #2365 0
[end-of-instance]
[mk-app] #2364 fuel_bool_default #837
[mk-app] #2365 fuel_bool #837
[mk-app] #2366 lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.? #34
[mk-app] #2367 and #2323 #190
[attach-meaning] #275 arith 32
[mk-app] #2368 vstd!seq.Seq.len.? #125 #1167 #44
[mk-app] #2369 <= #676 #191
[mk-app] #2370 and #729 #2369
[mk-app] #2371 <= #191 #2368
[mk-app] #2372 and #2370 #2371
[attach-meaning] #314 arith 2147483648
[mk-app] #2373 Int
[attach-meaning] #2373 arith 2147483647
[mk-app] #2374 <= #315 #2350
[mk-app] #2375 <= #2350 #2373
[mk-app] #2376 and #2374 #2375
[mk-app] #2377 => #2372 #2376
[mk-app] #2378 => #2367 #2377
[mk-quant] #2379 user_lib__Chap28__MCSSSpec__MCSSSpec__sums_fit_i32_17 2 #2356 #2378
[attach-var-names] #2379 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[mk-app] #2380 = #2366 #2379
[mk-app] #2381 pattern #2366
[mk-quant] #2382 internal_lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.?_definition 1 #2381 #2380
[attach-var-names] #2382 (|s!| ; |Poly|)
[mk-app] #2383 => #2365 #2382
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #729
[mk-app] #2384 = #729 #738
[instance] 0 #2384
[attach-enode] #2384 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2384 * #370 #191
[mk-app] #2385 + #2384 #676
[attach-meaning] #370 arith (- 1)
[mk-app] #2386 + #191 #2339
[mk-app] #2384 >= #2386 #341
[inst-discovered] theory-solving 0 arith# ; #2369
[mk-app] #2385 = #2369 #2384
[instance] 0 #2385
[attach-enode] #2385 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2385 * #370 #2368
[mk-app] #2387 + #191 #2385
[mk-app] #2388 <= #2387 #341
[inst-discovered] theory-solving 0 arith# ; #2371
[mk-app] #2389 = #2371 #2388
[instance] 0 #2389
[attach-enode] #2389 0
[end-of-instance]
[mk-app] #2389 and #738 #2384 #2388
[attach-meaning] #317 arith (- 2147483648)
[inst-discovered] theory-solving 0 arith# ; #315
[mk-app] #2390 = #315 #317
[instance] 0 #2390
[attach-enode] #2390 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2390 * #370 #2350
[attach-meaning] #314 arith 2147483648
[attach-meaning] #317 arith (- 2147483648)
[mk-app] #2391 >= #2350 #317
[mk-app] #2390 <= #317 #2350
[inst-discovered] theory-solving 0 arith# ; #2390
[mk-app] #2392 = #2390 #2391
[instance] 0 #2392
[attach-enode] #2392 0
[end-of-instance]
[mk-app] #2390 and #2391 #2375
[mk-app] #2392 not #2389
[mk-app] #2393 or #2392 #2390
[mk-app] #2394 => #2389 #2390
[inst-discovered] theory-solving 0 basic# ; #2394
[mk-app] #2395 = #2394 #2393
[instance] 0 #2395
[attach-enode] #2395 0
[end-of-instance]
[mk-app] #2394 not #2367
[mk-app] #2395 or #2394 #2392 #2390
[mk-app] #2396 => #2367 #2393
[inst-discovered] theory-solving 0 basic# ; #2396
[mk-app] #2397 = #2396 #2395
[instance] 0 #2397
[attach-enode] #2397 0
[end-of-instance]
[mk-quant] #2393 user_lib__Chap28__MCSSSpec__MCSSSpec__sums_fit_i32_17 2 #2356 #2395
[attach-var-names] #2393 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[mk-app] #2396 = #2366 #2393
[mk-quant] #2397 internal_lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.?_definition 1 #2381 #2396
[attach-var-names] #2397 (|s!| ; |Poly|)
[mk-app] #2398 not #2365
[mk-app] #2399 or #2398 #2397
[mk-app] #2400 => #2365 #2397
[inst-discovered] theory-solving 0 basic# ; #2400
[mk-app] #2401 = #2400 #2399
[instance] 0 #2401
[attach-enode] #2401 0
[end-of-instance]
[mk-app] #2400 fuel_bool_default #836
[mk-app] #2401 fuel_bool #836
[mk-app] #2402 lib!Chap28.MCSSSpec.MCSSSpec.is_mcss_of.? #33 #34
[attach-meaning] #275 arith 32
[mk-app] #2403 vstd!seq.Seq.len.? #125 #1167 #64
[mk-app] #2404 < #676 #191
[mk-app] #2405 and #729 #2404
[mk-app] #2406 <= #191 #2403
[mk-app] #2407 and #2405 #2406
[mk-app] #2408 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #64 #33 #34
[mk-app] #2409 = #2408 #666
[mk-app] #2410 and #2407 #2409
[mk-app] #2411 and #2367 #2410
[mk-app] #2412 pattern #2408
[mk-quant] #2413 user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_18 2 #2412 #2411
[attach-var-names] #2413 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[attach-meaning] #275 arith 32
[mk-app] #2414 <= #2408 #666
[mk-app] #2415 => #2407 #2414
[mk-app] #2416 => #2367 #2415
[mk-quant] #2417 user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_19 2 #2412 #2416
[attach-var-names] #2417 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[mk-app] #2418 and #2413 #2417
[mk-app] #2419 = #2402 #2418
[mk-app] #2420 pattern #2402
[mk-quant] #2421 internal_lib!Chap28.MCSSSpec.MCSSSpec.is_mcss_of.?_definition 2 #2420 #2419
[attach-var-names] #2421 (|m!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #2422 => #2401 #2421
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #729
[mk-app] #2423 = #729 #738
[instance] 0 #2423
[attach-enode] #2423 0
[end-of-instance]
[mk-app] #2423 <= #191 #676
[mk-app] #2424 not #2423
[inst-discovered] theory-solving 0 arith# ; #2404
[mk-app] #2425 = #2404 #2424
[instance] 0 #2425
[attach-enode] #2425 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2425 <= #2386 #341
[inst-discovered] theory-solving 0 arith# ; #2423
[mk-app] #2426 = #2423 #2425
[instance] 0 #2426
[attach-enode] #2426 0
[end-of-instance]
[mk-app] #2426 not #2425
[attach-meaning] #370 arith (- 1)
[mk-app] #2427 * #370 #2403
[mk-app] #2428 + #191 #2427
[mk-app] #2429 <= #2428 #341
[inst-discovered] theory-solving 0 arith# ; #2406
[mk-app] #2430 = #2406 #2429
[instance] 0 #2430
[attach-enode] #2430 0
[end-of-instance]
[mk-app] #2430 and #738 #2426 #2429
[mk-app] #2431 and #2323 #190 #738 #2426 #2429 #2409
[mk-app] #2432 and #2367 #2430 #2409
[inst-discovered] theory-solving 0 basic# ; #2432
[mk-app] #2433 = #2432 #2431
[instance] 0 #2433
[attach-enode] #2433 0
[end-of-instance]
[mk-quant] #2432 user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_18 2 #2412 #2431
[attach-var-names] #2432 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #729
[mk-app] #2423 = #729 #738
[instance] 0 #2423
[attach-enode] #2423 0
[end-of-instance]
[mk-app] #2423 <= #191 #676
[mk-app] #2424 not #2423
[inst-discovered] theory-solving 0 arith# ; #2404
[mk-app] #2430 = #2404 #2424
[instance] 0 #2430
[attach-enode] #2430 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #2423
[mk-app] #2430 = #2423 #2425
[instance] 0 #2430
[attach-enode] #2430 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #2406
[mk-app] #2430 = #2406 #2429
[instance] 0 #2430
[attach-enode] #2430 0
[end-of-instance]
[mk-app] #2430 and #738 #2426 #2429
[attach-meaning] #370 arith (- 1)
[mk-app] #2433 + #1679 #2408
[attach-meaning] #370 arith (- 1)
[mk-app] #2434 * #370 #2408
[mk-app] #2435 + #666 #2434
[mk-app] #2433 >= #2435 #341
[inst-discovered] theory-solving 0 arith# ; #2414
[mk-app] #2436 = #2414 #2433
[instance] 0 #2436
[attach-enode] #2436 0
[end-of-instance]
[mk-app] #2436 not #2430
[mk-app] #2437 or #2436 #2433
[mk-app] #2438 => #2430 #2433
[inst-discovered] theory-solving 0 basic# ; #2438
[mk-app] #2439 = #2438 #2437
[instance] 0 #2439
[attach-enode] #2439 0
[end-of-instance]
[mk-app] #2438 or #2394 #2436 #2433
[mk-app] #2439 => #2367 #2437
[inst-discovered] theory-solving 0 basic# ; #2439
[mk-app] #2440 = #2439 #2438
[instance] 0 #2440
[attach-enode] #2440 0
[end-of-instance]
[mk-quant] #2437 user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_19 2 #2412 #2438
[attach-var-names] #2437 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[mk-app] #2423 and #2432 #2437
[mk-app] #2424 = #2402 #2423
[mk-quant] #2439 internal_lib!Chap28.MCSSSpec.MCSSSpec.is_mcss_of.?_definition 2 #2420 #2424
[attach-var-names] #2439 (|m!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #2440 not #2401
[mk-app] #2441 or #2440 #2439
[mk-app] #2442 => #2401 #2439
[inst-discovered] theory-solving 0 basic# ; #2442
[mk-app] #2443 = #2442 #2441
[instance] 0 #2443
[attach-enode] #2443 0
[end-of-instance]
[mk-app] #2442 req%lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.MaxContigSubSumOptTrait.max_contig_sub_sum_opt. #65 #66 #34
[mk-app] #2443 %%global_location_label%%6
[attach-meaning] #275 arith 32
[mk-app] #2444 vstd!view.View.view.? #125 #1168 #1324
[mk-app] #2445 lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.? #2444
[mk-app] #2446 => #2443 #2445
[mk-app] #2447 %%global_location_label%%7
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #2448 vstd!seq.Seq.len.? #125 #1167 #2444
[mk-app] #2449 uHi #274
[mk-app] #2450 - #2449 #296
[mk-app] #2451 < #2448 #2450
[mk-app] #2452 => #2447 #2451
[mk-app] #2453 and #2446 #2452
[mk-app] #2454 = #2442 #2453
[mk-app] #2455 pattern #2442
[mk-quant] #2456 internal_req__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.MaxContigSubSumOptTrait.max_contig_sub_sum_opt._definition 3 #2455 #2454
[attach-var-names] #2456 (|a!| ; |Poly|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2457 not #2443
[mk-app] #2458 or #2457 #2445
[inst-discovered] theory-solving 0 basic# ; #2446
[mk-app] #2459 = #2446 #2458
[instance] 0 #2459
[attach-enode] #2459 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2459 * #370 #296
[mk-app] #2460 + #2449 #2459
[inst-discovered] theory-solving 0 arith# ; #2450
[mk-app] #2461 = #2450 #2460
[instance] 0 #2461
[attach-enode] #2461 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #2459
[mk-app] #2461 = #2459 #370
[instance] 0 #2461
[attach-enode] #2461 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2461 + #370 #2449
[mk-app] #2462 + #2449 #370
[inst-discovered] theory-solving 0 arith# ; #2462
[mk-app] #2463 = #2462 #2461
[instance] 0 #2463
[attach-enode] #2463 0
[end-of-instance]
[mk-app] #2462 <= #2461 #2448
[mk-app] #2463 not #2462
[mk-app] #2464 < #2448 #2461
[inst-discovered] theory-solving 0 arith# ; #2464
[mk-app] #2465 = #2464 #2463
[instance] 0 #2465
[attach-enode] #2465 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2464 * #370 #2448
[mk-app] #2465 + #2464 #2449
[attach-meaning] #370 arith (- 1)
[mk-app] #2466 * #370 #2449
[mk-app] #2467 + #2448 #2466
[attach-meaning] #370 arith (- 1)
[mk-app] #2464 >= #2467 #370
[inst-discovered] theory-solving 0 arith# ; #2462
[mk-app] #2465 = #2462 #2464
[instance] 0 #2465
[attach-enode] #2465 0
[end-of-instance]
[mk-app] #2465 not #2464
[mk-app] #2468 not #2447
[mk-app] #2469 or #2468 #2465
[mk-app] #2470 => #2447 #2465
[inst-discovered] theory-solving 0 basic# ; #2470
[mk-app] #2471 = #2470 #2469
[instance] 0 #2471
[attach-enode] #2471 0
[end-of-instance]
[mk-app] #2470 and #2458 #2469
[mk-app] #2471 = #2442 #2470
[mk-quant] #2472 internal_req__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.MaxContigSubSumOptTrait.max_contig_sub_sum_opt._definition 3 #2455 #2471
[attach-var-names] #2472 (|a!| ; |Poly|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2461 ens%lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.MaxContigSubSumOptTrait.max_contig_sub_sum_opt. #1004 #268 #33 #34
[attach-meaning] #275 arith 32
[mk-app] #2459 TYPE%core!option.Option. #125 #1167
[mk-app] #2460 has_type #34 #2459
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #2462 vstd!view.View.view.? #125 #1168 #2260
[mk-app] #2463 vstd!seq.Seq.len.? #125 #1167 #2462
[mk-app] #2473 = #2463 #341
[mk-app] #2474 => #2473 #1265
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #2475 > #2463 #341
[mk-app] #2476 => #2475 #1278
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #2477 core!option.Option./Some/0 #125 #1167 #1210
[mk-app] #2478 lib!Chap28.MCSSSpec.MCSSSpec.is_mcss_of.? #2462 #2477
[mk-app] #2479 => #1278 #2478
[mk-app] #2480 and #2460 #2474 #2476 #2479
[mk-app] #2481 = #2461 #2480
[mk-app] #2482 pattern #2461
[mk-quant] #2483 internal_ens__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.MaxContigSubSumOptTrait.max_contig_sub_sum_opt._definition 4 #2482 #2481
[attach-var-names] #2483 (|mcss!| ; |Poly|) (|a!| ; |Poly|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[inst-discovered] theory-solving 0 datatype# ; #1265
[mk-app] #2484 = #1265 #1272
[instance] 0 #2484
[attach-enode] #2484 0
[end-of-instance]
[mk-app] #2484 not #2473
[mk-app] #2485 or #2484 #1272
[mk-app] #2486 => #2473 #1272
[inst-discovered] theory-solving 0 basic# ; #2486
[mk-app] #2487 = #2486 #2485
[instance] 0 #2487
[attach-enode] #2487 0
[end-of-instance]
[mk-app] #2486 <= #2463 #341
[mk-app] #2487 not #2486
[inst-discovered] theory-solving 0 arith# ; #2475
[mk-app] #2488 = #2475 #2487
[instance] 0 #2488
[attach-enode] #2488 0
[end-of-instance]
[inst-discovered] theory-solving 0 datatype# ; #1278
[mk-app] #2488 = #1278 #1286
[instance] 0 #2488
[attach-enode] #2488 0
[end-of-instance]
[mk-app] #2488 or #2486 #1286
[mk-app] #2489 => #2487 #1286
[inst-discovered] theory-solving 0 basic# ; #2489
[mk-app] #2490 = #2489 #2488
[instance] 0 #2490
[attach-enode] #2490 0
[end-of-instance]
[mk-app] #2489 not #1286
[mk-app] #2490 or #2489 #2478
[mk-app] #2491 => #1286 #2478
[inst-discovered] theory-solving 0 basic# ; #2491
[mk-app] #2492 = #2491 #2490
[instance] 0 #2492
[attach-enode] #2492 0
[end-of-instance]
[mk-app] #2491 and #2460 #2485 #2488 #2490
[mk-app] #2492 = #2461 #2491
[mk-quant] #2493 internal_ens__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.MaxContigSubSumOptTrait.max_contig_sub_sum_opt._definition 4 #2482 #2492
[attach-var-names] #2493 (|mcss!| ; |Poly|) (|a!| ; |Poly|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2487 fuel_bool_default #822
[mk-app] #2494 fuel_bool #822
[mk-app] #2495 REF #65
[mk-app] #2496 vstd!view.View.view.? #2495 #66 #34
[mk-app] #2497 = #2496 #1803
[mk-app] #2498 => #2293 #2497
[mk-app] #2499 pattern #2496
[mk-quant] #2500 internal_vstd!view.View.view.?_definition 3 #2499 #2498
[attach-var-names] #2500 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2501 => #2494 #2500
[mk-app] #2502 not #2293
[mk-app] #2503 or #2502 #2497
[inst-discovered] theory-solving 0 basic# ; #2498
[mk-app] #2504 = #2498 #2503
[instance] 0 #2504
[attach-enode] #2504 0
[end-of-instance]
[mk-quant] #2504 internal_vstd!view.View.view.?_definition 3 #2499 #2503
[attach-var-names] #2504 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2505 not #2494
[mk-app] #2506 or #2505 #2504
[mk-app] #2507 => #2494 #2504
[inst-discovered] theory-solving 0 basic# ; #2507
[mk-app] #2508 = #2507 #2506
[instance] 0 #2508
[attach-enode] #2508 0
[end-of-instance]
[mk-app] #2507 fuel_bool_default #823
[mk-app] #2508 fuel_bool #823
[mk-app] #2509 BOX #125 #1147 #65
[mk-app] #2510 vstd!view.View.view.? #2509 #66 #34
[mk-app] #2511 = #2510 #1803
[mk-app] #2512 => #2293 #2511
[mk-app] #2513 pattern #2510
[mk-quant] #2514 internal_vstd!view.View.view.?_definition 3 #2513 #2512
[attach-var-names] #2514 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2515 => #2508 #2514
[mk-app] #2516 or #2502 #2511
[inst-discovered] theory-solving 0 basic# ; #2512
[mk-app] #2517 = #2512 #2516
[instance] 0 #2517
[attach-enode] #2517 0
[end-of-instance]
[mk-quant] #2517 internal_vstd!view.View.view.?_definition 3 #2513 #2516
[attach-var-names] #2517 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2518 not #2508
[mk-app] #2519 or #2518 #2517
[mk-app] #2520 => #2508 #2517
[inst-discovered] theory-solving 0 basic# ; #2520
[mk-app] #2521 = #2520 #2519
[instance] 0 #2521
[attach-enode] #2521 0
[end-of-instance]
[mk-app] #2520 fuel_bool_default #824
[mk-app] #2521 fuel_bool #824
[mk-app] #2522 RC #125 #1147 #65
[mk-app] #2523 vstd!view.View.view.? #2522 #66 #34
[mk-app] #2524 = #2523 #1803
[mk-app] #2525 => #2294 #2524
[mk-app] #2526 pattern #2523
[mk-quant] #2527 internal_vstd!view.View.view.?_definition 3 #2526 #2525
[attach-var-names] #2527 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2528 => #2521 #2527
[mk-app] #2529 or #2302 #2524
[inst-discovered] theory-solving 0 basic# ; #2525
[mk-app] #2530 = #2525 #2529
[instance] 0 #2530
[attach-enode] #2530 0
[end-of-instance]
[mk-quant] #2530 internal_vstd!view.View.view.?_definition 3 #2526 #2529
[attach-var-names] #2530 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2531 not #2521
[mk-app] #2532 or #2531 #2530
[mk-app] #2533 => #2521 #2530
[inst-discovered] theory-solving 0 basic# ; #2533
[mk-app] #2534 = #2533 #2532
[instance] 0 #2534
[attach-enode] #2534 0
[end-of-instance]
[mk-app] #2533 fuel_bool_default #827
[mk-app] #2534 fuel_bool #827
[mk-app] #2535 vstd!view.View.view.? #125 #1345 #34
[mk-app] #2536 = #2535 #34
[mk-app] #2537 pattern #2535
[mk-quant] #2538 internal_vstd!view.View.view.?_definition 1 #2537 #2536
[attach-var-names] #2538 (|self!| ; |Poly|)
[mk-app] #2539 => #2534 #2538
[mk-app] #2540 not #2534
[mk-app] #2541 or #2540 #2538
[inst-discovered] theory-solving 0 basic# ; #2539
[mk-app] #2542 = #2539 #2541
[instance] 0 #2542
[attach-enode] #2542 0
[end-of-instance]
[mk-app] #2542 fuel_bool_default #831
[mk-app] #2543 %%lambda%%1 #1004 #268
[mk-app] #2544 %%apply%%1 #2543 #33 #34
[mk-app] #2545 vstd!view.View.view.? #1004 #268 #34
[mk-app] #2546 = #2544 #2545
[mk-app] #2547 pattern #2544
[mk-quant] #2548 k!3583 4 #2547 #2546
[attach-var-names] #2548 (|t$| ; |Poly|) (|_i$| ; |Poly|) (|%%hole%%1| ; |Type|) (|%%hole%%0| ; |Dcr|)
[mk-app] #2549 fuel_bool #831
[mk-app] #2550 vstd!view.View.view.? #125 #1297 #34
[mk-app] #2551 proj%%vstd!view.View./V #65 #66
[mk-app] #2552 %%lambda%%1 #65 #66
[mk-app] #2553 mk_fun #2552
[mk-app] #2554 Poly%fun%2. #2553
[mk-app] #2555 vstd!seq_lib.impl&%0.map.? #65 #66 #2551 #1804 #2243 #2554
[mk-app] #2556 = #2550 #2555
[mk-app] #2557 => #2294 #2556
[mk-app] #2558 pattern #2550
[mk-quant] #2559 internal_vstd!view.View.view.?_definition 3 #2558 #2557
[attach-var-names] #2559 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2560 => #2549 #2559
[mk-app] #2561 or #2302 #2556
[inst-discovered] theory-solving 0 basic# ; #2557
[mk-app] #2562 = #2557 #2561
[instance] 0 #2562
[attach-enode] #2562 0
[end-of-instance]
[mk-quant] #2562 internal_vstd!view.View.view.?_definition 3 #2558 #2561
[attach-var-names] #2562 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2563 not #2549
[mk-app] #2564 or #2563 #2562
[mk-app] #2565 => #2549 #2562
[inst-discovered] theory-solving 0 basic# ; #2565
[mk-app] #2566 = #2565 #2564
[instance] 0 #2566
[attach-enode] #2566 0
[end-of-instance]
[mk-app] #2565 tr_bound%vstd!view.View. #125 #1519
[mk-app] #2566 => #1438 #2565
[mk-app] #2567 pattern #2565
[mk-quant] #2568 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__impl&__2_trait_impl_definition 2 #2567 #2566
[attach-var-names] #2568 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2569 or #1445 #2565
[inst-discovered] theory-solving 0 basic# ; #2566
[mk-app] #2570 = #2566 #2569
[instance] 0 #2570
[attach-enode] #2570 0
[end-of-instance]
[mk-quant] #2570 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__impl&__2_trait_impl_definition 2 #2567 #2569
[attach-var-names] #2570 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2571 fuel_bool_default #838
[mk-app] #2572 fuel_bool #838
[mk-app] #2573 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #33 #34
[mk-app] #2574 I #341
[mk-app] #2575 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #33 #2574 #34
[mk-app] #2576 = #2573 #2575
[mk-app] #2577 pattern #2573
[mk-quant] #2578 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.?_definition 2 #2577 #2576
[attach-var-names] #2578 (|k!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #2579 => #2572 #2578
[mk-app] #2580 not #2572
[mk-app] #2581 or #2580 #2578
[inst-discovered] theory-solving 0 basic# ; #2579
[mk-app] #2582 = #2579 #2581
[instance] 0 #2582
[attach-enode] #2582 0
[end-of-instance]
[mk-app] #2582 fuel_bool_default #839
[mk-app] #2583 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_min_prefix_sum.? #44 #33 #2314
[mk-app] #2584 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_min_prefix_sum.? #44 #33 #2316
[mk-app] #2585 = #2583 #2584
[mk-app] #2586 pattern #2583
[mk-quant] #2587 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum._fuel_to_zero_definition 3 #2586 #2585
[attach-var-names] #2587 (|fuel%| ; |Fuel|) (|k!| ; |Poly|) (|s!| ; |Poly|)
[attach-meaning] #275 arith 32
[mk-app] #2588 and #2348 #2323
[mk-app] #2589 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_min_prefix_sum.? #44 #33 #2325
[mk-app] #2590 < #676 #341
[attach-meaning] #2373 arith 2147483647
[mk-app] #2591 = #676 #341
[mk-app] #2592 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #44 #2574
[mk-app] #2593 Sub #676 #296
[mk-app] #2594 I #2593
[mk-app] #2595 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_min_prefix_sum.? #44 #2594 #2314
[mk-app] #2596 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #44 #33
[mk-app] #2597 < #2596 #2595
[mk-app] #2598 if #2597 #2596 #2595
[mk-app] #2599 if #2591 #2592 #2598
[mk-app] #2600 if #2590 #2373 #2599
[mk-app] #2601 = #2589 #2600
[mk-app] #2602 => #2588 #2601
[mk-app] #2603 pattern #2589
[mk-quant] #2604 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum._fuel_to_body_definition 3 #2603 #2602
[attach-var-names] #2604 (|fuel%| ; |Fuel|) (|k!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #2605 not #729
[inst-discovered] theory-solving 0 arith# ; #2590
[mk-app] #2606 = #2590 #2605
[instance] 0 #2606
[attach-enode] #2606 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #729
[mk-app] #2606 = #729 #738
[instance] 0 #2606
[attach-enode] #2606 0
[end-of-instance]
[mk-app] #2606 not #738
[mk-app] #2607 <= #2595 #2596
[mk-app] #2608 not #2607
[inst-discovered] theory-solving 0 arith# ; #2597
[mk-app] #2609 = #2597 #2608
[instance] 0 #2609
[attach-enode] #2609 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2609 * #370 #2596
[mk-app] #2610 + #2595 #2609
[mk-app] #2611 <= #2610 #341
[inst-discovered] theory-solving 0 arith# ; #2607
[mk-app] #2612 = #2607 #2611
[instance] 0 #2612
[attach-enode] #2612 0
[end-of-instance]
[mk-app] #2612 not #2611
[mk-app] #2613 if #2611 #2595 #2596
[mk-app] #2614 if #2612 #2596 #2595
[inst-discovered] theory-solving 0 arith# ; #2614
[mk-app] #2615 = #2614 #2613
[instance] 0 #2615
[attach-enode] #2615 0
[end-of-instance]
[mk-app] #2614 if #2591 #2592 #2613
[mk-app] #2615 if #738 #2614 #2373
[mk-app] #2616 if #2606 #2373 #2614
[inst-discovered] theory-solving 0 arith# ; #2616
[mk-app] #2617 = #2616 #2615
[instance] 0 #2617
[attach-enode] #2617 0
[end-of-instance]
[mk-app] #2616 = #2589 #2615
[mk-app] #2617 not #2588
[mk-app] #2618 or #2617 #2616
[mk-app] #2619 => #2588 #2616
[inst-discovered] theory-solving 0 basic# ; #2619
[mk-app] #2620 = #2619 #2618
[instance] 0 #2620
[attach-enode] #2620 0
[end-of-instance]
[mk-quant] #2619 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum._fuel_to_body_definition 3 #2603 #2618
[attach-var-names] #2619 (|fuel%| ; |Fuel|) (|k!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #2606 fuel_bool #839
[attach-meaning] #275 arith 32
[mk-app] #2605 has_type #33 #1188
[mk-app] #2612 and #2605 #190
[mk-app] #2607 lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.? #33 #34
[mk-app] #2608 fuel_nat%lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.
[mk-app] #2620 succ #2608
[mk-app] #2621 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_min_prefix_sum.? #33 #34 #2620
[mk-app] #2622 = #2607 #2621
[mk-app] #2623 => #2612 #2622
[mk-app] #2624 pattern #2607
[mk-quant] #2625 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.?_definition 2 #2624 #2623
[attach-var-names] #2625 (|k!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #2626 => #2606 #2625
[mk-app] #2627 not #2612
[mk-app] #2628 or #2627 #2622
[inst-discovered] theory-solving 0 basic# ; #2623
[mk-app] #2629 = #2623 #2628
[instance] 0 #2629
[attach-enode] #2629 0
[end-of-instance]
[mk-quant] #2629 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.?_definition 2 #2624 #2628
[attach-var-names] #2629 (|k!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #2630 not #2606
[mk-app] #2631 or #2630 #2629
[mk-app] #2632 => #2606 #2629
[inst-discovered] theory-solving 0 basic# ; #2632
[mk-app] #2633 = #2632 #2631
[instance] 0 #2633
[attach-enode] #2633 0
[end-of-instance]
[mk-app] #2632 tr_bound%vstd!view.View. #1407 #46
[mk-app] #2633 => #1359 #2632
[mk-app] #2634 pattern #2632
[mk-quant] #2635 internal_vstd__view__impl&__0_trait_impl_definition 2 #2634 #2633
[attach-var-names] #2635 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2636 or #1365 #2632
[inst-discovered] theory-solving 0 basic# ; #2633
[mk-app] #2637 = #2633 #2636
[instance] 0 #2637
[attach-enode] #2637 0
[end-of-instance]
[mk-quant] #2637 internal_vstd__view__impl&__0_trait_impl_definition 2 #2634 #2636
[attach-var-names] #2637 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2638 tr_bound%vstd!view.View. #1423 #46
[mk-app] #2639 => #1359 #2638
[mk-app] #2640 pattern #2638
[mk-quant] #2641 internal_vstd__view__impl&__2_trait_impl_definition 2 #2640 #2639
[attach-var-names] #2641 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2642 or #1365 #2638
[inst-discovered] theory-solving 0 basic# ; #2639
[mk-app] #2643 = #2639 #2642
[instance] 0 #2643
[attach-enode] #2643 0
[end-of-instance]
[mk-quant] #2643 internal_vstd__view__impl&__2_trait_impl_definition 2 #2640 #2642
[attach-var-names] #2643 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2644 tr_bound%vstd!view.View. #1439 #46
[mk-app] #2645 => #1438 #2644
[mk-app] #2646 pattern #2644
[mk-quant] #2647 internal_vstd__view__impl&__4_trait_impl_definition 2 #2646 #2645
[attach-var-names] #2647 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2648 or #1445 #2644
[inst-discovered] theory-solving 0 basic# ; #2645
[mk-app] #2649 = #2645 #2648
[instance] 0 #2649
[attach-enode] #2649 0
[end-of-instance]
[mk-quant] #2649 internal_vstd__view__impl&__4_trait_impl_definition 2 #2646 #2648
[attach-var-names] #2649 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2650 tr_bound%vstd!view.View. #125 #1345
[mk-app] #2651 tr_bound%core!alloc.Allocator. #1407 #46
[mk-app] #2652 => #1368 #2651
[mk-app] #2653 pattern #2651
[mk-quant] #2654 internal_core__alloc__impl&__2_trait_impl_definition 2 #2653 #2652
[attach-var-names] #2654 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2655 not #1368
[mk-app] #2656 or #2655 #2651
[inst-discovered] theory-solving 0 basic# ; #2652
[mk-app] #2657 = #2652 #2656
[instance] 0 #2657
[attach-enode] #2657 0
[end-of-instance]
[mk-quant] #2657 internal_core__alloc__impl&__2_trait_impl_definition 2 #2653 #2656
[attach-var-names] #2657 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2658 and #1373 #1926 #1368
[mk-app] #2659 BOX #45 #46 #1004
[mk-app] #2660 tr_bound%core!alloc.Allocator. #2659 #268
[mk-app] #2661 => #2658 #2660
[mk-app] #2662 pattern #2660
[mk-quant] #2663 internal_alloc__boxed__impl&__49_trait_impl_definition 4 #2662 #2661
[attach-var-names] #2663 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2664 not #2658
[mk-app] #2665 or #2664 #2660
[inst-discovered] theory-solving 0 basic# ; #2661
[mk-app] #2666 = #2661 #2665
[instance] 0 #2666
[attach-enode] #2666 0
[end-of-instance]
[mk-quant] #2666 internal_alloc__boxed__impl&__49_trait_impl_definition 4 #2662 #2665
[attach-var-names] #2666 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2667 RC #45 #46 #1004
[mk-app] #2668 tr_bound%core!alloc.Allocator. #2667 #268
[mk-app] #2669 => #2658 #2668
[mk-app] #2670 pattern #2668
[mk-quant] #2671 internal_alloc__rc__impl&__115_trait_impl_definition 4 #2670 #2669
[attach-var-names] #2671 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2672 or #2664 #2668
[inst-discovered] theory-solving 0 basic# ; #2669
[mk-app] #2673 = #2669 #2672
[instance] 0 #2673
[attach-enode] #2673 0
[end-of-instance]
[mk-quant] #2673 internal_alloc__rc__impl&__115_trait_impl_definition 4 #2670 #2672
[attach-var-names] #2673 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2674 ARC #45 #46 #1004
[mk-app] #2675 tr_bound%core!alloc.Allocator. #2674 #268
[mk-app] #2676 => #2658 #2675
[mk-app] #2677 pattern #2675
[mk-quant] #2678 internal_alloc__sync__impl&__117_trait_impl_definition 4 #2677 #2676
[attach-var-names] #2678 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #2679 or #2664 #2675
[inst-discovered] theory-solving 0 basic# ; #2676
[mk-app] #2680 = #2676 #2679
[instance] 0 #2680
[attach-enode] #2680 0
[end-of-instance]
[mk-quant] #2680 internal_alloc__sync__impl&__117_trait_impl_definition 4 #2677 #2679
[attach-var-names] #2680 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-var] #2681 2
[mk-app] #2682 req%lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_snoc. #2681 #224 #127
[mk-app] #2683 %%global_location_label%%8
[mk-app] #2684 => #2683 #632
[mk-app] #2685 %%global_location_label%%9
[mk-app] #2686 < #224 #127
[mk-app] #2687 => #2685 #2686
[mk-app] #2688 %%global_location_label%%10
[attach-meaning] #275 arith 32
[mk-app] #2689 Poly%vstd!seq.Seq<i32.>. #2681
[mk-app] #2690 vstd!seq.Seq.len.? #125 #1167 #2689
[mk-app] #2691 <= #127 #2690
[mk-app] #2692 => #2688 #2691
[mk-app] #2693 and #2684 #2687 #2692
[mk-app] #2694 = #2682 #2693
[mk-app] #2695 pattern #2682
[mk-quant] #2696 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_snoc._definition 3 #2695 #2694
[attach-var-names] #2696 (|hi!| ; |Int|) (|lo!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #632
[mk-app] #2697 = #632 #637
[instance] 0 #2697
[attach-enode] #2697 0
[end-of-instance]
[mk-app] #2697 not #2683
[mk-app] #2698 or #2697 #637
[mk-app] #2699 => #2683 #637
[inst-discovered] theory-solving 0 basic# ; #2699
[mk-app] #2700 = #2699 #2698
[instance] 0 #2700
[attach-enode] #2700 0
[end-of-instance]
[mk-app] #2699 <= #127 #224
[mk-app] #2700 not #2699
[inst-discovered] theory-solving 0 arith# ; #2686
[mk-app] #2701 = #2686 #2700
[instance] 0 #2701
[attach-enode] #2701 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2701 + #127 #564
[mk-app] #2702 <= #2701 #341
[inst-discovered] theory-solving 0 arith# ; #2699
[mk-app] #2703 = #2699 #2702
[instance] 0 #2703
[attach-enode] #2703 0
[end-of-instance]
[mk-app] #2703 not #2702
[mk-app] #2704 not #2685
[mk-app] #2705 or #2704 #2703
[mk-app] #2706 => #2685 #2703
[inst-discovered] theory-solving 0 basic# ; #2706
[mk-app] #2707 = #2706 #2705
[instance] 0 #2707
[attach-enode] #2707 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2706 * #370 #2690
[mk-app] #2707 + #127 #2706
[mk-app] #2708 <= #2707 #341
[inst-discovered] theory-solving 0 arith# ; #2691
[mk-app] #2709 = #2691 #2708
[instance] 0 #2709
[attach-enode] #2709 0
[end-of-instance]
[mk-app] #2709 not #2688
[mk-app] #2710 or #2709 #2708
[mk-app] #2711 => #2688 #2708
[inst-discovered] theory-solving 0 basic# ; #2711
[mk-app] #2712 = #2711 #2710
[instance] 0 #2712
[attach-enode] #2712 0
[end-of-instance]
[mk-app] #2711 and #2698 #2705 #2710
[mk-app] #2712 = #2682 #2711
[mk-quant] #2713 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_snoc._definition 3 #2695 #2712
[attach-var-names] #2713 (|hi!| ; |Int|) (|lo!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #2699 ens%lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_snoc. #2681 #224 #127
[mk-app] #2700 I #224
[mk-app] #2714 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #2689 #2700 #170
[mk-app] #2715 Sub #127 #296
[mk-app] #2716 I #2715
[mk-app] #2717 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #2689 #2700 #2716
[attach-meaning] #275 arith 32
[mk-app] #2718 vstd!seq.Seq.index.? #125 #1167 #2689 #2716
[mk-app] #2719 %I #2718
[mk-app] #2720 Add #2717 #2719
[mk-app] #2721 = #2714 #2720
[mk-app] #2722 = #2699 #2721
[mk-app] #2723 pattern #2699
[mk-quant] #2724 internal_ens__lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_snoc._definition 3 #2723 #2722
[attach-var-names] #2724 (|hi!| ; |Int|) (|lo!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #2725 req%lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_via_prefix. #2681 #224 #127
[mk-app] #2726 %%global_location_label%%11
[attach-meaning] #275 arith 32
[mk-app] #2727 <= #224 #127
[mk-app] #2728 and #632 #2727
[mk-app] #2729 and #2728 #2691
[mk-app] #2730 => #2726 #2729
[mk-app] #2731 = #2725 #2730
[mk-app] #2732 pattern #2725
[mk-quant] #2733 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_via_prefix._definition 3 #2732 #2731
[attach-var-names] #2733 (|hi!| ; |Int|) (|lo!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #632
[mk-app] #2734 = #632 #637
[instance] 0 #2734
[attach-enode] #2734 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2734 * #370 #127
[mk-app] #2735 + #2734 #224
[attach-meaning] #370 arith (- 1)
[mk-app] #2734 >= #2701 #341
[inst-discovered] theory-solving 0 arith# ; #2727
[mk-app] #2735 = #2727 #2734
[instance] 0 #2735
[attach-enode] #2735 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #2691
[mk-app] #2735 = #2691 #2708
[instance] 0 #2735
[attach-enode] #2735 0
[end-of-instance]
[mk-app] #2735 and #637 #2734 #2708
[mk-app] #2736 not #2726
[mk-app] #2737 or #2736 #2735
[mk-app] #2738 => #2726 #2735
[inst-discovered] theory-solving 0 basic# ; #2738
[mk-app] #2739 = #2738 #2737
[instance] 0 #2739
[attach-enode] #2739 0
[end-of-instance]
[mk-app] #2738 = #2725 #2737
[mk-quant] #2739 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_via_prefix._definition 3 #2732 #2738
[attach-var-names] #2739 (|hi!| ; |Int|) (|lo!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #2740 ens%lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_via_prefix. #2681 #224 #127
[mk-app] #2741 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #2689 #170
[mk-app] #2742 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #2689 #2700
[mk-app] #2743 Sub #2741 #2742
[mk-app] #2744 = #2714 #2743
[mk-app] #2745 = #2740 #2744
[mk-app] #2746 pattern #2740
[mk-quant] #2747 internal_ens__lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_via_prefix._definition 3 #2746 #2745
[attach-var-names] #2747 (|hi!| ; |Int|) (|lo!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-var] #2748 1
[mk-app] #2749 req%lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_achieved. #2748 #127
[mk-app] #2750 %%global_location_label%%12
[mk-app] #2751 => #2750 #353
[mk-app] #2752 = #2749 #2751
[mk-app] #2753 pattern #2749
[mk-quant] #2754 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_achieved._definition 2 #2753 #2752
[attach-var-names] #2754 (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #2755 not #2750
[mk-app] #2756 or #2755 #353
[inst-discovered] theory-solving 0 basic# ; #2751
[mk-app] #2757 = #2751 #2756
[instance] 0 #2757
[attach-enode] #2757 0
[end-of-instance]
[mk-app] #2757 = #2749 #2756
[mk-quant] #2758 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_achieved._definition 2 #2753 #2757
[attach-var-names] #2758 (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #2759 ens%lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_achieved. #2748 #127
[mk-app] #2760 <= #191 #224
[mk-app] #2761 and #517 #2760
[mk-app] #2762 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #2689 #34
[mk-app] #2763 lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.? #2689 #2700
[mk-app] #2764 = #2762 #2763
[mk-app] #2765 and #2761 #2764
[mk-app] #2766 and #190 #2765
[mk-app] #2767 pattern #2762
[mk-quant] #2768 user_lib__Chap28__MCSSSpec__MCSSSpec__lemma_min_prefix_sum_achieved_20 1 #2767 #2766
[attach-var-names] #2768 (|j$| ; |Poly|)
[mk-app] #2769 = #2759 #2768
[mk-app] #2770 pattern #2759
[mk-quant] #2771 internal_ens__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_achieved._definition 2 #2770 #2769
[attach-var-names] #2771 (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[attach-meaning] #370 arith (- 1)
[mk-app] #2772 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #2772 = #517 #521
[instance] 0 #2772
[attach-enode] #2772 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2772 + #191 #564
[mk-app] #2773 <= #2772 #341
[inst-discovered] theory-solving 0 arith# ; #2760
[mk-app] #2774 = #2760 #2773
[instance] 0 #2774
[attach-enode] #2774 0
[end-of-instance]
[mk-app] #2774 and #190 #521 #2773 #2764
[mk-quant] #2775 user_lib__Chap28__MCSSSpec__MCSSSpec__lemma_min_prefix_sum_achieved_20 1 #2767 #2774
[attach-var-names] #2775 (|j$| ; |Poly|)
[mk-app] #2776 = #2759 #2775
[mk-quant] #2777 internal_ens__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_achieved._definition 2 #2770 #2776
[attach-var-names] #2777 (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #2778 req%lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_is_min. #2681 #224 #127
[mk-app] #2779 %%global_location_label%%13
[mk-app] #2780 <= #127 #224
[mk-app] #2781 and #344 #2780
[mk-app] #2782 => #2779 #2781
[mk-app] #2783 = #2778 #2782
[mk-app] #2784 pattern #2778
[mk-quant] #2785 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_is_min._definition 3 #2784 #2783
[attach-var-names] #2785 (|j!| ; |Int|) (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[attach-meaning] #370 arith (- 1)
[mk-app] #2786 * #370 #127
[inst-discovered] theory-solving 0 arith# ; #344
[mk-app] #2786 = #344 #353
[instance] 0 #2786
[attach-enode] #2786 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #2780
[mk-app] #2786 = #2780 #2702
[instance] 0 #2786
[attach-enode] #2786 0
[end-of-instance]
[mk-app] #2786 and #353 #2702
[mk-app] #2787 not #2779
[mk-app] #2788 or #2787 #2786
[mk-app] #2789 => #2779 #2786
[inst-discovered] theory-solving 0 basic# ; #2789
[mk-app] #2790 = #2789 #2788
[instance] 0 #2790
[attach-enode] #2790 0
[end-of-instance]
[mk-app] #2789 = #2778 #2788
[mk-quant] #2790 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_is_min._definition 3 #2784 #2789
[attach-var-names] #2790 (|j!| ; |Int|) (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #2791 ens%lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_is_min. #2681 #224 #127
[mk-app] #2792 <= #2763 #2741
[mk-app] #2793 = #2791 #2792
[mk-app] #2794 pattern #2791
[mk-quant] #2795 internal_ens__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_is_min._definition 3 #2794 #2793
[attach-var-names] #2795 (|j!| ; |Int|) (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[attach-meaning] #370 arith (- 1)
[mk-app] #2796 * #370 #2741
[mk-app] #2797 + #2796 #2763
[attach-meaning] #370 arith (- 1)
[mk-app] #2798 * #370 #2763
[mk-app] #2799 + #2741 #2798
[mk-app] #2796 >= #2799 #341
[inst-discovered] theory-solving 0 arith# ; #2792
[mk-app] #2797 = #2792 #2796
[instance] 0 #2797
[attach-enode] #2797 0
[end-of-instance]
[mk-app] #2797 = #2791 #2796
[mk-quant] #2800 internal_ens__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_is_min._definition 3 #2794 #2797
[attach-var-names] #2800 (|j!| ; |Int|) (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #2801 req%lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.lemma_prefix_opt_is_mcss. #2681 #224 #127
[mk-app] #2802 %%global_location_label%%14
[attach-meaning] #275 arith 32
[mk-app] #2803 = #127 #2690
[mk-app] #2804 => #2802 #2803
[mk-app] #2805 %%global_location_label%%15
[mk-app] #2806 > #127 #341
[mk-app] #2807 => #2805 #2806
[mk-app] #2808 %%global_location_label%%16
[mk-app] #2809 lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.? #2689
[mk-app] #2810 => #2808 #2809
[mk-app] #2811 %%global_location_label%%17
[mk-app] #2812 <= #296 #191
[mk-app] #2813 and #2812 #2760
[mk-var] #2814 3
[mk-app] #2815 Poly%vstd!seq.Seq<i32.>. #2814
[mk-app] #2816 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #2815 #34
[mk-app] #2817 Sub #191 #296
[mk-app] #2818 I #2817
[mk-app] #2819 lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.? #2815 #2818
[mk-app] #2820 Sub #2816 #2819
[mk-app] #2821 = #761 #2820
[mk-app] #2822 and #2813 #2821
[mk-app] #2823 and #190 #2822
[mk-app] #2824 pattern #2816
[mk-quant] #2825 user_lib__Chap28__MaxContigSubSumOptStEph__MaxContigSubSumOptStEph__lemma_prefix_opt_is_mcss_21 1 #2824 #2823
[attach-var-names] #2825 (|hi$| ; |Poly|)
[mk-app] #2826 => #2811 #2825
[mk-app] #2827 %%global_location_label%%18
[mk-app] #2828 >= #761 #2820
[mk-app] #2829 => #2813 #2828
[mk-app] #2830 => #190 #2829
[mk-quant] #2831 user_lib__Chap28__MaxContigSubSumOptStEph__MaxContigSubSumOptStEph__lemma_prefix_opt_is_mcss_22 1 #2824 #2830
[attach-var-names] #2831 (|hi$| ; |Poly|)
[mk-app] #2832 => #2827 #2831
[mk-app] #2833 and #2804 #2807 #2810 #2826 #2832
[mk-app] #2834 = #2801 #2833
[mk-app] #2835 pattern #2801
[mk-quant] #2836 internal_req__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.lemma_prefix_opt_is_mcss._definition 3 #2835 #2834
[attach-var-names] #2836 (|n!| ; |Int|) (|m!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #2837 not #2802
[mk-app] #2838 or #2837 #2803
[inst-discovered] theory-solving 0 basic# ; #2804
[mk-app] #2839 = #2804 #2838
[instance] 0 #2839
[attach-enode] #2839 0
[end-of-instance]
[inst-discovered] theory-solving 0 arith# ; #2806
[mk-app] #2839 = #2806 #651
[instance] 0 #2839
[attach-enode] #2839 0
[end-of-instance]
[mk-app] #2839 not #2805
[mk-app] #2840 or #2839 #651
[mk-app] #2841 => #2805 #651
[inst-discovered] theory-solving 0 basic# ; #2841
[mk-app] #2842 = #2841 #2840
[instance] 0 #2842
[attach-enode] #2842 0
[end-of-instance]
[mk-app] #2841 not #2808
[mk-app] #2842 or #2841 #2809
[inst-discovered] theory-solving 0 basic# ; #2810
[mk-app] #2843 = #2810 #2842
[instance] 0 #2843
[attach-enode] #2843 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2843 * #370 #191
[attach-meaning] #370 arith (- 1)
[mk-app] #2844 >= #191 #296
[inst-discovered] theory-solving 0 arith# ; #2812
[mk-app] #2843 = #2812 #2844
[instance] 0 #2843
[attach-enode] #2843 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #2760
[mk-app] #2843 = #2760 #2773
[instance] 0 #2843
[attach-enode] #2843 0
[end-of-instance]
[mk-app] #2843 and #2844 #2773
[mk-app] #2845 and #190 #2844 #2773 #2821
[mk-app] #2846 and #190 #2843 #2821
[inst-discovered] theory-solving 0 basic# ; #2846
[mk-app] #2847 = #2846 #2845
[instance] 0 #2847
[attach-enode] #2847 0
[end-of-instance]
[mk-quant] #2846 user_lib__Chap28__MaxContigSubSumOptStEph__MaxContigSubSumOptStEph__lemma_prefix_opt_is_mcss_21 1 #2824 #2845
[attach-var-names] #2846 (|hi$| ; |Poly|)
[mk-app] #2843 not #2811
[mk-app] #2847 or #2843 #2846
[mk-app] #2848 => #2811 #2846
[inst-discovered] theory-solving 0 basic# ; #2848
[mk-app] #2849 = #2848 #2847
[instance] 0 #2849
[attach-enode] #2849 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2848 * #370 #191
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #2812
[mk-app] #2848 = #2812 #2844
[instance] 0 #2848
[attach-enode] #2848 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[inst-discovered] theory-solving 0 arith# ; #2760
[mk-app] #2848 = #2760 #2773
[instance] 0 #2848
[attach-enode] #2848 0
[end-of-instance]
[mk-app] #2848 and #2844 #2773
[attach-meaning] #370 arith (- 1)
[mk-app] #2849 * #370 #2820
[mk-app] #2850 + #761 #2849
[mk-app] #2851 >= #2850 #341
[inst-discovered] theory-solving 0 arith# ; #2828
[mk-app] #2852 = #2828 #2851
[instance] 0 #2852
[attach-enode] #2852 0
[end-of-instance]
[mk-app] #2852 not #2848
[mk-app] #2853 or #2852 #2851
[mk-app] #2854 => #2848 #2851
[inst-discovered] theory-solving 0 basic# ; #2854
[mk-app] #2855 = #2854 #2853
[instance] 0 #2855
[attach-enode] #2855 0
[end-of-instance]
[mk-app] #2854 or #197 #2852 #2851
[mk-app] #2855 => #190 #2853
[inst-discovered] theory-solving 0 basic# ; #2855
[mk-app] #2856 = #2855 #2854
[instance] 0 #2856
[attach-enode] #2856 0
[end-of-instance]
[mk-quant] #2853 user_lib__Chap28__MaxContigSubSumOptStEph__MaxContigSubSumOptStEph__lemma_prefix_opt_is_mcss_22 1 #2824 #2854
[attach-var-names] #2853 (|hi$| ; |Poly|)
[mk-app] #2855 not #2827
[mk-app] #2856 or #2855 #2853
[mk-app] #2857 => #2827 #2853
[inst-discovered] theory-solving 0 basic# ; #2857
[mk-app] #2858 = #2857 #2856
[instance] 0 #2858
[attach-enode] #2858 0
[end-of-instance]
[mk-app] #2857 and #2838 #2840 #2842 #2847 #2856
[mk-app] #2858 = #2801 #2857
[mk-quant] #2859 internal_req__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.lemma_prefix_opt_is_mcss._definition 3 #2835 #2858
[attach-var-names] #2859 (|n!| ; |Int|) (|m!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #2860 ens%lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.lemma_prefix_opt_is_mcss. #2681 #224 #127
[mk-app] #2861 lib!Chap28.MCSSSpec.MCSSSpec.is_mcss_of.? #2689 #2700
[mk-app] #2862 = #2860 #2861
[mk-app] #2863 pattern #2860
[mk-quant] #2864 internal_ens__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.lemma_prefix_opt_is_mcss._definition 3 #2863 #2862
[attach-var-names] #2864 (|n!| ; |Int|) (|m!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[inst-discovered] theory-solving 0 basic# ; #780
[mk-app] #2865 = #780 #780
[instance] 0 #2865
[attach-enode] #2865 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #905
[mk-app] #2865 = #905 #905
[instance] 0 #2865
[attach-enode] #2865 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #924
[mk-app] #2865 = #924 #924
[instance] 0 #2865
[attach-enode] #2865 0
[end-of-instance]
[mk-app] #2865 not #1
[inst-discovered] theory-solving 0 basic# ; #2865
[mk-app] #2866 = #2865 #2
[instance] 0 #2866
[attach-enode] #2866 0
[end-of-instance]
[mk-app] #2865 or #2 #960
[inst-discovered] theory-solving 0 basic# ; #2865
[mk-app] #2866 = #2865 #960
[instance] 0 #2866
[attach-enode] #2866 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1655
[mk-app] #962 = #1655 #1655
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1706
[mk-app] #962 = #1706 #1706
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1728
[mk-app] #962 = #1728 #1728
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1779
[mk-app] #962 = #1779 #1779
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1800
[mk-app] #962 = #1800 #1800
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1992
[mk-app] #962 = #1992 #1992
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2004
[mk-app] #962 = #2004 #2004
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2017
[mk-app] #962 = #2017 #2017
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2485
[mk-app] #962 = #2485 #2485
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2488
[mk-app] #962 = #2488 #2488
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2485
[mk-app] #962 = #2485 #2485
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2488
[mk-app] #962 = #2488 #2488
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2017
[mk-app] #962 = #2017 #2017
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2004
[mk-app] #962 = #2004 #2004
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1992
[mk-app] #962 = #1992 #1992
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1800
[mk-app] #962 = #1800 #1800
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1779
[mk-app] #962 = #1779 #1779
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1728
[mk-app] #962 = #1728 #1728
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1706
[mk-app] #962 = #1706 #1706
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1655
[mk-app] #962 = #1655 #1655
[instance] 0 #962
[attach-enode] #962 0
[end-of-instance]
[mk-app] #962 and #920 #942 #943 #944 #945 #946 #883 #947 #948 #949 #950 #951 #901 #952 #953 #954 #955 #956 #932 #957 #958 #959
[mk-app] #963 and #1 #920 #942 #943 #944 #945 #946 #883 #947 #948 #949 #950 #951 #901 #952 #953 #954 #955 #956 #932 #957 #958 #959
[inst-discovered] theory-solving 0 basic# ; #963
[mk-app] #2865 = #963 #962
[instance] 0 #2865
[attach-enode] #2865 0
[end-of-instance]
[mk-app] #963 not #1
[inst-discovered] theory-solving 0 basic# ; #963
[mk-app] #2865 = #963 #2
[instance] 0 #2865
[attach-enode] #2865 0
[end-of-instance]
[mk-app] #963 and #926 #1
[inst-discovered] theory-solving 0 basic# ; #963
[mk-app] #2865 = #963 #926
[instance] 0 #2865
[attach-enode] #2865 0
[end-of-instance]
[mk-app] #963 or #2 #926
[inst-discovered] theory-solving 0 basic# ; #963
[mk-app] #2865 = #963 #926
[instance] 0 #2865
[attach-enode] #2865 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #924
[mk-app] #930 = #924 #924
[instance] 0 #930
[attach-enode] #930 0
[end-of-instance]
[mk-app] #930 not #1
[inst-discovered] theory-solving 0 basic# ; #930
[mk-app] #931 = #930 #2
[instance] 0 #931
[attach-enode] #931 0
[end-of-instance]
[mk-app] #930 or #2 #916
[inst-discovered] theory-solving 0 basic# ; #930
[mk-app] #931 = #930 #916
[instance] 0 #931
[attach-enode] #931 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #905
[mk-app] #918 = #905 #905
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #780
[mk-app] #918 = #780 #780
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #780
[mk-app] #918 = #780 #780
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #905
[mk-app] #918 = #905 #905
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #924
[mk-app] #918 = #924 #924
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1655
[mk-app] #918 = #1655 #1655
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1706
[mk-app] #918 = #1706 #1706
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1728
[mk-app] #918 = #1728 #1728
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1779
[mk-app] #918 = #1779 #1779
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1800
[mk-app] #918 = #1800 #1800
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1992
[mk-app] #918 = #1992 #1992
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2004
[mk-app] #918 = #2004 #2004
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2017
[mk-app] #918 = #2017 #2017
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2485
[mk-app] #918 = #2485 #2485
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2488
[mk-app] #918 = #2488 #2488
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #780
[mk-app] #918 = #780 #780
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #905
[mk-app] #918 = #905 #905
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #924
[mk-app] #918 = #924 #924
[instance] 0 #918
[attach-enode] #918 0
[end-of-instance]
[mk-app] #918 T%0!skolem_internal_crate__fun__1_constructor_inner_definition!0 #161 #66 #972
[mk-app] #919 has_type #918 #972
[mk-app] #930 not #919
[mk-app] #931 %%apply%%0 #161 #918
[mk-app] #963 has_type #931 #66
[mk-app] #2865 or #930 #963
[mk-app] #2866 not #2865
[mk-app] #2867 or #2866 #993
[mk-quant] #2868 internal_crate__fun__1_constructor_definition 5 #995 #2867
[attach-var-names] #2868 (|x| ; |%%Function%%|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #2869 not #1030
[mk-app] #2870 not #1031
[mk-app] #2871 T%0!skolem_internal_crate__fun__1_inner_ext_equal_definition!1 #34 #33 #1028 #972 #1027
[mk-app] #2872 has_type #2871 #1027
[mk-app] #2873 not #2872
[mk-app] #2874 %%apply%%0 #1036 #2871
[mk-app] #2875 %%apply%%0 #975 #2871
[mk-app] #2876 ext_eq #1028 #972 #2874 #2875
[mk-app] #2877 or #2873 #2876
[mk-app] #2878 not #2877
[mk-app] #2879 or #2869 #2870 #2878
[mk-app] #2880 or #2879 #1043
[mk-quant] #2881 internal_crate__fun__1_ext_equal_definition 7 #1045 #2880
[attach-var-names] #2881 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #2882 or #2869 #2870 #2878 #1043
[inst-discovered] theory-solving 0 basic# ; #2882
[mk-app] #2883 = #2882 #2882
[instance] 0 #2883
[attach-enode] #2883 0
[end-of-instance]
[mk-quant] #2883 internal_crate__fun__1_ext_equal_definition 7 #1045 #2882
[attach-var-names] #2883 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #2879 T%1!skolem_internal_crate__fun__2_constructor_inner_definition!2 #161 #66 #972 #1027
[mk-app] #2880 T%0!skolem_internal_crate__fun__2_constructor_inner_definition!3 #161 #66 #972 #1027
[mk-app] #2881 has_type #2880 #1027
[mk-app] #2884 has_type #2879 #972
[mk-app] #2885 and #2881 #2884
[mk-app] #2886 not #2885
[mk-app] #2887 %%apply%%1 #161 #2880 #2879
[mk-app] #2888 has_type #2887 #66
[mk-app] #2889 or #2886 #2888
[mk-app] #2890 not #2889
[mk-app] #2891 or #2890 #1081
[mk-quant] #2892 internal_crate__fun__2_constructor_definition 7 #1083 #2891
[attach-var-names] #2892 (|x| ; |%%Function%%|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #2893 not #1114
[mk-app] #2894 not #1115
[mk-app] #2895 T%1!skolem_internal_crate__fun__2_inner_ext_equal_definition!4 #34 #33 #1028 #972 #1027 #1070
[mk-app] #2896 T%0!skolem_internal_crate__fun__2_inner_ext_equal_definition!5 #34 #33 #1028 #972 #1027 #1070
[mk-app] #2897 has_type #2896 #1070
[mk-app] #2898 has_type #2895 #1027
[mk-app] #2899 and #2897 #2898
[mk-app] #2900 not #2899
[mk-app] #2901 %Poly%fun%2. #33
[mk-app] #2902 %%apply%%1 #2901 #2896 #2895
[mk-app] #2903 %%apply%%1 #1061 #2896 #2895
[mk-app] #2904 ext_eq #1028 #972 #2902 #2903
[mk-app] #2905 or #2900 #2904
[mk-app] #2906 not #2905
[mk-app] #2907 or #2893 #2894 #2906
[mk-app] #2908 or #2907 #1130
[mk-quant] #2909 internal_crate__fun__2_ext_equal_definition 9 #1132 #2908
[attach-var-names] #2909 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #2910 or #2893 #2894 #2906 #1130
[inst-discovered] theory-solving 0 basic# ; #2910
[mk-app] #2911 = #2910 #2910
[instance] 0 #2911
[attach-enode] #2911 0
[end-of-instance]
[mk-quant] #2911 internal_crate__fun__2_ext_equal_definition 9 #1132 #2910
[attach-var-names] #2911 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[inst-discovered] theory-solving 0 basic# ; #1655
[mk-app] #2907 = #1655 #1655
[instance] 0 #2907
[attach-enode] #2907 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1706
[mk-app] #2907 = #1706 #1706
[instance] 0 #2907
[attach-enode] #2907 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1728
[mk-app] #2907 = #1728 #1728
[instance] 0 #2907
[attach-enode] #2907 0
[end-of-instance]
[mk-app] #2907 not #1754
[mk-app] #2908 not #1756
[mk-app] #2909 i$!skolem_user_vstd__seq__axiom_seq_ext_equal_7!6 #34 #33 #268 #1004
[mk-app] #2912 has_type #2909 #189
[mk-app] #2913 not #2912
[mk-app] #2914 %I #2909
[mk-app] #2915 >= #2914 #341
[mk-app] #2916 + #2914 #1559
[mk-app] #2917 >= #2916 #341
[mk-app] #2918 not #2917
[mk-app] #2919 and #2915 #2918
[mk-app] #2920 not #2919
[mk-app] #2921 vstd!seq.Seq.index.? #1004 #268 #33 #2909
[mk-app] #2922 vstd!seq.Seq.index.? #1004 #268 #34 #2909
[mk-app] #2923 = #2921 #2922
[mk-app] #2924 or #2913 #2920 #2923
[mk-app] #2925 not #2924
[mk-app] #2926 or #2908 #2925
[mk-app] #2927 or #1754 #2926
[mk-app] #2928 or #2907 #1771
[mk-app] #2929 and #2928 #2927
[mk-app] #2930 or #1778 #1652 #2929
[mk-quant] #2931 user_vstd__seq__axiom_seq_ext_equal_8 4 #1768 #2930
[attach-var-names] #2931 (|s2!| ; |Poly|) (|s1!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2932 or #1780 #2931
[mk-app] #2933 + #1559 #2914
[inst-discovered] theory-solving 0 arith# ; #2916
[mk-app] #2934 = #2916 #2933
[instance] 0 #2934
[attach-enode] #2934 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2934 * #370 #2914
[mk-app] #2935 + #1548 #2934
[mk-app] #2936 <= #2935 #341
[mk-app] #2937 >= #2933 #341
[inst-discovered] theory-solving 0 arith# ; #2937
[mk-app] #2938 = #2937 #2936
[instance] 0 #2938
[attach-enode] #2938 0
[end-of-instance]
[mk-app] #2933 not #2936
[mk-app] #2937 and #2915 #2933
[mk-app] #2938 not #2937
[mk-app] #2939 or #2913 #2938 #2923
[mk-app] #2940 not #2939
[mk-app] #2941 or #2908 #2940
[mk-app] #2942 or #1754 #2908 #2940
[mk-app] #2943 or #1754 #2941
[inst-discovered] theory-solving 0 basic# ; #2943
[mk-app] #2944 = #2943 #2942
[instance] 0 #2944
[attach-enode] #2944 0
[end-of-instance]
[mk-app] #2943 and #2928 #2942
[mk-app] #2944 or #1778 #1652 #2943
[inst-discovered] theory-solving 0 basic# ; #2944
[mk-app] #2945 = #2944 #2944
[instance] 0 #2945
[attach-enode] #2945 0
[end-of-instance]
[mk-quant] #2945 user_vstd__seq__axiom_seq_ext_equal_8 4 #1768 #2944
[attach-var-names] #2945 (|s2!| ; |Poly|) (|s1!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2941 or #1780 #2945
[mk-app] #2927 not #1783
[mk-app] #2929 i$!skolem_user_vstd__seq__axiom_seq_ext_equal_deep_9!7 #34 #33 #268 #1004
[mk-app] #2930 has_type #2929 #189
[mk-app] #2931 not #2930
[mk-app] #2932 %I #2929
[mk-app] #2946 >= #2932 #341
[mk-app] #2947 + #2932 #1559
[mk-app] #2948 >= #2947 #341
[mk-app] #2949 not #2948
[mk-app] #2950 and #2946 #2949
[mk-app] #2951 not #2950
[mk-app] #2952 vstd!seq.Seq.index.? #1004 #268 #33 #2929
[mk-app] #2953 vstd!seq.Seq.index.? #1004 #268 #34 #2929
[mk-app] #2954 ext_eq #1 #268 #2952 #2953
[mk-app] #2955 or #2931 #2951 #2954
[mk-app] #2956 not #2955
[mk-app] #2957 or #2908 #2956
[mk-app] #2958 or #1783 #2957
[mk-app] #2959 or #2927 #1795
[mk-app] #2960 and #2959 #2958
[mk-app] #2961 or #1778 #1652 #2960
[mk-quant] #2962 user_vstd__seq__axiom_seq_ext_equal_deep_10 4 #1792 #2961
[attach-var-names] #2962 (|s2!| ; |Poly|) (|s1!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2963 or #1801 #2962
[mk-app] #2964 + #1559 #2932
[inst-discovered] theory-solving 0 arith# ; #2947
[mk-app] #2965 = #2947 #2964
[instance] 0 #2965
[attach-enode] #2965 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2965 * #370 #2932
[mk-app] #2966 + #1548 #2965
[mk-app] #2967 <= #2966 #341
[mk-app] #2968 >= #2964 #341
[inst-discovered] theory-solving 0 arith# ; #2968
[mk-app] #2969 = #2968 #2967
[instance] 0 #2969
[attach-enode] #2969 0
[end-of-instance]
[mk-app] #2964 not #2967
[mk-app] #2968 and #2946 #2964
[mk-app] #2969 not #2968
[mk-app] #2970 or #2931 #2969 #2954
[mk-app] #2971 not #2970
[mk-app] #2972 or #2908 #2971
[mk-app] #2973 or #1783 #2908 #2971
[mk-app] #2974 or #1783 #2972
[inst-discovered] theory-solving 0 basic# ; #2974
[mk-app] #2975 = #2974 #2973
[instance] 0 #2975
[attach-enode] #2975 0
[end-of-instance]
[mk-app] #2974 and #2959 #2973
[mk-app] #2975 or #1778 #1652 #2974
[inst-discovered] theory-solving 0 basic# ; #2975
[mk-app] #2976 = #2975 #2975
[instance] 0 #2976
[attach-enode] #2976 0
[end-of-instance]
[mk-quant] #2976 user_vstd__seq__axiom_seq_ext_equal_deep_10 4 #1792 #2975
[attach-var-names] #2976 (|s2!| ; |Poly|) (|s1!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #2972 or #1801 #2976
[inst-discovered] theory-solving 0 basic# ; #1992
[mk-app] #2958 = #1992 #1992
[instance] 0 #2958
[attach-enode] #2958 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2004
[mk-app] #2958 = #2004 #2004
[instance] 0 #2958
[attach-enode] #2958 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2017
[mk-app] #2958 = #2017 #2017
[instance] 0 #2958
[attach-enode] #2958 0
[end-of-instance]
[mk-app] #2958 not #2206
[mk-app] #2960 not #2208
[mk-app] #2961 not #2212
[mk-app] #2962 i$!skolem_user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphTrait__from_vec_16!8 #34 #33 #268 #1004 #984 #1003
[mk-app] #2963 has_type #2962 #189
[mk-app] #2977 not #2963
[mk-app] #2978 %I #2962
[mk-app] #2979 >= #2978 #341
[mk-app] #2980 * #370 #2211
[mk-app] #2981 + #2978 #2980
[mk-app] #2982 >= #2981 #341
[mk-app] #2983 not #2982
[mk-app] #2984 and #2979 #2983
[mk-app] #2985 not #2984
[mk-app] #2986 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.? #1003 #984 #1004 #268 #34 #2962
[mk-app] #2987 vstd!seq.Seq.index.? #1004 #268 #1956 #2962
[mk-app] #2988 = #2986 #2987
[mk-app] #2989 or #2977 #2985 #2988
[mk-app] #2990 not #2989
[mk-app] #2991 or #997 #2960 #2961 #2990
[mk-app] #2992 or #2206 #2991
[mk-app] #2993 or #2958 #2229
[mk-app] #2994 and #2993 #2992
[mk-quant] #2995 internal_ens__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.from_vec._definition 6 #2227 #2994
[attach-var-names] #2995 (|seq!| ; |Poly|) (|elts!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2996 + #2980 #2978
[inst-discovered] theory-solving 0 arith# ; #2981
[mk-app] #2997 = #2981 #2996
[instance] 0 #2997
[attach-enode] #2997 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #2997 * #370 #2978
[mk-app] #2998 + #2211 #2997
[mk-app] #2999 <= #2998 #341
[mk-app] #3000 >= #2996 #341
[inst-discovered] theory-solving 0 arith# ; #3000
[mk-app] #3001 = #3000 #2999
[instance] 0 #3001
[attach-enode] #3001 0
[end-of-instance]
[mk-app] #2996 not #2999
[mk-app] #3000 and #2979 #2996
[mk-app] #3001 not #3000
[mk-app] #3002 or #2977 #3001 #2988
[mk-app] #3003 not #3002
[mk-app] #3004 or #997 #2960 #2961 #3003
[mk-app] #3005 or #2206 #997 #2960 #2961 #3003
[mk-app] #3006 or #2206 #3004
[inst-discovered] theory-solving 0 basic# ; #3006
[mk-app] #3007 = #3006 #3005
[instance] 0 #3007
[attach-enode] #3007 0
[end-of-instance]
[mk-app] #3006 and #2993 #3005
[mk-quant] #3007 internal_ens__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.from_vec._definition 6 #2227 #3006
[attach-var-names] #3007 (|seq!| ; |Poly|) (|elts!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #2992 not #2366
[mk-app] #2994 hi$!skolem_user_lib__Chap28__MCSSSpec__MCSSSpec__sums_fit_i32_17!9 #34
[mk-app] #2995 lo$!skolem_user_lib__Chap28__MCSSSpec__MCSSSpec__sums_fit_i32_17!10 #34
[mk-app] #3004 has_type #2995 #189
[mk-app] #3008 has_type #2994 #189
[mk-app] #3009 and #3004 #3008
[mk-app] #3010 not #3009
[mk-app] #3011 %I #2995
[mk-app] #3012 >= #3011 #341
[mk-app] #3013 %I #2994
[mk-app] #3014 * #370 #3011
[mk-app] #3015 + #3013 #3014
[mk-app] #3016 >= #3015 #341
[mk-app] #3017 vstd!seq.Seq.len.? #125 #1167 #34
[mk-app] #3018 * #370 #3017
[mk-app] #3019 + #3013 #3018
[mk-app] #3020 <= #3019 #341
[mk-app] #3021 and #3012 #3016 #3020
[mk-app] #3022 not #3021
[mk-app] #3023 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #34 #2995 #2994
[mk-app] #3024 >= #3023 #317
[mk-app] #3025 <= #3023 #2373
[mk-app] #3026 and #3024 #3025
[mk-app] #3027 or #3010 #3022 #3026
[mk-app] #3028 not #3027
[mk-app] #3029 or #2366 #3028
[mk-app] #3030 or #2992 #2393
[mk-app] #3031 and #3030 #3029
[mk-quant] #3032 internal_lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.?_definition 1 #2381 #3031
[attach-var-names] #3032 (|s!| ; |Poly|)
[mk-app] #3033 or #2398 #3032
[mk-app] #3034 + #3014 #3013
[inst-discovered] theory-solving 0 arith# ; #3015
[mk-app] #3035 = #3015 #3034
[instance] 0 #3035
[attach-enode] #3035 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3035 * #370 #3013
[mk-app] #3036 + #3011 #3035
[mk-app] #3037 <= #3036 #341
[mk-app] #3038 >= #3034 #341
[inst-discovered] theory-solving 0 arith# ; #3038
[mk-app] #3039 = #3038 #3037
[instance] 0 #3039
[attach-enode] #3039 0
[end-of-instance]
[mk-app] #3034 and #3012 #3037 #3020
[mk-app] #3038 not #3034
[mk-app] #3039 or #3010 #3038 #3026
[mk-app] #3040 not #3039
[mk-app] #3041 or #2366 #3040
[mk-app] #3042 and #3030 #3041
[mk-quant] #3043 internal_lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.?_definition 1 #2381 #3042
[attach-var-names] #3043 (|s!| ; |Poly|)
[mk-app] #3044 or #2398 #3043
[mk-app] #3029 not #2402
[mk-app] #3031 hi$!skolem_user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_18!11 #34 #33
[mk-app] #3032 lo$!skolem_user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_18!12 #34 #33
[mk-app] #3033 has_type #3032 #189
[mk-app] #3045 has_type #3031 #189
[mk-app] #3046 %I #3032
[mk-app] #3047 >= #3046 #341
[mk-app] #3048 %I #3031
[mk-app] #3049 * #370 #3046
[mk-app] #3050 + #3048 #3049
[mk-app] #3051 <= #3050 #341
[mk-app] #3052 not #3051
[mk-app] #3053 vstd!seq.Seq.len.? #125 #1167 #33
[mk-app] #3054 * #370 #3053
[mk-app] #3055 + #3048 #3054
[mk-app] #3056 <= #3055 #341
[mk-app] #3057 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #33 #3032 #3031
[mk-app] #3058 = #3057 #191
[mk-app] #3059 and #3033 #3045 #3047 #3052 #3056 #3058
[mk-app] #3060 and #3059 #2437
[mk-app] #3061 not #2431
[mk-quant] #3062 user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_18 2 #2412 #3061
[attach-var-names] #3062 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[mk-app] #3063 hi$!skolem_user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_19!13 #34 #33
[mk-app] #3064 lo$!skolem_user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_19!14 #34 #33
[mk-app] #3065 has_type #3064 #189
[mk-app] #3066 has_type #3063 #189
[mk-app] #3067 and #3065 #3066
[mk-app] #3068 not #3067
[mk-app] #3069 %I #3064
[mk-app] #3070 >= #3069 #341
[mk-app] #3071 %I #3063
[mk-app] #3072 * #370 #3069
[mk-app] #3073 + #3071 #3072
[mk-app] #3074 <= #3073 #341
[mk-app] #3075 not #3074
[mk-app] #3076 + #3071 #3054
[mk-app] #3077 <= #3076 #341
[mk-app] #3078 and #3070 #3075 #3077
[mk-app] #3079 not #3078
[mk-app] #3080 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #33 #3064 #3063
[mk-app] #3081 * #370 #3080
[mk-app] #3082 + #191 #3081
[mk-app] #3083 >= #3082 #341
[mk-app] #3084 or #3068 #3079 #3083
[mk-app] #3085 not #3084
[mk-app] #3086 or #3062 #3085
[mk-app] #3087 or #2402 #3086
[mk-app] #3088 or #3029 #3060
[mk-app] #3089 and #3088 #3087
[mk-quant] #3090 internal_lib!Chap28.MCSSSpec.MCSSSpec.is_mcss_of.?_definition 2 #2420 #3089
[attach-var-names] #3090 (|m!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #3091 or #2440 #3090
[mk-app] #3092 + #3049 #3048
[inst-discovered] theory-solving 0 arith# ; #3050
[mk-app] #3093 = #3050 #3092
[instance] 0 #3093
[attach-enode] #3093 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3093 * #370 #3048
[mk-app] #3094 + #3046 #3093
[mk-app] #3095 >= #3094 #341
[mk-app] #3096 <= #3092 #341
[inst-discovered] theory-solving 0 arith# ; #3096
[mk-app] #3097 = #3096 #3095
[instance] 0 #3097
[attach-enode] #3097 0
[end-of-instance]
[mk-app] #3092 not #3095
[mk-app] #3096 and #3033 #3045 #3047 #3092 #3056 #3058
[mk-app] #3097 and #3033 #3045 #3047 #3092 #3056 #3058 #2437
[mk-app] #3098 and #3096 #2437
[inst-discovered] theory-solving 0 basic# ; #3098
[mk-app] #3099 = #3098 #3097
[instance] 0 #3099
[attach-enode] #3099 0
[end-of-instance]
[mk-app] #3098 or #3029 #3097
[mk-app] #3099 + #3072 #3071
[inst-discovered] theory-solving 0 arith# ; #3073
[mk-app] #3100 = #3073 #3099
[instance] 0 #3100
[attach-enode] #3100 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3100 * #370 #3071
[mk-app] #3101 + #3069 #3100
[mk-app] #3102 >= #3101 #341
[mk-app] #3103 <= #3099 #341
[inst-discovered] theory-solving 0 arith# ; #3103
[mk-app] #3104 = #3103 #3102
[instance] 0 #3104
[attach-enode] #3104 0
[end-of-instance]
[mk-app] #3099 not #3102
[mk-app] #3103 + #3054 #3071
[inst-discovered] theory-solving 0 arith# ; #3076
[mk-app] #3104 = #3076 #3103
[instance] 0 #3104
[attach-enode] #3104 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3104 + #3053 #3100
[mk-app] #3105 >= #3104 #341
[mk-app] #3106 <= #3103 #341
[inst-discovered] theory-solving 0 arith# ; #3106
[mk-app] #3107 = #3106 #3105
[instance] 0 #3107
[attach-enode] #3107 0
[end-of-instance]
[mk-app] #3103 and #3070 #3099 #3105
[mk-app] #3106 not #3103
[mk-app] #3107 or #3068 #3106 #3083
[mk-app] #3108 not #3107
[mk-app] #3109 or #2402 #3062 #3108
[inst-discovered] theory-solving 0 basic# ; #3109
[mk-app] #3110 = #3109 #3109
[instance] 0 #3110
[attach-enode] #3110 0
[end-of-instance]
[mk-app] #3110 and #3098 #3109
[mk-quant] #3111 internal_lib!Chap28.MCSSSpec.MCSSSpec.is_mcss_of.?_definition 2 #2420 #3110
[attach-var-names] #3111 (|m!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #3096 or #2440 #3111
[inst-discovered] theory-solving 0 basic# ; #2485
[mk-app] #3060 = #2485 #2485
[instance] 0 #3060
[attach-enode] #3060 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2488
[mk-app] #3060 = #2488 #2488
[instance] 0 #3060
[attach-enode] #3060 0
[end-of-instance]
[mk-app] #3060 not #2759
[mk-app] #3088 j$!skolem_user_lib__Chap28__MCSSSpec__MCSSSpec__lemma_min_prefix_sum_achieved_20!15 #127 #2748
[mk-app] #3086 has_type #3088 #189
[mk-app] #3087 %I #3088
[mk-app] #3089 >= #3087 #341
[mk-app] #3090 * #370 #127
[mk-app] #3091 + #3087 #3090
[mk-app] #3112 <= #3091 #341
[mk-app] #3113 Poly%vstd!seq.Seq<i32.>. #2748
[mk-app] #3114 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #3113 #3088
[mk-app] #3115 lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.? #3113 #170
[mk-app] #3116 = #3114 #3115
[mk-app] #3117 and #3086 #3089 #3112 #3116
[mk-app] #3118 not #2774
[mk-quant] #3119 user_lib__Chap28__MCSSSpec__MCSSSpec__lemma_min_prefix_sum_achieved_20 1 #2767 #3118
[attach-var-names] #3119 (|j$| ; |Poly|)
[mk-app] #3120 or #2759 #3119
[mk-app] #3121 or #3060 #3117
[mk-app] #3122 and #3121 #3120
[mk-quant] #3123 internal_ens__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_achieved._definition 2 #2770 #3122
[attach-var-names] #3123 (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #3124 + #3090 #3087
[inst-discovered] theory-solving 0 arith# ; #3091
[mk-app] #3125 = #3091 #3124
[instance] 0 #3125
[attach-enode] #3125 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3125 * #370 #3087
[mk-app] #3126 + #127 #3125
[mk-app] #3127 >= #3126 #341
[mk-app] #3128 <= #3124 #341
[inst-discovered] theory-solving 0 arith# ; #3128
[mk-app] #3129 = #3128 #3127
[instance] 0 #3129
[attach-enode] #3129 0
[end-of-instance]
[mk-app] #3124 and #3086 #3089 #3127 #3116
[mk-app] #3128 or #3060 #3124
[mk-app] #3129 and #3128 #3120
[mk-quant] #3130 internal_ens__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_achieved._definition 2 #2770 #3129
[attach-var-names] #3130 (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #3121 not #2801
[mk-app] #3122 hi$!skolem_user_lib__Chap28__MaxContigSubSumOptStEph__MaxContigSubSumOptStEph__lemma_prefix_opt_is_mcss_21!16 #127 #224 #2681
[mk-app] #3123 has_type #3122 #189
[mk-app] #3131 %I #3122
[mk-app] #3132 >= #3131 #296
[mk-app] #3133 + #3131 #3090
[mk-app] #3134 <= #3133 #341
[mk-app] #3135 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #2689 #3122
[mk-app] #3136 Sub #3131 #296
[mk-app] #3137 I #3136
[mk-app] #3138 lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.? #2689 #3137
[mk-app] #3139 Sub #3135 #3138
[mk-app] #3140 = #224 #3139
[mk-app] #3141 and #3123 #3132 #3134 #3140
[mk-app] #3142 or #2843 #3141
[mk-app] #3143 and #2838 #2840 #2842 #3142 #2856
[mk-app] #3144 not #2838
[mk-app] #3145 not #2840
[mk-app] #3146 not #2842
[mk-app] #3147 not #2845
[mk-quant] #3148 user_lib__Chap28__MaxContigSubSumOptStEph__MaxContigSubSumOptStEph__lemma_prefix_opt_is_mcss_21 1 #2824 #3147
[attach-var-names] #3148 (|hi$| ; |Poly|)
[mk-app] #3149 and #2811 #3148
[mk-app] #3150 hi$!skolem_user_lib__Chap28__MaxContigSubSumOptStEph__MaxContigSubSumOptStEph__lemma_prefix_opt_is_mcss_22!17 #127 #224 #2681
[mk-app] #3151 has_type #3150 #189
[mk-app] #3152 not #3151
[mk-app] #3153 %I #3150
[mk-app] #3154 >= #3153 #296
[mk-app] #3155 + #3153 #3090
[mk-app] #3156 <= #3155 #341
[mk-app] #3157 and #3154 #3156
[mk-app] #3158 not #3157
[mk-app] #3159 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #2689 #3150
[mk-app] #3160 Sub #3153 #296
[mk-app] #3161 I #3160
[mk-app] #3162 lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.? #2689 #3161
[mk-app] #3163 Sub #3159 #3162
[mk-app] #3164 * #370 #3163
[mk-app] #3165 + #224 #3164
[mk-app] #3166 >= #3165 #341
[mk-app] #3167 or #3152 #3158 #3166
[mk-app] #3168 not #3167
[mk-app] #3169 and #2827 #3168
[mk-app] #3170 or #3144 #3145 #3146 #3149 #3169
[mk-app] #3171 or #2801 #3170
[mk-app] #3172 or #3121 #3143
[mk-app] #3173 and #3172 #3171
[mk-quant] #3174 internal_req__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.lemma_prefix_opt_is_mcss._definition 3 #2835 #3173
[attach-var-names] #3174 (|n!| ; |Int|) (|m!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #3175 + #3090 #3131
[inst-discovered] theory-solving 0 arith# ; #3133
[mk-app] #3176 = #3133 #3175
[instance] 0 #3176
[attach-enode] #3176 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3176 * #370 #3131
[mk-app] #3177 + #127 #3176
[mk-app] #3178 >= #3177 #341
[mk-app] #3179 <= #3175 #341
[inst-discovered] theory-solving 0 arith# ; #3179
[mk-app] #3180 = #3179 #3178
[instance] 0 #3180
[attach-enode] #3180 0
[end-of-instance]
[mk-app] #3175 and #3123 #3132 #3178 #3140
[mk-app] #3179 or #2843 #3175
[mk-app] #3180 and #2838 #2840 #2842 #3179 #2856
[mk-app] #3181 or #3121 #3180
[mk-app] #3182 + #3090 #3153
[inst-discovered] theory-solving 0 arith# ; #3155
[mk-app] #3183 = #3155 #3182
[instance] 0 #3183
[attach-enode] #3183 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3183 * #370 #3153
[mk-app] #3184 + #127 #3183
[mk-app] #3185 >= #3184 #341
[mk-app] #3186 <= #3182 #341
[inst-discovered] theory-solving 0 arith# ; #3186
[mk-app] #3187 = #3186 #3185
[instance] 0 #3187
[attach-enode] #3187 0
[end-of-instance]
[mk-app] #3182 and #3154 #3185
[mk-app] #3186 not #3182
[mk-app] #3187 or #3152 #3186 #3166
[mk-app] #3188 not #3187
[mk-app] #3189 and #2827 #3188
[mk-app] #3190 or #2801 #3144 #3145 #3146 #3149 #3189
[inst-discovered] theory-solving 0 basic# ; #3190
[mk-app] #3191 = #3190 #3190
[instance] 0 #3191
[attach-enode] #3191 0
[end-of-instance]
[mk-app] #3191 and #3181 #3190
[mk-quant] #3192 internal_req__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.lemma_prefix_opt_is_mcss._definition 3 #2835 #3191
[attach-var-names] #3192 (|n!| ; |Int|) (|m!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #3072 not #68
[mk-app] #3073 or #3072 #1231
[mk-app] #3074 not #3073
[inst-discovered] theory-solving 0 basic# ; #70
[mk-app] #3075 = #70 #3074
[instance] 0 #3075
[attach-enode] #3075 0
[end-of-instance]
[mk-app] #3075 not #3074
[inst-discovered] theory-solving 0 basic# ; #3075
[mk-app] #3076 = #3075 #3073
[instance] 0 #3076
[attach-enode] #3076 0
[end-of-instance]
[mk-app] #3075 or #3072 #1231 #72
[mk-app] #3076 or #3073 #72
[inst-discovered] theory-solving 0 basic# ; #3076
[mk-app] #3077 = #3076 #3075
[instance] 0 #3077
[attach-enode] #3077 0
[end-of-instance]
[mk-quant] #3076 prelude_mut_ref_update_has_type 4 #74 #3075
[attach-var-names] #3076 (|arg| ; |Poly|) (|t| ; |Type|) (|d| ; |Dcr|) (|m| ; |Poly|)
[mk-app] #3073 not #150
[mk-app] #3074 not #158
[mk-app] #3077 or #3073 #3074
[mk-app] #3078 not #3077
[inst-discovered] theory-solving 0 basic# ; #159
[mk-app] #3079 = #159 #3078
[instance] 0 #3079
[attach-enode] #3079 0
[end-of-instance]
[mk-quant] #3079 prelude_as_type 2 #155 #3078
[attach-var-names] #3079 (|t| ; |Type|) (|x| ; |Poly|)
[mk-app] #3084 not #352
[mk-app] #3085 not #351
[mk-app] #2916 or #3084 #3085
[mk-app] #2917 not #2916
[inst-discovered] theory-solving 0 basic# ; #354
[mk-app] #2918 = #354 #2917
[instance] 0 #2918
[attach-enode] #2918 0
[end-of-instance]
[mk-quant] #2918 prelude_nat_clip 1 #348 #2917
[attach-var-names] #2918 (|i| ; |Int|)
[mk-app] #2919 or #350 #377
[mk-app] #2920 not #2919
[inst-discovered] theory-solving 0 basic# ; #380
[mk-app] #2924 = #380 #2920
[instance] 0 #2924
[attach-enode] #2924 0
[end-of-instance]
[mk-app] #2924 not #2920
[inst-discovered] theory-solving 0 basic# ; #2924
[mk-app] #2925 = #2924 #2919
[instance] 0 #2925
[attach-enode] #2925 0
[end-of-instance]
[mk-app] #2924 or #350 #377 #362
[mk-app] #2925 or #2919 #362
[inst-discovered] theory-solving 0 basic# ; #2925
[mk-app] #2926 = #2925 #2924
[instance] 0 #2926
[attach-enode] #2926 0
[end-of-instance]
[mk-app] #2925 not #369
[mk-app] #2926 not #2924
[mk-app] #2775 or #2925 #371 #2926
[mk-app] #2846 not #2775
[mk-app] #2432 and #369 #372 #2924
[inst-discovered] theory-solving 0 basic# ; #2432
[mk-app] #2947 = #2432 #2846
[instance] 0 #2947
[attach-enode] #2947 0
[end-of-instance]
[mk-quant] #2432 prelude_u_clip 2 #365 #2846
[attach-var-names] #2432 (|i| ; |Int|) (|bits| ; |Int|)
[mk-app] #2919 not #403
[mk-app] #2920 or #2919 #408
[mk-app] #2947 not #2920
[inst-discovered] theory-solving 0 basic# ; #411
[mk-app] #2948 = #411 #2947
[instance] 0 #2948
[attach-enode] #2948 0
[end-of-instance]
[mk-app] #2948 not #2947
[inst-discovered] theory-solving 0 basic# ; #2948
[mk-app] #2949 = #2948 #2920
[instance] 0 #2949
[attach-enode] #2949 0
[end-of-instance]
[mk-app] #2948 or #2919 #408 #389
[mk-app] #2949 or #2920 #389
[inst-discovered] theory-solving 0 basic# ; #2949
[mk-app] #2950 = #2949 #2948
[instance] 0 #2950
[attach-enode] #2950 0
[end-of-instance]
[mk-app] #2949 not #396
[mk-app] #2950 not #2948
[mk-app] #2951 or #2949 #399 #2950
[mk-app] #2955 not #2951
[mk-app] #2956 and #396 #402 #2948
[inst-discovered] theory-solving 0 basic# ; #2956
[mk-app] #2957 = #2956 #2955
[instance] 0 #2957
[attach-enode] #2957 0
[end-of-instance]
[mk-quant] #2956 prelude_i_clip 2 #392 #2955
[attach-var-names] #2956 (|i| ; |Int|) (|bits| ; |Int|)
[mk-app] #2920 not #435
[mk-app] #2947 not #398
[mk-app] #2957 or #2920 #2947
[mk-app] #3090 not #2957
[inst-discovered] theory-solving 0 basic# ; #434
[mk-app] #3155 = #434 #3090
[instance] 0 #3155
[attach-enode] #3155 0
[end-of-instance]
[mk-app] #3155 not #438
[mk-app] #3156 not #420
[mk-app] #3157 or #3155 #3156
[mk-app] #3158 not #3157
[inst-discovered] theory-solving 0 basic# ; #436
[mk-app] #3167 = #436 #3158
[instance] 0 #3167
[attach-enode] #3167 0
[end-of-instance]
[mk-app] #3167 or #3090 #3158
[mk-app] #3168 not #423
[mk-app] #3169 or #350 #3168
[mk-app] #3014 not #3169
[inst-discovered] theory-solving 0 basic# ; #439
[mk-app] #3015 = #439 #3014
[instance] 0 #3015
[attach-enode] #3015 0
[end-of-instance]
[mk-app] #3015 not #442
[mk-app] #3016 not #426
[mk-app] #3021 or #3015 #3016
[mk-app] #3022 not #3021
[inst-discovered] theory-solving 0 basic# ; #440
[mk-app] #3027 = #440 #3022
[instance] 0 #3027
[attach-enode] #3027 0
[end-of-instance]
[mk-app] #3027 or #3014 #3022
[mk-app] #3028 not #3027
[mk-app] #2980 or #3028 #429
[mk-app] #2981 not #3167
[mk-app] #2982 not #2980
[mk-app] #2983 or #2981 #2982
[mk-app] #2984 not #2983
[mk-app] #2985 and #3167 #2980
[inst-discovered] theory-solving 0 basic# ; #2985
[mk-app] #2989 = #2985 #2984
[instance] 0 #2989
[attach-enode] #2989 0
[end-of-instance]
[mk-quant] #2985 prelude_char_clip 1 #432 #2984
[attach-var-names] #2985 (|i| ; |Int|)
[mk-app] #2989 or #350 #377
[mk-app] #2990 not #2989
[inst-discovered] theory-solving 0 basic# ; #380
[mk-app] #2991 = #380 #2990
[instance] 0 #2991
[attach-enode] #2991 0
[end-of-instance]
[mk-app] #2991 = #2989 #447
[mk-app] #1085 not #2991
[mk-app] #1086 = #447 #2990
[inst-discovered] theory-solving 0 basic# ; #1086
[mk-app] #1087 = #1086 #1085
[instance] 0 #1087
[attach-enode] #1087 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1085
[mk-app] #1086 = #1085 #1085
[instance] 0 #1086
[attach-enode] #1086 0
[end-of-instance]
[mk-quant] #1086 prelude_u_inv 2 #449 #1085
[attach-var-names] #1086 (|i| ; |Int|) (|bits| ; |Int|)
[mk-app] #2990 or #2919 #408
[mk-app] #1087 not #2990
[inst-discovered] theory-solving 0 basic# ; #411
[mk-app] #1134 = #411 #1087
[instance] 0 #1134
[attach-enode] #1134 0
[end-of-instance]
[mk-app] #1134 = #2990 #451
[mk-app] #1135 not #1134
[mk-app] #1136 = #451 #1087
[inst-discovered] theory-solving 0 basic# ; #1136
[mk-app] #1047 = #1136 #1135
[instance] 0 #1047
[attach-enode] #1047 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1135
[mk-app] #1136 = #1135 #1135
[instance] 0 #1136
[attach-enode] #1136 0
[end-of-instance]
[mk-quant] #1136 prelude_i_inv 2 #455 #1135
[attach-var-names] #1136 (|i| ; |Int|) (|bits| ; |Int|)
[inst-discovered] theory-solving 0 basic# ; #439
[mk-app] #1087 = #439 #3014
[instance] 0 #1087
[attach-enode] #1087 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #440
[mk-app] #1087 = #440 #3022
[instance] 0 #1087
[attach-enode] #1087 0
[end-of-instance]
[mk-app] #1087 = #457 #3027
[mk-quant] #1047 prelude_char_inv 1 #461 #1087
[attach-var-names] #1047 (|i| ; |Int|)
[mk-app] #1048 not #637
[mk-app] #1049 or #1048 #350
[mk-app] #3091 not #1049
[inst-discovered] theory-solving 0 basic# ; #638
[mk-app] #3112 = #638 #3091
[instance] 0 #3112
[attach-enode] #3112 0
[end-of-instance]
[mk-app] #3112 not #3091
[inst-discovered] theory-solving 0 basic# ; #3112
[mk-app] #3117 = #3112 #1049
[instance] 0 #3117
[attach-enode] #3117 0
[end-of-instance]
[mk-app] #3091 or #1048 #350 #640
[mk-app] #3112 or #1049 #640
[inst-discovered] theory-solving 0 basic# ; #3112
[mk-app] #3117 = #3112 #3091
[instance] 0 #3117
[attach-enode] #3117 0
[end-of-instance]
[mk-quant] #1049 prelude_mul_nats 2 #568 #3091
[attach-var-names] #1049 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #3112 or #1048 #650
[mk-app] #3117 not #3112
[inst-discovered] theory-solving 0 basic# ; #652
[mk-app] #3133 = #652 #3117
[instance] 0 #3133
[attach-enode] #3133 0
[end-of-instance]
[mk-app] #3133 not #3117
[inst-discovered] theory-solving 0 basic# ; #3133
[mk-app] #3134 = #3133 #3112
[instance] 0 #3134
[attach-enode] #3134 0
[end-of-instance]
[mk-app] #3117 not #654
[mk-app] #3133 not #653
[mk-app] #3134 or #3117 #3133
[mk-app] #3141 not #3134
[inst-discovered] theory-solving 0 basic# ; #657
[mk-app] #3049 = #657 #3141
[instance] 0 #3049
[attach-enode] #3049 0
[end-of-instance]
[mk-app] #3049 or #1048 #650 #3141
[mk-app] #3050 or #3112 #3141
[inst-discovered] theory-solving 0 basic# ; #3050
[mk-app] #3051 = #3050 #3049
[instance] 0 #3051
[attach-enode] #3051 0
[end-of-instance]
[mk-quant] #3050 prelude_div_unsigned_in_bounds 2 #578 #3049
[attach-var-names] #3050 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #3112 or #1048 #650
[mk-app] #3051 not #3112
[inst-discovered] theory-solving 0 basic# ; #652
[mk-app] #3052 = #652 #3051
[instance] 0 #3052
[attach-enode] #3052 0
[end-of-instance]
[mk-app] #3052 not #3051
[inst-discovered] theory-solving 0 basic# ; #3052
[mk-app] #3059 = #3052 #3112
[instance] 0 #3059
[attach-enode] #3059 0
[end-of-instance]
[mk-app] #3051 not #667
[mk-app] #3052 or #3051 #671
[mk-app] #3059 not #3052
[inst-discovered] theory-solving 0 basic# ; #673
[mk-app] #998 = #673 #3059
[instance] 0 #998
[attach-enode] #998 0
[end-of-instance]
[mk-app] #998 or #1048 #650 #3059
[mk-app] #999 or #3112 #3059
[inst-discovered] theory-solving 0 basic# ; #999
[mk-app] #2847 = #999 #998
[instance] 0 #2847
[attach-enode] #2847 0
[end-of-instance]
[mk-quant] #999 prelude_mod_unsigned_in_bounds 2 #587 #998
[attach-var-names] #999 (|y| ; |Int|) (|x| ; |Int|)
[mk-app] #3112 not #668
[mk-app] #2847 not #677
[mk-app] #2857 or #3112 #2847
[mk-app] #2858 not #2857
[inst-discovered] theory-solving 0 basic# ; #678
[mk-app] #2859 = #678 #2858
[instance] 0 #2859
[attach-enode] #2859 0
[end-of-instance]
[mk-app] #2859 not #2858
[inst-discovered] theory-solving 0 basic# ; #2859
[mk-app] #2776 = #2859 #2857
[instance] 0 #2776
[attach-enode] #2776 0
[end-of-instance]
[mk-app] #2859 or #3112 #2847 #680
[mk-app] #2776 or #2857 #680
[inst-discovered] theory-solving 0 basic# ; #2776
[mk-app] #2777 = #2776 #2859
[instance] 0 #2777
[attach-enode] #2777 0
[end-of-instance]
[mk-quant] #2776 prelude_bit_xor_u_inv 3 #683 #2859
[attach-var-names] #2776 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #2857 not #688
[mk-app] #2858 not #689
[mk-app] #2777 or #2857 #2858
[mk-app] #2423 not #2777
[inst-discovered] theory-solving 0 basic# ; #690
[mk-app] #2424 = #690 #2423
[instance] 0 #2424
[attach-enode] #2424 0
[end-of-instance]
[mk-app] #2424 not #2423
[inst-discovered] theory-solving 0 basic# ; #2424
[mk-app] #2439 = #2424 #2777
[instance] 0 #2439
[attach-enode] #2439 0
[end-of-instance]
[mk-app] #2424 or #2857 #2858 #691
[mk-app] #2439 or #2777 #691
[inst-discovered] theory-solving 0 basic# ; #2439
[mk-app] #2441 = #2439 #2424
[instance] 0 #2441
[attach-enode] #2441 0
[end-of-instance]
[mk-quant] #2439 prelude_bit_xor_i_inv 3 #694 #2424
[attach-var-names] #2439 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #2777 or #3112 #2847
[mk-app] #2423 not #2777
[inst-discovered] theory-solving 0 basic# ; #678
[mk-app] #2441 = #678 #2423
[instance] 0 #2441
[attach-enode] #2441 0
[end-of-instance]
[mk-app] #2441 not #2423
[inst-discovered] theory-solving 0 basic# ; #2441
[mk-app] #2396 = #2441 #2777
[instance] 0 #2396
[attach-enode] #2396 0
[end-of-instance]
[mk-app] #2441 or #3112 #2847 #700
[mk-app] #2396 or #2777 #700
[inst-discovered] theory-solving 0 basic# ; #2396
[mk-app] #2397 = #2396 #2441
[instance] 0 #2397
[attach-enode] #2397 0
[end-of-instance]
[mk-quant] #2396 prelude_bit_or_u_inv 3 #703 #2441
[attach-var-names] #2396 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #2777 or #2857 #2858
[mk-app] #2423 not #2777
[inst-discovered] theory-solving 0 basic# ; #690
[mk-app] #2397 = #690 #2423
[instance] 0 #2397
[attach-enode] #2397 0
[end-of-instance]
[mk-app] #2397 not #2423
[inst-discovered] theory-solving 0 basic# ; #2397
[mk-app] #2399 = #2397 #2777
[instance] 0 #2399
[attach-enode] #2399 0
[end-of-instance]
[mk-app] #2397 or #2857 #2858 #707
[mk-app] #2399 or #2777 #707
[inst-discovered] theory-solving 0 basic# ; #2399
[mk-app] #2230 = #2399 #2397
[instance] 0 #2230
[attach-enode] #2230 0
[end-of-instance]
[mk-quant] #2399 prelude_bit_or_i_inv 3 #710 #2397
[attach-var-names] #2399 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #2777 or #3112 #2847
[mk-app] #2423 not #2777
[inst-discovered] theory-solving 0 basic# ; #678
[mk-app] #2230 = #678 #2423
[instance] 0 #2230
[attach-enode] #2230 0
[end-of-instance]
[mk-app] #2230 not #2423
[inst-discovered] theory-solving 0 basic# ; #2230
[mk-app] #2239 = #2230 #2777
[instance] 0 #2239
[attach-enode] #2239 0
[end-of-instance]
[mk-app] #2230 or #3112 #2847 #715
[mk-app] #2239 or #2777 #715
[inst-discovered] theory-solving 0 basic# ; #2239
[mk-app] #1796 = #2239 #2230
[instance] 0 #1796
[attach-enode] #1796 0
[end-of-instance]
[mk-quant] #2239 prelude_bit_and_u_inv 3 #718 #2230
[attach-var-names] #2239 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #2777 or #2857 #2858
[mk-app] #2423 not #2777
[inst-discovered] theory-solving 0 basic# ; #690
[mk-app] #1796 = #690 #2423
[instance] 0 #1796
[attach-enode] #1796 0
[end-of-instance]
[mk-app] #1796 not #2423
[inst-discovered] theory-solving 0 basic# ; #1796
[mk-app] #1800 = #1796 #2777
[instance] 0 #1800
[attach-enode] #1800 0
[end-of-instance]
[mk-app] #1796 or #2857 #2858 #722
[mk-app] #1800 or #2777 #722
[inst-discovered] theory-solving 0 basic# ; #1800
[mk-app] #1799 = #1800 #1796
[instance] 0 #1799
[attach-enode] #1799 0
[end-of-instance]
[mk-quant] #1800 prelude_bit_and_i_inv 3 #725 #1796
[attach-var-names] #1800 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #2777 not #738
[mk-app] #2423 or #3112 #2777
[mk-app] #1799 not #2423
[inst-discovered] theory-solving 0 basic# ; #737
[mk-app] #1802 = #737 #1799
[instance] 0 #1802
[attach-enode] #1802 0
[end-of-instance]
[mk-app] #1802 not #1799
[inst-discovered] theory-solving 0 basic# ; #1802
[mk-app] #1772 = #1802 #2423
[instance] 0 #1772
[attach-enode] #1772 0
[end-of-instance]
[mk-app] #1799 or #3112 #2777 #732
[mk-app] #1802 or #2423 #732
[inst-discovered] theory-solving 0 basic# ; #1802
[mk-app] #1772 = #1802 #1799
[instance] 0 #1772
[attach-enode] #1772 0
[end-of-instance]
[mk-quant] #2423 prelude_bit_shr_u_inv 3 #735 #1799
[attach-var-names] #2423 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #1802 or #2857 #2777
[mk-app] #1772 not #1802
[inst-discovered] theory-solving 0 basic# ; #748
[mk-app] #1779 = #748 #1772
[instance] 0 #1779
[attach-enode] #1779 0
[end-of-instance]
[mk-app] #1779 not #1772
[inst-discovered] theory-solving 0 basic# ; #1779
[mk-app] #1777 = #1779 #1802
[instance] 0 #1777
[attach-enode] #1777 0
[end-of-instance]
[mk-app] #1772 or #2857 #2777 #743
[mk-app] #1779 or #1802 #743
[inst-discovered] theory-solving 0 basic# ; #1779
[mk-app] #1777 = #1779 #1772
[instance] 0 #1777
[attach-enode] #1777 0
[end-of-instance]
[mk-quant] #1802 prelude_bit_shr_i_inv 3 #746 #1772
[attach-var-names] #1802 (|bits| ; |Int|) (|y| ; |Poly|) (|x| ; |Poly|)
[mk-app] #1779 not #773
[mk-app] #1777 or #1779 #777
[mk-app] #1781 not #1777
[inst-discovered] theory-solving 0 basic# ; #779
[mk-app] #1137 = #779 #1781
[instance] 0 #1137
[attach-enode] #1137 0
[end-of-instance]
[mk-app] #1137 not #766
[mk-app] #1138 not #133
[mk-app] #1139 or #1137 #1138
[mk-app] #1140 not #1139
[inst-discovered] theory-solving 0 basic# ; #767
[mk-app] #1088 = #767 #1140
[instance] 0 #1088
[attach-enode] #1088 0
[end-of-instance]
[mk-app] #1088 or #1781 #1140
[inst-discovered] theory-solving 0 basic# ; #1088
[mk-app] #1089 = #1088 #1088
[instance] 0 #1089
[attach-enode] #1089 0
[end-of-instance]
[mk-app] #1089 = #762 #1088
[mk-quant] #1090 prelude_check_decrease_int 3 #770 #1089
[attach-var-names] #1090 (|otherwise| ; |Bool|) (|prev| ; |Int|) (|cur| ; |Int|)
[mk-app] #1050 not #785
[mk-app] #1051 or #1050 #1138
[mk-app] #1052 not #1051
[inst-discovered] theory-solving 0 basic# ; #786
[mk-app] #1053 = #786 #1052
[instance] 0 #1053
[attach-enode] #1053 0
[end-of-instance]
[mk-app] #1053 or #784 #1052
[inst-discovered] theory-solving 0 basic# ; #1053
[mk-app] #1000 = #1053 #1053
[instance] 0 #1000
[attach-enode] #1000 0
[end-of-instance]
[mk-app] #1000 = #772 #1053
[mk-quant] #1001 prelude_check_decrease_height 3 #789 #1000
[attach-var-names] #1001 (|otherwise| ; |Bool|) (|prev| ; |Poly|) (|cur| ; |Poly|)
[mk-app] #1002 not #794
[mk-app] #962 or #1002 #795
[mk-app] #3142 not #962
[inst-discovered] theory-solving 0 basic# ; #797
[mk-app] #3143 = #797 #3142
[instance] 0 #3143
[attach-enode] #3143 0
[end-of-instance]
[mk-app] #3143 = #962 #793
[mk-app] #3172 not #3143
[mk-app] #3170 = #793 #3142
[inst-discovered] theory-solving 0 basic# ; #3170
[mk-app] #3171 = #3170 #3172
[instance] 0 #3171
[attach-enode] #3171 0
[end-of-instance]
[mk-app] #3142 not #962
[inst-discovered] theory-solving 0 basic# ; #3172
[mk-app] #3142 = #3172 #3172
[instance] 0 #3142
[attach-enode] #3142 0
[end-of-instance]
[mk-quant] #3142 prelude_height_lt 2 #799 #3172
[attach-var-names] #3142 (|y| ; |Height|) (|x| ; |Height|)
[mk-app] #3170 not #884
[mk-app] #3171 not #885
[mk-app] #3173 not #886
[mk-app] #3174 not #887
[mk-app] #3193 not #888
[mk-app] #3194 not #889
[mk-app] #3195 not #890
[mk-app] #3196 not #891
[mk-app] #3197 not #892
[mk-app] #3198 not #893
[mk-app] #3199 not #894
[mk-app] #3200 not #895
[mk-app] #3201 not #896
[mk-app] #3202 or #3170 #3171 #3173 #3174 #3193 #3194 #3195 #3196 #3197 #3198 #3199 #3200 #3201
[mk-app] #3203 not #3202
[inst-discovered] theory-solving 0 basic# ; #897
[mk-app] #3204 = #897 #3203
[instance] 0 #3204
[attach-enode] #3204 0
[end-of-instance]
[mk-app] #3204 or #899 #3203
[inst-discovered] theory-solving 0 basic# ; #905
[mk-app] #3205 = #905 #905
[instance] 0 #3205
[attach-enode] #3205 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #924
[mk-app] #3205 = #924 #924
[instance] 0 #3205
[attach-enode] #3205 0
[end-of-instance]
[mk-app] #3205 not #933
[mk-app] #3206 not #934
[mk-app] #3207 not #935
[mk-app] #3208 not #936
[mk-app] #3209 or #3205 #3206 #3207 #3208
[mk-app] #3210 not #3209
[inst-discovered] theory-solving 0 basic# ; #937
[mk-app] #3211 = #937 #3210
[instance] 0 #3211
[attach-enode] #3211 0
[end-of-instance]
[mk-app] #3211 or #939 #3210
[mk-app] #3212 not #1006
[mk-app] #3213 not #1007
[mk-app] #3214 or #3212 #3213
[mk-app] #3215 not #3214
[inst-discovered] theory-solving 0 basic# ; #1008
[mk-app] #3216 = #1008 #3215
[instance] 0 #3216
[attach-enode] #3216 0
[end-of-instance]
[mk-app] #3216 not #3215
[inst-discovered] theory-solving 0 basic# ; #3216
[mk-app] #3217 = #3216 #3214
[instance] 0 #3217
[attach-enode] #3217 0
[end-of-instance]
[mk-app] #3216 or #3212 #3213 #1010
[mk-app] #3217 or #3214 #1010
[inst-discovered] theory-solving 0 basic# ; #3217
[mk-app] #3218 = #3217 #3216
[instance] 0 #3218
[attach-enode] #3218 0
[end-of-instance]
[mk-quant] #3217 internal_crate__fun__1_apply_definition 6 #1012 #3216
[attach-var-names] #3217 (|x| ; |%%Function%%|) (|T%0| ; |Poly|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #3214 or #3212 #3213
[mk-app] #3215 not #3214
[inst-discovered] theory-solving 0 basic# ; #1008
[mk-app] #3218 = #1008 #3215
[instance] 0 #3218
[attach-enode] #3218 0
[end-of-instance]
[mk-app] #3218 not #3215
[inst-discovered] theory-solving 0 basic# ; #3218
[mk-app] #3219 = #3218 #3214
[instance] 0 #3219
[attach-enode] #3219 0
[end-of-instance]
[mk-app] #3218 or #3212 #3213 #1020
[mk-app] #3219 or #3214 #1020
[inst-discovered] theory-solving 0 basic# ; #3219
[mk-app] #3220 = #3219 #3218
[instance] 0 #3220
[attach-enode] #3220 0
[end-of-instance]
[mk-quant] #3219 internal_crate__fun__1_height_apply_definition 6 #1022 #3218
[attach-var-names] #3219 (|x| ; |%%Function%%|) (|T%0| ; |Poly|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[inst-discovered] theory-solving 0 basic# ; #2882
[mk-app] #3214 = #2882 #2882
[instance] 0 #3214
[attach-enode] #3214 0
[end-of-instance]
[mk-app] #3214 not #2881
[mk-app] #3215 not #2884
[mk-app] #3220 or #3214 #3215
[mk-app] #3221 not #3220
[inst-discovered] theory-solving 0 basic# ; #2885
[mk-app] #3222 = #2885 #3221
[instance] 0 #3222
[attach-enode] #3222 0
[end-of-instance]
[mk-app] #3222 not #3221
[inst-discovered] theory-solving 0 basic# ; #3222
[mk-app] #3223 = #3222 #3220
[instance] 0 #3223
[attach-enode] #3223 0
[end-of-instance]
[mk-app] #3221 or #3214 #3215 #2888
[mk-app] #3222 or #3220 #2888
[inst-discovered] theory-solving 0 basic# ; #3222
[mk-app] #3223 = #3222 #3221
[instance] 0 #3223
[attach-enode] #3223 0
[end-of-instance]
[mk-app] #3220 not #3221
[mk-app] #3222 or #3220 #1081
[mk-quant] #3223 internal_crate__fun__2_constructor_definition 7 #1083 #3222
[attach-var-names] #3223 (|x| ; |%%Function%%|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #3224 not #1093
[mk-app] #3225 not #1094
[mk-app] #3226 not #1095
[mk-app] #3227 or #3224 #3225 #3226
[mk-app] #3228 not #3227
[inst-discovered] theory-solving 0 basic# ; #1096
[mk-app] #3229 = #1096 #3228
[instance] 0 #3229
[attach-enode] #3229 0
[end-of-instance]
[mk-app] #3229 not #3228
[inst-discovered] theory-solving 0 basic# ; #3229
[mk-app] #3230 = #3229 #3227
[instance] 0 #3230
[attach-enode] #3230 0
[end-of-instance]
[mk-app] #3229 or #3224 #3225 #3226 #1098
[mk-app] #3230 or #3227 #1098
[inst-discovered] theory-solving 0 basic# ; #3230
[mk-app] #3231 = #3230 #3229
[instance] 0 #3231
[attach-enode] #3231 0
[end-of-instance]
[mk-quant] #3230 internal_crate__fun__2_apply_definition 9 #1100 #3229
[attach-var-names] #3230 (|x| ; |%%Function%%|) (|T%1| ; |Poly|) (|T%0| ; |Poly|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #3227 or #3224 #3225 #3226
[mk-app] #3228 not #3227
[inst-discovered] theory-solving 0 basic# ; #1096
[mk-app] #3231 = #1096 #3228
[instance] 0 #3231
[attach-enode] #3231 0
[end-of-instance]
[mk-app] #3231 not #3228
[inst-discovered] theory-solving 0 basic# ; #3231
[mk-app] #3232 = #3231 #3227
[instance] 0 #3232
[attach-enode] #3232 0
[end-of-instance]
[mk-app] #3231 or #3224 #3225 #3226 #1108
[mk-app] #3232 or #3227 #1108
[inst-discovered] theory-solving 0 basic# ; #3232
[mk-app] #3233 = #3232 #3231
[instance] 0 #3233
[attach-enode] #3233 0
[end-of-instance]
[mk-quant] #3232 internal_crate__fun__2_height_apply_definition 9 #1110 #3231
[attach-var-names] #3232 (|x| ; |%%Function%%|) (|T%1| ; |Poly|) (|T%0| ; |Poly|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #3227 not #2897
[mk-app] #3228 not #2898
[mk-app] #3233 or #3227 #3228
[mk-app] #3234 not #3233
[inst-discovered] theory-solving 0 basic# ; #2899
[mk-app] #3235 = #2899 #3234
[instance] 0 #3235
[attach-enode] #3235 0
[end-of-instance]
[mk-app] #3235 not #3234
[inst-discovered] theory-solving 0 basic# ; #3235
[mk-app] #3236 = #3235 #3233
[instance] 0 #3236
[attach-enode] #3236 0
[end-of-instance]
[mk-app] #3234 or #3227 #3228 #2904
[mk-app] #3235 or #3233 #2904
[inst-discovered] theory-solving 0 basic# ; #3235
[mk-app] #3236 = #3235 #3234
[instance] 0 #3236
[attach-enode] #3236 0
[end-of-instance]
[mk-app] #3233 not #3234
[mk-app] #3235 or #2893 #2894 #3233 #1130
[inst-discovered] theory-solving 0 basic# ; #3235
[mk-app] #3236 = #3235 #3235
[instance] 0 #3236
[attach-enode] #3236 0
[end-of-instance]
[mk-quant] #3236 internal_crate__fun__2_ext_equal_definition 9 #1132 #3235
[attach-var-names] #3236 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|T%2&| ; |Type|) (|T%2&.| ; |Dcr|) (|T%1&| ; |Type|) (|T%1&.| ; |Dcr|) (|T%0&| ; |Type|) (|T%0&.| ; |Dcr|)
[mk-app] #3237 not #1261
[mk-app] #3238 not #1262
[mk-app] #3239 not #1271
[mk-app] #3240 not #1272
[mk-app] #3241 or #3237 #3238 #3239 #3240
[mk-app] #3242 not #3241
[inst-discovered] theory-solving 0 basic# ; #1273
[mk-app] #3243 = #1273 #3242
[instance] 0 #3243
[attach-enode] #3243 0
[end-of-instance]
[mk-app] #3243 not #3242
[inst-discovered] theory-solving 0 basic# ; #3243
[mk-app] #3244 = #3243 #3241
[instance] 0 #3244
[attach-enode] #3244 0
[end-of-instance]
[mk-app] #3242 or #3237 #3238 #3239 #3240 #1267
[mk-app] #3243 or #3241 #1267
[inst-discovered] theory-solving 0 basic# ; #3243
[mk-app] #3244 = #3243 #3242
[instance] 0 #3244
[attach-enode] #3244 0
[end-of-instance]
[mk-quant] #3241 internal_core!option.Option./None_ext_equal_definition 5 #1269 #3242
[attach-var-names] #3241 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #3243 not #1285
[mk-app] #3244 not #1281
[mk-app] #3245 or #3237 #3238 #3243 #2489 #3244
[mk-app] #3246 not #3245
[inst-discovered] theory-solving 0 basic# ; #1287
[mk-app] #3247 = #1287 #3246
[instance] 0 #3247
[attach-enode] #3247 0
[end-of-instance]
[mk-app] #3247 not #3246
[inst-discovered] theory-solving 0 basic# ; #3247
[mk-app] #3248 = #3247 #3245
[instance] 0 #3248
[attach-enode] #3248 0
[end-of-instance]
[mk-app] #3246 or #3237 #3238 #3243 #2489 #3244 #1267
[mk-app] #3247 or #3245 #1267
[inst-discovered] theory-solving 0 basic# ; #3247
[mk-app] #3248 = #3247 #3246
[instance] 0 #3248
[attach-enode] #3248 0
[end-of-instance]
[mk-quant] #3245 internal_core!option.Option./Some_ext_equal_definition 5 #1269 #3246
[attach-var-names] #3245 (|y| ; |Poly|) (|x| ; |Poly|) (|deep| ; |Bool|) (|V&| ; |Type|) (|V&.| ; |Dcr|)
[mk-app] #3247 or #1652 #1493
[mk-app] #3248 not #3247
[inst-discovered] theory-solving 0 basic# ; #1374
[mk-app] #3249 = #1374 #3248
[instance] 0 #3249
[attach-enode] #3249 0
[end-of-instance]
[mk-app] #3249 or #1378 #3248
[mk-quant] #3250 internal_vstd__std_specs__option__OptionAdditionalFns_trait_type_bounds_definition 4 #1376 #3249
[attach-var-names] #3250 (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #3251 not #1382
[mk-app] #3252 not #1384
[mk-app] #3253 not #1387
[mk-app] #3254 or #3251 #3252 #3253 #1493
[mk-app] #3255 not #3254
[inst-discovered] theory-solving 0 basic# ; #1393
[mk-app] #3256 = #1393 #3255
[instance] 0 #3256
[attach-enode] #3256 0
[end-of-instance]
[mk-app] #3256 or #1394 #3255
[mk-quant] #3257 internal_vstd__std_specs__vec__VecAdditionalSpecFns_trait_type_bounds_definition 4 #1391 #3256
[attach-var-names] #3257 (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[inst-discovered] theory-solving 0 basic# ; #1374
[mk-app] #3258 = #1374 #3248
[instance] 0 #3258
[attach-enode] #3258 0
[end-of-instance]
[mk-app] #3258 or #1401 #3248
[mk-quant] #3259 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphTrait_trait_type_bounds_definition 4 #1399 #3258
[attach-var-names] #3259 (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #3260 or #1493 #1365
[mk-app] #3261 not #3260
[inst-discovered] theory-solving 0 basic# ; #1438
[mk-app] #3262 = #1438 #3261
[instance] 0 #3262
[attach-enode] #3262 0
[end-of-instance]
[mk-app] #3262 not #3261
[inst-discovered] theory-solving 0 basic# ; #3262
[mk-app] #3263 = #3262 #3260
[instance] 0 #3263
[attach-enode] #3263 0
[end-of-instance]
[mk-app] #3262 or #1493 #1365 #1441
[mk-app] #3263 or #3260 #1441
[inst-discovered] theory-solving 0 basic# ; #3263
[mk-app] #3264 = #3263 #3262
[instance] 0 #3264
[attach-enode] #3264 0
[end-of-instance]
[mk-quant] #3263 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1443 #3262
[attach-var-names] #3263 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3260 or #1493 #1365
[mk-app] #3261 not #3260
[inst-discovered] theory-solving 0 basic# ; #1438
[mk-app] #3264 = #1438 #3261
[instance] 0 #3264
[attach-enode] #3264 0
[end-of-instance]
[mk-app] #3264 not #3261
[inst-discovered] theory-solving 0 basic# ; #3264
[mk-app] #3265 = #3264 #3260
[instance] 0 #3265
[attach-enode] #3265 0
[end-of-instance]
[mk-app] #3264 or #1493 #1365 #1449
[mk-app] #3265 or #3260 #1449
[inst-discovered] theory-solving 0 basic# ; #3265
[mk-app] #3266 = #3265 #3264
[instance] 0 #3266
[attach-enode] #3266 0
[end-of-instance]
[mk-quant] #3265 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1451 #3264
[attach-var-names] #3265 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3260 or #1493 #1365
[mk-app] #3261 not #3260
[inst-discovered] theory-solving 0 basic# ; #1438
[mk-app] #3266 = #1438 #3261
[instance] 0 #3266
[attach-enode] #3266 0
[end-of-instance]
[mk-app] #3266 not #3261
[inst-discovered] theory-solving 0 basic# ; #3266
[mk-app] #3267 = #3266 #3260
[instance] 0 #3267
[attach-enode] #3267 0
[end-of-instance]
[mk-app] #3266 or #1493 #1365 #1457
[mk-app] #3267 or #3260 #1457
[inst-discovered] theory-solving 0 basic# ; #3267
[mk-app] #3268 = #3267 #3266
[instance] 0 #3268
[attach-enode] #3268 0
[end-of-instance]
[mk-quant] #3267 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1459 #3266
[attach-var-names] #3267 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3260 or #1493 #1365
[mk-app] #3261 not #3260
[inst-discovered] theory-solving 0 basic# ; #1438
[mk-app] #3268 = #1438 #3261
[instance] 0 #3268
[attach-enode] #3268 0
[end-of-instance]
[mk-app] #3268 not #3261
[inst-discovered] theory-solving 0 basic# ; #3268
[mk-app] #3269 = #3268 #3260
[instance] 0 #3269
[attach-enode] #3269 0
[end-of-instance]
[mk-app] #3268 or #1493 #1365 #1464
[mk-app] #3269 or #3260 #1464
[inst-discovered] theory-solving 0 basic# ; #3269
[mk-app] #3270 = #3269 #3268
[instance] 0 #3270
[attach-enode] #3270 0
[end-of-instance]
[mk-quant] #3269 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1466 #3268
[attach-var-names] #3269 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3260 or #1652 #1493 #2655
[mk-app] #3261 not #3260
[inst-discovered] theory-solving 0 basic# ; #1470
[mk-app] #3270 = #1470 #3261
[instance] 0 #3270
[attach-enode] #3270 0
[end-of-instance]
[mk-app] #3270 not #3261
[inst-discovered] theory-solving 0 basic# ; #3270
[mk-app] #3271 = #3270 #3260
[instance] 0 #3271
[attach-enode] #3271 0
[end-of-instance]
[mk-app] #3270 or #1652 #1493 #2655 #1473
[mk-app] #3271 or #3260 #1473
[inst-discovered] theory-solving 0 basic# ; #3271
[mk-app] #3272 = #3271 #3270
[instance] 0 #3272
[attach-enode] #3272 0
[end-of-instance]
[mk-quant] #3271 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 4 #1475 #3270
[attach-var-names] #3271 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3260 or #1652 #1493 #2655
[mk-app] #3261 not #3260
[inst-discovered] theory-solving 0 basic# ; #1470
[mk-app] #3272 = #1470 #3261
[instance] 0 #3272
[attach-enode] #3272 0
[end-of-instance]
[mk-app] #3272 not #3261
[inst-discovered] theory-solving 0 basic# ; #3272
[mk-app] #3273 = #3272 #3260
[instance] 0 #3273
[attach-enode] #3273 0
[end-of-instance]
[mk-app] #3272 or #1652 #1493 #2655 #1482
[mk-app] #3273 or #3260 #1482
[inst-discovered] theory-solving 0 basic# ; #3273
[mk-app] #3274 = #3273 #3272
[instance] 0 #3274
[attach-enode] #3274 0
[end-of-instance]
[mk-quant] #3273 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 4 #1484 #3272
[attach-var-names] #3273 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3260 or #1493 #1365
[mk-app] #3261 not #3260
[inst-discovered] theory-solving 0 basic# ; #1438
[mk-app] #3274 = #1438 #3261
[instance] 0 #3274
[attach-enode] #3274 0
[end-of-instance]
[mk-app] #3274 not #3261
[inst-discovered] theory-solving 0 basic# ; #3274
[mk-app] #3275 = #3274 #3260
[instance] 0 #3275
[attach-enode] #3275 0
[end-of-instance]
[mk-app] #3274 or #1493 #1365 #1521
[mk-app] #3275 or #3260 #1521
[inst-discovered] theory-solving 0 basic# ; #3275
[mk-app] #3276 = #3275 #3274
[instance] 0 #3276
[attach-enode] #3276 0
[end-of-instance]
[mk-quant] #3275 internal_proj____vstd!view.View./V_assoc_type_impl_true_definition 2 #1523 #3274
[attach-var-names] #3275 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3260 or #1493 #1365
[mk-app] #3261 not #3260
[inst-discovered] theory-solving 0 basic# ; #1438
[mk-app] #3276 = #1438 #3261
[instance] 0 #3276
[attach-enode] #3276 0
[end-of-instance]
[mk-app] #3276 not #3261
[inst-discovered] theory-solving 0 basic# ; #3276
[mk-app] #3277 = #3276 #3260
[instance] 0 #3277
[attach-enode] #3277 0
[end-of-instance]
[mk-app] #3276 or #1493 #1365 #1529
[mk-app] #3277 or #3260 #1529
[inst-discovered] theory-solving 0 basic# ; #3277
[mk-app] #3278 = #3277 #3276
[instance] 0 #3278
[attach-enode] #3278 0
[end-of-instance]
[mk-quant] #3277 internal_proj__vstd!view.View./V_assoc_type_impl_false_definition 2 #1531 #3276
[attach-var-names] #3277 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3260 not #521
[mk-app] #3261 or #3260 #1557
[mk-app] #3278 not #3261
[inst-discovered] theory-solving 0 basic# ; #1561
[mk-app] #3279 = #1561 #3278
[instance] 0 #3279
[attach-enode] #3279 0
[end-of-instance]
[mk-app] #3279 or #1562 #3278
[mk-app] #3280 = #1546 #3279
[mk-quant] #3281 internal_req__vstd!seq.Seq.index._definition 4 #1553 #3280
[attach-var-names] #3281 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3282 not #1555
[mk-app] #3283 or #3282 #197
[mk-app] #3284 not #3283
[inst-discovered] theory-solving 0 basic# ; #1556
[mk-app] #3285 = #1556 #3284
[instance] 0 #3285
[attach-enode] #3285 0
[end-of-instance]
[mk-app] #3285 not #3284
[inst-discovered] theory-solving 0 basic# ; #3285
[mk-app] #3286 = #3285 #3283
[instance] 0 #3286
[attach-enode] #3286 0
[end-of-instance]
[mk-app] #3285 or #3282 #197 #1567
[mk-app] #3286 or #3283 #1567
[inst-discovered] theory-solving 0 basic# ; #3286
[mk-app] #3287 = #3286 #3285
[instance] 0 #3287
[attach-enode] #3287 0
[end-of-instance]
[mk-quant] #3286 internal_vstd!seq.Seq.index.?_pre_post_definition 4 #1569 #3285
[attach-var-names] #3286 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[inst-discovered] theory-solving 0 basic# ; #1561
[mk-app] #3283 = #1561 #3278
[instance] 0 #3283
[attach-enode] #3283 0
[end-of-instance]
[mk-app] #3283 or #1582 #3278
[mk-app] #3284 = #1574 #3283
[mk-quant] #3287 internal_req__vstd!seq.impl&__0.spec_index._definition 4 #1578 #3284
[attach-var-names] #3287 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3288 or #3282 #197
[mk-app] #3289 not #3288
[inst-discovered] theory-solving 0 basic# ; #1556
[mk-app] #3290 = #1556 #3289
[instance] 0 #3290
[attach-enode] #3290 0
[end-of-instance]
[mk-app] #3290 not #3289
[inst-discovered] theory-solving 0 basic# ; #3290
[mk-app] #3291 = #3290 #3288
[instance] 0 #3291
[attach-enode] #3291 0
[end-of-instance]
[mk-app] #3290 or #3282 #197 #1593
[mk-app] #3291 or #3288 #1593
[inst-discovered] theory-solving 0 basic# ; #3291
[mk-app] #3292 = #3291 #3290
[instance] 0 #3292
[attach-enode] #3292 0
[end-of-instance]
[mk-quant] #3291 internal_vstd!seq.impl&__0.spec_index.?_pre_post_definition 4 #1588 #3290
[attach-var-names] #3291 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3288 or #3282 #197
[mk-app] #3289 not #3288
[inst-discovered] theory-solving 0 basic# ; #1556
[mk-app] #3292 = #1556 #3289
[instance] 0 #3292
[attach-enode] #3292 0
[end-of-instance]
[mk-app] #3292 not #3289
[inst-discovered] theory-solving 0 basic# ; #3292
[mk-app] #3293 = #3292 #3288
[instance] 0 #3293
[attach-enode] #3293 0
[end-of-instance]
[mk-app] #3292 or #1652 #3260 #1557
[mk-app] #3293 not #3292
[inst-discovered] theory-solving 0 basic# ; #1609
[mk-app] #3294 = #1609 #3293
[instance] 0 #3294
[attach-enode] #3294 0
[end-of-instance]
[mk-app] #3294 not #3293
[inst-discovered] theory-solving 0 basic# ; #3294
[mk-app] #3295 = #3294 #3292
[instance] 0 #3295
[attach-enode] #3295 0
[end-of-instance]
[mk-app] #3293 or #3282 #197 #1652 #3260 #1557 #1601
[mk-app] #3294 or #3288 #3292 #1601
[inst-discovered] theory-solving 0 basic# ; #3294
[mk-app] #3295 = #3294 #3293
[instance] 0 #3295
[attach-enode] #3295 0
[end-of-instance]
[mk-quant] #3292 user_vstd__seq__axiom_seq_index_decreases_0 4 #1604 #3293
[attach-var-names] #3292 (|i!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3288 or #1607 #3292
[mk-app] #3289 not #1628
[mk-app] #3294 not #1629
[mk-app] #3295 or #3289 #3294
[mk-app] #3296 not #3295
[inst-discovered] theory-solving 0 basic# ; #1630
[mk-app] #3297 = #1630 #3296
[instance] 0 #3297
[attach-enode] #3297 0
[end-of-instance]
[mk-app] #3297 not #3296
[inst-discovered] theory-solving 0 basic# ; #3297
[mk-app] #3298 = #3297 #3295
[instance] 0 #3298
[attach-enode] #3298 0
[end-of-instance]
[mk-app] #3297 or #3289 #3294 #1633
[mk-app] #3298 or #3295 #1633
[inst-discovered] theory-solving 0 basic# ; #3298
[mk-app] #3299 = #3298 #3297
[instance] 0 #3299
[attach-enode] #3299 0
[end-of-instance]
[mk-quant] #3298 internal_vstd!seq.Seq.new.?_pre_post_definition 6 #1635 #3297
[attach-var-names] #3298 (|f!| ; |Poly|) (|len!| ; |Poly|) (|impl%1&| ; |Type|) (|impl%1&.| ; |Dcr|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3295 not #1642
[mk-app] #3296 or #3289 #3295
[mk-app] #3299 not #3296
[inst-discovered] theory-solving 0 basic# ; #1643
[mk-app] #3300 = #1643 #3299
[instance] 0 #3300
[attach-enode] #3300 0
[end-of-instance]
[mk-app] #3300 not #3299
[inst-discovered] theory-solving 0 basic# ; #3300
[mk-app] #3301 = #3300 #3296
[instance] 0 #3301
[attach-enode] #3301 0
[end-of-instance]
[mk-app] #3300 or #3289 #3295 #1652 #1646
[mk-app] #3301 or #3296 #1652 #1646
[inst-discovered] theory-solving 0 basic# ; #3301
[mk-app] #3302 = #3301 #3300
[instance] 0 #3302
[attach-enode] #3302 0
[end-of-instance]
[mk-quant] #3301 user_vstd__seq__axiom_seq_new_len_2 4 #1649 #3300
[attach-var-names] #3301 (|f!| ; |Poly|) (|len!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3296 or #1656 #3301
[mk-app] #3299 not #1659
[mk-app] #3302 not #1661
[mk-app] #3303 or #3299 #3302 #197
[mk-app] #3304 not #3303
[inst-discovered] theory-solving 0 basic# ; #1662
[mk-app] #3305 = #1662 #3304
[instance] 0 #3305
[attach-enode] #3305 0
[end-of-instance]
[mk-app] #3305 not #3304
[inst-discovered] theory-solving 0 basic# ; #3305
[mk-app] #3306 = #3305 #3303
[instance] 0 #3306
[attach-enode] #3306 0
[end-of-instance]
[mk-app] #3305 not #1663
[mk-app] #3306 or #3305 #3260 #1677
[mk-app] #3307 not #3306
[inst-discovered] theory-solving 0 basic# ; #1681
[mk-app] #3308 = #1681 #3307
[instance] 0 #3308
[attach-enode] #3308 0
[end-of-instance]
[mk-app] #3308 not #3307
[inst-discovered] theory-solving 0 basic# ; #3308
[mk-app] #3309 = #3308 #3306
[instance] 0 #3309
[attach-enode] #3309 0
[end-of-instance]
[mk-app] #3307 or #3299 #3302 #197 #3305 #3260 #1677 #1669
[mk-app] #3308 or #3303 #3306 #1669
[inst-discovered] theory-solving 0 basic# ; #3308
[mk-app] #3309 = #3308 #3307
[instance] 0 #3309
[attach-enode] #3309 0
[end-of-instance]
[mk-quant] #3306 user_vstd__seq__axiom_seq_new_index_3 5 #1672 #3307
[attach-var-names] #3306 (|i!| ; |Poly|) (|f!| ; |Poly|) (|len!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3303 or #1675 #3306
[mk-app] #3304 or #3282 #3294
[mk-app] #3308 not #3304
[inst-discovered] theory-solving 0 basic# ; #1686
[mk-app] #3309 = #1686 #3308
[instance] 0 #3309
[attach-enode] #3309 0
[end-of-instance]
[mk-app] #3309 not #3308
[inst-discovered] theory-solving 0 basic# ; #3309
[mk-app] #3310 = #3309 #3304
[instance] 0 #3310
[attach-enode] #3310 0
[end-of-instance]
[mk-app] #3309 or #3282 #3294 #1688
[mk-app] #3310 or #3304 #1688
[inst-discovered] theory-solving 0 basic# ; #3310
[mk-app] #3311 = #3310 #3309
[instance] 0 #3311
[attach-enode] #3311 0
[end-of-instance]
[mk-quant] #3310 internal_vstd!seq.Seq.push.?_pre_post_definition 4 #1690 #3309
[attach-var-names] #3310 (|a!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3304 or #3282 #3294
[mk-app] #3308 not #3304
[inst-discovered] theory-solving 0 basic# ; #1686
[mk-app] #3311 = #1686 #3308
[instance] 0 #3311
[attach-enode] #3311 0
[end-of-instance]
[mk-app] #3311 not #3308
[inst-discovered] theory-solving 0 basic# ; #3311
[mk-app] #3312 = #3311 #3304
[instance] 0 #3312
[attach-enode] #3312 0
[end-of-instance]
[mk-app] #3311 or #3282 #3294 #1652 #1699
[mk-app] #3312 or #3304 #1652 #1699
[inst-discovered] theory-solving 0 basic# ; #3312
[mk-app] #3313 = #3312 #3311
[instance] 0 #3313
[attach-enode] #3313 0
[end-of-instance]
[mk-quant] #3312 user_vstd__seq__axiom_seq_push_len_4 4 #1702 #3311
[attach-var-names] #3312 (|a!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3304 or #1707 #3312
[mk-app] #3308 not #1711
[mk-app] #3313 not #1712
[mk-app] #3314 or #3308 #3313 #197
[mk-app] #3315 not #3314
[inst-discovered] theory-solving 0 basic# ; #1713
[mk-app] #3316 = #1713 #3315
[instance] 0 #3316
[attach-enode] #3316 0
[end-of-instance]
[mk-app] #3316 not #3315
[inst-discovered] theory-solving 0 basic# ; #3316
[mk-app] #3317 = #3316 #3314
[instance] 0 #3317
[attach-enode] #3317 0
[end-of-instance]
[mk-app] #3316 not #1715
[mk-app] #3317 or #3305 #3316
[mk-app] #3318 not #3317
[inst-discovered] theory-solving 0 basic# ; #1716
[mk-app] #3319 = #1716 #3318
[instance] 0 #3319
[attach-enode] #3319 0
[end-of-instance]
[mk-app] #3319 not #3318
[inst-discovered] theory-solving 0 basic# ; #3319
[mk-app] #3320 = #3319 #3317
[instance] 0 #3320
[attach-enode] #3320 0
[end-of-instance]
[mk-app] #3319 or #3308 #3313 #197 #3305 #3316 #1719
[mk-app] #3320 or #3314 #3317 #1719
[inst-discovered] theory-solving 0 basic# ; #3320
[mk-app] #3321 = #3320 #3319
[instance] 0 #3321
[attach-enode] #3321 0
[end-of-instance]
[mk-quant] #3320 user_vstd__seq__axiom_seq_push_index_same_5 5 #1722 #3319
[attach-var-names] #3320 (|i!| ; |Poly|) (|a!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3317 or #1729 #3320
[mk-app] #3318 or #3308 #3313 #197
[mk-app] #3314 not #3318
[inst-discovered] theory-solving 0 basic# ; #1713
[mk-app] #3315 = #1713 #3314
[instance] 0 #3315
[attach-enode] #3315 0
[end-of-instance]
[mk-app] #3315 not #3314
[inst-discovered] theory-solving 0 basic# ; #3315
[mk-app] #3321 = #3315 #3318
[instance] 0 #3321
[attach-enode] #3321 0
[end-of-instance]
[mk-app] #3315 or #3305 #3260 #1743
[mk-app] #3321 not #3315
[inst-discovered] theory-solving 0 basic# ; #1747
[mk-app] #3322 = #1747 #3321
[instance] 0 #3322
[attach-enode] #3322 0
[end-of-instance]
[mk-app] #3322 not #3321
[inst-discovered] theory-solving 0 basic# ; #3322
[mk-app] #3323 = #3322 #3315
[instance] 0 #3323
[attach-enode] #3323 0
[end-of-instance]
[mk-app] #3321 or #3308 #3313 #197 #3305 #3260 #1743 #1736
[mk-app] #3322 or #3318 #3315 #1736
[inst-discovered] theory-solving 0 basic# ; #3322
[mk-app] #3323 = #3322 #3321
[instance] 0 #3323
[attach-enode] #3323 0
[end-of-instance]
[mk-quant] #3315 user_vstd__seq__axiom_seq_push_index_different_6 5 #1722 #3321
[attach-var-names] #3315 (|i!| ; |Poly|) (|a!| ; |Poly|) (|s!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3318 or #1741 #3315
[mk-app] #3314 not #1752
[mk-app] #3322 or #3282 #3314
[mk-app] #3323 not #3322
[inst-discovered] theory-solving 0 basic# ; #1753
[mk-app] #3324 = #1753 #3323
[instance] 0 #3324
[attach-enode] #3324 0
[end-of-instance]
[mk-app] #3324 not #3323
[inst-discovered] theory-solving 0 basic# ; #3324
[mk-app] #3325 = #3324 #3322
[instance] 0 #3325
[attach-enode] #3325 0
[end-of-instance]
[mk-app] #3324 or #3260 #1743
[mk-app] #3325 not #3324
[inst-discovered] theory-solving 0 basic# ; #1773
[mk-app] #3326 = #1773 #3325
[instance] 0 #3326
[attach-enode] #3326 0
[end-of-instance]
[mk-app] #3326 not #3325
[inst-discovered] theory-solving 0 basic# ; #3326
[mk-app] #3327 = #3326 #3324
[instance] 0 #3327
[attach-enode] #3327 0
[end-of-instance]
[mk-app] #3325 or #197 #3260 #1743 #1758
[mk-app] #3326 or #197 #3324 #1758
[inst-discovered] theory-solving 0 basic# ; #3326
[mk-app] #3327 = #3326 #3325
[instance] 0 #3327
[attach-enode] #3327 0
[end-of-instance]
[mk-quant] #3326 user_vstd__seq__axiom_seq_ext_equal_7 1 #1761 #1762 #3325
[attach-var-names] #3326 (|i$| ; |Poly|)
[mk-app] #3324 not #3326
[mk-app] #3327 or #2908 #3324
[mk-app] #3328 not #3327
[mk-app] #3329 and #1756 #3326
[inst-discovered] theory-solving 0 basic# ; #3329
[mk-app] #3330 = #3329 #3328
[instance] 0 #3330
[attach-enode] #3330 0
[end-of-instance]
[mk-app] #3329 or #2907 #3328
[mk-app] #3330 not #2915
[mk-app] #3331 or #3330 #2936
[mk-app] #3332 not #3331
[inst-discovered] theory-solving 0 basic# ; #2937
[mk-app] #3333 = #2937 #3332
[instance] 0 #3333
[attach-enode] #3333 0
[end-of-instance]
[mk-app] #3333 not #3332
[inst-discovered] theory-solving 0 basic# ; #3333
[mk-app] #3334 = #3333 #3331
[instance] 0 #3334
[attach-enode] #3334 0
[end-of-instance]
[mk-app] #3332 or #2913 #3330 #2936 #2923
[mk-app] #3333 or #2913 #3331 #2923
[inst-discovered] theory-solving 0 basic# ; #3333
[mk-app] #3334 = #3333 #3332
[instance] 0 #3334
[attach-enode] #3334 0
[end-of-instance]
[mk-app] #3331 not #3332
[mk-app] #3333 or #1754 #2908 #3331
[inst-discovered] theory-solving 0 basic# ; #3333
[mk-app] #3334 = #3333 #3333
[instance] 0 #3334
[attach-enode] #3334 0
[end-of-instance]
[mk-app] #3334 not #3329
[mk-app] #3335 not #3333
[mk-app] #3336 or #3334 #3335
[mk-app] #3337 not #3336
[mk-app] #3338 and #3329 #3333
[inst-discovered] theory-solving 0 basic# ; #3338
[mk-app] #3339 = #3338 #3337
[instance] 0 #3339
[attach-enode] #3339 0
[end-of-instance]
[mk-app] #3338 or #3282 #3314 #1652 #3337
[mk-app] #3339 or #3322 #1652 #3337
[inst-discovered] theory-solving 0 basic# ; #3339
[mk-app] #3340 = #3339 #3338
[instance] 0 #3340
[attach-enode] #3340 0
[end-of-instance]
[mk-quant] #3339 user_vstd__seq__axiom_seq_ext_equal_8 4 #1768 #3338
[attach-var-names] #3339 (|s2!| ; |Poly|) (|s1!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3322 or #1780 #3339
[mk-app] #3323 or #3282 #3314
[mk-app] #3340 not #3323
[inst-discovered] theory-solving 0 basic# ; #1753
[mk-app] #3341 = #1753 #3340
[instance] 0 #3341
[attach-enode] #3341 0
[end-of-instance]
[mk-app] #3341 not #3340
[inst-discovered] theory-solving 0 basic# ; #3341
[mk-app] #3342 = #3341 #3323
[instance] 0 #3342
[attach-enode] #3342 0
[end-of-instance]
[mk-app] #3341 or #3260 #1743
[mk-app] #3342 not #3341
[inst-discovered] theory-solving 0 basic# ; #1773
[mk-app] #3343 = #1773 #3342
[instance] 0 #3343
[attach-enode] #3343 0
[end-of-instance]
[mk-app] #3343 not #3342
[inst-discovered] theory-solving 0 basic# ; #3343
[mk-app] #3344 = #3343 #3341
[instance] 0 #3344
[attach-enode] #3344 0
[end-of-instance]
[mk-app] #3342 or #197 #3260 #1743 #1784
[mk-app] #3343 or #197 #3341 #1784
[inst-discovered] theory-solving 0 basic# ; #3343
[mk-app] #3344 = #3343 #3342
[instance] 0 #3344
[attach-enode] #3344 0
[end-of-instance]
[mk-quant] #3343 user_vstd__seq__axiom_seq_ext_equal_deep_9 1 #1761 #1762 #3342
[attach-var-names] #3343 (|i$| ; |Poly|)
[mk-app] #3341 not #3343
[mk-app] #3344 or #2908 #3341
[mk-app] #3345 not #3344
[mk-app] #3346 and #1756 #3343
[inst-discovered] theory-solving 0 basic# ; #3346
[mk-app] #3347 = #3346 #3345
[instance] 0 #3347
[attach-enode] #3347 0
[end-of-instance]
[mk-app] #3346 or #2927 #3345
[mk-app] #3347 not #2946
[mk-app] #3348 or #3347 #2967
[mk-app] #3349 not #3348
[inst-discovered] theory-solving 0 basic# ; #2968
[mk-app] #3350 = #2968 #3349
[instance] 0 #3350
[attach-enode] #3350 0
[end-of-instance]
[mk-app] #3350 not #3349
[inst-discovered] theory-solving 0 basic# ; #3350
[mk-app] #3351 = #3350 #3348
[instance] 0 #3351
[attach-enode] #3351 0
[end-of-instance]
[mk-app] #3349 or #2931 #3347 #2967 #2954
[mk-app] #3350 or #2931 #3348 #2954
[inst-discovered] theory-solving 0 basic# ; #3350
[mk-app] #3351 = #3350 #3349
[instance] 0 #3351
[attach-enode] #3351 0
[end-of-instance]
[mk-app] #3348 not #3349
[mk-app] #3350 or #1783 #2908 #3348
[inst-discovered] theory-solving 0 basic# ; #3350
[mk-app] #3351 = #3350 #3350
[instance] 0 #3351
[attach-enode] #3351 0
[end-of-instance]
[mk-app] #3351 not #3346
[mk-app] #3352 not #3350
[mk-app] #3353 or #3351 #3352
[mk-app] #3354 not #3353
[mk-app] #3355 and #3346 #3350
[inst-discovered] theory-solving 0 basic# ; #3355
[mk-app] #3356 = #3355 #3354
[instance] 0 #3356
[attach-enode] #3356 0
[end-of-instance]
[mk-app] #3355 or #3282 #3314 #1652 #3354
[mk-app] #3356 or #3323 #1652 #3354
[inst-discovered] theory-solving 0 basic# ; #3356
[mk-app] #3357 = #3356 #3355
[instance] 0 #3357
[attach-enode] #3357 0
[end-of-instance]
[mk-quant] #3356 user_vstd__seq__axiom_seq_ext_equal_deep_10 4 #1792 #3355
[attach-var-names] #3356 (|s2!| ; |Poly|) (|s1!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3323 or #1801 #3356
[mk-app] #3340 or #1652 #1493 #2655
[mk-app] #3357 not #3340
[inst-discovered] theory-solving 0 basic# ; #1470
[mk-app] #3358 = #1470 #3357
[instance] 0 #3358
[attach-enode] #3358 0
[end-of-instance]
[mk-app] #3358 not #3357
[inst-discovered] theory-solving 0 basic# ; #3358
[mk-app] #3359 = #3358 #3340
[instance] 0 #3359
[attach-enode] #3359 0
[end-of-instance]
[mk-app] #3358 or #1652 #1493 #2655 #1871
[mk-app] #3359 or #3340 #1871
[inst-discovered] theory-solving 0 basic# ; #3359
[mk-app] #3360 = #3359 #3358
[instance] 0 #3360
[attach-enode] #3360 0
[end-of-instance]
[mk-quant] #3359 internal_vstd__view__impl&__8_trait_impl_definition 4 #1873 #3358
[attach-var-names] #3359 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3340 not #1879
[mk-app] #3357 or #3305 #1850 #3340
[mk-app] #3360 not #3357
[inst-discovered] theory-solving 0 basic# ; #1888
[mk-app] #3361 = #1888 #3360
[instance] 0 #3361
[attach-enode] #3361 0
[end-of-instance]
[mk-app] #3361 not #3360
[inst-discovered] theory-solving 0 basic# ; #3361
[mk-app] #3362 = #3361 #3357
[instance] 0 #3362
[attach-enode] #3362 0
[end-of-instance]
[mk-app] #3360 or #1868 #3305 #1850 #3340 #1883
[mk-app] #3361 or #1868 #3357 #1883
[inst-discovered] theory-solving 0 basic# ; #3361
[mk-app] #3362 = #3361 #3360
[instance] 0 #3362
[attach-enode] #3362 0
[end-of-instance]
[mk-quant] #3357 user_vstd__std_specs__vec__axiom_spec_len_11 5 #1866 #3360
[attach-var-names] #3357 (|v!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3361 or #1892 #3357
[mk-app] #3362 or #3260 #1907
[mk-app] #3363 not #3362
[inst-discovered] theory-solving 0 basic# ; #1911
[mk-app] #3364 = #1911 #3363
[instance] 0 #3364
[attach-enode] #3364 0
[end-of-instance]
[mk-app] #3364 or #1912 #3363
[mk-app] #3365 = #1895 #3364
[mk-quant] #3366 internal_req__vstd!std_specs.vec.VecAdditionalSpecFns.spec_index._definition 6 #1903 #3365
[attach-var-names] #3366 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #3367 or #3213 #197
[mk-app] #3368 not #3367
[inst-discovered] theory-solving 0 basic# ; #1905
[mk-app] #3369 = #1905 #3368
[instance] 0 #3369
[attach-enode] #3369 0
[end-of-instance]
[mk-app] #3369 not #3368
[inst-discovered] theory-solving 0 basic# ; #3369
[mk-app] #3370 = #3369 #3367
[instance] 0 #3370
[attach-enode] #3370 0
[end-of-instance]
[mk-app] #3369 or #3213 #197 #1916
[mk-app] #3370 or #3367 #1916
[inst-discovered] theory-solving 0 basic# ; #3370
[mk-app] #3371 = #3370 #3369
[instance] 0 #3371
[attach-enode] #3371 0
[end-of-instance]
[mk-quant] #3370 internal_vstd!std_specs.vec.VecAdditionalSpecFns.spec_index.?_pre_post_definition 6 #1918 #3369
[attach-var-names] #3370 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #3367 not #1925
[mk-app] #3368 not #1926
[mk-app] #3371 or #3367 #1652 #3368
[mk-app] #3372 not #3371
[inst-discovered] theory-solving 0 basic# ; #1927
[mk-app] #3373 = #1927 #3372
[instance] 0 #3373
[attach-enode] #3373 0
[end-of-instance]
[mk-app] #3373 not #3372
[inst-discovered] theory-solving 0 basic# ; #3373
[mk-app] #3374 = #3373 #3371
[instance] 0 #3374
[attach-enode] #3374 0
[end-of-instance]
[mk-app] #3373 or #3367 #1652 #3368 #1932
[mk-app] #3374 or #3371 #1932
[inst-discovered] theory-solving 0 basic# ; #3374
[mk-app] #3375 = #3374 #3373
[instance] 0 #3375
[attach-enode] #3375 0
[end-of-instance]
[mk-quant] #3374 internal_vstd!std_specs.vec.VecAdditionalSpecFns.spec_index.?_definition 6 #1934 #3373
[attach-var-names] #3374 (|i!| ; |Poly|) (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3371 or #1940 #3374
[mk-app] #3372 or #1652 #1493 #2655
[mk-app] #3375 not #3372
[inst-discovered] theory-solving 0 basic# ; #1470
[mk-app] #3376 = #1470 #3375
[instance] 0 #3376
[attach-enode] #3376 0
[end-of-instance]
[mk-app] #3376 not #3375
[inst-discovered] theory-solving 0 basic# ; #3376
[mk-app] #3377 = #3376 #3372
[instance] 0 #3377
[attach-enode] #3377 0
[end-of-instance]
[mk-app] #3376 or #1652 #1493 #2655 #1942
[mk-app] #3377 or #3372 #1942
[inst-discovered] theory-solving 0 basic# ; #3377
[mk-app] #3378 = #3377 #3376
[instance] 0 #3378
[attach-enode] #3378 0
[end-of-instance]
[mk-quant] #3377 internal_vstd__std_specs__vec__impl&__0_trait_impl_definition 4 #1944 #3376
[attach-var-names] #3377 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3372 not #1950
[mk-app] #3375 or #3372 #197
[mk-app] #3378 not #3375
[inst-discovered] theory-solving 0 basic# ; #1951
[mk-app] #3379 = #1951 #3378
[instance] 0 #3379
[attach-enode] #3379 0
[end-of-instance]
[mk-app] #3379 not #3378
[inst-discovered] theory-solving 0 basic# ; #3379
[mk-app] #3380 = #3379 #3375
[instance] 0 #3380
[attach-enode] #3380 0
[end-of-instance]
[mk-app] #3379 or #1652 #3260 #1967
[mk-app] #3380 not #3379
[inst-discovered] theory-solving 0 basic# ; #1971
[mk-app] #3381 = #1971 #3380
[instance] 0 #3381
[attach-enode] #3381 0
[end-of-instance]
[mk-app] #3381 not #3380
[inst-discovered] theory-solving 0 basic# ; #3381
[mk-app] #3382 = #3381 #3379
[instance] 0 #3382
[attach-enode] #3382 0
[end-of-instance]
[mk-app] #3380 or #3372 #197 #1652 #3260 #1967 #1959
[mk-app] #3381 or #3375 #3379 #1959
[inst-discovered] theory-solving 0 basic# ; #3381
[mk-app] #3382 = #3381 #3380
[instance] 0 #3382
[attach-enode] #3382 0
[end-of-instance]
[mk-quant] #3379 user_vstd__std_specs__vec__axiom_vec_index_decreases_12 4 #1962 #3380
[attach-var-names] #3379 (|i!| ; |Poly|) (|v!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3375 or #1965 #3379
[mk-app] #3378 or #3372 #197
[mk-app] #3381 not #3378
[inst-discovered] theory-solving 0 basic# ; #1951
[mk-app] #3382 = #1951 #3381
[instance] 0 #3382
[attach-enode] #3382 0
[end-of-instance]
[mk-app] #3382 not #3381
[inst-discovered] theory-solving 0 basic# ; #3382
[mk-app] #3383 = #3382 #3378
[instance] 0 #3383
[attach-enode] #3383 0
[end-of-instance]
[mk-app] #3382 or #3260 #1967
[mk-app] #3383 not #3382
[inst-discovered] theory-solving 0 basic# ; #1988
[mk-app] #3384 = #1988 #3383
[instance] 0 #3384
[attach-enode] #3384 0
[end-of-instance]
[mk-app] #3384 not #3383
[inst-discovered] theory-solving 0 basic# ; #3384
[mk-app] #3385 = #3384 #3382
[instance] 0 #3385
[attach-enode] #3385 0
[end-of-instance]
[mk-app] #3383 or #3372 #197 #1652 #3260 #1967 #1989 #1978
[mk-app] #3384 or #3378 #1652 #3382 #1989 #1978
[inst-discovered] theory-solving 0 basic# ; #3384
[mk-app] #3385 = #3384 #3383
[instance] 0 #3385
[attach-enode] #3385 0
[end-of-instance]
[mk-quant] #3382 user_vstd__std_specs__vec__axiom_vec_has_resolved_13 4 #1983 #3383
[attach-var-names] #3382 (|i!| ; |Poly|) (|vec!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3378 or #1986 #3382
[inst-discovered] theory-solving 0 basic# ; #2004
[mk-app] #3381 = #2004 #2004
[instance] 0 #3381
[attach-enode] #3381 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2017
[mk-app] #3381 = #2017 #2017
[instance] 0 #3381
[attach-enode] #3381 0
[end-of-instance]
[mk-app] #3381 not #2053
[mk-app] #3384 not #2055
[mk-app] #3385 or #3381 #3384
[mk-app] #3386 not #3385
[inst-discovered] theory-solving 0 basic# ; #2056
[mk-app] #3387 = #2056 #3386
[instance] 0 #3387
[attach-enode] #3387 0
[end-of-instance]
[mk-app] #3387 not #3386
[inst-discovered] theory-solving 0 basic# ; #3387
[mk-app] #3388 = #3387 #3385
[instance] 0 #3388
[attach-enode] #3388 0
[end-of-instance]
[mk-app] #3387 or #3381 #3384 #2057
[mk-app] #3388 or #3385 #2057
[inst-discovered] theory-solving 0 basic# ; #3388
[mk-app] #3389 = #3388 #3387
[instance] 0 #3389
[attach-enode] #3389 0
[end-of-instance]
[mk-quant] #3388 internal_vstd!seq_lib.impl&__0.map.?_pre_post_definition 6 #2048 #3387
[attach-var-names] #3388 (|f!| ; |Poly|) (|self!| ; |Poly|) (|B&| ; |Type|) (|B&.| ; |Dcr|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3385 not #2127
[mk-app] #3386 not #2131
[mk-app] #3389 or #3385 #3386
[mk-app] #3390 not #3389
[inst-discovered] theory-solving 0 basic# ; #2132
[mk-app] #3391 = #2132 #3390
[instance] 0 #3391
[attach-enode] #3391 0
[end-of-instance]
[mk-app] #3391 = #3389 #2125
[mk-app] #3392 not #3391
[mk-app] #3393 = #2125 #3390
[inst-discovered] theory-solving 0 basic# ; #3393
[mk-app] #3394 = #3393 #3392
[instance] 0 #3394
[attach-enode] #3394 0
[end-of-instance]
[mk-app] #3390 not #3389
[inst-discovered] theory-solving 0 basic# ; #3392
[mk-app] #3390 = #3392 #3392
[instance] 0 #3390
[attach-enode] #3390 0
[end-of-instance]
[mk-quant] #3390 internal_ens__alloc!vec.impl&__43.push._definition 7 #2134 #3392
[attach-var-names] #3390 (|value!| ; |Poly|) (|vec!| ; |Poly|) (|pre%vec!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3393 not #2137
[mk-app] #3394 not #2140
[mk-app] #3395 or #3393 #3394
[mk-app] #3396 not #3395
[inst-discovered] theory-solving 0 basic# ; #2141
[mk-app] #3397 = #2141 #3396
[instance] 0 #3397
[attach-enode] #3397 0
[end-of-instance]
[mk-app] #3397 = #3395 #2136
[mk-app] #3398 not #3397
[mk-app] #3399 = #2136 #3396
[inst-discovered] theory-solving 0 basic# ; #3399
[mk-app] #3400 = #3399 #3398
[instance] 0 #3400
[attach-enode] #3400 0
[end-of-instance]
[mk-app] #3396 not #3395
[inst-discovered] theory-solving 0 basic# ; #3398
[mk-app] #3396 = #3398 #3398
[instance] 0 #3396
[attach-enode] #3396 0
[end-of-instance]
[mk-quant] #3396 internal_ens__alloc!vec.impl&__0.with_capacity._definition 4 #2143 #3398
[attach-var-names] #3396 (|v!| ; |Poly|) (|capacity!| ; |Int|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3399 or #3213 #197
[mk-app] #3400 not #3399
[inst-discovered] theory-solving 0 basic# ; #1905
[mk-app] #3401 = #1905 #3400
[instance] 0 #3401
[attach-enode] #3401 0
[end-of-instance]
[mk-app] #3401 not #3400
[inst-discovered] theory-solving 0 basic# ; #3401
[mk-app] #3402 = #3401 #3399
[instance] 0 #3402
[attach-enode] #3402 0
[end-of-instance]
[mk-app] #3401 or #3213 #197 #2169
[mk-app] #3402 or #3399 #2169
[inst-discovered] theory-solving 0 basic# ; #3402
[mk-app] #3403 = #3402 #3401
[instance] 0 #3403
[attach-enode] #3403 0
[end-of-instance]
[mk-quant] #3402 internal_lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.?_pre_post_definition 6 #2179 #3401
[attach-var-names] #3402 (|i!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #3399 not #2184
[mk-app] #3400 or #213 #3399
[mk-app] #3403 not #3400
[inst-discovered] theory-solving 0 basic# ; #2185
[mk-app] #3404 = #2185 #3403
[instance] 0 #3404
[attach-enode] #3404 0
[end-of-instance]
[mk-app] #3404 = #3400 #2183
[mk-app] #3405 not #3404
[mk-app] #3406 = #2183 #3403
[inst-discovered] theory-solving 0 basic# ; #3406
[mk-app] #3407 = #3406 #3405
[instance] 0 #3407
[attach-enode] #3407 0
[end-of-instance]
[mk-app] #3403 not #3400
[inst-discovered] theory-solving 0 basic# ; #3405
[mk-app] #3403 = #3405 #3405
[instance] 0 #3403
[attach-enode] #3403 0
[end-of-instance]
[mk-quant] #3403 internal_ens__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.length._definition 6 #2187 #3405
[attach-var-names] #3403 (|len!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #3406 not #2201
[mk-app] #3407 or #2080 #3406
[mk-app] #3408 not #3407
[inst-discovered] theory-solving 0 basic# ; #2202
[mk-app] #3409 = #2202 #3408
[instance] 0 #3409
[attach-enode] #3409 0
[end-of-instance]
[mk-app] #3409 = #3407 #2195
[mk-app] #3410 not #3409
[mk-app] #3411 = #2195 #3408
[inst-discovered] theory-solving 0 basic# ; #3411
[mk-app] #3412 = #3411 #3410
[instance] 0 #3412
[attach-enode] #3412 0
[end-of-instance]
[mk-app] #3408 not #3407
[inst-discovered] theory-solving 0 basic# ; #3410
[mk-app] #3408 = #3410 #3410
[instance] 0 #3408
[attach-enode] #3408 0
[end-of-instance]
[mk-quant] #3408 internal_ens__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth._definition 7 #2204 #3410
[attach-var-names] #3408 (|nth_elem!| ; |Poly|) (|index!| ; |Poly|) (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #3411 or #3260 #2231
[mk-app] #3412 not #3411
[inst-discovered] theory-solving 0 basic# ; #2235
[mk-app] #3413 = #2235 #3412
[instance] 0 #3413
[attach-enode] #3413 0
[end-of-instance]
[mk-app] #3413 not #3412
[inst-discovered] theory-solving 0 basic# ; #3413
[mk-app] #3414 = #3413 #3411
[instance] 0 #3414
[attach-enode] #3414 0
[end-of-instance]
[mk-app] #3412 or #197 #3260 #2231 #2220
[mk-app] #3413 or #197 #3411 #2220
[inst-discovered] theory-solving 0 basic# ; #3413
[mk-app] #3414 = #3413 #3412
[instance] 0 #3414
[attach-enode] #3414 0
[end-of-instance]
[mk-quant] #3411 user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphTrait__from_vec_16 1 #2223 #3412
[attach-var-names] #3411 (|i$| ; |Poly|)
[mk-app] #3413 not #3411
[mk-app] #3414 or #997 #2960 #2961 #3413
[mk-app] #3415 not #3414
[mk-app] #3416 and #985 #2208 #2212 #3411
[inst-discovered] theory-solving 0 basic# ; #3416
[mk-app] #3417 = #3416 #3415
[instance] 0 #3417
[attach-enode] #3417 0
[end-of-instance]
[mk-app] #3416 or #2958 #3415
[mk-app] #3417 not #2979
[mk-app] #3418 or #3417 #2999
[mk-app] #3419 not #3418
[inst-discovered] theory-solving 0 basic# ; #3000
[mk-app] #3420 = #3000 #3419
[instance] 0 #3420
[attach-enode] #3420 0
[end-of-instance]
[mk-app] #3420 not #3419
[inst-discovered] theory-solving 0 basic# ; #3420
[mk-app] #3421 = #3420 #3418
[instance] 0 #3421
[attach-enode] #3421 0
[end-of-instance]
[mk-app] #3419 or #2977 #3417 #2999 #2988
[mk-app] #3420 or #2977 #3418 #2988
[inst-discovered] theory-solving 0 basic# ; #3420
[mk-app] #3421 = #3420 #3419
[instance] 0 #3421
[attach-enode] #3421 0
[end-of-instance]
[mk-app] #3418 not #3419
[mk-app] #3420 or #2206 #997 #2960 #2961 #3418
[inst-discovered] theory-solving 0 basic# ; #3420
[mk-app] #3421 = #3420 #3420
[instance] 0 #3421
[attach-enode] #3421 0
[end-of-instance]
[mk-app] #3421 not #3416
[mk-app] #3422 not #3420
[mk-app] #3423 or #3421 #3422
[mk-app] #3424 not #3423
[mk-app] #3425 and #3416 #3420
[inst-discovered] theory-solving 0 basic# ; #3425
[mk-app] #3426 = #3425 #3424
[instance] 0 #3426
[attach-enode] #3426 0
[end-of-instance]
[mk-quant] #3425 internal_ens__lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.from_vec._definition 6 #2227 #3424
[attach-var-names] #3425 (|seq!| ; |Poly|) (|elts!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #3426 or #1850 #2502
[mk-app] #3427 not #3426
[inst-discovered] theory-solving 0 basic# ; #2294
[mk-app] #3428 = #2294 #3427
[instance] 0 #3428
[attach-enode] #3428 0
[end-of-instance]
[mk-app] #3428 not #3427
[inst-discovered] theory-solving 0 basic# ; #3428
[mk-app] #3429 = #3428 #3426
[instance] 0 #3429
[attach-enode] #3429 0
[end-of-instance]
[mk-app] #3428 or #1850 #2502 #2297
[mk-app] #3429 or #3426 #2297
[inst-discovered] theory-solving 0 basic# ; #3429
[mk-app] #3430 = #3429 #3428
[instance] 0 #3430
[attach-enode] #3430 0
[end-of-instance]
[mk-quant] #3429 internal_vstd!view.View.view.?_definition 3 #2299 #3428
[attach-var-names] #3429 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3426 or #2305 #3429
[mk-app] #3427 or #1493 #1365
[mk-app] #3430 not #3427
[inst-discovered] theory-solving 0 basic# ; #1438
[mk-app] #3431 = #1438 #3430
[instance] 0 #3431
[attach-enode] #3431 0
[end-of-instance]
[mk-app] #3431 not #3430
[inst-discovered] theory-solving 0 basic# ; #3431
[mk-app] #3432 = #3431 #3427
[instance] 0 #3432
[attach-enode] #3432 0
[end-of-instance]
[mk-app] #3431 or #1493 #1365 #2307
[mk-app] #3432 or #3427 #2307
[inst-discovered] theory-solving 0 basic# ; #3432
[mk-app] #3433 = #3432 #3431
[instance] 0 #3433
[attach-enode] #3433 0
[end-of-instance]
[mk-quant] #3432 internal_vstd__view__impl&__6_trait_impl_definition 2 #2309 #3431
[attach-var-names] #3432 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3427 not #2321
[mk-app] #3430 not #2322
[mk-app] #3433 not #2323
[mk-app] #3434 or #3427 #3430 #3433
[mk-app] #3435 not #3434
[inst-discovered] theory-solving 0 basic# ; #2324
[mk-app] #3436 = #2324 #3435
[instance] 0 #3436
[attach-enode] #3436 0
[end-of-instance]
[mk-app] #3436 not #3435
[inst-discovered] theory-solving 0 basic# ; #3436
[mk-app] #3437 = #3436 #3434
[instance] 0 #3437
[attach-enode] #3437 0
[end-of-instance]
[mk-app] #3436 or #3427 #3430 #3433 #2343
[mk-app] #3437 or #3434 #2343
[inst-discovered] theory-solving 0 basic# ; #3437
[mk-app] #3438 = #3437 #3436
[instance] 0 #3438
[attach-enode] #3438 0
[end-of-instance]
[mk-quant] #3437 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum._fuel_to_body_definition 4 #2337 #3436
[attach-var-names] #3437 (|fuel%| ; |Fuel|) (|hi!| ; |Poly|) (|lo!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #3434 not #2348
[mk-app] #3435 or #3434 #3433 #197
[mk-app] #3438 not #3435
[inst-discovered] theory-solving 0 basic# ; #2349
[mk-app] #3439 = #2349 #3438
[instance] 0 #3439
[attach-enode] #3439 0
[end-of-instance]
[mk-app] #3439 not #3438
[inst-discovered] theory-solving 0 basic# ; #3439
[mk-app] #3440 = #3439 #3435
[instance] 0 #3440
[attach-enode] #3440 0
[end-of-instance]
[mk-app] #3439 or #3434 #3433 #197 #2354
[mk-app] #3440 or #3435 #2354
[inst-discovered] theory-solving 0 basic# ; #3440
[mk-app] #3441 = #3440 #3439
[instance] 0 #3441
[attach-enode] #3441 0
[end-of-instance]
[mk-quant] #3440 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.?_definition 3 #2356 #3439
[attach-var-names] #3440 (|hi!| ; |Poly|) (|lo!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #3435 or #2362 #3440
[mk-app] #3438 or #3433 #197
[mk-app] #3441 not #3438
[inst-discovered] theory-solving 0 basic# ; #2367
[mk-app] #3442 = #2367 #3441
[instance] 0 #3442
[attach-enode] #3442 0
[end-of-instance]
[mk-app] #3442 not #3441
[inst-discovered] theory-solving 0 basic# ; #3442
[mk-app] #3443 = #3442 #3438
[instance] 0 #3443
[attach-enode] #3443 0
[end-of-instance]
[mk-app] #3442 not #2384
[mk-app] #3443 not #2388
[mk-app] #3444 or #2777 #3442 #3443
[mk-app] #3445 not #3444
[inst-discovered] theory-solving 0 basic# ; #2389
[mk-app] #3446 = #2389 #3445
[instance] 0 #3446
[attach-enode] #3446 0
[end-of-instance]
[mk-app] #3446 not #3445
[inst-discovered] theory-solving 0 basic# ; #3446
[mk-app] #3447 = #3446 #3444
[instance] 0 #3447
[attach-enode] #3447 0
[end-of-instance]
[mk-app] #3445 not #2391
[mk-app] #3446 not #2375
[mk-app] #3447 or #3445 #3446
[mk-app] #3448 not #3447
[inst-discovered] theory-solving 0 basic# ; #2390
[mk-app] #3449 = #2390 #3448
[instance] 0 #3449
[attach-enode] #3449 0
[end-of-instance]
[mk-app] #3449 or #3433 #197 #2777 #3442 #3443 #3448
[mk-app] #3450 or #3438 #3444 #3448
[inst-discovered] theory-solving 0 basic# ; #3450
[mk-app] #3451 = #3450 #3449
[instance] 0 #3451
[attach-enode] #3451 0
[end-of-instance]
[mk-quant] #3444 user_lib__Chap28__MCSSSpec__MCSSSpec__sums_fit_i32_17 2 #2356 #3449
[attach-var-names] #3444 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[mk-app] #3438 or #2992 #3444
[mk-app] #3441 not #3004
[mk-app] #3450 not #3008
[mk-app] #3451 or #3441 #3450
[mk-app] #3452 not #3451
[inst-discovered] theory-solving 0 basic# ; #3009
[mk-app] #3453 = #3009 #3452
[instance] 0 #3453
[attach-enode] #3453 0
[end-of-instance]
[mk-app] #3453 not #3452
[inst-discovered] theory-solving 0 basic# ; #3453
[mk-app] #3454 = #3453 #3451
[instance] 0 #3454
[attach-enode] #3454 0
[end-of-instance]
[mk-app] #3452 not #3012
[mk-app] #3453 not #3037
[mk-app] #3454 not #3020
[mk-app] #3455 or #3452 #3453 #3454
[mk-app] #3456 not #3455
[inst-discovered] theory-solving 0 basic# ; #3034
[mk-app] #3457 = #3034 #3456
[instance] 0 #3457
[attach-enode] #3457 0
[end-of-instance]
[mk-app] #3457 not #3456
[inst-discovered] theory-solving 0 basic# ; #3457
[mk-app] #3458 = #3457 #3455
[instance] 0 #3458
[attach-enode] #3458 0
[end-of-instance]
[mk-app] #3456 not #3024
[mk-app] #3457 not #3025
[mk-app] #3458 or #3456 #3457
[mk-app] #3459 not #3458
[inst-discovered] theory-solving 0 basic# ; #3026
[mk-app] #3460 = #3026 #3459
[instance] 0 #3460
[attach-enode] #3460 0
[end-of-instance]
[mk-app] #3460 or #3441 #3450 #3452 #3453 #3454 #3459
[mk-app] #3461 or #3451 #3455 #3459
[inst-discovered] theory-solving 0 basic# ; #3461
[mk-app] #3462 = #3461 #3460
[instance] 0 #3462
[attach-enode] #3462 0
[end-of-instance]
[mk-app] #3455 not #3460
[mk-app] #3451 or #2366 #3455
[mk-app] #3461 not #3438
[mk-app] #3462 not #3451
[mk-app] #3463 or #3461 #3462
[mk-app] #3464 not #3463
[mk-app] #3465 and #3438 #3451
[inst-discovered] theory-solving 0 basic# ; #3465
[mk-app] #3466 = #3465 #3464
[instance] 0 #3466
[attach-enode] #3466 0
[end-of-instance]
[mk-quant] #3465 internal_lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.?_definition 1 #2381 #3464
[attach-var-names] #3465 (|s!| ; |Poly|)
[mk-app] #3466 or #2398 #3465
[mk-app] #3467 or #3433 #197
[mk-app] #3468 not #3467
[inst-discovered] theory-solving 0 basic# ; #2367
[mk-app] #3469 = #2367 #3468
[instance] 0 #3469
[attach-enode] #3469 0
[end-of-instance]
[mk-app] #3469 not #3468
[inst-discovered] theory-solving 0 basic# ; #3469
[mk-app] #3470 = #3469 #3467
[instance] 0 #3470
[attach-enode] #3470 0
[end-of-instance]
[mk-app] #3469 not #2429
[mk-app] #3470 or #2777 #2425 #3469
[mk-app] #3471 not #3470
[inst-discovered] theory-solving 0 basic# ; #2430
[mk-app] #3472 = #2430 #3471
[instance] 0 #3472
[attach-enode] #3472 0
[end-of-instance]
[mk-app] #3472 not #3471
[inst-discovered] theory-solving 0 basic# ; #3472
[mk-app] #3473 = #3472 #3470
[instance] 0 #3473
[attach-enode] #3473 0
[end-of-instance]
[mk-app] #3471 or #3433 #197 #2777 #2425 #3469 #2433
[mk-app] #3472 or #3467 #3470 #2433
[inst-discovered] theory-solving 0 basic# ; #3472
[mk-app] #3473 = #3472 #3471
[instance] 0 #3473
[attach-enode] #3473 0
[end-of-instance]
[mk-quant] #3470 user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_19 2 #2412 #3471
[attach-var-names] #3470 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[mk-app] #3467 not #3033
[mk-app] #3468 not #3045
[mk-app] #3472 not #3047
[mk-app] #3473 not #3056
[mk-app] #3474 not #3058
[mk-app] #3475 not #3470
[mk-app] #3476 or #3467 #3468 #3472 #3095 #3473 #3474 #3475
[mk-app] #3477 not #3476
[mk-app] #3478 and #3033 #3045 #3047 #3092 #3056 #3058 #3470
[inst-discovered] theory-solving 0 basic# ; #3478
[mk-app] #3479 = #3478 #3477
[instance] 0 #3479
[attach-enode] #3479 0
[end-of-instance]
[mk-app] #3478 or #3029 #3477
[mk-app] #3479 not #2409
[mk-app] #3480 or #3433 #197 #2777 #2425 #3469 #3479
[mk-app] #3481 not #3480
[inst-discovered] theory-solving 0 basic# ; #2431
[mk-app] #3482 = #2431 #3481
[instance] 0 #3482
[attach-enode] #3482 0
[end-of-instance]
[mk-app] #3482 not #3481
[inst-discovered] theory-solving 0 basic# ; #3482
[mk-app] #3483 = #3482 #3480
[instance] 0 #3483
[attach-enode] #3483 0
[end-of-instance]
[mk-quant] #3481 user_lib__Chap28__MCSSSpec__MCSSSpec__is_mcss_of_18 2 #2412 #3480
[attach-var-names] #3481 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[mk-app] #3482 not #3065
[mk-app] #3483 not #3066
[mk-app] #3484 or #3482 #3483
[mk-app] #3485 not #3484
[inst-discovered] theory-solving 0 basic# ; #3067
[mk-app] #3486 = #3067 #3485
[instance] 0 #3486
[attach-enode] #3486 0
[end-of-instance]
[mk-app] #3486 not #3485
[inst-discovered] theory-solving 0 basic# ; #3486
[mk-app] #3487 = #3486 #3484
[instance] 0 #3487
[attach-enode] #3487 0
[end-of-instance]
[mk-app] #3485 not #3070
[mk-app] #3486 not #3105
[mk-app] #3487 or #3485 #3102 #3486
[mk-app] #3488 not #3487
[inst-discovered] theory-solving 0 basic# ; #3103
[mk-app] #3489 = #3103 #3488
[instance] 0 #3489
[attach-enode] #3489 0
[end-of-instance]
[mk-app] #3489 not #3488
[inst-discovered] theory-solving 0 basic# ; #3489
[mk-app] #3490 = #3489 #3487
[instance] 0 #3490
[attach-enode] #3490 0
[end-of-instance]
[mk-app] #3488 or #3482 #3483 #3485 #3102 #3486 #3083
[mk-app] #3489 or #3484 #3487 #3083
[inst-discovered] theory-solving 0 basic# ; #3489
[mk-app] #3490 = #3489 #3488
[instance] 0 #3490
[attach-enode] #3490 0
[end-of-instance]
[mk-app] #3487 not #3488
[mk-app] #3484 or #2402 #3481 #3487
[inst-discovered] theory-solving 0 basic# ; #3484
[mk-app] #3489 = #3484 #3484
[instance] 0 #3489
[attach-enode] #3489 0
[end-of-instance]
[mk-app] #3489 not #3478
[mk-app] #3490 not #3484
[mk-app] #3491 or #3489 #3490
[mk-app] #3492 not #3491
[mk-app] #3493 and #3478 #3484
[inst-discovered] theory-solving 0 basic# ; #3493
[mk-app] #3494 = #3493 #3492
[instance] 0 #3494
[attach-enode] #3494 0
[end-of-instance]
[mk-quant] #3493 internal_lib!Chap28.MCSSSpec.MCSSSpec.is_mcss_of.?_definition 2 #2420 #3492
[attach-var-names] #3493 (|m!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #3494 or #2440 #3493
[mk-app] #3495 not #2458
[mk-app] #3496 not #2469
[mk-app] #3497 or #3495 #3496
[mk-app] #3498 not #3497
[inst-discovered] theory-solving 0 basic# ; #2470
[mk-app] #3499 = #2470 #3498
[instance] 0 #3499
[attach-enode] #3499 0
[end-of-instance]
[mk-app] #3499 = #3497 #2442
[mk-app] #3500 not #3499
[mk-app] #3501 = #2442 #3498
[inst-discovered] theory-solving 0 basic# ; #3501
[mk-app] #3502 = #3501 #3500
[instance] 0 #3502
[attach-enode] #3502 0
[end-of-instance]
[mk-app] #3498 not #3497
[inst-discovered] theory-solving 0 basic# ; #3500
[mk-app] #3498 = #3500 #3500
[instance] 0 #3498
[attach-enode] #3498 0
[end-of-instance]
[mk-quant] #3498 internal_req__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.MaxContigSubSumOptTrait.max_contig_sub_sum_opt._definition 3 #2455 #3500
[attach-var-names] #3498 (|a!| ; |Poly|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[inst-discovered] theory-solving 0 basic# ; #2485
[mk-app] #3501 = #2485 #2485
[instance] 0 #3501
[attach-enode] #3501 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2488
[mk-app] #3501 = #2488 #2488
[instance] 0 #3501
[attach-enode] #3501 0
[end-of-instance]
[mk-app] #3501 not #2460
[mk-app] #3502 not #2485
[mk-app] #3503 not #2488
[mk-app] #3504 not #2490
[mk-app] #3505 or #3501 #3502 #3503 #3504
[mk-app] #3506 not #3505
[inst-discovered] theory-solving 0 basic# ; #2491
[mk-app] #3507 = #2491 #3506
[instance] 0 #3507
[attach-enode] #3507 0
[end-of-instance]
[mk-app] #3507 = #3505 #2461
[mk-app] #3508 not #3507
[mk-app] #3509 = #2461 #3506
[inst-discovered] theory-solving 0 basic# ; #3509
[mk-app] #3510 = #3509 #3508
[instance] 0 #3510
[attach-enode] #3510 0
[end-of-instance]
[mk-app] #3506 not #3505
[inst-discovered] theory-solving 0 basic# ; #3508
[mk-app] #3506 = #3508 #3508
[instance] 0 #3506
[attach-enode] #3506 0
[end-of-instance]
[mk-quant] #3506 internal_ens__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.MaxContigSubSumOptTrait.max_contig_sub_sum_opt._definition 4 #2482 #3508
[attach-var-names] #3506 (|mcss!| ; |Poly|) (|a!| ; |Poly|) (|Self%&| ; |Type|) (|Self%&.| ; |Dcr|)
[mk-app] #3509 or #1850 #2502
[mk-app] #3510 not #3509
[inst-discovered] theory-solving 0 basic# ; #2294
[mk-app] #3511 = #2294 #3510
[instance] 0 #3511
[attach-enode] #3511 0
[end-of-instance]
[mk-app] #3511 not #3510
[inst-discovered] theory-solving 0 basic# ; #3511
[mk-app] #3512 = #3511 #3509
[instance] 0 #3512
[attach-enode] #3512 0
[end-of-instance]
[mk-app] #3511 or #1850 #2502 #2524
[mk-app] #3512 or #3509 #2524
[inst-discovered] theory-solving 0 basic# ; #3512
[mk-app] #3513 = #3512 #3511
[instance] 0 #3513
[attach-enode] #3513 0
[end-of-instance]
[mk-quant] #3512 internal_vstd!view.View.view.?_definition 3 #2526 #3511
[attach-var-names] #3512 (|self!| ; |Poly|) (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3509 or #2531 #3512
[mk-app] #3510 or #1850 #2502
[mk-app] #3513 not #3510
[inst-discovered] theory-solving 0 basic# ; #2294
[mk-app] #3514 = #2294 #3513
[instance] 0 #3514
[attach-enode] #3514 0
[end-of-instance]
[mk-app] #3514 not #3513
[inst-discovered] theory-solving 0 basic# ; #3514
[mk-app] #3515 = #3514 #3510
[instance] 0 #3515
[attach-enode] #3515 0
[end-of-instance]
[mk-app] #3514 or #1850 #2502 #2556
[mk-app] #3515 or #3510 #2556
[inst-discovered] theory-solving 0 basic# ; #3515
[mk-app] #3516 = #3515 #3514
[instance] 0 #3516
[attach-enode] #3516 0
[end-of-instance]
[mk-quant] #3515 internal_vstd!view.View.view.?_definition 3 #2558 #3514
[attach-var-names] #3515 (|self!| ; |Poly|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3510 or #2563 #3515
[mk-app] #3513 or #1493 #1365
[mk-app] #3516 not #3513
[inst-discovered] theory-solving 0 basic# ; #1438
[mk-app] #3517 = #1438 #3516
[instance] 0 #3517
[attach-enode] #3517 0
[end-of-instance]
[mk-app] #3517 not #3516
[inst-discovered] theory-solving 0 basic# ; #3517
[mk-app] #3518 = #3517 #3513
[instance] 0 #3518
[attach-enode] #3518 0
[end-of-instance]
[mk-app] #3517 or #1493 #1365 #2565
[mk-app] #3518 or #3513 #2565
[inst-discovered] theory-solving 0 basic# ; #3518
[mk-app] #3519 = #3518 #3517
[instance] 0 #3519
[attach-enode] #3519 0
[end-of-instance]
[mk-quant] #3518 internal_lib__Chap19__ArraySeqStEph__ArraySeqStEph__impl&__2_trait_impl_definition 2 #2567 #3517
[attach-var-names] #3518 (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3513 or #3434 #3433
[mk-app] #3516 not #3513
[inst-discovered] theory-solving 0 basic# ; #2588
[mk-app] #3519 = #2588 #3516
[instance] 0 #3519
[attach-enode] #3519 0
[end-of-instance]
[mk-app] #3519 not #3516
[inst-discovered] theory-solving 0 basic# ; #3519
[mk-app] #3520 = #3519 #3513
[instance] 0 #3520
[attach-enode] #3520 0
[end-of-instance]
[mk-app] #3519 or #3434 #3433 #2616
[mk-app] #3520 or #3513 #2616
[inst-discovered] theory-solving 0 basic# ; #3520
[mk-app] #3521 = #3520 #3519
[instance] 0 #3521
[attach-enode] #3521 0
[end-of-instance]
[mk-quant] #3520 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum._fuel_to_body_definition 3 #2603 #3519
[attach-var-names] #3520 (|fuel%| ; |Fuel|) (|k!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #3513 not #2605
[mk-app] #3516 or #3513 #197
[mk-app] #3521 not #3516
[inst-discovered] theory-solving 0 basic# ; #2612
[mk-app] #3522 = #2612 #3521
[instance] 0 #3522
[attach-enode] #3522 0
[end-of-instance]
[mk-app] #3522 not #3521
[inst-discovered] theory-solving 0 basic# ; #3522
[mk-app] #3523 = #3522 #3516
[instance] 0 #3523
[attach-enode] #3523 0
[end-of-instance]
[mk-app] #3522 or #3513 #197 #2622
[mk-app] #3523 or #3516 #2622
[inst-discovered] theory-solving 0 basic# ; #3523
[mk-app] #3524 = #3523 #3522
[instance] 0 #3524
[attach-enode] #3524 0
[end-of-instance]
[mk-quant] #3523 internal_lib!Chap28.MCSSSpec.MCSSSpec.spec_min_prefix_sum.?_definition 2 #2624 #3522
[attach-var-names] #3523 (|k!| ; |Poly|) (|s!| ; |Poly|)
[mk-app] #3516 or #2630 #3523
[mk-app] #3521 or #1493 #1365
[mk-app] #3524 not #3521
[inst-discovered] theory-solving 0 basic# ; #1438
[mk-app] #3525 = #1438 #3524
[instance] 0 #3525
[attach-enode] #3525 0
[end-of-instance]
[mk-app] #3525 not #3524
[inst-discovered] theory-solving 0 basic# ; #3525
[mk-app] #3526 = #3525 #3521
[instance] 0 #3526
[attach-enode] #3526 0
[end-of-instance]
[mk-app] #3525 or #1493 #1365 #2644
[mk-app] #3526 or #3521 #2644
[inst-discovered] theory-solving 0 basic# ; #3526
[mk-app] #3527 = #3526 #3525
[instance] 0 #3527
[attach-enode] #3527 0
[end-of-instance]
[mk-quant] #3526 internal_vstd__view__impl&__4_trait_impl_definition 2 #2646 #3525
[attach-var-names] #3526 (|A&| ; |Type|) (|A&.| ; |Dcr|)
[mk-app] #3521 or #1493 #3368 #2655
[mk-app] #3524 not #3521
[inst-discovered] theory-solving 0 basic# ; #2658
[mk-app] #3527 = #2658 #3524
[instance] 0 #3527
[attach-enode] #3527 0
[end-of-instance]
[mk-app] #3527 not #3524
[inst-discovered] theory-solving 0 basic# ; #3527
[mk-app] #3528 = #3527 #3521
[instance] 0 #3528
[attach-enode] #3528 0
[end-of-instance]
[mk-app] #3527 or #1493 #3368 #2655 #2660
[mk-app] #3528 or #3521 #2660
[inst-discovered] theory-solving 0 basic# ; #3528
[mk-app] #3529 = #3528 #3527
[instance] 0 #3529
[attach-enode] #3529 0
[end-of-instance]
[mk-quant] #3528 internal_alloc__boxed__impl&__49_trait_impl_definition 4 #2662 #3527
[attach-var-names] #3528 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3521 or #1493 #3368 #2655
[mk-app] #3524 not #3521
[inst-discovered] theory-solving 0 basic# ; #2658
[mk-app] #3529 = #2658 #3524
[instance] 0 #3529
[attach-enode] #3529 0
[end-of-instance]
[mk-app] #3529 not #3524
[inst-discovered] theory-solving 0 basic# ; #3529
[mk-app] #3530 = #3529 #3521
[instance] 0 #3530
[attach-enode] #3530 0
[end-of-instance]
[mk-app] #3529 or #1493 #3368 #2655 #2668
[mk-app] #3530 or #3521 #2668
[inst-discovered] theory-solving 0 basic# ; #3530
[mk-app] #3531 = #3530 #3529
[instance] 0 #3531
[attach-enode] #3531 0
[end-of-instance]
[mk-quant] #3530 internal_alloc__rc__impl&__115_trait_impl_definition 4 #2670 #3529
[attach-var-names] #3530 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3521 or #1493 #3368 #2655
[mk-app] #3524 not #3521
[inst-discovered] theory-solving 0 basic# ; #2658
[mk-app] #3531 = #2658 #3524
[instance] 0 #3531
[attach-enode] #3531 0
[end-of-instance]
[mk-app] #3531 not #3524
[inst-discovered] theory-solving 0 basic# ; #3531
[mk-app] #3532 = #3531 #3521
[instance] 0 #3532
[attach-enode] #3532 0
[end-of-instance]
[mk-app] #3531 or #1493 #3368 #2655 #2675
[mk-app] #3532 or #3521 #2675
[inst-discovered] theory-solving 0 basic# ; #3532
[mk-app] #3533 = #3532 #3531
[instance] 0 #3533
[attach-enode] #3533 0
[end-of-instance]
[mk-quant] #3532 internal_alloc__sync__impl&__117_trait_impl_definition 4 #2677 #3531
[attach-var-names] #3532 (|A&| ; |Type|) (|A&.| ; |Dcr|) (|T&| ; |Type|) (|T&.| ; |Dcr|)
[mk-app] #3521 not #2698
[mk-app] #3524 not #2705
[mk-app] #3533 not #2710
[mk-app] #3534 or #3521 #3524 #3533
[mk-app] #3535 not #3534
[inst-discovered] theory-solving 0 basic# ; #2711
[mk-app] #3536 = #2711 #3535
[instance] 0 #3536
[attach-enode] #3536 0
[end-of-instance]
[mk-app] #3536 = #3534 #2682
[mk-app] #3537 not #3536
[mk-app] #3538 = #2682 #3535
[inst-discovered] theory-solving 0 basic# ; #3538
[mk-app] #3539 = #3538 #3537
[instance] 0 #3539
[attach-enode] #3539 0
[end-of-instance]
[mk-app] #3535 not #3534
[inst-discovered] theory-solving 0 basic# ; #3537
[mk-app] #3535 = #3537 #3537
[instance] 0 #3535
[attach-enode] #3535 0
[end-of-instance]
[mk-quant] #3535 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_snoc._definition 3 #2695 #3537
[attach-var-names] #3535 (|hi!| ; |Int|) (|lo!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #3538 not #2734
[mk-app] #3539 not #2708
[mk-app] #3540 or #1048 #3538 #3539
[mk-app] #3541 not #3540
[inst-discovered] theory-solving 0 basic# ; #2735
[mk-app] #3542 = #2735 #3541
[instance] 0 #3542
[attach-enode] #3542 0
[end-of-instance]
[mk-app] #3542 or #2736 #3541
[mk-app] #3543 = #2725 #3542
[mk-quant] #3544 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_via_prefix._definition 3 #2732 #3543
[attach-var-names] #3544 (|hi!| ; |Int|) (|lo!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #3545 not #3086
[mk-app] #3546 not #3089
[mk-app] #3547 not #3127
[mk-app] #3548 not #3116
[mk-app] #3549 or #3545 #3546 #3547 #3548
[mk-app] #3550 not #3549
[inst-discovered] theory-solving 0 basic# ; #3124
[mk-app] #3551 = #3124 #3550
[instance] 0 #3551
[attach-enode] #3551 0
[end-of-instance]
[mk-app] #3551 or #3060 #3550
[mk-app] #3552 not #2773
[mk-app] #3553 not #2764
[mk-app] #3554 or #197 #3260 #3552 #3553
[mk-app] #3555 not #3554
[inst-discovered] theory-solving 0 basic# ; #2774
[mk-app] #3556 = #2774 #3555
[instance] 0 #3556
[attach-enode] #3556 0
[end-of-instance]
[mk-app] #3556 not #3555
[inst-discovered] theory-solving 0 basic# ; #3556
[mk-app] #3557 = #3556 #3554
[instance] 0 #3557
[attach-enode] #3557 0
[end-of-instance]
[mk-quant] #3555 user_lib__Chap28__MCSSSpec__MCSSSpec__lemma_min_prefix_sum_achieved_20 1 #2767 #3554
[attach-var-names] #3555 (|j$| ; |Poly|)
[mk-app] #3556 or #2759 #3555
[mk-app] #3557 not #3551
[mk-app] #3558 not #3556
[mk-app] #3559 or #3557 #3558
[mk-app] #3560 not #3559
[mk-app] #3561 and #3551 #3556
[inst-discovered] theory-solving 0 basic# ; #3561
[mk-app] #3562 = #3561 #3560
[instance] 0 #3562
[attach-enode] #3562 0
[end-of-instance]
[mk-quant] #3561 internal_ens__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_achieved._definition 2 #2770 #3560
[attach-var-names] #3561 (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #3562 or #350 #2703
[mk-app] #3563 not #3562
[inst-discovered] theory-solving 0 basic# ; #2786
[mk-app] #3564 = #2786 #3563
[instance] 0 #3564
[attach-enode] #3564 0
[end-of-instance]
[mk-app] #3564 or #2787 #3563
[mk-app] #3565 = #2778 #3564
[mk-quant] #3566 internal_req__lib!Chap28.MCSSSpec.MCSSSpec.lemma_min_prefix_sum_is_min._definition 3 #2784 #3565
[attach-var-names] #3566 (|j!| ; |Int|) (|k!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[mk-app] #3567 not #3123
[mk-app] #3568 not #3132
[mk-app] #3569 not #3178
[mk-app] #3570 not #3140
[mk-app] #3571 or #3567 #3568 #3569 #3570
[mk-app] #3572 not #3571
[inst-discovered] theory-solving 0 basic# ; #3175
[mk-app] #3573 = #3175 #3572
[instance] 0 #3573
[attach-enode] #3573 0
[end-of-instance]
[mk-app] #3573 or #2843 #3572
[mk-app] #3574 not #2844
[mk-app] #3575 or #3574 #3552
[mk-app] #3576 not #3575
[inst-discovered] theory-solving 0 basic# ; #2848
[mk-app] #3577 = #2848 #3576
[instance] 0 #3577
[attach-enode] #3577 0
[end-of-instance]
[mk-app] #3577 not #3576
[inst-discovered] theory-solving 0 basic# ; #3577
[mk-app] #3578 = #3577 #3575
[instance] 0 #3578
[attach-enode] #3578 0
[end-of-instance]
[mk-app] #3576 or #197 #3574 #3552 #2851
[mk-app] #3577 or #197 #3575 #2851
[inst-discovered] theory-solving 0 basic# ; #3577
[mk-app] #3578 = #3577 #3576
[instance] 0 #3578
[attach-enode] #3578 0
[end-of-instance]
[mk-quant] #3575 user_lib__Chap28__MaxContigSubSumOptStEph__MaxContigSubSumOptStEph__lemma_prefix_opt_is_mcss_22 1 #2824 #3576
[attach-var-names] #3575 (|hi$| ; |Poly|)
[mk-app] #3577 or #2855 #3575
[mk-app] #3578 not #3573
[mk-app] #3579 not #3577
[mk-app] #3580 or #3144 #3145 #3146 #3578 #3579
[mk-app] #3581 not #3580
[mk-app] #3582 and #2838 #2840 #2842 #3573 #3577
[inst-discovered] theory-solving 0 basic# ; #3582
[mk-app] #3583 = #3582 #3581
[instance] 0 #3583
[attach-enode] #3583 0
[end-of-instance]
[mk-app] #3582 or #3121 #3581
[mk-app] #3583 not #2821
[mk-app] #3584 or #197 #3574 #3552 #3583
[mk-app] #3585 not #3584
[inst-discovered] theory-solving 0 basic# ; #2845
[mk-app] #3586 = #2845 #3585
[instance] 0 #3586
[attach-enode] #3586 0
[end-of-instance]
[mk-app] #3586 not #3585
[inst-discovered] theory-solving 0 basic# ; #3586
[mk-app] #3587 = #3586 #3584
[instance] 0 #3587
[attach-enode] #3587 0
[end-of-instance]
[mk-quant] #3585 user_lib__Chap28__MaxContigSubSumOptStEph__MaxContigSubSumOptStEph__lemma_prefix_opt_is_mcss_21 1 #2824 #3584
[attach-var-names] #3585 (|hi$| ; |Poly|)
[mk-app] #3586 not #3585
[mk-app] #3587 or #2843 #3586
[mk-app] #3588 not #3587
[mk-app] #3589 and #2811 #3585
[inst-discovered] theory-solving 0 basic# ; #3589
[mk-app] #3590 = #3589 #3588
[instance] 0 #3590
[attach-enode] #3590 0
[end-of-instance]
[mk-app] #3589 not #3154
[mk-app] #3590 not #3185
[mk-app] #3591 or #3589 #3590
[mk-app] #3592 not #3591
[inst-discovered] theory-solving 0 basic# ; #3182
[mk-app] #3593 = #3182 #3592
[instance] 0 #3593
[attach-enode] #3593 0
[end-of-instance]
[mk-app] #3593 not #3592
[inst-discovered] theory-solving 0 basic# ; #3593
[mk-app] #3594 = #3593 #3591
[instance] 0 #3594
[attach-enode] #3594 0
[end-of-instance]
[mk-app] #3592 or #3152 #3589 #3590 #3166
[mk-app] #3593 or #3152 #3591 #3166
[inst-discovered] theory-solving 0 basic# ; #3593
[mk-app] #3594 = #3593 #3592
[instance] 0 #3594
[attach-enode] #3594 0
[end-of-instance]
[mk-app] #3591 not #3592
[mk-app] #3593 or #2855 #3152 #3589 #3590 #3166
[mk-app] #3594 not #3593
[mk-app] #3595 and #2827 #3591
[inst-discovered] theory-solving 0 basic# ; #3595
[mk-app] #3596 = #3595 #3594
[instance] 0 #3596
[attach-enode] #3596 0
[end-of-instance]
[mk-app] #3592 or #2801 #3144 #3145 #3146 #3588 #3594
[inst-discovered] theory-solving 0 basic# ; #3592
[mk-app] #3591 = #3592 #3592
[instance] 0 #3591
[attach-enode] #3591 0
[end-of-instance]
[mk-app] #3591 not #3582
[mk-app] #3595 not #3592
[mk-app] #3596 or #3591 #3595
[mk-app] #3597 not #3596
[mk-app] #3598 and #3582 #3592
[inst-discovered] theory-solving 0 basic# ; #3598
[mk-app] #3599 = #3598 #3597
[instance] 0 #3599
[attach-enode] #3599 0
[end-of-instance]
[mk-quant] #3598 internal_req__lib!Chap28.MaxContigSubSumOptStEph.MaxContigSubSumOptStEph.lemma_prefix_opt_is_mcss._definition 3 #2835 #3597
[attach-var-names] #3598 (|n!| ; |Int|) (|m!| ; |Int|) (|s!| ; |vstd!seq.Seq<i32.>.|)
[inst-discovered] theory-solving 0 basic# ; #3075
[mk-app] #3175 = #3075 #3075
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2924
[mk-app] #3175 = #2924 #2924
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2775
[mk-app] #3175 = #2775 #2775
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2948
[mk-app] #3175 = #2948 #2948
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2951
[mk-app] #3175 = #2951 #2951
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2957
[mk-app] #3175 = #2957 #2957
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3157
[mk-app] #3175 = #3157 #3157
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3169
[mk-app] #3175 = #3169 #3169
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3021
[mk-app] #3175 = #3021 #3021
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #2989
[inst-discovered] theory-solving 0 basic# ; #1085
[mk-app] #3175 = #1085 #1085
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #2990
[inst-discovered] theory-solving 0 basic# ; #1135
[mk-app] #3175 = #1135 #1135
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3169
[mk-app] #3175 = #3169 #3169
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3021
[mk-app] #3175 = #3021 #3021
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3091
[mk-app] #3175 = #3091 #3091
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3049
[mk-app] #3175 = #3049 #3049
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #998
[mk-app] #3175 = #998 #998
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2859
[mk-app] #3175 = #2859 #2859
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2424
[mk-app] #3175 = #2424 #2424
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2441
[mk-app] #3175 = #2441 #2441
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2397
[mk-app] #3175 = #2397 #2397
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2230
[mk-app] #3175 = #2230 #2230
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1796
[mk-app] #3175 = #1796 #1796
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1799
[mk-app] #3175 = #1799 #1799
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1772
[mk-app] #3175 = #1772 #1772
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1139
[mk-app] #3175 = #1139 #1139
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1088
[mk-app] #3175 = #1088 #1088
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1051
[mk-app] #3175 = #1051 #1051
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1053
[mk-app] #3175 = #1053 #1053
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #962
[inst-discovered] theory-solving 0 basic# ; #3172
[mk-app] #3175 = #3172 #3172
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3202
[mk-app] #3175 = #3202 #3202
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #905
[mk-app] #3175 = #905 #905
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #924
[mk-app] #3175 = #924 #924
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3209
[mk-app] #3175 = #3209 #3209
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3216
[mk-app] #3175 = #3216 #3216
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3218
[mk-app] #3175 = #3218 #3218
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2882
[mk-app] #3175 = #2882 #2882
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3229
[mk-app] #3175 = #3229 #3229
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3231
[mk-app] #3175 = #3231 #3231
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3235
[mk-app] #3175 = #3235 #3235
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3242
[mk-app] #3175 = #3242 #3242
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3246
[mk-app] #3175 = #3246 #3246
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3247
[mk-app] #3175 = #3247 #3247
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3254
[mk-app] #3175 = #3254 #3254
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3247
[mk-app] #3175 = #3247 #3247
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3270
[mk-app] #3175 = #3270 #3270
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3272
[mk-app] #3175 = #3272 #3272
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3285
[mk-app] #3175 = #3285 #3285
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3290
[mk-app] #3175 = #3290 #3290
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3293
[mk-app] #3175 = #3293 #3293
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3297
[mk-app] #3175 = #3297 #3297
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3300
[mk-app] #3175 = #3300 #3300
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3307
[mk-app] #3175 = #3307 #3307
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3309
[mk-app] #3175 = #3309 #3309
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3311
[mk-app] #3175 = #3311 #3311
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3319
[mk-app] #3175 = #3319 #3319
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3321
[mk-app] #3175 = #3321 #3321
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3325
[mk-app] #3175 = #3325 #3325
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3332
[mk-app] #3175 = #3332 #3332
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3333
[mk-app] #3175 = #3333 #3333
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3336
[mk-app] #3175 = #3336 #3336
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3338
[mk-app] #3175 = #3338 #3338
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3350
[mk-app] #3175 = #3350 #3350
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3353
[mk-app] #3175 = #3353 #3353
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3355
[mk-app] #3175 = #3355 #3355
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3358
[mk-app] #3175 = #3358 #3358
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3360
[mk-app] #3175 = #3360 #3360
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3369
[mk-app] #3175 = #3369 #3369
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3373
[mk-app] #3175 = #3373 #3373
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3376
[mk-app] #3175 = #3376 #3376
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3380
[mk-app] #3175 = #3380 #3380
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3383
[mk-app] #3175 = #3383 #3383
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2004
[mk-app] #3175 = #2004 #2004
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2017
[mk-app] #3175 = #2017 #2017
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3387
[mk-app] #3175 = #3387 #3387
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3389
[inst-discovered] theory-solving 0 basic# ; #3392
[mk-app] #3175 = #3392 #3392
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3395
[inst-discovered] theory-solving 0 basic# ; #3398
[mk-app] #3175 = #3398 #3398
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3401
[mk-app] #3175 = #3401 #3401
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3400
[inst-discovered] theory-solving 0 basic# ; #3405
[mk-app] #3175 = #3405 #3405
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3407
[inst-discovered] theory-solving 0 basic# ; #3410
[mk-app] #3175 = #3410 #3410
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3412
[mk-app] #3175 = #3412 #3412
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3419
[mk-app] #3175 = #3419 #3419
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3420
[mk-app] #3175 = #3420 #3420
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3423
[mk-app] #3175 = #3423 #3423
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3436
[mk-app] #3175 = #3436 #3436
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3439
[mk-app] #3175 = #3439 #3439
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3447
[mk-app] #3175 = #3447 #3447
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3449
[mk-app] #3175 = #3449 #3449
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3458
[mk-app] #3175 = #3458 #3458
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3460
[mk-app] #3175 = #3460 #3460
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3463
[mk-app] #3175 = #3463 #3463
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3471
[mk-app] #3175 = #3471 #3471
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3476
[mk-app] #3175 = #3476 #3476
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3480
[mk-app] #3175 = #3480 #3480
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3488
[mk-app] #3175 = #3488 #3488
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3484
[mk-app] #3175 = #3484 #3484
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3491
[mk-app] #3175 = #3491 #3491
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3497
[inst-discovered] theory-solving 0 basic# ; #3500
[mk-app] #3175 = #3500 #3500
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2485
[mk-app] #3175 = #2485 #2485
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2488
[mk-app] #3175 = #2488 #2488
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3505
[mk-app] #3175 = #3505 #3505
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3505
[inst-discovered] theory-solving 0 basic# ; #3508
[mk-app] #3175 = #3508 #3508
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3519
[mk-app] #3175 = #3519 #3519
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3522
[mk-app] #3175 = #3522 #3522
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3527
[mk-app] #3175 = #3527 #3527
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3529
[mk-app] #3175 = #3529 #3529
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3531
[mk-app] #3175 = #3531 #3531
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3534
[inst-discovered] theory-solving 0 basic# ; #3537
[mk-app] #3175 = #3537 #3537
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3549
[mk-app] #3175 = #3549 #3549
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3559
[mk-app] #3175 = #3559 #3559
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3580
[mk-app] #3175 = #3580 #3580
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3592
[mk-app] #3175 = #3592 #3592
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3596
[mk-app] #3175 = #3596 #3596
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3075
[mk-app] #3175 = #3075 #3075
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2924
[mk-app] #3175 = #2924 #2924
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2775
[mk-app] #3175 = #2775 #2775
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2948
[mk-app] #3175 = #2948 #2948
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2951
[mk-app] #3175 = #2951 #2951
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2957
[mk-app] #3175 = #2957 #2957
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3157
[mk-app] #3175 = #3157 #3157
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3169
[mk-app] #3175 = #3169 #3169
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3021
[mk-app] #3175 = #3021 #3021
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #2989
[inst-discovered] theory-solving 0 basic# ; #1085
[mk-app] #3175 = #1085 #1085
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #2990
[inst-discovered] theory-solving 0 basic# ; #1135
[mk-app] #3175 = #1135 #1135
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3169
[mk-app] #3175 = #3169 #3169
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3021
[mk-app] #3175 = #3021 #3021
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3091
[mk-app] #3175 = #3091 #3091
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3049
[mk-app] #3175 = #3049 #3049
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #998
[mk-app] #3175 = #998 #998
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2859
[mk-app] #3175 = #2859 #2859
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2424
[mk-app] #3175 = #2424 #2424
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2441
[mk-app] #3175 = #2441 #2441
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2397
[mk-app] #3175 = #2397 #2397
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2230
[mk-app] #3175 = #2230 #2230
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1796
[mk-app] #3175 = #1796 #1796
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1799
[mk-app] #3175 = #1799 #1799
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1772
[mk-app] #3175 = #1772 #1772
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1139
[mk-app] #3175 = #1139 #1139
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1088
[mk-app] #3175 = #1088 #1088
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1051
[mk-app] #3175 = #1051 #1051
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #1053
[mk-app] #3175 = #1053 #1053
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #962
[inst-discovered] theory-solving 0 basic# ; #3172
[mk-app] #3175 = #3172 #3172
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #905
[mk-app] #3175 = #905 #905
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #924
[mk-app] #3175 = #924 #924
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3216
[mk-app] #3175 = #3216 #3216
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3218
[mk-app] #3175 = #3218 #3218
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2882
[mk-app] #3175 = #2882 #2882
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3229
[mk-app] #3175 = #3229 #3229
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3231
[mk-app] #3175 = #3231 #3231
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3235
[mk-app] #3175 = #3235 #3235
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3242
[mk-app] #3175 = #3242 #3242
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3246
[mk-app] #3175 = #3246 #3246
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3247
[mk-app] #3175 = #3247 #3247
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3254
[mk-app] #3175 = #3254 #3254
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3247
[mk-app] #3175 = #3247 #3247
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3270
[mk-app] #3175 = #3270 #3270
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3272
[mk-app] #3175 = #3272 #3272
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3285
[mk-app] #3175 = #3285 #3285
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3290
[mk-app] #3175 = #3290 #3290
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3293
[mk-app] #3175 = #3293 #3293
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3297
[mk-app] #3175 = #3297 #3297
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3300
[mk-app] #3175 = #3300 #3300
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3307
[mk-app] #3175 = #3307 #3307
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3309
[mk-app] #3175 = #3309 #3309
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3311
[mk-app] #3175 = #3311 #3311
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3319
[mk-app] #3175 = #3319 #3319
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3321
[mk-app] #3175 = #3321 #3321
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3325
[mk-app] #3175 = #3325 #3325
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3332
[mk-app] #3175 = #3332 #3332
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3333
[mk-app] #3175 = #3333 #3333
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3336
[mk-app] #3175 = #3336 #3336
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3338
[mk-app] #3175 = #3338 #3338
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3350
[mk-app] #3175 = #3350 #3350
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3353
[mk-app] #3175 = #3353 #3353
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3355
[mk-app] #3175 = #3355 #3355
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3358
[mk-app] #3175 = #3358 #3358
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3360
[mk-app] #3175 = #3360 #3360
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3369
[mk-app] #3175 = #3369 #3369
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3373
[mk-app] #3175 = #3373 #3373
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3376
[mk-app] #3175 = #3376 #3376
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3380
[mk-app] #3175 = #3380 #3380
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3383
[mk-app] #3175 = #3383 #3383
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2004
[mk-app] #3175 = #2004 #2004
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2017
[mk-app] #3175 = #2017 #2017
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3387
[mk-app] #3175 = #3387 #3387
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3389
[inst-discovered] theory-solving 0 basic# ; #3392
[mk-app] #3175 = #3392 #3392
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3395
[inst-discovered] theory-solving 0 basic# ; #3398
[mk-app] #3175 = #3398 #3398
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3401
[mk-app] #3175 = #3401 #3401
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3400
[inst-discovered] theory-solving 0 basic# ; #3405
[mk-app] #3175 = #3405 #3405
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3407
[inst-discovered] theory-solving 0 basic# ; #3410
[mk-app] #3175 = #3410 #3410
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3412
[mk-app] #3175 = #3412 #3412
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3419
[mk-app] #3175 = #3419 #3419
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3420
[mk-app] #3175 = #3420 #3420
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3423
[mk-app] #3175 = #3423 #3423
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3436
[mk-app] #3175 = #3436 #3436
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3439
[mk-app] #3175 = #3439 #3439
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3447
[mk-app] #3175 = #3447 #3447
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3449
[mk-app] #3175 = #3449 #3449
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3458
[mk-app] #3175 = #3458 #3458
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3460
[mk-app] #3175 = #3460 #3460
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3463
[mk-app] #3175 = #3463 #3463
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3471
[mk-app] #3175 = #3471 #3471
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3476
[mk-app] #3175 = #3476 #3476
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3480
[mk-app] #3175 = #3480 #3480
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3488
[mk-app] #3175 = #3488 #3488
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3484
[mk-app] #3175 = #3484 #3484
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3491
[mk-app] #3175 = #3491 #3491
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3497
[inst-discovered] theory-solving 0 basic# ; #3500
[mk-app] #3175 = #3500 #3500
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2485
[mk-app] #3175 = #2485 #2485
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #2488
[mk-app] #3175 = #2488 #2488
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3505
[mk-app] #3175 = #3505 #3505
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3505
[inst-discovered] theory-solving 0 basic# ; #3508
[mk-app] #3175 = #3508 #3508
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3519
[mk-app] #3175 = #3519 #3519
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3522
[mk-app] #3175 = #3522 #3522
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3527
[mk-app] #3175 = #3527 #3527
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3529
[mk-app] #3175 = #3529 #3529
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3531
[mk-app] #3175 = #3531 #3531
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 not #3534
[inst-discovered] theory-solving 0 basic# ; #3537
[mk-app] #3175 = #3537 #3537
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3549
[mk-app] #3175 = #3549 #3549
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3559
[mk-app] #3175 = #3559 #3559
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3580
[mk-app] #3175 = #3580 #3580
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3592
[mk-app] #3175 = #3592 #3592
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3596
[mk-app] #3175 = #3596 #3596
[instance] 0 #3175
[attach-enode] #3175 0
[end-of-instance]
[mk-app] #3175 or #899 #884
[mk-app] #3179 or #899 #885
[mk-app] #2848 or #899 #886
[mk-app] #2852 or #899 #887
[mk-app] #2854 or #899 #888
[mk-app] #2853 or #899 #889
[mk-app] #2856 or #899 #890
[mk-app] #3180 or #899 #891
[mk-app] #3181 or #899 #892
[mk-app] #2845 or #899 #893
[mk-app] #3147 or #899 #894
[mk-app] #3148 or #899 #895
[mk-app] #3149 or #899 #896
[mk-app] #3182 or #939 #933
[mk-app] #3186 or #939 #934
[mk-app] #3187 or #939 #935
[mk-app] #3188 or #939 #936
[assign] #39 justification -1: 
[assign] #43 justification -1: 
[assign] #56 justification -1: 
[assign] #63 justification -1: 
[assign] #3076 justification -1: 
[assign] #88 justification -1: 
[assign] #92 justification -1: 
[assign] #96 justification -1: 
[assign] #100 justification -1: 
[assign] #104 justification -1: 
[assign] #108 justification -1: 
[assign] #112 justification -1: 
[assign] #116 justification -1: 
[assign] #120 justification -1: 
[assign] #124 justification -1: 
[attach-enode] #125 0
[attach-enode] #126 0
[assign] #126 justification -1: 
[assign] #132 justification -1: 
[assign] #138 justification -1: 
[assign] #143 justification -1: 
[assign] #148 justification -1: 
[assign] #3079 justification -1: 
[assign] #165 justification -1: 
[assign] #169 justification -1: 
[assign] #174 justification -1: 
[assign] #178 justification -1: 
[assign] #188 justification -1: 
[assign] #199 justification -1: 
[assign] #207 justification -1: 
[assign] #215 justification -1: 
[assign] #223 justification -1: 
[assign] #232 justification -1: 
[assign] #240 justification -1: 
[assign] #248 justification -1: 
[assign] #256 justification -1: 
[assign] #266 justification -1: 
[assign] #273 justification -1: 
[attach-enode] #274 0
[attach-enode] #275 0
[attach-enode] #276 0
[attach-enode] #277 0
[attach-enode] #278 0
[attach-enode] #280 0
[attach-enode] #281 0
[attach-enode] #282 0
[attach-enode] #283 0
[assign] #283 justification -1: 
[attach-enode] #284 0
[attach-enode] #285 0
[attach-enode] #286 0
[attach-enode] #287 0
[assign] #287 justification -1: 
[attach-enode] #288 0
[attach-enode] #289 0
[attach-enode] #290 0
[assign] #290 justification -1: 
[attach-enode] #291 0
[attach-enode] #292 0
[attach-enode] #293 0
[assign] #293 justification -1: 
[attach-enode] #294 0
[attach-enode] #295 0
[attach-enode] #300 0
[attach-enode] #301 0
[assign] #301 justification -1: 
[attach-enode] #302 0
[attach-enode] #305 0
[attach-enode] #306 0
[assign] #306 justification -1: 
[attach-enode] #307 0
[attach-enode] #311 0
[attach-enode] #312 0
[assign] #312 justification -1: 
[attach-enode] #313 0
[attach-enode] #317 0
[attach-enode] #318 0
[assign] #318 justification -1: 
[attach-enode] #319 0
[attach-enode] #323 0
[attach-enode] #324 0
[assign] #324 justification -1: 
[attach-enode] #325 0
[attach-enode] #329 0
[attach-enode] #330 0
[assign] #330 justification -1: 
[attach-enode] #331 0
[attach-enode] #332 0
[assign] #332 justification -1: 
[attach-enode] #333 0
[attach-enode] #308 0
[attach-enode] #334 0
[assign] #334 justification -1: 
[attach-enode] #335 0
[attach-enode] #314 0
[attach-enode] #336 0
[assign] #336 justification -1: 
[attach-enode] #337 0
[attach-enode] #320 0
[attach-enode] #338 0
[assign] #338 justification -1: 
[attach-enode] #339 0
[attach-enode] #326 0
[attach-enode] #340 0
[assign] #340 justification -1: 
[assign] #2918 justification -1: 
[assign] #2432 justification -1: 
[assign] #2956 justification -1: 
[assign] #2985 justification -1: 
[assign] #1086 justification -1: 
[assign] #1136 justification -1: 
[assign] #1047 justification -1: 
[assign] #467 justification -1: 
[assign] #473 justification -1: 
[assign] #481 justification -1: 
[assign] #489 justification -1: 
[assign] #496 justification -1: 
[assign] #503 justification -1: 
[assign] #509 justification -1: 
[assign] #516 justification -1: 
[assign] #522 justification -1: 
[assign] #527 justification -1: 
[assign] #532 justification -1: 
[assign] #537 justification -1: 
[assign] #542 justification -1: 
[assign] #546 justification -1: 
[assign] #552 justification -1: 
[assign] #567 justification -1: 
[assign] #574 justification -1: 
[assign] #583 justification -1: 
[assign] #592 justification -1: 
[assign] #598 justification -1: 
[assign] #615 justification -1: 
[assign] #622 justification -1: 
[assign] #631 justification -1: 
[assign] #1049 justification -1: 
[assign] #3050 justification -1: 
[assign] #999 justification -1: 
[assign] #2776 justification -1: 
[assign] #2439 justification -1: 
[assign] #2396 justification -1: 
[assign] #2399 justification -1: 
[assign] #2239 justification -1: 
[assign] #1800 justification -1: 
[assign] #2423 justification -1: 
[assign] #1802 justification -1: 
[assign] #760 justification -1: 
[assign] #1090 justification -1: 
[assign] #1001 justification -1: 
[assign] #3142 justification -1: 
[mk-app] #3211 distinct-aux-f!!19 #801
[mk-app] #3204 unique-value!20
[attach-enode] #3204 0
[mk-app] #3189 = #3211 #3204
[attach-enode] #801 0
[attach-enode] #3211 0
[attach-enode] #3189 0
[assign] #3189 justification -1: 
[mk-app] #3190 distinct-aux-f!!19 #802
[mk-app] #3191 unique-value!21
[attach-enode] #3191 0
[mk-app] #3192 = #3190 #3191
[attach-enode] #802 0
[attach-enode] #3190 0
[attach-enode] #3192 0
[assign] #3192 justification -1: 
[mk-app] #2786 distinct-aux-f!!19 #803
[mk-app] #2788 unique-value!22
[attach-enode] #2788 0
[mk-app] #2789 = #2786 #2788
[attach-enode] #803 0
[attach-enode] #2786 0
[attach-enode] #2789 0
[assign] #2789 justification -1: 
[mk-app] #2790 distinct-aux-f!!19 #804
[mk-app] #3124 unique-value!23
[attach-enode] #3124 0
[mk-app] #3128 = #2790 #3124
[attach-enode] #804 0
[attach-enode] #2790 0
[attach-enode] #3128 0
[assign] #3128 justification -1: 
[mk-app] #2774 distinct-aux-f!!19 #805
[mk-app] #3118 unique-value!24
[attach-enode] #3118 0
[mk-app] #3119 = #2774 #3118
[attach-enode] #805 0
[attach-enode] #2774 0
[attach-enode] #3119 0
[assign] #3119 justification -1: 
[mk-app] #3120 distinct-aux-f!!19 #806
[mk-app] #3129 unique-value!25
[attach-enode] #3129 0
[mk-app] #3130 = #3120 #3129
[attach-enode] #806 0
[attach-enode] #3120 0
[attach-enode] #3130 0
[assign] #3130 justification -1: 
[mk-app] #2735 distinct-aux-f!!19 #807
[mk-app] #2737 unique-value!26
[attach-enode] #2737 0
[mk-app] #2738 = #2735 #2737
[attach-enode] #807 0
[attach-enode] #2735 0
[attach-enode] #2738 0
[assign] #2738 justification -1: 
[mk-app] #2739 distinct-aux-f!!19 #808
[mk-app] #2711 unique-value!27
[attach-enode] #2711 0
[mk-app] #2712 = #2739 #2711
[attach-enode] #808 0
[attach-enode] #2739 0
[attach-enode] #2712 0
[assign] #2712 justification -1: 
[mk-app] #2713 distinct-aux-f!!19 #809
[mk-app] #2664 unique-value!28
[attach-enode] #2664 0
[mk-app] #2679 = #2713 #2664
[attach-enode] #809 0
[attach-enode] #2713 0
[attach-enode] #2679 0
[assign] #2679 justification -1: 
[mk-app] #2680 distinct-aux-f!!19 #810
[mk-app] #2672 unique-value!29
[attach-enode] #2672 0
[mk-app] #2673 = #2680 #2672
[attach-enode] #810 0
[attach-enode] #2680 0
[attach-enode] #2673 0
[assign] #2673 justification -1: 
[mk-app] #2665 distinct-aux-f!!19 #811
[mk-app] #2666 unique-value!30
[attach-enode] #2666 0
[mk-app] #1445 = #2665 #2666
[attach-enode] #811 0
[attach-enode] #2665 0
[attach-enode] #1445 0
[assign] #1445 justification -1: 
[mk-app] #2648 distinct-aux-f!!19 #812
[mk-app] #2649 unique-value!31
[attach-enode] #2649 0
[mk-app] #2627 = #2648 #2649
[attach-enode] #812 0
[attach-enode] #2648 0
[attach-enode] #2627 0
[assign] #2627 justification -1: 
[mk-app] #2628 distinct-aux-f!!19 #813
[mk-app] #2629 unique-value!32
[attach-enode] #2629 0
[mk-app] #2631 = #2628 #2629
[attach-enode] #813 0
[attach-enode] #2628 0
[attach-enode] #2631 0
[assign] #2631 justification -1: 
[mk-app] #2617 distinct-aux-f!!19 #814
[mk-app] #2618 unique-value!33
[attach-enode] #2618 0
[mk-app] #2619 = #2617 #2618
[attach-enode] #814 0
[attach-enode] #2617 0
[attach-enode] #2619 0
[assign] #2619 justification -1: 
[mk-app] #2569 distinct-aux-f!!19 #815
[mk-app] #2570 unique-value!34
[attach-enode] #2570 0
[mk-app] #2302 = #2569 #2570
[attach-enode] #815 0
[attach-enode] #2569 0
[attach-enode] #2302 0
[assign] #2302 justification -1: 
[mk-app] #2561 distinct-aux-f!!19 #816
[mk-app] #2562 unique-value!35
[attach-enode] #2562 0
[mk-app] #2564 = #2561 #2562
[attach-enode] #816 0
[attach-enode] #2561 0
[attach-enode] #2564 0
[assign] #2564 justification -1: 
[mk-app] #2529 distinct-aux-f!!19 #817
[mk-app] #2530 unique-value!36
[attach-enode] #2530 0
[mk-app] #2532 = #2529 #2530
[attach-enode] #817 0
[attach-enode] #2529 0
[attach-enode] #2532 0
[assign] #2532 justification -1: 
[mk-app] #2491 distinct-aux-f!!19 #818
[mk-app] #2492 unique-value!37
[attach-enode] #2492 0
[mk-app] #2493 = #2491 #2492
[attach-enode] #818 0
[attach-enode] #2491 0
[attach-enode] #2493 0
[assign] #2493 justification -1: 
[mk-app] #2470 distinct-aux-f!!19 #819
[mk-app] #2471 unique-value!38
[attach-enode] #2471 0
[mk-app] #2472 = #2470 #2471
[attach-enode] #819 0
[attach-enode] #2470 0
[attach-enode] #2472 0
[assign] #2472 justification -1: 
[mk-app] #3092 distinct-aux-f!!19 #820
[mk-app] #2394 unique-value!39
[attach-enode] #2394 0
[mk-app] #2426 = #3092 #2394
[attach-enode] #820 0
[attach-enode] #3092 0
[attach-enode] #2426 0
[assign] #2426 justification -1: 
[mk-app] #2430 distinct-aux-f!!19 #821
[mk-app] #2436 unique-value!40
[attach-enode] #2436 0
[mk-app] #2438 = #2430 #2436
[attach-enode] #821 0
[attach-enode] #2430 0
[attach-enode] #2438 0
[assign] #2438 justification -1: 
[mk-app] #2437 distinct-aux-f!!19 #822
[mk-app] #3097 unique-value!41
[attach-enode] #3097 0
[mk-app] #3098 = #2437 #3097
[attach-enode] #822 0
[attach-enode] #2437 0
[attach-enode] #3098 0
[assign] #3098 justification -1: 
[mk-app] #2431 distinct-aux-f!!19 #823
[mk-app] #3061 unique-value!42
[attach-enode] #3061 0
[mk-app] #3062 = #2431 #3061
[attach-enode] #823 0
[attach-enode] #2431 0
[attach-enode] #3062 0
[assign] #3062 justification -1: 
[mk-app] #3067 distinct-aux-f!!19 #824
[mk-app] #3068 unique-value!43
[attach-enode] #3068 0
[mk-app] #3099 = #3067 #3068
[attach-enode] #824 0
[attach-enode] #3067 0
[attach-enode] #3099 0
[assign] #3099 justification -1: 
[mk-app] #3103 distinct-aux-f!!19 #825
[mk-app] #3106 unique-value!44
[attach-enode] #3106 0
[mk-app] #3107 = #3103 #3106
[attach-enode] #825 0
[attach-enode] #3103 0
[attach-enode] #3107 0
[assign] #3107 justification -1: 
[mk-app] #3108 distinct-aux-f!!19 #826
[mk-app] #3109 unique-value!45
[attach-enode] #3109 0
[mk-app] #3110 = #3108 #3109
[attach-enode] #826 0
[attach-enode] #3108 0
[attach-enode] #3110 0
[assign] #3110 justification -1: 
[mk-app] #3111 distinct-aux-f!!19 #827
[mk-app] #3096 unique-value!46
[attach-enode] #3096 0
[mk-app] #2389 = #3111 #3096
[attach-enode] #827 0
[attach-enode] #3111 0
[attach-enode] #2389 0
[assign] #2389 justification -1: 
[mk-app] #2392 distinct-aux-f!!19 #828
[mk-app] #2390 unique-value!47
[attach-enode] #2390 0
[mk-app] #2395 = #2392 #2390
[attach-enode] #828 0
[attach-enode] #2392 0
[attach-enode] #2395 0
[assign] #2395 justification -1: 
[mk-app] #2393 distinct-aux-f!!19 #829
[mk-app] #3030 unique-value!48
[attach-enode] #3030 0
[mk-app] #3009 = #2393 #3030
[attach-enode] #829 0
[attach-enode] #2393 0
[attach-enode] #3009 0
[assign] #3009 justification -1: 
[mk-app] #3010 distinct-aux-f!!19 #830
[mk-app] #3034 unique-value!49
[attach-enode] #3034 0
[mk-app] #3038 = #3010 #3034
[attach-enode] #830 0
[attach-enode] #3010 0
[attach-enode] #3038 0
[assign] #3038 justification -1: 
[mk-app] #3026 distinct-aux-f!!19 #831
[mk-app] #3039 unique-value!50
[attach-enode] #3039 0
[mk-app] #3040 = #3026 #3039
[attach-enode] #831 0
[attach-enode] #3026 0
[attach-enode] #3040 0
[assign] #3040 justification -1: 
[mk-app] #3041 distinct-aux-f!!19 #832
[mk-app] #3042 unique-value!51
[attach-enode] #3042 0
[mk-app] #3043 = #3041 #3042
[attach-enode] #832 0
[attach-enode] #3041 0
[attach-enode] #3043 0
[assign] #3043 justification -1: 
[mk-app] #3044 distinct-aux-f!!19 #833
[mk-app] #2359 unique-value!52
[attach-enode] #2359 0
[mk-app] #2360 = #3044 #2359
[attach-enode] #833 0
[attach-enode] #3044 0
[attach-enode] #2360 0
[assign] #2360 justification -1: 
[mk-app] #2361 distinct-aux-f!!19 #834
[mk-app] #2363 unique-value!53
[attach-enode] #2363 0
[mk-app] #2344 = #2361 #2363
[attach-enode] #834 0
[attach-enode] #2361 0
[attach-enode] #2344 0
[assign] #2344 justification -1: 
[mk-app] #2345 distinct-aux-f!!19 #835
[mk-app] #2346 unique-value!54
[attach-enode] #2346 0
[mk-app] #2311 = #2345 #2346
[attach-enode] #835 0
[attach-enode] #2345 0
[attach-enode] #2311 0
[assign] #2311 justification -1: 
[mk-app] #2312 distinct-aux-f!!19 #836
[mk-app] #2303 unique-value!55
[attach-enode] #2303 0
[mk-app] #2304 = #2312 #2303
[attach-enode] #836 0
[attach-enode] #2312 0
[attach-enode] #2304 0
[assign] #2304 justification -1: 
[mk-app] #2306 distinct-aux-f!!19 #837
[mk-app] #2232 unique-value!56
[attach-enode] #2232 0
[mk-app] #2235 = #2306 #2232
[attach-enode] #837 0
[attach-enode] #2306 0
[attach-enode] #2235 0
[assign] #2235 justification -1: 
[mk-app] #2236 distinct-aux-f!!19 #838
[mk-app] #2238 unique-value!57
[attach-enode] #2238 0
[mk-app] #2237 = #2236 #2238
[attach-enode] #838 0
[attach-enode] #2236 0
[attach-enode] #2237 0
[assign] #2237 justification -1: 
[mk-app] #2229 distinct-aux-f!!19 #839
[mk-app] #2993 unique-value!58
[attach-enode] #2993 0
[mk-app] #2996 = #2229 #2993
[attach-enode] #839 0
[attach-enode] #2229 0
[attach-enode] #2996 0
[assign] #2996 justification -1: 
[mk-app] #3000 distinct-aux-f!!19 #840
[mk-app] #3001 unique-value!59
[attach-enode] #3001 0
[mk-app] #3002 = #3000 #3001
[attach-enode] #840 0
[attach-enode] #3000 0
[attach-enode] #3002 0
[assign] #3002 justification -1: 
[mk-app] #3003 distinct-aux-f!!19 #841
[mk-app] #3005 unique-value!60
[attach-enode] #3005 0
[mk-app] #3006 = #3003 #3005
[attach-enode] #841 0
[attach-enode] #3003 0
[attach-enode] #3006 0
[assign] #3006 justification -1: 
[mk-app] #3007 distinct-aux-f!!19 #842
[mk-app] #1920 unique-value!61
[attach-enode] #1920 0
[mk-app] #2181 = #3007 #1920
[attach-enode] #842 0
[attach-enode] #3007 0
[attach-enode] #2181 0
[assign] #2181 justification -1: 
[mk-app] #2182 distinct-aux-f!!19 #843
[mk-app] #2060 unique-value!62
[attach-enode] #2060 0
[mk-app] #2061 = #2182 #2060
[attach-enode] #843 0
[attach-enode] #2182 0
[attach-enode] #2061 0
[assign] #2061 justification -1: 
[mk-app] #2062 distinct-aux-f!!19 #844
[mk-app] #1974 unique-value!63
[attach-enode] #1974 0
[mk-app] #1968 = #2062 #1974
[attach-enode] #844 0
[attach-enode] #2062 0
[attach-enode] #1968 0
[assign] #1968 justification -1: 
[mk-app] #1988 distinct-aux-f!!19 #845
[mk-app] #1991 unique-value!64
[attach-enode] #1991 0
[mk-app] #1992 = #1988 #1991
[attach-enode] #845 0
[attach-enode] #1988 0
[attach-enode] #1992 0
[assign] #1992 justification -1: 
[mk-app] #1990 distinct-aux-f!!19 #846
[mk-app] #1987 unique-value!65
[attach-enode] #1987 0
[mk-app] #1971 = #1990 #1987
[attach-enode] #846 0
[attach-enode] #1990 0
[attach-enode] #1971 0
[assign] #1971 justification -1: 
[mk-app] #1972 distinct-aux-f!!19 #847
[mk-app] #1975 unique-value!66
[attach-enode] #1975 0
[mk-app] #1973 = #1972 #1975
[attach-enode] #847 0
[attach-enode] #1972 0
[attach-enode] #1973 0
[assign] #1973 justification -1: 
[mk-app] #1966 distinct-aux-f!!19 #848
[mk-app] #1477 unique-value!67
[attach-enode] #1477 0
[mk-app] #1946 = #1966 #1477
[attach-enode] #848 0
[attach-enode] #1966 0
[attach-enode] #1946 0
[assign] #1946 justification -1: 
[mk-app] #1947 distinct-aux-f!!19 #849
[mk-app] #1937 unique-value!68
[attach-enode] #1937 0
[mk-app] #1938 = #1947 #1937
[attach-enode] #849 0
[attach-enode] #1947 0
[attach-enode] #1938 0
[assign] #1938 justification -1: 
[mk-app] #1939 distinct-aux-f!!19 #850
[mk-app] #1941 unique-value!69
[attach-enode] #1941 0
[mk-app] #1921 = #1939 #1941
[attach-enode] #850 0
[attach-enode] #1939 0
[attach-enode] #1921 0
[assign] #1921 justification -1: 
[mk-app] #1922 distinct-aux-f!!19 #851
[mk-app] #1908 unique-value!70
[attach-enode] #1908 0
[mk-app] #1911 = #1922 #1908
[attach-enode] #851 0
[attach-enode] #1922 0
[attach-enode] #1911 0
[assign] #1911 justification -1: 
[mk-app] #1913 distinct-aux-f!!19 #852
[mk-app] #1914 unique-value!71
[attach-enode] #1914 0
[mk-app] #1915 = #1913 #1914
[attach-enode] #852 0
[attach-enode] #1913 0
[attach-enode] #1915 0
[assign] #1915 justification -1: 
[mk-app] #1888 distinct-aux-f!!19 #853
[mk-app] #1889 unique-value!72
[attach-enode] #1889 0
[mk-app] #1891 = #1888 #1889
[attach-enode] #853 0
[attach-enode] #1888 0
[attach-enode] #1891 0
[assign] #1891 justification -1: 
[mk-app] #1890 distinct-aux-f!!19 #854
[mk-app] #1893 unique-value!73
[attach-enode] #1893 0
[mk-app] #1875 = #1890 #1893
[attach-enode] #854 0
[attach-enode] #1890 0
[attach-enode] #1875 0
[assign] #1875 justification -1: 
[mk-app] #1876 distinct-aux-f!!19 #855
[mk-app] #1778 unique-value!74
[attach-enode] #1778 0
[mk-app] #1744 = #1876 #1778
[attach-enode] #855 0
[attach-enode] #1876 0
[attach-enode] #1744 0
[assign] #1744 justification -1: 
[mk-app] #1773 distinct-aux-f!!19 #856
[mk-app] #1774 unique-value!75
[attach-enode] #1774 0
[mk-app] #1798 = #1773 #1774
[attach-enode] #856 0
[attach-enode] #1773 0
[attach-enode] #1798 0
[assign] #1798 justification -1: 
[mk-app] #1797 distinct-aux-f!!19 #857
[mk-app] #1795 unique-value!76
[attach-enode] #1795 0
[mk-app] #2959 = #1797 #1795
[attach-enode] #857 0
[attach-enode] #1797 0
[attach-enode] #2959 0
[assign] #2959 justification -1: 
[mk-app] #2964 distinct-aux-f!!19 #858
[mk-app] #2968 unique-value!77
[attach-enode] #2968 0
[mk-app] #2969 = #2964 #2968
[attach-enode] #858 0
[attach-enode] #2964 0
[attach-enode] #2969 0
[assign] #2969 justification -1: 
[mk-app] #2970 distinct-aux-f!!19 #859
[mk-app] #2971 unique-value!78
[attach-enode] #2971 0
[mk-app] #2973 = #2970 #2971
[attach-enode] #859 0
[attach-enode] #2970 0
[attach-enode] #2973 0
[assign] #2973 justification -1: 
[mk-app] #2974 distinct-aux-f!!19 #860
[mk-app] #2975 unique-value!79
[attach-enode] #2975 0
[mk-app] #2976 = #2974 #2975
[attach-enode] #860 0
[attach-enode] #2974 0
[attach-enode] #2976 0
[assign] #2976 justification -1: 
[mk-app] #2972 distinct-aux-f!!19 #861
[mk-app] #1776 unique-value!80
[attach-enode] #1776 0
[mk-app] #1775 = #2972 #1776
[attach-enode] #861 0
[attach-enode] #2972 0
[attach-enode] #1775 0
[assign] #1775 justification -1: 
[mk-app] #1771 distinct-aux-f!!19 #862
[mk-app] #2928 unique-value!81
[attach-enode] #2928 0
[mk-app] #2933 = #1771 #2928
[attach-enode] #862 0
[attach-enode] #1771 0
[attach-enode] #2933 0
[assign] #2933 justification -1: 
[mk-app] #2937 distinct-aux-f!!19 #863
[mk-app] #2938 unique-value!82
[attach-enode] #2938 0
[mk-app] #2939 = #2937 #2938
[attach-enode] #863 0
[attach-enode] #2937 0
[attach-enode] #2939 0
[assign] #2939 justification -1: 
[mk-app] #2940 distinct-aux-f!!19 #864
[mk-app] #2942 unique-value!83
[attach-enode] #2942 0
[mk-app] #2943 = #2940 #2942
[attach-enode] #864 0
[attach-enode] #2940 0
[attach-enode] #2943 0
[assign] #2943 justification -1: 
[mk-app] #2944 distinct-aux-f!!19 #865
[mk-app] #2945 unique-value!84
[attach-enode] #2945 0
[mk-app] #2941 = #2944 #2945
[attach-enode] #865 0
[attach-enode] #2944 0
[attach-enode] #2941 0
[assign] #2941 justification -1: 
[mk-app] #1727 distinct-aux-f!!19 #866
[mk-app] #1747 unique-value!85
[attach-enode] #1747 0
[mk-app] #1748 = #1727 #1747
[attach-enode] #866 0
[attach-enode] #1727 0
[attach-enode] #1748 0
[assign] #1748 justification -1: 
[mk-app] #1750 distinct-aux-f!!19 #867
[mk-app] #1749 unique-value!86
[attach-enode] #1749 0
[mk-app] #1742 = #1750 #1749
[attach-enode] #867 0
[attach-enode] #1750 0
[attach-enode] #1742 0
[assign] #1742 justification -1: 
[mk-app] #1725 distinct-aux-f!!19 #868
[mk-app] #1728 unique-value!87
[attach-enode] #1728 0
[mk-app] #1726 = #1725 #1728
[attach-enode] #868 0
[attach-enode] #1725 0
[attach-enode] #1726 0
[assign] #1726 justification -1: 
[mk-app] #1730 distinct-aux-f!!19 #869
[mk-app] #1692 unique-value!88
[attach-enode] #1692 0
[mk-app] #1706 = #1730 #1692
[attach-enode] #869 0
[attach-enode] #1730 0
[attach-enode] #1706 0
[assign] #1706 justification -1: 
[mk-app] #1705 distinct-aux-f!!19 #870
[mk-app] #1708 unique-value!89
[attach-enode] #1708 0
[mk-app] #1693 = #1705 #1708
[attach-enode] #870 0
[attach-enode] #1705 0
[attach-enode] #1693 0
[assign] #1693 justification -1: 
[mk-app] #1694 distinct-aux-f!!19 #871
[mk-app] #1684 unique-value!90
[attach-enode] #1684 0
[mk-app] #1678 = #1694 #1684
[attach-enode] #871 0
[attach-enode] #1694 0
[attach-enode] #1678 0
[assign] #1678 justification -1: 
[mk-app] #1681 distinct-aux-f!!19 #872
[mk-app] #1682 unique-value!91
[attach-enode] #1682 0
[mk-app] #1685 = #1681 #1682
[attach-enode] #872 0
[attach-enode] #1681 0
[attach-enode] #1685 0
[assign] #1685 justification -1: 
[mk-app] #1683 distinct-aux-f!!19 #873
[mk-app] #1676 unique-value!92
[attach-enode] #1676 0
[mk-app] #1654 = #1683 #1676
[attach-enode] #873 0
[attach-enode] #1683 0
[attach-enode] #1654 0
[assign] #1654 justification -1: 
[mk-app] #1655 distinct-aux-f!!19 #874
[mk-app] #1653 unique-value!93
[attach-enode] #1653 0
[mk-app] #1657 = #1655 #1653
[attach-enode] #874 0
[attach-enode] #1655 0
[attach-enode] #1657 0
[assign] #1657 justification -1: 
[mk-app] #1637 distinct-aux-f!!19 #875
[mk-app] #1638 unique-value!94
[attach-enode] #1638 0
[mk-app] #1639 = #1637 #1638
[attach-enode] #875 0
[attach-enode] #1637 0
[attach-enode] #1639 0
[assign] #1639 justification -1: 
[mk-app] #1571 distinct-aux-f!!19 #876
[mk-app] #1558 unique-value!95
[attach-enode] #1558 0
[mk-app] #1609 = #1571 #1558
[attach-enode] #876 0
[attach-enode] #1571 0
[attach-enode] #1609 0
[assign] #1609 justification -1: 
[mk-app] #1610 distinct-aux-f!!19 #877
[mk-app] #1612 unique-value!96
[attach-enode] #1612 0
[mk-app] #1611 = #1610 #1612
[attach-enode] #877 0
[attach-enode] #1610 0
[attach-enode] #1611 0
[assign] #1611 justification -1: 
[mk-app] #1608 distinct-aux-f!!19 #878
[mk-app] #1596 unique-value!97
[attach-enode] #1596 0
[mk-app] #1597 = #1608 #1596
[attach-enode] #878 0
[attach-enode] #1608 0
[attach-enode] #1597 0
[assign] #1597 justification -1: 
[mk-app] #1561 distinct-aux-f!!19 #879
[mk-app] #1583 unique-value!98
[attach-enode] #1583 0
[mk-app] #1584 = #1561 #1583
[attach-enode] #879 0
[attach-enode] #1561 0
[attach-enode] #1584 0
[assign] #1584 justification -1: 
[mk-app] #1585 distinct-aux-f!!19 #880
[mk-app] #1572 unique-value!99
[attach-enode] #1572 0
[mk-app] #1573 = #1585 #1572
[attach-enode] #880 0
[attach-enode] #1585 0
[attach-enode] #1573 0
[assign] #1573 justification -1: 
[mk-app] #1563 distinct-aux-f!!19 #881
[mk-app] #1564 unique-value!100
[attach-enode] #1564 0
[mk-app] #1565 = #1563 #1564
[attach-enode] #881 0
[attach-enode] #1563 0
[attach-enode] #1565 0
[assign] #1565 justification -1: 
[attach-enode] #883 0
[attach-enode] #884 0
[attach-enode] #885 0
[attach-enode] #886 0
[attach-enode] #887 0
[attach-enode] #888 0
[attach-enode] #889 0
[attach-enode] #890 0
[attach-enode] #891 0
[attach-enode] #892 0
[attach-enode] #893 0
[attach-enode] #894 0
[attach-enode] #895 0
[attach-enode] #896 0
[attach-enode] #901 0
[attach-enode] #902 0
[attach-enode] #907 0
[assign] #907 justification -1: 
[attach-enode] #908 0
[assign] #908 justification -1: 
[attach-enode] #909 0
[assign] #909 justification -1: 
[attach-enode] #910 0
[assign] #910 justification -1: 
[attach-enode] #911 0
[assign] #911 justification -1: 
[attach-enode] #912 0
[assign] #912 justification -1: 
[attach-enode] #913 0
[assign] #913 justification -1: 
[attach-enode] #914 0
[assign] #914 justification -1: 
[attach-enode] #915 0
[assign] #915 justification -1: 
[attach-enode] #920 0
[attach-enode] #921 0
[attach-enode] #926 0
[assign] #926 justification -1: 
[attach-enode] #932 0
[attach-enode] #933 0
[attach-enode] #934 0
[attach-enode] #935 0
[attach-enode] #936 0
[attach-enode] #941 0
[assign] #941 justification -1: 
[assign] #920 justification -1: 
[attach-enode] #942 0
[assign] #942 justification -1: 
[attach-enode] #943 0
[assign] #943 justification -1: 
[attach-enode] #944 0
[assign] #944 justification -1: 
[attach-enode] #945 0
[assign] #945 justification -1: 
[attach-enode] #946 0
[assign] #946 justification -1: 
[assign] #883 justification -1: 
[attach-enode] #947 0
[assign] #947 justification -1: 
[attach-enode] #948 0
[assign] #948 justification -1: 
[attach-enode] #949 0
[assign] #949 justification -1: 
[attach-enode] #950 0
[assign] #950 justification -1: 
[attach-enode] #951 0
[assign] #951 justification -1: 
[assign] #901 justification -1: 
[attach-enode] #952 0
[assign] #952 justification -1: 
[attach-enode] #953 0
[assign] #953 justification -1: 
[attach-enode] #954 0
[assign] #954 justification -1: 
[attach-enode] #955 0
[assign] #955 justification -1: 
[attach-enode] #956 0
[assign] #956 justification -1: 
[assign] #932 justification -1: 
[attach-enode] #957 0
[assign] #957 justification -1: 
[attach-enode] #958 0
[assign] #958 justification -1: 
[attach-enode] #959 0
[assign] #959 justification -1: 
[attach-enode] #964 0
[assign] #964 justification -1: 
[attach-enode] #906 0
[assign] #906 justification -1: 
[attach-enode] #925 0
[assign] #925 justification -1: 
[attach-enode] #927 0
[assign] #927 justification -1: 
[assign] #970 justification -1: 
[assign] #983 justification -1: 
[assign] #2868 justification -1: 
[assign] #3217 justification -1: 
[assign] #3219 justification -1: 
[assign] #2883 justification -1: 
[assign] #1058 justification -1: 
[assign] #1069 justification -1: 
[assign] #3223 justification -1: 
[assign] #3230 justification -1: 
[assign] #3232 justification -1: 
[assign] #3236 justification -1: 
[assign] #1146 justification -1: 
[assign] #1157 justification -1: 
[assign] #1160 justification -1: 
[assign] #1166 justification -1: 
[assign] #1178 justification -1: 
[assign] #1181 justification -1: 
[assign] #1187 justification -1: 
[assign] #1198 justification -1: 
[assign] #1201 justification -1: 
[assign] #1207 justification -1: 
[assign] #1218 justification -1: 
[assign] #1224 justification -1: 
[assign] #1233 justification -1: 
[assign] #1244 justification -1: 
[assign] #1251 justification -1: 
[assign] #1259 justification -1: 
[assign] #3241 justification -1: 
[assign] #3245 justification -1: 
[assign] #1296 justification -1: 
[assign] #1307 justification -1: 
[assign] #1318 justification -1: 
[assign] #1323 justification -1: 
[assign] #1330 justification -1: 
[assign] #1339 justification -1: 
[assign] #1344 justification -1: 
[assign] #1355 justification -1: 
[assign] #1358 justification -1: 
[assign] #1367 justification -1: 
[assign] #3250 justification -1: 
[assign] #3257 justification -1: 
[assign] #3259 justification -1: 
[assign] #1414 justification -1: 
[assign] #1422 justification -1: 
[assign] #1430 justification -1: 
[assign] #1437 justification -1: 
[assign] #3263 justification -1: 
[assign] #3265 justification -1: 
[assign] #3267 justification -1: 
[assign] #3269 justification -1: 
[assign] #3271 justification -1: 
[assign] #3273 justification -1: 
[assign] #1495 justification -1: 
[assign] #1502 justification -1: 
[attach-enode] #1345 0
[attach-enode] #1503 0
[attach-enode] #1504 0
[assign] #1504 justification -1: 
[attach-enode] #1505 0
[attach-enode] #1506 0
[assign] #1506 justification -1: 
[attach-enode] #140 0
[attach-enode] #1507 0
[attach-enode] #1508 0
[assign] #1508 justification -1: 
[attach-enode] #1509 0
[attach-enode] #1510 0
[assign] #1510 justification -1: 
[attach-enode] #208 0
[attach-enode] #1511 0
[attach-enode] #1512 0
[assign] #1512 justification -1: 
[attach-enode] #1513 0
[attach-enode] #1514 0
[assign] #1514 justification -1: 
[attach-enode] #1167 0
[attach-enode] #1515 0
[attach-enode] #1516 0
[assign] #1516 justification -1: 
[attach-enode] #1517 0
[attach-enode] #1518 0
[assign] #1518 justification -1: 
[assign] #3275 justification -1: 
[assign] #3277 justification -1: 
[assign] #1545 justification -1: 
[assign] #3281 justification -1: 
[assign] #3286 justification -1: 
[assign] #3287 justification -1: 
[attach-enode] #1580 0
[assign] #1580 justification -1: 
[attach-enode] #1581 0
[assign] #3291 justification -1: 
[attach-enode] #1598 0
[assign] #1616 justification -1: 
[attach-enode] #1617 0
[assign] #3298 justification -1: 
[attach-enode] #1640 0
[attach-enode] #1658 0
[assign] #3310 justification -1: 
[attach-enode] #1695 0
[attach-enode] #1709 0
[attach-enode] #1731 0
[attach-enode] #1751 0
[attach-enode] #1782 0
[assign] #1810 justification -1: 
[attach-enode] #1811 0
[assign] #1811 justification -1: 
[attach-enode] #1812 0
[attach-enode] #1820 0
[assign] #1820 justification -1: 
[attach-enode] #1821 0
[assign] #1821 justification -1: 
[attach-enode] #1822 0
[attach-enode] #1830 0
[assign] #1830 justification -1: 
[attach-enode] #1831 0
[assign] #1831 justification -1: 
[attach-enode] #1832 0
[attach-enode] #1840 0
[assign] #1840 justification -1: 
[attach-enode] #1841 0
[assign] #1841 justification -1: 
[attach-enode] #1842 0
[assign] #1860 justification -1: 
[assign] #1870 justification -1: 
[assign] #3359 justification -1: 
[attach-enode] #1877 0
[attach-enode] #1147 0
[attach-enode] #1894 0
[assign] #1894 justification -1: 
[assign] #3366 justification -1: 
[assign] #3370 justification -1: 
[attach-enode] #1923 0
[assign] #1923 justification -1: 
[attach-enode] #1924 0
[assign] #3377 justification -1: 
[attach-enode] #1948 0
[attach-enode] #1976 0
[attach-enode] #1993 0
[attach-enode] #2007 0
[attach-enode] #2021 0
[assign] #2021 justification -1: 
[attach-enode] #2022 0
[attach-enode] #2032 0
[assign] #2032 justification -1: 
[assign] #2038 justification -1: 
[attach-enode] #2039 0
[assign] #3388 justification -1: 
[attach-enode] #2063 0
[assign] #2063 justification -1: 
[attach-enode] #2064 0
[assign] #2082 justification -1: 
[attach-enode] #2083 0
[assign] #2083 justification -1: 
[attach-enode] #2084 0
[assign] #2100 justification -1: 
[assign] #2110 justification -1: 
[attach-enode] #2111 0
[assign] #2111 justification -1: 
[attach-enode] #2112 0
[assign] #2124 justification -1: 
[assign] #3390 justification -1: 
[assign] #3396 justification -1: 
[assign] #2151 justification -1: 
[assign] #2158 justification -1: 
[assign] #2177 justification -1: 
[assign] #3402 justification -1: 
[assign] #3403 justification -1: 
[assign] #2200 justification -1: 
[assign] #3408 justification -1: 
[assign] #3425 justification -1: 
[attach-enode] #2240 0
[assign] #2240 justification -1: 
[attach-enode] #2241 0
[attach-enode] #2255 0
[assign] #2255 justification -1: 
[attach-enode] #2256 0
[attach-enode] #2272 0
[assign] #2272 justification -1: 
[attach-enode] #2273 0
[assign] #2290 justification -1: 
[attach-enode] #2291 0
[assign] #2291 justification -1: 
[attach-enode] #2292 0
[assign] #3432 justification -1: 
[attach-enode] #2313 0
[assign] #2313 justification -1: 
[assign] #2320 justification -1: 
[assign] #3437 justification -1: 
[attach-enode] #2347 0
[attach-enode] #2364 0
[assign] #2364 justification -1: 
[attach-enode] #2365 0
[attach-enode] #2400 0
[assign] #2400 justification -1: 
[attach-enode] #2401 0
[assign] #3498 justification -1: 
[assign] #3506 justification -1: 
[attach-enode] #2487 0
[assign] #2487 justification -1: 
[attach-enode] #2494 0
[attach-enode] #2507 0
[assign] #2507 justification -1: 
[attach-enode] #2508 0
[attach-enode] #2520 0
[assign] #2520 justification -1: 
[attach-enode] #2521 0
[attach-enode] #2533 0
[assign] #2533 justification -1: 
[attach-enode] #2534 0
[attach-enode] #2542 0
[assign] #2542 justification -1: 
[assign] #2548 justification -1: 
[attach-enode] #2549 0
[assign] #3518 justification -1: 
[attach-enode] #2571 0
[assign] #2571 justification -1: 
[attach-enode] #2572 0
[attach-enode] #2582 0
[assign] #2582 justification -1: 
[assign] #2587 justification -1: 
[assign] #3520 justification -1: 
[attach-enode] #2606 0
[assign] #2637 justification -1: 
[assign] #2643 justification -1: 
[assign] #3526 justification -1: 
[attach-enode] #2650 0
[assign] #2650 justification -1: 
[assign] #2657 justification -1: 
[assign] #3528 justification -1: 
[assign] #3530 justification -1: 
[assign] #3532 justification -1: 
[assign] #3535 justification -1: 
[assign] #2724 justification -1: 
[assign] #3544 justification -1: 
[assign] #2747 justification -1: 
[assign] #2758 justification -1: 
[assign] #3561 justification -1: 
[assign] #3566 justification -1: 
[assign] #2800 justification -1: 
[assign] #3598 justification -1: 
[assign] #2864 justification -1: 
[assign] #921 bin 207
[assign] #896 bin 182
[assign] #895 bin 182
[assign] #894 bin 182
[assign] #893 bin 182
[assign] #892 bin 182
[assign] #891 bin 182
[assign] #890 bin 182
[assign] #889 bin 182
[assign] #888 bin 182
[assign] #887 bin 182
[assign] #886 bin 182
[assign] #885 bin 182
[assign] #884 bin 182
[assign] #902 bin 196
[assign] #936 bin 210
[assign] #935 bin 210
[assign] #934 bin 210
[assign] #933 bin 210
[attach-enode] #145 0
[attach-enode] #189 0
[attach-enode] #200 0
[attach-enode] #216 0
[attach-enode] #249 0
[attach-enode] #1168 0
[attach-enode] #1188 0
[attach-enode] #1219 0
[attach-enode] #1220 0
[eq-expl] #125 root
[eq-expl] #140 root
[new-match] 0x61d50e5fe6a0 #1367 #1363 #140 #125 ; #1820
[eq-expl] #1167 root
[new-match] 0x61d50e5fe6d8 #1367 #1363 #1167 #125 ; #1830
[eq-expl] #208 root
[new-match] 0x61d50e5fe710 #1367 #1363 #208 #125 ; #1840
[eq-expl] #1345 root
[new-match] 0x61d50e5fe748 #1367 #1363 #1345 #125 ; #2650
[push] 0
[mk-app] #1533 a!
[attach-meaning] #275 arith 32
[mk-app] #1534 TYPE%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #125 #1167
[mk-app] #1525 has_type #1533 #1534
[mk-app] #1526 tmp%1
[mk-app] #1486 has_type #1526 #208
[mk-app] #1487 n@
[mk-app] #1478 uInv #274 #1487
[mk-app] #1479 tmp%2
[attach-meaning] #275 arith 32
[mk-app] #1468 has_type #1479 #1168
[mk-app] #1469 tmp%3
[mk-app] #1461 uInv #274 #1469
[attach-meaning] #275 arith 32
[mk-app] #1462 rsum@0
[mk-app] #1453 iInv #275 #1462
[mk-app] #1454 i@0
[mk-app] #1446 uInv #274 #1454
[mk-app] #1447 <= #341 #1454
[mk-app] #1402 <= #1454 #1487
[mk-app] #1403 and #1447 #1402
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #1393 %Poly%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #1533
[mk-app] #1395 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq #1393
[mk-app] #1396 vstd!view.View.view.? #125 #1168 #1395
[mk-app] #1379 vstd!seq.Seq.len.? #125 #1167 #1396
[mk-app] #1380 = #1487 #1379
[attach-meaning] #275 arith 32
[mk-app] #1287 lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.? #1396
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #1288 pv@0
[mk-app] #1289 Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #1288
[mk-app] #1290 vstd!view.View.view.? #125 #1168 #1289
[mk-app] #1273 vstd!seq.Seq.len.? #125 #1167 #1290
[mk-app] #1274 Add #1454 #296
[mk-app] #1275 = #1273 #1274
[attach-meaning] #275 arith 32
[mk-app] #1276 I #1454
[mk-app] #2899 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #1396 #1276
[mk-app] #2900 = #1462 #2899
[mk-app] #2905 <= #191 #1454
[mk-app] #2906 and #517 #2905
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #2910 vstd!seq.Seq.index.? #125 #1167 #1290 #34
[mk-app] #2911 %I #2910
[attach-meaning] #275 arith 32
[mk-app] #1102 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #1396 #34
[mk-app] #1112 = #2911 #1102
[mk-app] #1113 => #2906 #1112
[mk-app] #1103 => #190 #1113
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #1104 pattern #2910
[mk-quant] #2885 user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS__max_contig_sub_sum_opt_28 1 #1104 #1103
[attach-var-names] #2885 (|k$| ; |Poly|)
[mk-app] #2886 decrease%init0%0
[mk-app] #2889 Sub #1487 #1454
[mk-app] #2890 = #2886 #2889
[mk-app] #2891 < #1454 #1487
[mk-app] #2892 tmp%4
[attach-meaning] #275 arith 32
[mk-app] #1014 %Poly%vstd!seq.Seq<i32.>. #1396
[mk-app] #1024 = #2892 #1014
[mk-app] #1025 tmp%5
[mk-app] #1015 = #1025 #1274
[mk-app] #1016 %%location_label%%0
[mk-app] #940 req%lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_snoc. #2892 #341 #1025
[mk-app] #900 => #1016 #940
[mk-app] #778 ens%lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_snoc. #2892 #341 #1025
[mk-app] #779 tmp%6
[attach-meaning] #275 arith 32
[mk-app] #780 I #1274
[mk-app] #781 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #1396 #2574 #780
[attach-meaning] #275 arith 32
[mk-app] #782 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #1396 #2574 #1276
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #748 vstd!seq.Seq.index.? #125 #1167 #1396 #1276
[mk-app] #749 %I #748
[mk-app] #750 Add #782 #749
[mk-app] #751 = #781 #750
[mk-app] #737 = #779 #751
[mk-app] #739 %%location_label%%1
[mk-app] #740 => #739 #779
[mk-app] #741 tmp%8
[mk-app] #696 = #741 #1462
[mk-app] #727 %%location_label%%2
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #728 req%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth. #125 #1534 #125 #1167 #1533 #1276
[mk-app] #685 => #727 #728
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #720 tmp%7
[mk-app] #721 ens%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth. #125 #1534 #125 #1167 #1533 #1276 #720
[mk-app] #712 %%location_label%%3
[attach-meaning] #275 arith 32
[mk-app] #713 %I #720
[mk-app] #705 Add #741 #713
[mk-app] #706 iInv #275 #705
[mk-app] #697 => #712 #706
[attach-meaning] #275 arith 32
[mk-app] #698 rsum@1
[attach-meaning] #275 arith 32
[mk-app] #686 iClip #275 #705
[mk-app] #687 = #698 #686
[attach-meaning] #275 arith 32
[mk-app] #652 pv@1
[mk-app] #658 Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #652
[mk-app] #672 I #698
[mk-app] #673 ens%alloc!vec.impl&%43.push. #125 #1167 #125 #1147 #1289 #658 #672
[mk-app] #674 %%location_label%%4
[mk-app] #675 uInv #274 #1274
[mk-app] #657 => #674 #675
[mk-app] #659 i@1
[mk-app] #660 uClip #274 #1274
[mk-app] #638 = #659 #660
[mk-app] #639 %%location_label%%5
[mk-app] #641 <= #341 #659
[mk-app] #642 <= #659 #1487
[mk-app] #439 and #641 #642
[mk-app] #440 => #639 #439
[mk-app] #441 %%location_label%%6
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #463 => #441 #1380
[mk-app] #464 %%location_label%%7
[attach-meaning] #275 arith 32
[mk-app] #409 => #464 #1287
[mk-app] #411 %%location_label%%8
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #459 vstd!view.View.view.? #125 #1168 #658
[mk-app] #460 vstd!seq.Seq.len.? #125 #1167 #459
[mk-app] #378 Add #659 #296
[mk-app] #380 = #460 #378
[mk-app] #453 => #411 #380
[mk-app] #454 %%location_label%%9
[attach-meaning] #275 arith 32
[mk-app] #434 I #659
[mk-app] #436 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #1396 #434
[mk-app] #437 = #698 #436
[mk-app] #443 => #454 #437
[mk-app] #444 %%location_label%%10
[mk-app] #445 <= #191 #659
[mk-app] #446 and #517 #445
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #402 vstd!seq.Seq.index.? #125 #1167 #459 #34
[mk-app] #412 %I #402
[attach-meaning] #275 arith 32
[mk-app] #413 = #412 #1102
[mk-app] #414 => #446 #413
[mk-app] #415 => #190 #414
[attach-meaning] #275 arith 32
[attach-meaning] #275 arith 32
[mk-app] #372 pattern #402
[mk-quant] #381 user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS__max_contig_sub_sum_opt_28 1 #372 #415
[attach-var-names] #381 (|k$| ; |Poly|)
[mk-app] #382 => #444 #381
[mk-app] #383 %%location_label%%11
[mk-app] #384 Sub #1487 #659
[mk-app] #354 check_decrease_int #384 #2886 #2
[mk-app] #355 => #383 #354
[mk-app] #159 and #382 #355
[mk-app] #160 and #443 #159
[mk-app] #76 and #453 #160
[mk-app] #77 and #409 #76
[mk-app] #78 and #463 #77
[mk-app] #3599 and #440 #78
[mk-app] #3600 => #638 #3599
[mk-app] #3601 => #675 #3600
[mk-app] #3602 and #657 #3601
[mk-app] #3603 => #673 #3602
[mk-app] #3604 => #687 #3603
[mk-app] #3605 => #706 #3604
[mk-app] #3606 and #697 #3605
[mk-app] #3607 => #721 #3606
[mk-app] #3608 and #685 #3607
[mk-app] #3609 => #696 #3608
[mk-app] #3610 => #779 #3609
[mk-app] #3611 and #740 #3610
[mk-app] #3612 => #737 #3611
[mk-app] #3613 => #778 #3612
[mk-app] #3614 and #900 #3613
[mk-app] #3615 => #1015 #3614
[mk-app] #3616 => #1024 #3615
[mk-app] #3617 => #2891 #3616
[mk-app] #3618 => #2890 #3617
[mk-app] #3619 => #2885 #3618
[mk-app] #3620 => #2900 #3619
[mk-app] #3621 => #1275 #3620
[mk-app] #3622 => #1287 #3621
[mk-app] #3623 => #1380 #3622
[mk-app] #3624 => #1403 #3623
[mk-app] #3625 not #3624
[attach-meaning] #370 arith (- 1)
[mk-app] #3626 * #370 #1454
[mk-app] #3627 >= #1454 #341
[inst-discovered] theory-solving 0 arith# ; #1447
[mk-app] #3626 = #1447 #3627
[instance] 0 #3626
[attach-enode] #3626 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3626 * #370 #1487
[mk-app] #3628 + #1454 #3626
[mk-app] #3629 <= #3628 #341
[inst-discovered] theory-solving 0 arith# ; #1402
[mk-app] #3630 = #1402 #3629
[instance] 0 #3630
[attach-enode] #3630 0
[end-of-instance]
[mk-app] #3630 and #3627 #3629
[attach-meaning] #370 arith (- 1)
[mk-app] #3631 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #3631 = #517 #521
[instance] 0 #3631
[attach-enode] #3631 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3631 * #370 #1454
[mk-app] #3632 + #191 #3631
[mk-app] #3633 <= #3632 #341
[inst-discovered] theory-solving 0 arith# ; #2905
[mk-app] #3634 = #2905 #3633
[instance] 0 #3634
[attach-enode] #3634 0
[end-of-instance]
[mk-app] #3634 and #521 #3633
[mk-app] #3635 not #3634
[mk-app] #3636 or #3635 #1112
[mk-app] #3637 => #3634 #1112
[inst-discovered] theory-solving 0 basic# ; #3637
[mk-app] #3638 = #3637 #3636
[instance] 0 #3638
[attach-enode] #3638 0
[end-of-instance]
[mk-app] #3637 or #197 #3635 #1112
[mk-app] #3638 => #190 #3636
[inst-discovered] theory-solving 0 basic# ; #3638
[mk-app] #3639 = #3638 #3637
[instance] 0 #3639
[attach-enode] #3639 0
[end-of-instance]
[mk-quant] #3636 user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS__max_contig_sub_sum_opt_28 1 #1104 #3637
[attach-var-names] #3636 (|k$| ; |Poly|)
[mk-app] #3638 <= #1487 #1454
[mk-app] #3639 not #3638
[inst-discovered] theory-solving 0 arith# ; #2891
[mk-app] #3640 = #2891 #3639
[instance] 0 #3640
[attach-enode] #3640 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3640 + #3631 #1487
[attach-meaning] #370 arith (- 1)
[mk-app] #3640 >= #3628 #341
[inst-discovered] theory-solving 0 arith# ; #3638
[mk-app] #3641 = #3638 #3640
[instance] 0 #3641
[attach-enode] #3641 0
[end-of-instance]
[mk-app] #3641 not #3640
[mk-app] #3642 not #1016
[mk-app] #3643 or #3642 #940
[inst-discovered] theory-solving 0 basic# ; #900
[mk-app] #3644 = #900 #3643
[instance] 0 #3644
[attach-enode] #3644 0
[end-of-instance]
[mk-app] #3644 not #739
[mk-app] #3645 or #3644 #779
[inst-discovered] theory-solving 0 basic# ; #740
[mk-app] #3646 = #740 #3645
[instance] 0 #3646
[attach-enode] #3646 0
[end-of-instance]
[mk-app] #3646 not #727
[mk-app] #3647 or #3646 #728
[inst-discovered] theory-solving 0 basic# ; #685
[mk-app] #3648 = #685 #3647
[instance] 0 #3648
[attach-enode] #3648 0
[end-of-instance]
[mk-app] #3648 not #712
[mk-app] #3649 or #3648 #706
[inst-discovered] theory-solving 0 basic# ; #697
[mk-app] #3650 = #697 #3649
[instance] 0 #3650
[attach-enode] #3650 0
[end-of-instance]
[mk-app] #3650 not #674
[mk-app] #3651 or #3650 #675
[inst-discovered] theory-solving 0 basic# ; #657
[mk-app] #3652 = #657 #3651
[instance] 0 #3652
[attach-enode] #3652 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3652 * #370 #659
[mk-app] #3653 >= #659 #341
[inst-discovered] theory-solving 0 arith# ; #641
[mk-app] #3652 = #641 #3653
[instance] 0 #3652
[attach-enode] #3652 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3652 + #659 #3626
[mk-app] #3654 <= #3652 #341
[inst-discovered] theory-solving 0 arith# ; #642
[mk-app] #3655 = #642 #3654
[instance] 0 #3655
[attach-enode] #3655 0
[end-of-instance]
[mk-app] #3655 and #3653 #3654
[mk-app] #3656 not #639
[mk-app] #3657 or #3656 #3655
[mk-app] #3658 => #639 #3655
[inst-discovered] theory-solving 0 basic# ; #3658
[mk-app] #3659 = #3658 #3657
[instance] 0 #3659
[attach-enode] #3659 0
[end-of-instance]
[mk-app] #3658 not #441
[mk-app] #3659 or #3658 #1380
[inst-discovered] theory-solving 0 basic# ; #463
[mk-app] #3660 = #463 #3659
[instance] 0 #3660
[attach-enode] #3660 0
[end-of-instance]
[mk-app] #3660 not #464
[mk-app] #3661 or #3660 #1287
[inst-discovered] theory-solving 0 basic# ; #409
[mk-app] #3662 = #409 #3661
[instance] 0 #3662
[attach-enode] #3662 0
[end-of-instance]
[mk-app] #3662 not #411
[mk-app] #3663 or #3662 #380
[inst-discovered] theory-solving 0 basic# ; #453
[mk-app] #3664 = #453 #3663
[instance] 0 #3664
[attach-enode] #3664 0
[end-of-instance]
[mk-app] #3664 not #454
[mk-app] #3665 or #3664 #437
[inst-discovered] theory-solving 0 basic# ; #443
[mk-app] #3666 = #443 #3665
[instance] 0 #3666
[attach-enode] #3666 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3666 * #370 #191
[inst-discovered] theory-solving 0 arith# ; #517
[mk-app] #3666 = #517 #521
[instance] 0 #3666
[attach-enode] #3666 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3666 * #370 #659
[mk-app] #3667 + #191 #3666
[mk-app] #3668 <= #3667 #341
[inst-discovered] theory-solving 0 arith# ; #445
[mk-app] #3669 = #445 #3668
[instance] 0 #3669
[attach-enode] #3669 0
[end-of-instance]
[mk-app] #3669 and #521 #3668
[mk-app] #3670 not #3669
[mk-app] #3671 or #3670 #413
[mk-app] #3672 => #3669 #413
[inst-discovered] theory-solving 0 basic# ; #3672
[mk-app] #3673 = #3672 #3671
[instance] 0 #3673
[attach-enode] #3673 0
[end-of-instance]
[mk-app] #3672 or #197 #3670 #413
[mk-app] #3673 => #190 #3671
[inst-discovered] theory-solving 0 basic# ; #3673
[mk-app] #3674 = #3673 #3672
[instance] 0 #3674
[attach-enode] #3674 0
[end-of-instance]
[mk-quant] #3671 user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS__max_contig_sub_sum_opt_28 1 #372 #3672
[attach-var-names] #3671 (|k$| ; |Poly|)
[mk-app] #3673 not #444
[mk-app] #3674 or #3673 #3671
[mk-app] #3675 => #444 #3671
[inst-discovered] theory-solving 0 basic# ; #3675
[mk-app] #3676 = #3675 #3674
[instance] 0 #3676
[attach-enode] #3676 0
[end-of-instance]
[mk-app] #3675 not #383
[mk-app] #3676 or #3675 #354
[inst-discovered] theory-solving 0 basic# ; #355
[mk-app] #3677 = #355 #3676
[instance] 0 #3677
[attach-enode] #3677 0
[end-of-instance]
[mk-app] #3677 and #3657 #3659 #3661 #3663 #3665 #3674 #3676
[mk-app] #3678 not #638
[mk-app] #3679 or #3678 #3677
[mk-app] #3680 => #638 #3677
[inst-discovered] theory-solving 0 basic# ; #3680
[mk-app] #3681 = #3680 #3679
[instance] 0 #3681
[attach-enode] #3681 0
[end-of-instance]
[mk-app] #3680 not #675
[mk-app] #3681 or #3680 #3678 #3677
[mk-app] #3682 => #675 #3679
[inst-discovered] theory-solving 0 basic# ; #3682
[mk-app] #3683 = #3682 #3681
[instance] 0 #3683
[attach-enode] #3683 0
[end-of-instance]
[mk-app] #3679 and #3651 #3681
[mk-app] #3682 not #673
[mk-app] #3683 or #3682 #3679
[mk-app] #3684 => #673 #3679
[inst-discovered] theory-solving 0 basic# ; #3684
[mk-app] #3685 = #3684 #3683
[instance] 0 #3685
[attach-enode] #3685 0
[end-of-instance]
[mk-app] #3684 not #687
[mk-app] #3685 or #3684 #3682 #3679
[mk-app] #3686 => #687 #3683
[inst-discovered] theory-solving 0 basic# ; #3686
[mk-app] #3687 = #3686 #3685
[instance] 0 #3687
[attach-enode] #3687 0
[end-of-instance]
[mk-app] #3683 not #706
[mk-app] #3686 or #3683 #3684 #3682 #3679
[mk-app] #3687 => #706 #3685
[inst-discovered] theory-solving 0 basic# ; #3687
[mk-app] #3688 = #3687 #3686
[instance] 0 #3688
[attach-enode] #3688 0
[end-of-instance]
[mk-app] #3685 and #3649 #3686
[mk-app] #3687 not #721
[mk-app] #3688 or #3687 #3685
[mk-app] #3689 => #721 #3685
[inst-discovered] theory-solving 0 basic# ; #3689
[mk-app] #3690 = #3689 #3688
[instance] 0 #3690
[attach-enode] #3690 0
[end-of-instance]
[mk-app] #3689 and #3647 #3688
[mk-app] #3690 not #696
[mk-app] #3691 or #3690 #3689
[mk-app] #3692 => #696 #3689
[inst-discovered] theory-solving 0 basic# ; #3692
[mk-app] #3693 = #3692 #3691
[instance] 0 #3693
[attach-enode] #3693 0
[end-of-instance]
[mk-app] #3692 not #779
[mk-app] #3693 or #3692 #3690 #3689
[mk-app] #3694 => #779 #3691
[inst-discovered] theory-solving 0 basic# ; #3694
[mk-app] #3695 = #3694 #3693
[instance] 0 #3695
[attach-enode] #3695 0
[end-of-instance]
[mk-app] #3691 and #3645 #3693
[mk-app] #3694 not #737
[mk-app] #3695 or #3694 #3691
[mk-app] #3696 => #737 #3691
[inst-discovered] theory-solving 0 basic# ; #3696
[mk-app] #3697 = #3696 #3695
[instance] 0 #3697
[attach-enode] #3697 0
[end-of-instance]
[mk-app] #3696 not #778
[mk-app] #3697 or #3696 #3694 #3691
[mk-app] #3698 => #778 #3695
[inst-discovered] theory-solving 0 basic# ; #3698
[mk-app] #3699 = #3698 #3697
[instance] 0 #3699
[attach-enode] #3699 0
[end-of-instance]
[mk-app] #3695 and #3643 #3697
[mk-app] #3698 not #1015
[mk-app] #3699 or #3698 #3695
[mk-app] #3700 => #1015 #3695
[inst-discovered] theory-solving 0 basic# ; #3700
[mk-app] #3701 = #3700 #3699
[instance] 0 #3701
[attach-enode] #3701 0
[end-of-instance]
[mk-app] #3700 not #1024
[mk-app] #3701 or #3700 #3698 #3695
[mk-app] #3702 => #1024 #3699
[inst-discovered] theory-solving 0 basic# ; #3702
[mk-app] #3703 = #3702 #3701
[instance] 0 #3703
[attach-enode] #3703 0
[end-of-instance]
[mk-app] #3699 or #3640 #3700 #3698 #3695
[mk-app] #3702 => #3641 #3701
[inst-discovered] theory-solving 0 basic# ; #3702
[mk-app] #3703 = #3702 #3699
[instance] 0 #3703
[attach-enode] #3703 0
[end-of-instance]
[mk-app] #3701 not #2890
[mk-app] #3702 or #3701 #3640 #3700 #3698 #3695
[mk-app] #3703 => #2890 #3699
[inst-discovered] theory-solving 0 basic# ; #3703
[mk-app] #3704 = #3703 #3702
[instance] 0 #3704
[attach-enode] #3704 0
[end-of-instance]
[mk-app] #3699 not #3636
[mk-app] #3703 or #3699 #3701 #3640 #3700 #3698 #3695
[mk-app] #3704 => #3636 #3702
[inst-discovered] theory-solving 0 basic# ; #3704
[mk-app] #3705 = #3704 #3703
[instance] 0 #3705
[attach-enode] #3705 0
[end-of-instance]
[mk-app] #3702 not #2900
[mk-app] #3704 or #3702 #3699 #3701 #3640 #3700 #3698 #3695
[mk-app] #3705 => #2900 #3703
[inst-discovered] theory-solving 0 basic# ; #3705
[mk-app] #3706 = #3705 #3704
[instance] 0 #3706
[attach-enode] #3706 0
[end-of-instance]
[mk-app] #3703 not #1275
[mk-app] #3705 or #3703 #3702 #3699 #3701 #3640 #3700 #3698 #3695
[mk-app] #3706 => #1275 #3704
[inst-discovered] theory-solving 0 basic# ; #3706
[mk-app] #3707 = #3706 #3705
[instance] 0 #3707
[attach-enode] #3707 0
[end-of-instance]
[mk-app] #3704 not #1287
[mk-app] #3706 or #3704 #3703 #3702 #3699 #3701 #3640 #3700 #3698 #3695
[mk-app] #3707 => #1287 #3705
[inst-discovered] theory-solving 0 basic# ; #3707
[mk-app] #3708 = #3707 #3706
[instance] 0 #3708
[attach-enode] #3708 0
[end-of-instance]
[mk-app] #3705 not #1380
[mk-app] #3707 or #3705 #3704 #3703 #3702 #3699 #3701 #3640 #3700 #3698 #3695
[mk-app] #3708 => #1380 #3706
[inst-discovered] theory-solving 0 basic# ; #3708
[mk-app] #3709 = #3708 #3707
[instance] 0 #3709
[attach-enode] #3709 0
[end-of-instance]
[mk-app] #3706 not #3630
[mk-app] #3708 or #3706 #3705 #3704 #3703 #3702 #3699 #3701 #3640 #3700 #3698 #3695
[mk-app] #3709 => #3630 #3707
[inst-discovered] theory-solving 0 basic# ; #3709
[mk-app] #3710 = #3709 #3708
[instance] 0 #3710
[attach-enode] #3710 0
[end-of-instance]
[mk-app] #3707 not #3708
[mk-app] #3709 not #3695
[begin-check] 1
[mk-app] #3638 I #1025
[mk-app] #3639 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #1396 #2574 #3638
[mk-app] #3630 = #3639 #750
[mk-app] #3706 = #779 #3630
[mk-app] #3705 not #3706
[inst-discovered] theory-solving 0 basic# ; #3705
[mk-app] #3704 = #3705 #3705
[instance] 0 #3704
[attach-enode] #3704 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3645
[mk-app] #3704 = #3645 #3645
[instance] 0 #3704
[attach-enode] #3704 0
[end-of-instance]
[mk-app] #3704 uInv #274 #1025
[mk-app] #3703 or #3650 #3704
[mk-app] #3702 not #3704
[mk-app] #3699 uClip #274 #1025
[mk-app] #3701 = #659 #3699
[mk-app] #3700 not #3701
[mk-app] #3698 = #1487 #1487
[inst-discovered] theory-solving 0 arith# ; #3698
[mk-app] #3708 = #3698 #1
[instance] 0 #3708
[attach-enode] #3708 0
[end-of-instance]
[mk-app] #3698 or #3658 #1
[inst-discovered] theory-solving 0 basic# ; #3698
[mk-app] #3708 = #3698 #1
[instance] 0 #3708
[attach-enode] #3708 0
[end-of-instance]
[mk-app] #3698 or #3660 #1
[inst-discovered] theory-solving 0 basic# ; #3698
[mk-app] #3708 = #3698 #1
[instance] 0 #3708
[attach-enode] #3708 0
[end-of-instance]
[mk-app] #3698 and #3657 #3663 #3665 #3674 #3676
[mk-app] #3708 and #3657 #1 #1 #3663 #3665 #3674 #3676
[inst-discovered] theory-solving 0 basic# ; #3708
[mk-app] #3707 = #3708 #3698
[instance] 0 #3707
[attach-enode] #3707 0
[end-of-instance]
[mk-app] #3708 or #3702 #3700 #3698
[mk-app] #3707 and #3703 #3708
[mk-app] #3710 or #3683 #3684 #3682 #3707
[inst-discovered] theory-solving 0 basic# ; #3710
[mk-app] #3711 = #3710 #3710
[instance] 0 #3711
[attach-enode] #3711 0
[end-of-instance]
[mk-app] #3711 and #3649 #3710
[mk-app] #3712 or #3687 #3711
[mk-app] #3713 and #3647 #3712
[mk-app] #3714 or #3692 #3690 #3713
[mk-app] #3715 and #3645 #3714
[mk-app] #3716 or #3696 #3705 #3715
[mk-app] #3717 and #3643 #3716
[mk-app] #3718 not #3717
[inst-discovered] theory-solving 0 basic# ; #3705
[mk-app] #3694 = #3705 #3705
[instance] 0 #3694
[attach-enode] #3694 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3645
[mk-app] #3694 = #3645 #3645
[instance] 0 #3694
[attach-enode] #3694 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3710
[mk-app] #3694 = #3710 #3710
[instance] 0 #3694
[attach-enode] #3694 0
[end-of-instance]
[mk-app] #3694 = #1273 #1025
[inst-discovered] theory-solving 0 basic# ; #3705
[mk-app] #3651 = #3705 #3705
[instance] 0 #3651
[attach-enode] #3651 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3645
[mk-app] #3651 = #3645 #3645
[instance] 0 #3651
[attach-enode] #3651 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3710
[mk-app] #3651 = #3710 #3710
[instance] 0 #3651
[attach-enode] #3651 0
[end-of-instance]
[mk-app] #3651 not #3643
[mk-app] #3680 not #3645
[mk-app] #3678 not #3647
[mk-app] #3658 not #3649
[mk-app] #3659 not #3703
[mk-app] #3660 not #3657
[mk-app] #3661 not #3663
[mk-app] #3677 not #3665
[mk-app] #3681 k$!skolem_user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS__max_contig_sub_sum_opt_28!101
[mk-app] #3679 has_type #3681 #189
[mk-app] #3686 not #3679
[mk-app] #3685 %I #3681
[mk-app] #3688 >= #3685 #341
[mk-app] #3689 + #3685 #3666
[mk-app] #3693 <= #3689 #341
[mk-app] #3691 and #3688 #3693
[mk-app] #3697 not #3691
[mk-app] #3695 vstd!seq.Seq.index.? #125 #1167 #459 #3681
[mk-app] #3709 %I #3695
[mk-app] #3719 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #1396 #3681
[mk-app] #3720 = #3709 #3719
[mk-app] #3721 or #3686 #3697 #3720
[mk-app] #3722 not #3721
[mk-app] #3723 and #444 #3722
[mk-app] #3724 not #3676
[mk-app] #3725 or #3660 #3661 #3677 #3723 #3724
[mk-app] #3726 and #3704 #3701 #3725
[mk-app] #3727 or #3659 #3726
[mk-app] #3728 and #706 #687 #673 #3727
[mk-app] #3729 or #3658 #3728
[mk-app] #3730 and #721 #3729
[mk-app] #3731 or #3678 #3730
[mk-app] #3732 and #779 #696 #3731
[mk-app] #3733 or #3680 #3732
[mk-app] #3734 and #778 #3706 #3733
[mk-app] #3735 or #3651 #3734
[inst-discovered] theory-solving 0 basic# ; #3645
[mk-app] #3736 = #3645 #3645
[instance] 0 #3736
[attach-enode] #3736 0
[end-of-instance]
[mk-app] #3736 + #3666 #3685
[inst-discovered] theory-solving 0 arith# ; #3689
[mk-app] #3737 = #3689 #3736
[instance] 0 #3737
[attach-enode] #3737 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3737 * #370 #3685
[mk-app] #3738 + #659 #3737
[mk-app] #3739 >= #3738 #341
[mk-app] #3740 <= #3736 #341
[inst-discovered] theory-solving 0 arith# ; #3740
[mk-app] #3741 = #3740 #3739
[instance] 0 #3741
[attach-enode] #3741 0
[end-of-instance]
[mk-app] #3736 and #3688 #3739
[mk-app] #3740 not #3736
[mk-app] #3741 or #3686 #3740 #3720
[mk-app] #3742 not #3741
[mk-app] #3743 and #444 #3742
[mk-app] #3744 or #3660 #3661 #3677 #3743 #3724
[inst-discovered] theory-solving 0 basic# ; #3744
[mk-app] #3745 = #3744 #3744
[instance] 0 #3745
[attach-enode] #3745 0
[end-of-instance]
[mk-app] #3745 and #3704 #3701 #3744
[mk-app] #3746 or #3659 #3745
[mk-app] #3747 and #706 #687 #673 #3746
[mk-app] #3748 or #3658 #3747
[mk-app] #3749 and #721 #3748
[mk-app] #3750 or #3678 #3749
[mk-app] #3751 and #779 #696 #3750
[mk-app] #3752 or #3680 #3751
[mk-app] #3753 and #778 #3706 #3752
[mk-app] #3754 or #3651 #3753
[mk-app] #3666 not #3633
[mk-app] #3689 or #3260 #3666
[mk-app] #3693 not #3689
[inst-discovered] theory-solving 0 basic# ; #3634
[mk-app] #3691 = #3634 #3693
[instance] 0 #3691
[attach-enode] #3691 0
[end-of-instance]
[mk-app] #3691 not #3693
[inst-discovered] theory-solving 0 basic# ; #3691
[mk-app] #3697 = #3691 #3689
[instance] 0 #3697
[attach-enode] #3697 0
[end-of-instance]
[mk-app] #3693 or #197 #3260 #3666 #1112
[mk-app] #3691 or #197 #3689 #1112
[inst-discovered] theory-solving 0 basic# ; #3691
[mk-app] #3697 = #3691 #3693
[instance] 0 #3697
[attach-enode] #3697 0
[end-of-instance]
[mk-quant] #3689 user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS__max_contig_sub_sum_opt_28 1 #1104 #3693
[attach-var-names] #3689 (|k$| ; |Poly|)
[inst-discovered] theory-solving 0 basic# ; #3645
[mk-app] #3691 = #3645 #3645
[instance] 0 #3691
[attach-enode] #3691 0
[end-of-instance]
[mk-app] #3691 not #3653
[mk-app] #3697 not #3654
[mk-app] #3721 or #3691 #3697
[mk-app] #3667 not #3721
[inst-discovered] theory-solving 0 basic# ; #3655
[mk-app] #3668 = #3655 #3667
[instance] 0 #3668
[attach-enode] #3668 0
[end-of-instance]
[mk-app] #3668 or #3656 #3667
[mk-app] #3669 not #3668
[mk-app] #3670 not #3688
[mk-app] #3672 not #3739
[mk-app] #3671 or #3670 #3672
[mk-app] #3722 not #3671
[inst-discovered] theory-solving 0 basic# ; #3736
[mk-app] #3723 = #3736 #3722
[instance] 0 #3723
[attach-enode] #3723 0
[end-of-instance]
[mk-app] #3723 not #3722
[inst-discovered] theory-solving 0 basic# ; #3723
[mk-app] #3725 = #3723 #3671
[instance] 0 #3725
[attach-enode] #3725 0
[end-of-instance]
[mk-app] #3722 or #3686 #3670 #3672 #3720
[mk-app] #3723 or #3686 #3671 #3720
[inst-discovered] theory-solving 0 basic# ; #3723
[mk-app] #3725 = #3723 #3722
[instance] 0 #3725
[attach-enode] #3725 0
[end-of-instance]
[mk-app] #3671 not #3722
[mk-app] #3723 not #444
[mk-app] #3725 or #3723 #3686 #3670 #3672 #3720
[mk-app] #3726 not #3725
[mk-app] #3727 and #444 #3671
[inst-discovered] theory-solving 0 basic# ; #3727
[mk-app] #3728 = #3727 #3726
[instance] 0 #3728
[attach-enode] #3728 0
[end-of-instance]
[mk-app] #3722 or #3669 #3661 #3677 #3726 #3724
[inst-discovered] theory-solving 0 basic# ; #3722
[mk-app] #3671 = #3722 #3722
[instance] 0 #3671
[attach-enode] #3671 0
[end-of-instance]
[mk-app] #3671 not #3704
[mk-app] #3727 not #3701
[mk-app] #3728 not #3722
[mk-app] #3729 or #3671 #3727 #3728
[mk-app] #3730 not #3729
[mk-app] #3731 and #3704 #3701 #3722
[inst-discovered] theory-solving 0 basic# ; #3731
[mk-app] #3732 = #3731 #3730
[instance] 0 #3732
[attach-enode] #3732 0
[end-of-instance]
[mk-app] #3731 or #3659 #3730
[mk-app] #3732 not #706
[mk-app] #3733 not #687
[mk-app] #3734 not #673
[mk-app] #3735 not #3731
[mk-app] #3696 or #3732 #3733 #3734 #3735
[mk-app] #3705 not #3696
[mk-app] #3692 and #706 #687 #673 #3731
[inst-discovered] theory-solving 0 basic# ; #3692
[mk-app] #3690 = #3692 #3705
[instance] 0 #3690
[attach-enode] #3690 0
[end-of-instance]
[mk-app] #3692 or #3658 #3705
[mk-app] #3690 not #721
[mk-app] #3687 not #3692
[mk-app] #3683 or #3690 #3687
[mk-app] #3684 not #3683
[mk-app] #3682 and #721 #3692
[inst-discovered] theory-solving 0 basic# ; #3682
[mk-app] #3702 = #3682 #3684
[instance] 0 #3702
[attach-enode] #3702 0
[end-of-instance]
[mk-app] #3682 or #3678 #3684
[mk-app] #3702 not #779
[mk-app] #3700 not #696
[mk-app] #3673 not #3682
[mk-app] #3674 or #3702 #3700 #3673
[mk-app] #3698 not #3674
[mk-app] #3708 and #779 #696 #3682
[inst-discovered] theory-solving 0 basic# ; #3708
[mk-app] #3707 = #3708 #3698
[instance] 0 #3707
[attach-enode] #3707 0
[end-of-instance]
[mk-app] #3708 or #3680 #3698
[mk-app] #3707 not #778
[mk-app] #3710 not #3706
[mk-app] #3711 not #3708
[mk-app] #3712 or #3707 #3710 #3711
[mk-app] #3713 not #3712
[mk-app] #3714 and #778 #3706 #3708
[inst-discovered] theory-solving 0 basic# ; #3714
[mk-app] #3715 = #3714 #3713
[instance] 0 #3715
[attach-enode] #3715 0
[end-of-instance]
[mk-app] #3714 or #3651 #3713
[mk-app] #3655 or #197 #3260
[mk-app] #3657 or #3666 #1112
[mk-app] #3660 or #3655 #3657
[mk-quant] #3736 user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS__max_contig_sub_sum_opt_28 1 #1104 #3660
[attach-var-names] #3736 (|k$| ; |Poly|)
[mk-app] #3740 or #3723 #3686
[mk-app] #3741 or #3670 #3672
[mk-app] #3742 or #3740 #3741
[mk-app] #3743 or #3742 #3720
[mk-app] #3744 not #3743
[mk-app] #3745 or #3669 #3661
[mk-app] #3746 or #3677 #3744
[mk-app] #3747 or #3745 #3746
[mk-app] #3748 or #3747 #3724
[mk-app] #3749 not #3748
[mk-app] #3750 or #3671 #3727
[mk-app] #3751 or #3750 #3749
[mk-app] #3752 not #3751
[mk-app] #3753 or #3659 #3752
[mk-app] #3754 not #3753
[mk-app] #3634 or #3732 #3733
[mk-app] #3635 or #3734 #3754
[mk-app] #3637 or #3634 #3635
[mk-app] #3636 not #3637
[mk-app] #3715 or #3658 #3636
[mk-app] #3716 not #3715
[mk-app] #3717 or #3690 #3716
[mk-app] #3718 not #3717
[mk-app] #3755 or #3678 #3718
[mk-app] #3756 not #3755
[mk-app] #3757 or #3702 #3700
[mk-app] #3758 or #3757 #3756
[mk-app] #3759 not #3758
[mk-app] #3760 or #3680 #3759
[mk-app] #3761 not #3760
[mk-app] #3762 or #3707 #3710
[mk-app] #3763 or #3762 #3761
[mk-app] #3764 not #3763
[mk-app] #3765 or #3651 #3764
[mk-app] #3725 or #197 #3260 #3666 #1112
[inst-discovered] theory-solving 0 basic# ; #3660
[mk-app] #3726 = #3660 #3725
[instance] 0 #3726
[attach-enode] #3726 0
[end-of-instance]
[mk-quant] #3726 user_lib__Chap19__ArraySeqStEph__ArraySeqStEph__ArraySeqStEphS__max_contig_sub_sum_opt_28 1 #1104 #3725
[attach-var-names] #3726 (|k$| ; |Poly|)
[inst-discovered] theory-solving 0 basic# ; #3710
[mk-app] #3722 = #3710 #3710
[instance] 0 #3722
[attach-enode] #3722 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3645
[mk-app] #3722 = #3645 #3645
[instance] 0 #3722
[attach-enode] #3722 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3745
[mk-app] #3722 = #3745 #3745
[instance] 0 #3722
[attach-enode] #3722 0
[end-of-instance]
[mk-app] #3722 or #3723 #3686 #3670 #3672
[inst-discovered] theory-solving 0 basic# ; #3742
[mk-app] #3728 = #3742 #3722
[instance] 0 #3728
[attach-enode] #3728 0
[end-of-instance]
[mk-app] #3728 or #3723 #3686 #3670 #3672 #3720
[mk-app] #3729 or #3722 #3720
[inst-discovered] theory-solving 0 basic# ; #3729
[mk-app] #3730 = #3729 #3728
[instance] 0 #3730
[attach-enode] #3730 0
[end-of-instance]
[mk-app] #3729 not #3728
[mk-app] #3730 or #3677 #3729
[mk-app] #3731 or #3669 #3661 #3677 #3729
[mk-app] #3735 or #3745 #3730
[inst-discovered] theory-solving 0 basic# ; #3735
[mk-app] #3696 = #3735 #3731
[instance] 0 #3696
[attach-enode] #3696 0
[end-of-instance]
[mk-app] #3735 or #3669 #3661 #3677 #3729 #3724
[mk-app] #3696 or #3731 #3724
[inst-discovered] theory-solving 0 basic# ; #3696
[mk-app] #3705 = #3696 #3735
[instance] 0 #3705
[attach-enode] #3705 0
[end-of-instance]
[mk-app] #3696 not #3735
[mk-app] #3705 or #3671 #3727 #3696
[mk-app] #3692 or #3750 #3696
[inst-discovered] theory-solving 0 basic# ; #3692
[mk-app] #3687 = #3692 #3705
[instance] 0 #3687
[attach-enode] #3687 0
[end-of-instance]
[mk-app] #3692 not #3705
[mk-app] #3687 or #3659 #3692
[mk-app] #3683 not #3687
[mk-app] #3684 or #3734 #3683
[mk-app] #3682 or #3732 #3733 #3734 #3683
[mk-app] #3673 or #3634 #3684
[inst-discovered] theory-solving 0 basic# ; #3673
[mk-app] #3674 = #3673 #3682
[instance] 0 #3674
[attach-enode] #3674 0
[end-of-instance]
[mk-app] #3673 not #3682
[mk-app] #3674 or #3658 #3673
[mk-app] #3698 not #3674
[mk-app] #3708 or #3690 #3698
[mk-app] #3711 not #3708
[mk-app] #3712 or #3678 #3711
[mk-app] #3713 not #3712
[mk-app] #3714 or #3702 #3700 #3713
[mk-app] #3693 or #3757 #3713
[inst-discovered] theory-solving 0 basic# ; #3693
[mk-app] #3689 = #3693 #3714
[instance] 0 #3689
[attach-enode] #3689 0
[end-of-instance]
[mk-app] #3693 not #3714
[mk-app] #3689 or #3680 #3693
[mk-app] #3766 not #3689
[mk-app] #3767 or #3707 #3710 #3766
[mk-app] #3768 or #3762 #3766
[inst-discovered] theory-solving 0 basic# ; #3768
[mk-app] #3769 = #3768 #3767
[instance] 0 #3769
[attach-enode] #3769 0
[end-of-instance]
[mk-app] #3768 not #3767
[mk-app] #3769 or #3651 #3768
[inst-discovered] theory-solving 0 basic# ; #3645
[mk-app] #3763 = #3645 #3645
[instance] 0 #3763
[attach-enode] #3763 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3735
[mk-app] #3763 = #3735 #3735
[instance] 0 #3763
[attach-enode] #3763 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3682
[mk-app] #3763 = #3682 #3682
[instance] 0 #3763
[attach-enode] #3763 0
[end-of-instance]
[mk-app] #3763 or #3651 #778
[mk-app] #3764 or #3651 #3706
[mk-app] #3765 or #3651 #3689
[mk-app] #3767 or #778 #1016
[mk-app] #3768 not #940
[mk-app] #3769 or #778 #3768
[mk-app] #3660 or #3706 #1016
[mk-app] #3736 or #3706 #3768
[mk-app] #3770 or #3689 #1016
[mk-app] #3771 or #3689 #3768
[assign] #23 justification -1: 
[attach-enode] #1533 0
[attach-enode] #1534 0
[attach-enode] #1525 0
[assign] #1525 justification -1: 
[attach-enode] #1526 0
[attach-enode] #1486 0
[assign] #1486 justification -1: 
[attach-enode] #1487 0
[attach-enode] #1478 0
[assign] #1478 justification -1: 
[attach-enode] #1479 0
[attach-enode] #1468 0
[assign] #1468 justification -1: 
[attach-enode] #1469 0
[attach-enode] #1461 0
[assign] #1461 justification -1: 
[attach-enode] #1462 0
[attach-enode] #1453 0
[assign] #1453 justification -1: 
[attach-enode] #1454 0
[attach-enode] #1446 0
[assign] #1446 justification -1: 
[assign] #3627 justification -1: 
[attach-enode] #370 0
[attach-enode] #3626 0
[attach-enode] #3628 0
[assign] #3629 justification -1: 
[attach-enode] #1393 0
[mk-app] #3730 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #1393
[mk-app] #3722 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS #3730
[attach-enode] #3730 0
[attach-enode] #3722 0
[mk-app] #3684 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #3722
[mk-app] #3731 = #3730 #3684
[new-match] 0 datatype#21 datatype#18 #3730 ; #3722
[instance] 0 #3731
[attach-enode] #3684 0
[end-of-instance]
[attach-enode] #1395 0
[attach-enode] #1396 0
[attach-enode] #1379 0
[attach-enode] #1380 0
[assign] #1380 justification -1: 
[attach-enode] #1287 0
[assign] #1287 justification -1: 
[attach-enode] #1288 0
[attach-enode] #1289 0
[attach-enode] #1290 0
[attach-enode] #1273 0
[attach-enode] #1025 0
[attach-enode] #3694 0
[assign] #3694 justification -1: 
[attach-enode] #1276 0
[attach-enode] #2899 0
[attach-enode] #2900 0
[assign] #2900 justification -1: 
[assign] #3726 justification -1: 
[attach-enode] #2886 0
[attach-enode] #2889 0
[attach-enode] #2890 0
[assign] #2890 justification -1: 
[assign] (not #3640) justification -1: 
[attach-enode] #2892 0
[attach-enode] #1014 0
[attach-enode] #1024 0
[assign] #1024 justification -1: 
[attach-enode] #296 0
[attach-enode] #1274 0
[attach-enode] #1015 0
[assign] #1015 justification -1: 
[attach-enode] #341 0
[attach-enode] #778 0
[attach-enode] #940 0
[attach-enode] #2574 0
[attach-enode] #3638 0
[attach-enode] #3639 0
[attach-enode] #782 0
[attach-enode] #748 0
[attach-enode] #749 0
[attach-enode] #750 0
[attach-enode] #3630 0
[attach-enode] #741 0
[attach-enode] #696 0
[attach-enode] #728 0
[attach-enode] #720 0
[attach-enode] #721 0
[attach-enode] #713 0
[attach-enode] #705 0
[attach-enode] #706 0
[attach-enode] #698 0
[attach-enode] #686 0
[attach-enode] #687 0
[attach-enode] #652 0
[attach-enode] #658 0
[attach-enode] #672 0
[attach-enode] #673 0
[attach-enode] #3704 0
[attach-enode] #659 0
[attach-enode] #3699 0
[attach-enode] #3701 0
[attach-enode] #3652 0
[attach-enode] #459 0
[attach-enode] #460 0
[attach-enode] #378 0
[attach-enode] #380 0
[attach-enode] #434 0
[attach-enode] #436 0
[attach-enode] #437 0
[attach-enode] #3681 0
[attach-enode] #3679 0
[attach-enode] #3685 0
[attach-enode] #3737 0
[attach-enode] #3738 0
[attach-enode] #3695 0
[attach-enode] #3709 0
[attach-enode] #3719 0
[attach-enode] #3720 0
[attach-enode] #384 0
[attach-enode] #354 0
[assign] #29 bin 1
[eq-expl] #1533 root
[eq-expl] #1534 root
[eq-expl] #1393 ax ; #3722
[eq-expl] #3722 root
[new-match] 0x61d50e5dc488 #1330 #1327 #1533 #1167 #125 ; #1525 (#1534 #1534) #1393 (#1533 #1533) #1395 (#1393 #3722)
[new-match] 0x61d50e5dc4c8 #1307 #1303 #1533 #1167 #125 ; #1525 (#1534 #1534)
[eq-expl] #1526 root
[new-match] 0x61d50e5dc508 #527 #211 #1526 ; #1486 (#208 #208)
[new-match] 0x61d50e5dc538 #215 #211 #1526 ; #1486 (#208 #208)
[eq-expl] #1479 root
[eq-expl] #1168 root
[new-match] 0x61d50e5dc568 #1178 #1174 #1479 ; #1468 (#1168 #1168)
[eq-expl] #274 root
[eq-expl] #1487 lit #1380 ; #1379
[eq-expl] #1379 root
[new-match] 0x61d50e5dc598 #1086 #449 #1487 #274 ; #1478
[eq-expl] #1469 root
[new-match] 0x61d50e5dc5d0 #1086 #449 #1469 #274 ; #1461
[eq-expl] #1454 root
[new-match] 0x61d50e5dc608 #1086 #449 #1454 #274 ; #1446
[eq-expl] #275 root
[eq-expl] #1462 lit #2900 ; #2899
[eq-expl] #2899 root
[new-match] 0x61d50e5dc640 #1136 #455 #1462 #275 ; #1453
[eq-expl] #1396 root
[new-match] 0x61d50e5dc678 #1545 #1540 #1396 #1167 #125 ; #1379
[eq-expl] #1290 root
[new-match] 0x61d50e5dc6b8 #1545 #1540 #1290 #1167 #125 ; #1273
[eq-expl] #1395 root
[new-match] 0x61d50e5dc6f8 #1810 #1807 #1395 #1168 #125 ; #1396
[eq-expl] #1289 root
[new-match] 0x61d50e5dc738 #1810 #1807 #1289 #1168 #125 ; #1290
[new-match] 0x61d50e5dc778 #1323 #1322 #1393 ; #1395
[eq-expl] #1288 root
[new-match] 0x61d50e5dc7a8 #1166 #1165 #1288 ; #1289
[new-match] 0x61d50e5dc7d8 #174 #173 #1454 ; #1276
[new-match] 0x61d50e5dc808 #567 #559 #1454 #1487 ; #2889
[eq-expl] #296 root
[new-match] 0x61d50e5dc840 #552 #550 #296 #1454 ; #1274
[mk-app] #3731 not #1525
[mk-app] #3766 has_type #1395 #1168
[mk-app] #3642 or #3731 #3766
[mk-app] #3643 not #1330
[mk-app] #3651 or #3643 #3731 #3766
[instance] 0x61d50e5dc488 ; 1
[attach-enode] #3766 1
[assign] #3766 justification -1: 272 469
[end-of-instance]
[mk-app] #3765 Poly%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #1393
[mk-app] #3764 = #1533 #3765
[mk-app] #3763 or #3731 #3764
[mk-app] #3772 not #1307
[mk-app] #3773 or #3772 #3731 #3764
[instance] 0x61d50e5dc4c8 ; 1
[attach-enode] #3765 1
[attach-enode] #3764 1
[assign] #3764 justification -1: 269 469
[end-of-instance]
[mk-app] #3774 not #1486
[mk-app] #3775 %I #1526
[mk-app] #3776 uInv #274 #3775
[mk-app] #3777 or #3774 #3776
[mk-app] #3778 not #527
[mk-app] #3779 or #3778 #3774 #3776
[instance] 0x61d50e5dc508 ; 1
[attach-enode] #3775 1
[attach-enode] #3776 1
[assign] #3776 justification -1: 72 470
[end-of-instance]
[mk-app] #3780 I #3775
[mk-app] #3781 = #1526 #3780
[mk-app] #3782 or #3774 #3781
[mk-app] #3783 not #215
[mk-app] #3784 or #3783 #3774 #3781
[instance] 0x61d50e5dc538 ; 1
[attach-enode] #3780 1
[attach-enode] #3781 1
[assign] #3781 justification -1: 31 470
[end-of-instance]
[mk-app] #3785 not #1468
[mk-app] #3786 %Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #1479
[mk-app] #3787 Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #3786
[mk-app] #3788 = #1479 #3787
[mk-app] #3789 or #3785 #3788
[mk-app] #3790 not #1178
[mk-app] #3791 or #3790 #3785 #3788
[instance] 0x61d50e5dc568 ; 1
[attach-enode] #3786 1
[attach-enode] #3787 1
[attach-enode] #3788 1
[assign] #3788 justification -1: 254 472
[end-of-instance]
[mk-app] #3792 >= #1379 #341
[mk-app] #3793 not #3792
[mk-app] #3794 + #1379 #2466
[mk-app] #3795 >= #3794 #341
[mk-app] #3796 or #3793 #3795
[mk-app] #3797 uInv #274 #1379
[mk-app] #3798 = #3796 #3797
[mk-app] #3799 not #3798
[mk-app] #3800 not #3796
[inst-discovered] theory-solving 0 basic# ; #3799
[mk-app] #3800 = #3799 #3799
[instance] 0 #3800
[attach-enode] #3800 0
[end-of-instance]
[mk-app] #3800 not #1086
[mk-app] #3801 or #3800 #3799
[instance] 0x61d50e5dc598 ; 1
[attach-enode] #2449 1
[attach-enode] #2466 1
[attach-enode] #3794 1
[attach-enode] #3797 1
[assign] (not #3798) justification -1: 60
[end-of-instance]
[mk-app] #3802 >= #1469 #341
[mk-app] #3803 not #3802
[mk-app] #3804 + #1469 #2466
[mk-app] #3805 >= #3804 #341
[mk-app] #3806 or #3803 #3805
[mk-app] #3807 = #3806 #1461
[mk-app] #3808 not #3807
[mk-app] #3809 not #3806
[inst-discovered] theory-solving 0 basic# ; #3808
[mk-app] #3809 = #3808 #3808
[instance] 0 #3809
[attach-enode] #3809 0
[end-of-instance]
[mk-app] #3809 or #3800 #3808
[instance] 0x61d50e5dc5d0 ; 1
[attach-enode] #3804 1
[assign] (not #3807) justification -1: 60
[end-of-instance]
[mk-app] #3810 not #3627
[mk-app] #3811 + #1454 #2466
[mk-app] #3812 >= #3811 #341
[mk-app] #3813 or #3810 #3812
[mk-app] #3814 = #3813 #1446
[mk-app] #3815 not #3814
[mk-app] #3816 not #3813
[inst-discovered] theory-solving 0 basic# ; #3815
[mk-app] #3816 = #3815 #3815
[instance] 0 #3816
[attach-enode] #3816 0
[end-of-instance]
[mk-app] #3816 or #3800 #3815
[instance] 0x61d50e5dc608 ; 1
[attach-enode] #3811 1
[assign] (not #3814) justification -1: 60
[end-of-instance]
[mk-app] #3817 * #370 #313
[mk-app] #3818 + #2899 #3817
[mk-app] #3819 >= #3818 #341
[mk-app] #3820 not #3819
[mk-app] #3821 * #370 #335
[mk-app] #3822 + #2899 #3821
[mk-app] #3823 >= #3822 #341
[mk-app] #3824 or #3820 #3823
[mk-app] #3825 iInv #275 #2899
[mk-app] #3826 = #3824 #3825
[mk-app] #3827 not #3826
[mk-app] #3828 + #3817 #2899
[inst-discovered] theory-solving 0 arith# ; #3818
[mk-app] #3829 = #3818 #3828
[instance] 0 #3829
[attach-enode] #3829 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3829 * #370 #2899
[mk-app] #3830 + #313 #3829
[mk-app] #3831 <= #3830 #341
[mk-app] #3832 >= #3828 #341
[inst-discovered] theory-solving 0 arith# ; #3832
[mk-app] #3833 = #3832 #3831
[instance] 0 #3833
[attach-enode] #3833 0
[end-of-instance]
[mk-app] #3828 not #3831
[mk-app] #3832 + #3821 #2899
[inst-discovered] theory-solving 0 arith# ; #3822
[mk-app] #3833 = #3822 #3832
[instance] 0 #3833
[attach-enode] #3833 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3833 + #335 #3829
[mk-app] #3834 <= #3833 #341
[mk-app] #3835 >= #3832 #341
[inst-discovered] theory-solving 0 arith# ; #3835
[mk-app] #3836 = #3835 #3834
[instance] 0 #3836
[attach-enode] #3836 0
[end-of-instance]
[mk-app] #3832 or #3828 #3834
[mk-app] #3835 = #3832 #3825
[mk-app] #3836 not #3832
[mk-app] #3837 not #3835
[inst-discovered] theory-solving 0 basic# ; #3837
[mk-app] #3836 = #3837 #3837
[instance] 0 #3836
[attach-enode] #3836 0
[end-of-instance]
[mk-app] #3836 not #1136
[mk-app] #3838 or #3836 #3837
[instance] 0x61d50e5dc640 ; 1
[attach-enode] #3829 1
[attach-enode] #3830 1
[attach-enode] #3833 1
[attach-enode] #3825 1
[assign] (not #3835) justification -1: 61
[end-of-instance]
[mk-app] #3839 has_type #1396 #1188
[mk-app] #3840 not #3839
[mk-app] #3841 or #3840 #3792
[mk-app] #3842 not #1545
[mk-app] #3843 or #3842 #3840 #3792
[instance] 0x61d50e5dc678 ; 1
[attach-enode] #3839 1
[end-of-instance]
[mk-app] #3844 has_type #1290 #1188
[mk-app] #3845 not #3844
[mk-app] #3846 >= #1273 #341
[mk-app] #3847 or #3845 #3846
[mk-app] #3848 or #3842 #3845 #3846
[instance] 0x61d50e5dc6b8 ; 1
[attach-enode] #3844 1
[end-of-instance]
[mk-app] #3849 not #3766
[mk-app] #3850 proj%vstd!view.View./V #125 #1168
[mk-app] #3851 has_type #1396 #3850
[mk-app] #3852 or #3849 #3851
[mk-app] #3853 not #1810
[mk-app] #3854 or #3853 #3849 #3851
[instance] 0x61d50e5dc6f8 ; 1
[attach-enode] #3850 1
[attach-enode] #3851 1
[assign] #3851 justification -1: 332 538
[end-of-instance]
[mk-app] #3855 has_type #1289 #1168
[mk-app] #3856 not #3855
[mk-app] #3857 has_type #1290 #3850
[mk-app] #3858 or #3856 #3857
[mk-app] #3859 or #3853 #3856 #3857
[instance] 0x61d50e5dc738 ; 1
[attach-enode] #3855 1
[attach-enode] #3857 1
[end-of-instance]
[mk-app] #3860 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq #3722
[mk-app] #3861 = #3860 #3684
[inst-discovered] theory-solving 0 datatype# ; #3684
[mk-app] #3862 = #3684 #3730
[instance] 0 #3862
[attach-enode] #3862 0
[end-of-instance]
[mk-app] #3862 = #3860 #3730
[mk-app] #3863 not #1323
[mk-app] #3864 or #3863 #3862
[instance] 0x61d50e5dc778 ; 1
[attach-enode] #3860 1
[attach-enode] #3862 1
[assign] #3862 justification -1: 271
[end-of-instance]
[mk-app] #3865 %Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #1289
[mk-app] #3866 = #1288 #3865
[mk-app] #3867 not #1166
[mk-app] #3868 or #3867 #3866
[instance] 0x61d50e5dc7a8 ; 1
[attach-enode] #3865 1
[attach-enode] #3866 1
[assign] #3866 justification -1: 253
[end-of-instance]
[mk-app] #3869 %I #1276
[mk-app] #3870 = #1454 #3869
[mk-app] #3871 not #174
[mk-app] #3872 or #3871 #3870
[instance] 0x61d50e5dc7d8 ; 1
[attach-enode] #3869 1
[attach-enode] #3870 1
[assign] #3870 justification -1: 26
[end-of-instance]
[mk-app] #3873 * #370 #1379
[mk-app] #3874 Sub #1379 #1454
[mk-app] #3875 + #1454 #3873 #3874
[mk-app] #3876 = #3875 #341
[mk-app] #3877 + #3873 #1454 #3874
[inst-discovered] theory-solving 0 arith# ; #3875
[mk-app] #3878 = #3875 #3877
[instance] 0 #3878
[attach-enode] #3878 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[attach-meaning] #370 arith (- 1)
[mk-app] #3878 * #370 #3874
[mk-app] #3879 + #1379 #3631 #3878
[mk-app] #3880 = #3879 #341
[mk-app] #3881 = #3877 #341
[inst-discovered] theory-solving 0 arith# ; #3881
[mk-app] #3882 = #3881 #3880
[instance] 0 #3882
[attach-enode] #3882 0
[end-of-instance]
[mk-app] #3877 not #567
[mk-app] #3881 or #3877 #3880
[instance] 0x61d50e5dc808 ; 1
[attach-enode] #3631 1
[attach-enode] #3874 1
[attach-enode] #3878 1
[attach-enode] #3879 1
[attach-enode] #3880 1
[mk-app] #3882 <= #3879 #341
[mk-app] #3883 >= #3879 #341
[assign] #3880 justification -1: 78
[end-of-instance]
[mk-app] #3884 * #370 #1274
[mk-app] #3885 + #296 #1454 #3884
[mk-app] #3886 = #3885 #341
[mk-app] #3887 + #296 #3884 #1454
[inst-discovered] theory-solving 0 arith# ; #3885
[mk-app] #3888 = #3885 #3887
[instance] 0 #3888
[attach-enode] #3888 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3888 + #3884 #1454
[attach-meaning] #370 arith (- 1)
[mk-app] #3889 + #1274 #3631
[mk-app] #3888 = #3889 #296
[mk-app] #3890 = #3887 #341
[inst-discovered] theory-solving 0 arith# ; #3890
[mk-app] #3891 = #3890 #3888
[instance] 0 #3891
[attach-enode] #3891 0
[end-of-instance]
[mk-app] #3887 not #552
[mk-app] #3890 or #3887 #3888
[instance] 0x61d50e5dc840 ; 1
[attach-enode] #3889 1
[attach-enode] #3888 1
[mk-app] #3891 <= #3889 #296
[mk-app] #3892 >= #3889 #296
[assign] #3888 justification -1: 77
[end-of-instance]
[assign] (not #3806) clause -550 551
[assign] (not #3813) clause -553 554
[assign] #3882 clause 570 -569
[assign] #3883 clause 571 -569
[assign] #3891 clause 573 -572
[assign] #3892 clause 574 -572
[assign] #3802 clause 548 550
[assign] (not #3805) clause -549 550
[assign] (not #3812) clause -552 553
[assign] #3797 justification -1: 471 478
[assign] #3825 justification -1: 474 481
[mk-app] #3893 = #1379 #1487
[attach-meaning] #370 arith (- 1)
[mk-app] #3894 + #1379 #3626
[mk-app] #3895 <= #3894 #341
[mk-app] #3896 >= #3894 #341
[assign] #3893 justification -1: 478
[attach-enode] #3893 0
[attach-enode] #3894 0
[assign] #3895 justification -1: 575
[assign] #3896 justification -1: 575
[mk-app] #3897 <= #313 #317
[mk-app] #3898 >= #313 #317
[assign] #3897 justification -1: 48
[assign] #3898 justification -1: 48
[mk-app] #3899 <= #335 #314
[mk-app] #3900 >= #335 #314
[assign] #3899 justification -1: 53
[assign] #3900 justification -1: 53
[attach-meaning] #370 arith (- 1)
[mk-app] #3901 + #1273 #3884
[mk-app] #3902 <= #3901 #341
[mk-app] #3903 >= #3901 #341
[assign] #1275 justification -1: 480 486
[attach-enode] #1275 0
[attach-enode] #3884 0
[attach-enode] #3901 0
[assign] #3902 justification -1: 582
[assign] #3903 justification -1: 582
[eq-expl] #1395 cg (#1393 #3722) ; #3860
[eq-expl] #3860 lit #3862 ; #3730
[eq-expl] #3730 ax ; #3684
[eq-expl] #3684 root
[new-match] 0x61d50e668938 #1178 #1174 #1395 ; #3766 (#1168 #1168)
[new-match] 0x61d50e668968 #1296 #1295 #1393 ; #3765
[eq-expl] #3775 root
[new-match] 0x61d50e668998 #1086 #449 #3775 #274 ; #3776
[new-match] 0x61d50e6689d0 #174 #173 #3775 ; #3780
[eq-expl] #3786 root
[new-match] 0x61d50e668a00 #1166 #1165 #3786 ; #3787
[eq-expl] #1147 root
[new-match] 0x61d50e668a30 #3273 #1484 #1147 #125 #1167 #125 ; #3850 (#125 #125) (#1168 #1168)
[eq-expl] #1533 lit #3764 ; #3765
[eq-expl] #3765 root
[new-match] 0x61d50e668a78 #1318 #1314 #3730 #1167 #125 ; #1525 (#1533 #3765) (#1534 #1534) (#1393 #3722)
[eq-expl] #1526 lit #3781 ; #3780
[eq-expl] #3780 root
[new-match] 0x61d50e668ab8 #481 #477 #3775 ; #1486 (#208 #208) (#1526 #3780)
[eq-expl] #1479 lit #3788 ; #3787
[eq-expl] #3787 root
[new-match] 0x61d50e668ae8 #1181 #1180 #3786 ; #1468 (#1168 #1168) (#1479 #3787)
[mk-app] #3904 has_type #3684 #1168
[mk-app] #3905 not #3904
[mk-app] #3906 %Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #3684
[mk-app] #3907 Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #3906
[mk-app] #3908 = #3684 #3907
[mk-app] #3909 or #3905 #3908
[mk-app] #3910 has_type #3730 #1168
[mk-app] #3911 not #3910
[mk-app] #3912 %Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #3730
[mk-app] #3913 Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #3912
[mk-app] #3914 = #3730 #3913
[mk-app] #3915 or #3911 #3914
[mk-app] #3916 or #3790 #3911 #3914
[instance] 0x61d50e668938 ; 2
[attach-enode] #3910 2
[attach-enode] #3912 2
[attach-enode] #3913 2
[attach-enode] #3914 2
[end-of-instance]
[mk-app] #3915 >= #3775 #341
[mk-app] #3917 not #3915
[mk-app] #3918 + #3775 #2466
[mk-app] #3919 >= #3918 #341
[mk-app] #3920 or #3917 #3919
[mk-app] #3921 = #3920 #3776
[mk-app] #3922 not #3921
[mk-app] #3923 + #2466 #3775
[inst-discovered] theory-solving 0 arith# ; #3918
[mk-app] #3924 = #3918 #3923
[instance] 0 #3924
[attach-enode] #3924 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #3924 * #370 #3775
[mk-app] #3925 + #2449 #3924
[mk-app] #3926 <= #3925 #341
[mk-app] #3927 >= #3923 #341
[inst-discovered] theory-solving 0 arith# ; #3927
[mk-app] #3928 = #3927 #3926
[instance] 0 #3928
[attach-enode] #3928 0
[end-of-instance]
[mk-app] #3923 or #3917 #3926
[mk-app] #3927 = #3923 #3776
[mk-app] #3928 not #3923
[mk-app] #3929 not #3927
[inst-discovered] theory-solving 0 basic# ; #3929
[mk-app] #3928 = #3929 #3929
[instance] 0 #3928
[attach-enode] #3928 0
[end-of-instance]
[mk-app] #3928 or #3800 #3929
[instance] 0x61d50e668998 ; 2
[attach-enode] #3924 2
[attach-enode] #3925 2
[assign] (not #3927) justification -1: 60
[end-of-instance]
[mk-app] #3930 not #126
[mk-app] #3931 not #1894
[mk-app] #3932 = #3850 #1188
[mk-app] #3933 or #3930 #3930 #3931 #3932
[mk-app] #3934 or #3930 #3931 #3932
[inst-discovered] theory-solving 0 basic# ; #3933
[mk-app] #3935 = #3933 #3934
[instance] 0 #3935
[attach-enode] #3935 0
[end-of-instance]
[mk-app] #3935 not #3273
[mk-app] #3936 or #3935 #3930 #3931 #3932
[instance] 0x61d50e668a30 ; 2
[attach-enode] #3932 2
[assign] #3932 justification -1: 18 290 353
[end-of-instance]
[assign] (not #3796) clause -545 -546 547
[assign] (not #3832) clause -557 -558 559
[assign] (not #3923) clause -589 590
[assign] #3792 clause 543 545
[assign] (not #3795) clause -544 545
[assign] #3831 clause 555 557
[assign] (not #3834) clause -556 557
[assign] #3915 clause 587 589
[assign] (not #3926) clause -588 589
[assign] #3910 justification -1: 538 566
[assign] #3839 justification -1: 563 591
[assign] #3846 clause 562 -476 -574 -584
[eq-expl] #1188 root
[new-match] 0x61d50e669200 #1198 #1194 #1396 ; #3839 (#1188 #1188)
[mk-app] #3934 Poly%vstd!seq.Seq<i32.>. #1014
[mk-app] #3937 = #1396 #3934
[mk-app] #3938 or #3840 #3937
[mk-app] #3939 not #1198
[mk-app] #3940 or #3939 #3840 #3937
[instance] 0x61d50e669200 ; 2
[attach-enode] #3934 2
[attach-enode] #3937 2
[assign] #3937 justification -1: 257 560
[end-of-instance]
[assign] #3914 clause 586 -585
[eq-expl] #1014 root
[new-match] 0x61d50e669408 #1187 #1186 #1014 ; #3934
[eq-expl] #3912 root
[new-match] 0x61d50e669438 #1166 #1165 #3912 ; #3913
[eq-expl] #1396 lit #3937 ; #3934
[eq-expl] #3934 root
[new-match] 0x61d50e669468 #1201 #1200 #1014 ; #3839 (#1188 #1188) (#1396 #3934)
[eq-expl] #3913 lit #3914 ; #3730
[new-match] 0x61d50e669498 #1181 #1180 #3912 ; #3766 (#1168 #1168) (#1395 #3913)
[decide-and-or] #279 #276
[push] 1
[assign] #276 decision axiom
[mk-app] #3941 = #2449 #289
[mk-app] #3942 <= #2449 #289
[mk-app] #3943 >= #2449 #289
[assign] #3941 justification -1: 43 39
[attach-enode] #3941 0
[assign] #3942 justification -1: 593
[assign] #3943 justification -1: 593
[decide-and-or] #1592 #1591
[push] 2
[assign] (not #1581) decision axiom
[eq-expl] #810 root
[new-match] 0x61d50e6696c0 #29 #28 #810 ; #1581
[mk-app] #3944 = #1581 #1580
[mk-app] #3945 not #29
[mk-app] #3946 or #3945 #3944
[instance] 0x61d50e6696c0 ; 1
[assign] (not #3944) justification -1: 307 -308
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3944
[conflict] #3944
[pop] 1 3
[assign] #3944 axiom
[assign] #1581 clause 308 -596
[assign] #1589 bin 308
[decide-and-or] #3288 #1607
[push] 2
[assign] (not #1598) decision axiom
[eq-expl] #811 root
[new-match] 0x61d50e669720 #29 #28 #811 ; #1598
[mk-app] #3945 = #1598 #907
[mk-app] #3946 not #29
[mk-app] #3947 or #3946 #3945
[instance] 0x61d50e669720 ; 1
[assign] (not #3945) justification -1: 198 -311
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3945
[conflict] #3945
[pop] 1 3
[assign] #3945 axiom
[assign] #1598 clause 311 -597
[assign] #3292 bin 311
[decide-and-or] #1627 #1626
[push] 2
[assign] (not #1617) decision axiom
[eq-expl] #812 root
[new-match] 0x61d50e669780 #29 #28 #812 ; #1617
[mk-app] #3946 = #1617 #908
[mk-app] #3947 not #29
[mk-app] #3948 or #3947 #3946
[instance] 0x61d50e669780 ; 1
[assign] (not #3946) justification -1: 199 -314
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3946
[conflict] #3946
[pop] 1 3
[assign] #3946 axiom
[assign] #1617 clause 314 -598
[assign] #1625 bin 314
[decide-and-or] #3296 #1656
[push] 2
[assign] (not #1640) decision axiom
[eq-expl] #813 root
[new-match] 0x61d50e6697e0 #29 #28 #813 ; #1640
[mk-app] #3947 = #1640 #909
[mk-app] #3948 not #29
[mk-app] #3949 or #3948 #3947
[instance] 0x61d50e6697e0 ; 1
[assign] (not #3947) justification -1: 200 -317
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3947
[conflict] #3947
[pop] 1 3
[assign] #3947 axiom
[assign] #1640 clause 317 -599
[assign] #3301 bin 317
[decide-and-or] #3303 #1675
[push] 2
[assign] (not #1658) decision axiom
[eq-expl] #814 root
[new-match] 0x61d50e673358 #29 #28 #814 ; #1658
[mk-app] #3948 = #1658 #910
[mk-app] #3949 not #29
[mk-app] #3950 or #3949 #3948
[instance] 0x61d50e673358 ; 1
[assign] (not #3948) justification -1: 201 -319
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3948
[conflict] #3948
[pop] 1 3
[assign] #3948 axiom
[assign] #1658 clause 319 -600
[assign] #3306 bin 319
[decide-and-or] #3304 #1707
[push] 2
[assign] (not #1695) decision axiom
[eq-expl] #815 root
[new-match] 0x61d50e673410 #29 #28 #815 ; #1695
[mk-app] #3949 = #1695 #911
[mk-app] #3950 not #29
[mk-app] #3951 or #3950 #3949
[instance] 0x61d50e673410 ; 1
[assign] (not #3949) justification -1: 202 -322
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3949
[conflict] #3949
[pop] 1 3
[assign] #3949 axiom
[assign] #1695 clause 322 -601
[assign] #3312 bin 322
[decide-and-or] #3317 #1729
[push] 2
[assign] (not #1709) decision axiom
[eq-expl] #816 root
[new-match] 0x61d50e673470 #29 #28 #816 ; #1709
[mk-app] #3950 = #1709 #912
[mk-app] #3951 not #29
[mk-app] #3952 or #3951 #3950
[instance] 0x61d50e673470 ; 1
[assign] (not #3950) justification -1: 203 -324
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3950
[conflict] #3950
[pop] 1 3
[assign] #3950 axiom
[assign] #1709 clause 324 -602
[assign] #3320 bin 324
[decide-and-or] #3318 #1741
[push] 2
[assign] (not #1731) decision axiom
[eq-expl] #817 root
[new-match] 0x61d50e6734d0 #29 #28 #817 ; #1731
[mk-app] #3951 = #1731 #913
[mk-app] #3952 not #29
[mk-app] #3953 or #3952 #3951
[instance] 0x61d50e6734d0 ; 1
[assign] (not #3951) justification -1: 204 -326
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3951
[conflict] #3951
[pop] 1 3
[assign] #3951 axiom
[assign] #1731 clause 326 -603
[assign] #3315 bin 326
[decide-and-or] #3322 #1780
[push] 2
[assign] (not #1751) decision axiom
[eq-expl] #818 root
[new-match] 0x61d50e673530 #29 #28 #818 ; #1751
[mk-app] #3952 = #1751 #914
[mk-app] #3953 not #29
[mk-app] #3954 or #3953 #3952
[instance] 0x61d50e673530 ; 1
[assign] (not #3952) justification -1: 205 -328
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3952
[conflict] #3952
[pop] 1 3
[assign] #3952 axiom
[assign] #1751 clause 328 -604
[assign] #3339 bin 328
[decide-and-or] #3323 #1801
[push] 2
[assign] (not #1782) decision axiom
[eq-expl] #819 root
[new-match] 0x61d50e6735d8 #29 #28 #819 ; #1782
[mk-app] #3953 = #1782 #915
[mk-app] #3954 not #29
[mk-app] #3955 or #3954 #3953
[instance] 0x61d50e6735d8 ; 1
[assign] (not #3953) justification -1: 206 -330
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3953
[conflict] #3953
[pop] 1 3
[assign] #3953 axiom
[assign] #1782 clause 330 -605
[assign] #3356 bin 330
[decide-and-or] #1819 #1818
[push] 2
[assign] (not #1812) decision axiom
[eq-expl] #828 root
[new-match] 0x61d50e673680 #29 #28 #828 ; #1812
[mk-app] #3954 = #1812 #1811
[mk-app] #3955 not #29
[mk-app] #3956 or #3955 #3954
[instance] 0x61d50e673680 ; 1
[assign] (not #3954) justification -1: 333 -334
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3954
[conflict] #3954
[pop] 1 3
[assign] #3954 axiom
[assign] #1812 clause 334 -606
[assign] #1816 bin 334
[decide-and-or] #1829 #1828
[push] 2
[assign] (not #1822) decision axiom
[eq-expl] #830 root
[new-match] 0x61d50e673710 #29 #28 #830 ; #1822
[mk-app] #3955 = #1822 #1821
[mk-app] #3956 not #29
[mk-app] #3957 or #3956 #3955
[instance] 0x61d50e673710 ; 1
[assign] (not #3955) justification -1: 337 -338
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3955
[conflict] #3955
[pop] 1 3
[assign] #3955 axiom
[assign] #1822 clause 338 -607
[assign] #1826 bin 338
[decide-and-or] #1839 #1838
[push] 2
[assign] (not #1832) decision axiom
[eq-expl] #829 root
[new-match] 0x61d50e6737a0 #29 #28 #829 ; #1832
[mk-app] #3956 = #1832 #1831
[mk-app] #3957 not #29
[mk-app] #3958 or #3957 #3956
[instance] 0x61d50e6737a0 ; 1
[assign] (not #3956) justification -1: 341 -342
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3956
[conflict] #3956
[pop] 1 3
[assign] #3956 axiom
[assign] #1832 clause 342 -608
[assign] #1836 bin 342
[decide-and-or] #1854 #1853
[push] 2
[assign] (not #1842) decision axiom
[eq-expl] #826 root
[new-match] 0x61d50e673830 #29 #28 #826 ; #1842
[mk-app] #3957 = #1842 #1841
[mk-app] #3958 not #29
[mk-app] #3959 or #3958 #3957
[instance] 0x61d50e673830 ; 1
[assign] (not #3957) justification -1: 345 -346
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3957
[conflict] #3957
[pop] 1 3
[assign] #3957 axiom
[assign] #1842 clause 346 -609
[assign] #1852 bin 346
[decide-and-or] #3361 #1892
[push] 2
[assign] (not #1877) decision axiom
[eq-expl] #806 root
[new-match] 0x61d50e6738a8 #29 #28 #806 ; #1877
[mk-app] #3958 = #1877 #933
[mk-app] #3959 not #29
[mk-app] #3960 or #3959 #3958
[instance] 0x61d50e6738a8 ; 1
[assign] (not #3958) justification -1: 211 -351
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3958
[conflict] #3958
[pop] 1 3
[assign] #3958 axiom
[assign] #1877 clause 351 -610
[assign] #3357 bin 351
[decide-and-or] #3371 #1940
[push] 2
[assign] (not #1924) decision axiom
[eq-expl] #805 root
[new-match] 0x61d50e673908 #29 #28 #805 ; #1924
[mk-app] #3959 = #1924 #1923
[mk-app] #3960 not #29
[mk-app] #3961 or #3960 #3959
[instance] 0x61d50e673908 ; 1
[assign] (not #3959) justification -1: 356 -357
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3959
[conflict] #3959
[pop] 1 3
[assign] #3959 axiom
[assign] #1924 clause 357 -611
[assign] #3374 bin 357
[decide-and-or] #3375 #1965
[push] 2
[assign] (not #1948) decision axiom
[eq-expl] #807 root
[new-match] 0x61d50e673980 #29 #28 #807 ; #1948
[mk-app] #3960 = #1948 #934
[mk-app] #3961 not #29
[mk-app] #3962 or #3961 #3960
[instance] 0x61d50e673980 ; 1
[assign] (not #3960) justification -1: 212 -360
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3960
[conflict] #3960
[pop] 1 3
[assign] #3960 axiom
[assign] #1948 clause 360 -612
[assign] #3379 bin 360
[decide-and-or] #3378 #1986
[push] 2
[assign] (not #1976) decision axiom
[eq-expl] #808 root
[new-match] 0x61d50e673a28 #29 #28 #808 ; #1976
[mk-app] #3961 = #1976 #935
[mk-app] #3962 not #29
[mk-app] #3963 or #3962 #3961
[instance] 0x61d50e673a28 ; 1
[assign] (not #3961) justification -1: 213 -362
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3961
[conflict] #3961
[pop] 1 3
[assign] #3961 axiom
[assign] #1976 clause 362 -613
[assign] #3382 bin 362
[decide-and-or] #2006 #2005
[push] 2
[assign] (not #1993) decision axiom
[eq-expl] #809 root
[new-match] 0x61d50e673b18 #29 #28 #809 ; #1993
[mk-app] #3962 = #1993 #936
[mk-app] #3963 not #29
[mk-app] #3964 or #3963 #3962
[instance] 0x61d50e673b18 ; 1
[assign] (not #3962) justification -1: 214 -364
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3962
[conflict] #3962
[pop] 1 3
[assign] #3962 axiom
[assign] #1993 clause 364 -614
[assign] #2003 bin 364
[decide-and-or] #2020 #2019
[push] 2
[assign] (not #2007) decision axiom
[eq-expl] #821 root
[new-match] 0x61d50e673bc0 #29 #28 #821 ; #2007
[mk-app] #3963 = #2007 #926
[mk-app] #3964 not #29
[mk-app] #3965 or #3964 #3963
[instance] 0x61d50e673bc0 ; 1
[assign] (not #3963) justification -1: 209 -366
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3963
[conflict] #3963
[pop] 1 3
[assign] #3963 axiom
[assign] #2007 clause 366 -615
[assign] #2018 bin 366
[new-match] 0x61d50e673c08 #2018 #1540 #1396 #1167 #125 ; #1379
[new-match] 0x61d50e673c48 #2018 #1540 #1290 #1167 #125 ; #1273
[mk-app] #3964 = #1379 #341
[mk-app] #3965 has_type #3934 #1188
[mk-app] #3966 not #3965
[mk-app] #3967 vstd!seq.Seq.len.? #125 #1167 #3934
[mk-app] #3968 = #3967 #341
[mk-app] #3969 not #3968
[mk-app] #3970 vstd!seq.Seq.empty.? #125 #1167
[mk-app] #3971 ext_eq #2 #1188 #3934 #3970
[mk-app] #3972 or #3966 #3930 #3969 #3971
[inst-discovered] theory-solving 0 basic# ; #3972
[mk-app] #3973 = #3972 #3972
[instance] 0 #3973
[attach-enode] #3973 0
[end-of-instance]
[mk-app] #3973 not #2018
[mk-app] #3974 or #3973 #3966 #3930 #3969 #3971
[instance] 0x61d50e673c08 ; 1
[attach-enode] #3965 1
[attach-enode] #3967 1
[attach-enode] #3968 1
[attach-enode] #3970 1
[attach-enode] #3971 1
[end-of-instance]
[mk-app] #3975 = #1273 #341
[mk-app] #3976 not #3975
[mk-app] #3977 ext_eq #2 #1188 #1290 #3970
[mk-app] #3978 or #3845 #3930 #3976 #3977
[inst-discovered] theory-solving 0 basic# ; #3978
[mk-app] #3979 = #3978 #3978
[instance] 0 #3979
[attach-enode] #3979 0
[end-of-instance]
[mk-app] #3979 or #3973 #3845 #3930 #3976 #3977
[instance] 0x61d50e673c48 ; 1
[attach-enode] #3975 1
[mk-app] #3980 <= #1273 #341
[attach-enode] #3977 1
[end-of-instance]
[assign] #3965 justification -1: 560 592
[decide-and-or] #2031 #2030
[push] 2
[assign] (not #2022) decision axiom
[eq-expl] #803 root
[new-match] 0x61d50e674218 #29 #28 #803 ; #2022
[mk-app] #3981 = #2022 #2021
[mk-app] #3982 not #29
[mk-app] #3983 or #3982 #3981
[instance] 0x61d50e674218 ; 1
[assign] (not #3981) justification -1: 368 -369
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3981
[conflict] #3981
[pop] 1 3
[assign] #3981 axiom
[assign] #2022 clause 369 -622
[assign] #2029 bin 369
[decide-and-or] #2052 #2051
[push] 2
[assign] (not #2039) decision axiom
[eq-expl] #820 root
[new-match] 0x61d50e674278 #29 #28 #820 ; #2039
[mk-app] #3982 = #2039 #2032
[mk-app] #3983 not #29
[mk-app] #3984 or #3983 #3982
[instance] 0x61d50e674278 ; 1
[assign] (not #3982) justification -1: 371 -373
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3982
[conflict] #3982
[pop] 1 3
[assign] #3982 axiom
[assign] #2039 clause 373 -623
[assign] #2049 bin 373
[decide-and-or] #2073 #2072
[push] 2
[assign] (not #2064) decision axiom
[eq-expl] #802 root
[new-match] 0x61d50e6742d8 #29 #28 #802 ; #2064
[mk-app] #3983 = #2064 #2063
[mk-app] #3984 not #29
[mk-app] #3985 or #3984 #3983
[instance] 0x61d50e6742d8 ; 1
[assign] (not #3983) justification -1: 376 -377
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3983
[conflict] #3983
[pop] 1 3
[assign] #3983 axiom
[assign] #2064 clause 377 -624
[assign] #2071 bin 377
[decide-and-or] #2094 #2093
[push] 2
[assign] (not #2084) decision axiom
[eq-expl] #801 root
[new-match] 0x61d50e674338 #29 #28 #801 ; #2084
[mk-app] #3984 = #2084 #2083
[mk-app] #3985 not #29
[mk-app] #3986 or #3985 #3984
[instance] 0x61d50e674338 ; 1
[assign] (not #3984) justification -1: 380 -381
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3984
[conflict] #3984
[pop] 1 3
[assign] #3984 axiom
[assign] #2084 clause 381 -625
[assign] #2092 bin 381
[decide-and-or] #2119 #2118
[push] 2
[assign] (not #2112) decision axiom
[eq-expl] #804 root
[new-match] 0x61d50e6743b0 #29 #28 #804 ; #2112
[mk-app] #3985 = #2112 #2111
[mk-app] #3986 not #29
[mk-app] #3987 or #3986 #3985
[instance] 0x61d50e6743b0 ; 1
[assign] (not #3985) justification -1: 385 -386
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3985
[conflict] #3985
[pop] 1 3
[assign] #3985 axiom
[assign] #2112 clause 386 -626
[assign] #2116 bin 386
[decide-and-or] #2254 #2253
[push] 2
[assign] (not #2241) decision axiom
[eq-expl] #833 root
[new-match] 0x61d50e674410 #29 #28 #833 ; #2241
[mk-app] #3986 = #2241 #2240
[mk-app] #3987 not #29
[mk-app] #3988 or #3987 #3986
[instance] 0x61d50e674410 ; 1
[assign] (not #3986) justification -1: 399 -400
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3986
[conflict] #3986
[pop] 1 3
[assign] #3986 axiom
[assign] #2241 clause 400 -627
[assign] #2252 bin 400
[decide-and-or] #2271 #2270
[push] 2
[assign] (not #2256) decision axiom
[eq-expl] #834 root
[new-match] 0x61d50e674488 #29 #28 #834 ; #2256
[mk-app] #3987 = #2256 #2255
[mk-app] #3988 not #29
[mk-app] #3989 or #3988 #3987
[instance] 0x61d50e674488 ; 1
[assign] (not #3987) justification -1: 402 -403
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3987
[conflict] #3987
[pop] 1 3
[assign] #3987 axiom
[assign] #2256 clause 403 -628
[assign] #2269 bin 403
[decide-and-or] #2284 #2283
[push] 2
[assign] (not #2273) decision axiom
[eq-expl] #832 root
[new-match] 0x61d50e674500 #29 #28 #832 ; #2273
[mk-app] #3988 = #2273 #2272
[mk-app] #3989 not #29
[mk-app] #3990 or #3989 #3988
[instance] 0x61d50e674500 ; 1
[assign] (not #3988) justification -1: 405 -406
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3988
[conflict] #3988
[pop] 1 3
[assign] #3988 axiom
[assign] #2273 clause 406 -629
[assign] #2282 bin 406
[decide-and-or] #3426 #2305
[push] 2
[assign] (not #2292) decision axiom
[eq-expl] #825 root
[new-match] 0x61d50e674578 #29 #28 #825 ; #2292
[mk-app] #3989 = #2292 #2291
[mk-app] #3990 not #29
[mk-app] #3991 or #3990 #3989
[instance] 0x61d50e674578 ; 1
[assign] (not #3989) justification -1: 409 -410
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3989
[conflict] #3989
[pop] 1 3
[assign] #3989 axiom
[assign] #2292 clause 410 -630
[assign] #3429 bin 410
[decide-and-or] #3435 #2362
[push] 2
[assign] (not #2347) decision axiom
[eq-expl] #835 root
[new-match] 0x61d50e674608 #29 #28 #835 ; #2347
[mk-app] #3990 = #2347 #2313
[mk-app] #3991 not #29
[mk-app] #3992 or #3991 #3990
[instance] 0x61d50e674608 ; 1
[assign] (not #3990) justification -1: 413 -416
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3990
[conflict] #3990
[pop] 1 3
[assign] #3990 axiom
[assign] #2347 clause 416 -631
[assign] #3440 bin 416
[decide-and-or] #3466 #2398
[push] 2
[assign] (not #2365) decision axiom
[eq-expl] #837 root
[new-match] 0x61d50e674668 #29 #28 #837 ; #2365
[mk-app] #3991 = #2365 #2364
[mk-app] #3992 not #29
[mk-app] #3993 or #3992 #3991
[instance] 0x61d50e674668 ; 1
[assign] (not #3991) justification -1: 418 -419
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3991
[conflict] #3991
[pop] 1 3
[assign] #3991 axiom
[assign] #2365 clause 419 -632
[assign] #3465 bin 419
[new-match] 0x61d50e6746b0 #3465 #2381 #1396 ; #1287
[mk-app] #3992 hi$!skolem_user_lib__Chap28__MCSSSpec__MCSSSpec__sums_fit_i32_17!9 #3934
[mk-app] #3993 lo$!skolem_user_lib__Chap28__MCSSSpec__MCSSSpec__sums_fit_i32_17!10 #3934
[mk-app] #3994 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #3934 #3993 #3992
[mk-app] #3995 <= #3994 #2373
[mk-app] #3996 not #3995
[mk-app] #3997 >= #3994 #317
[mk-app] #3998 not #3997
[mk-app] #3999 or #3998 #3996
[mk-app] #4000 not #3999
[mk-app] #4001 * #370 #3967
[mk-app] #4002 %I #3992
[mk-app] #4003 + #4002 #4001
[mk-app] #4004 <= #4003 #341
[mk-app] #4005 not #4004
[mk-app] #4006 * #370 #4002
[mk-app] #4007 %I #3993
[mk-app] #4008 + #4007 #4006
[mk-app] #4009 <= #4008 #341
[mk-app] #4010 not #4009
[mk-app] #4011 >= #4007 #341
[mk-app] #4012 not #4011
[mk-app] #4013 has_type #3992 #189
[mk-app] #4014 not #4013
[mk-app] #4015 has_type #3993 #189
[mk-app] #4016 not #4015
[mk-app] #4017 or #4016 #4014 #4012 #4010 #4005 #4000
[mk-app] #4018 not #4017
[mk-app] #4019 lib!Chap28.MCSSSpec.MCSSSpec.sums_fit_i32.? #3934
[mk-app] #4020 or #4019 #4018
[mk-app] #4021 not #4020
[mk-app] #4022 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #3934 #33 #34
[mk-app] #4023 pattern #4022
[mk-app] #4024 <= #4022 #2373
[mk-app] #4025 not #4024
[mk-app] #4026 >= #4022 #317
[mk-app] #4027 not #4026
[mk-app] #4028 or #4027 #4025
[mk-app] #4029 not #4028
[mk-app] #4030 + #191 #4001
[mk-app] #4031 <= #4030 #341
[mk-app] #4032 not #4031
[mk-app] #4033 or #3433 #197 #2777 #3442 #4032 #4029
[mk-quant] #4034 user_lib__Chap28__MCSSSpec__MCSSSpec__sums_fit_i32_17 2 #4023 #4033
[attach-var-names] #4034 (|hi$| ; |Poly|) (|lo$| ; |Poly|)
[mk-app] #4035 not #4019
[mk-app] #4036 or #4035 #4034
[mk-app] #4037 not #4036
[mk-app] #4038 or #4037 #4021
[mk-app] #4039 not #4038
[inst-discovered] theory-solving 0 basic# ; #4028
[mk-app] #4040 = #4028 #4028
[instance] 0 #4040
[attach-enode] #4040 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4033
[mk-app] #4040 = #4033 #4033
[instance] 0 #4040
[attach-enode] #4040 0
[end-of-instance]
[mk-app] #4040 + #4006 #4007
[inst-discovered] theory-solving 0 arith# ; #4008
[mk-app] #4041 = #4008 #4040
[instance] 0 #4041
[attach-enode] #4041 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4041 * #370 #4007
[mk-app] #4042 + #4002 #4041
[mk-app] #4043 >= #4042 #341
[mk-app] #4044 <= #4040 #341
[inst-discovered] theory-solving 0 arith# ; #4044
[mk-app] #4045 = #4044 #4043
[instance] 0 #4045
[attach-enode] #4045 0
[end-of-instance]
[mk-app] #4040 not #4043
[mk-app] #4044 + #4001 #4002
[inst-discovered] theory-solving 0 arith# ; #4003
[mk-app] #4045 = #4003 #4044
[instance] 0 #4045
[attach-enode] #4045 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4045 + #3967 #4006
[mk-app] #4046 >= #4045 #341
[mk-app] #4047 <= #4044 #341
[inst-discovered] theory-solving 0 arith# ; #4047
[mk-app] #4048 = #4047 #4046
[instance] 0 #4048
[attach-enode] #4048 0
[end-of-instance]
[mk-app] #4044 not #4046
[inst-discovered] theory-solving 0 basic# ; #3999
[mk-app] #4047 = #3999 #3999
[instance] 0 #4047
[attach-enode] #4047 0
[end-of-instance]
[mk-app] #4047 or #4016 #4014 #4012 #4040 #4044 #4000
[inst-discovered] theory-solving 0 basic# ; #4047
[mk-app] #4048 = #4047 #4047
[instance] 0 #4048
[attach-enode] #4048 0
[end-of-instance]
[mk-app] #4048 not #4047
[mk-app] #4049 or #4019 #4048
[mk-app] #4050 not #4049
[mk-app] #4051 or #4037 #4050
[mk-app] #4052 not #4051
[mk-app] #4053 not #3465
[mk-app] #4054 or #4053 #4052
[instance] 0x61d50e6746b0 ; 1
[attach-enode] #4019 1
[attach-enode] #3993 1
[attach-enode] #4015 1
[attach-enode] #3992 1
[attach-enode] #4013 1
[attach-enode] #4007 1
[attach-enode] #4002 1
[attach-enode] #4041 1
[attach-enode] #4042 1
[attach-enode] #4006 1
[attach-enode] #4045 1
[attach-enode] #3994 1
[assign] (not #4051) justification -1: 420
[end-of-instance]
[assign] #4036 clause 635 646
[assign] #4049 clause 645 646
[assign] #4019 justification -1: 479 592
[mk-app] #4055 = #1379 #3967
[attach-meaning] #370 arith (- 1)
[mk-app] #4056 + #1379 #4001
[mk-app] #4057 <= #4056 #341
[mk-app] #4058 >= #4056 #341
[assign] #4055 justification -1: 592
[attach-enode] #4055 0
[attach-enode] #4001 0
[attach-enode] #4056 0
[assign] #4057 justification -1: 647
[assign] #4058 justification -1: 647
[assign] #4034 clause 634 -633 -635
[decide-and-or] #3494 #2440
[push] 2
[assign] (not #2401) decision axiom
[eq-expl] #836 root
[new-match] 0x61d50e675288 #29 #28 #836 ; #2401
[mk-app] #4059 = #2401 #2400
[mk-app] #4060 not #29
[mk-app] #4061 or #4060 #4059
[instance] 0x61d50e675288 ; 1
[assign] (not #4059) justification -1: 421 -422
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #4059
[conflict] #4059
[pop] 1 3
[assign] #4059 axiom
[assign] #2401 clause 422 -650
[assign] #3493 bin 422
[decide-and-or] #2506 #2505
[push] 2
[assign] (not #2494) decision axiom
[eq-expl] #822 root
[new-match] 0x61d50e6752e8 #29 #28 #822 ; #2494
[mk-app] #4060 = #2494 #2487
[mk-app] #4061 not #29
[mk-app] #4062 or #4061 #4060
[instance] 0x61d50e6752e8 ; 1
[assign] (not #4060) justification -1: 426 -427
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #4060
[conflict] #4060
[pop] 2 3
[assign] #3944 axiom
[assign] #3945 axiom
[assign] #3946 axiom
[assign] #3947 axiom
[assign] #3948 axiom
[assign] #3949 axiom
[assign] #3950 axiom
[assign] #3951 axiom
[assign] #3952 axiom
[assign] #3953 axiom
[assign] #3954 axiom
[assign] #3955 axiom
[assign] #3956 axiom
[assign] #3957 axiom
[assign] #3958 axiom
[assign] #3959 axiom
[assign] #3960 axiom
[assign] #3961 axiom
[assign] #3962 axiom
[assign] #3963 axiom
[assign] #3981 axiom
[assign] #3982 axiom
[assign] #3983 axiom
[assign] #3984 axiom
[assign] #3985 axiom
[assign] #3986 axiom
[assign] #3987 axiom
[assign] #3988 axiom
[assign] #3989 axiom
[assign] #3990 axiom
[assign] #3991 axiom
[assign] #4059 axiom
[assign] #4060 axiom
[assign] #1581 clause 308 -593
[assign] #1598 clause 311 -594
[assign] #1617 clause 314 -595
[assign] #1640 clause 317 -596
[assign] #1658 clause 319 -597
[assign] #1695 clause 322 -598
[assign] #1709 clause 324 -599
[assign] #1731 clause 326 -600
[assign] #1751 clause 328 -601
[assign] #1782 clause 330 -602
[assign] #1812 clause 334 -603
[assign] #1822 clause 338 -604
[assign] #1832 clause 342 -605
[assign] #1842 clause 346 -606
[assign] #1877 clause 351 -607
[assign] #1924 clause 357 -608
[assign] #1948 clause 360 -609
[assign] #1976 clause 362 -610
[assign] #1993 clause 364 -611
[assign] #2007 clause 366 -612
[assign] #2022 clause 369 -613
[assign] #2039 clause 373 -614
[assign] #2064 clause 377 -615
[assign] #2084 clause 381 -616
[assign] #2112 clause 386 -617
[assign] #2241 clause 400 -618
[assign] #2256 clause 403 -619
[assign] #2273 clause 406 -620
[assign] #2292 clause 410 -621
[assign] #2347 clause 416 -622
[assign] #2365 clause 419 -623
[assign] #2401 clause 422 -624
[assign] #2494 clause 427 -625
[assign] #1589 bin 308
[assign] #3292 bin 311
[assign] #1625 bin 314
[assign] #3301 bin 317
[assign] #3306 bin 319
[assign] #3312 bin 322
[assign] #3320 bin 324
[assign] #3315 bin 326
[assign] #3339 bin 328
[assign] #3356 bin 330
[assign] #1816 bin 334
[assign] #1826 bin 338
[assign] #1836 bin 342
[assign] #1852 bin 346
[assign] #3357 bin 351
[assign] #3374 bin 357
[assign] #3379 bin 360
[assign] #3382 bin 362
[assign] #2003 bin 364
[assign] #2018 bin 366
[assign] #2029 bin 369
[assign] #2049 bin 373
[assign] #2071 bin 377
[assign] #2092 bin 381
[assign] #2116 bin 386
[assign] #2252 bin 400
[assign] #2269 bin 403
[assign] #2282 bin 406
[assign] #3429 bin 410
[assign] #3440 bin 416
[assign] #3465 bin 419
[assign] #3493 bin 422
[assign] #2504 bin 427
[new-match] 0x61d50e673f80 #2018 #1540 #1396 #1167 #125 ; #1379
[new-match] 0x61d50e673fc0 #2018 #1540 #1290 #1167 #125 ; #1273
[new-match] 0x61d50e674000 #3465 #2381 #1396 ; #1287
[inst-discovered] theory-solving 0 basic# ; #3972
[mk-app] #3941 = #3972 #3972
[instance] 0 #3941
[attach-enode] #3941 0
[end-of-instance]
[mk-app] #3941 not #2018
[mk-app] #3942 or #3941 #3966 #3930 #3969 #3971
[instance] 0x61d50e673f80 ; 1
[attach-enode] #3965 1
[attach-enode] #3967 1
[attach-enode] #3968 1
[attach-enode] #3970 1
[attach-enode] #3971 1
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #3978
[mk-app] #3943 = #3978 #3978
[instance] 0 #3943
[attach-enode] #3943 0
[end-of-instance]
[mk-app] #3943 or #3941 #3845 #3930 #3976 #3977
[instance] 0x61d50e673fc0 ; 1
[attach-enode] #3975 1
[mk-app] #3980 <= #1273 #341
[attach-enode] #3977 1
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4028
[mk-app] #4041 = #4028 #4028
[instance] 0 #4041
[attach-enode] #4041 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4033
[mk-app] #4041 = #4033 #4033
[instance] 0 #4041
[attach-enode] #4041 0
[end-of-instance]
[mk-app] #4041 + #4006 #4007
[inst-discovered] theory-solving 0 arith# ; #4008
[mk-app] #4042 = #4008 #4041
[instance] 0 #4042
[attach-enode] #4042 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4042 * #370 #4007
[mk-app] #4043 + #4002 #4042
[mk-app] #4045 >= #4043 #341
[mk-app] #4046 <= #4041 #341
[inst-discovered] theory-solving 0 arith# ; #4046
[mk-app] #4040 = #4046 #4045
[instance] 0 #4040
[attach-enode] #4040 0
[end-of-instance]
[mk-app] #4041 not #4045
[mk-app] #4046 + #4001 #4002
[inst-discovered] theory-solving 0 arith# ; #4003
[mk-app] #4040 = #4003 #4046
[instance] 0 #4040
[attach-enode] #4040 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4040 + #3967 #4006
[mk-app] #4044 >= #4040 #341
[mk-app] #4047 <= #4046 #341
[inst-discovered] theory-solving 0 arith# ; #4047
[mk-app] #4048 = #4047 #4044
[instance] 0 #4048
[attach-enode] #4048 0
[end-of-instance]
[mk-app] #4046 not #4044
[inst-discovered] theory-solving 0 basic# ; #3999
[mk-app] #4047 = #3999 #3999
[instance] 0 #4047
[attach-enode] #4047 0
[end-of-instance]
[mk-app] #4047 or #4016 #4014 #4012 #4041 #4046 #4000
[inst-discovered] theory-solving 0 basic# ; #4047
[mk-app] #4048 = #4047 #4047
[instance] 0 #4048
[attach-enode] #4048 0
[end-of-instance]
[mk-app] #4048 not #4047
[mk-app] #4049 or #4019 #4048
[mk-app] #4050 not #4049
[mk-app] #4051 or #4037 #4050
[mk-app] #4055 not #4051
[mk-app] #4056 not #3465
[mk-app] #4057 or #4056 #4055
[instance] 0x61d50e674000 ; 1
[attach-enode] #4019 1
[attach-enode] #3993 1
[attach-enode] #4015 1
[attach-enode] #3992 1
[attach-enode] #4013 1
[attach-enode] #4007 1
[attach-enode] #4002 1
[attach-enode] #4042 1
[attach-enode] #4043 1
[attach-enode] #4006 1
[attach-enode] #4040 1
[attach-enode] #3994 1
[assign] (not #4051) justification -1: 420
[end-of-instance]
[assign] #4036 clause 634 645
[assign] #4049 clause 644 645
[assign] #3965 justification -1: 560 592
[assign] #4019 justification -1: 479 592
[mk-app] #4058 = #1379 #3967
[attach-meaning] #370 arith (- 1)
[mk-app] #4061 + #1379 #4001
[mk-app] #4062 <= #4061 #341
[mk-app] #4052 >= #4061 #341
[assign] #4058 justification -1: 592
[attach-enode] #4058 0
[attach-enode] #4001 0
[attach-enode] #4061 0
[assign] #4062 justification -1: 646
[assign] #4052 justification -1: 646
[assign] #4034 clause 633 -632 -634
[decide-and-or] #279 #276
[push] 1
[assign] #276 decision axiom
[mk-app] #4053 = #2449 #289
[mk-app] #4054 <= #2449 #289
[mk-app] #3973 >= #2449 #289
[assign] #4053 justification -1: 43 39
[attach-enode] #4053 0
[assign] #4054 justification -1: 649
[assign] #3973 justification -1: 649
[decide-and-or] #2519 #2518
[push] 2
[assign] (not #2508) decision axiom
[eq-expl] #823 root
[new-match] 0x61d50e691e48 #29 #28 #823 ; #2508
[mk-app] #3979 = #2508 #2507
[mk-app] #3974 not #29
[mk-app] #4063 or #3974 #3979
[instance] 0x61d50e691e48 ; 1
[assign] (not #3979) justification -1: 429 -430
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3979
[conflict] #3979
[pop] 1 3
[assign] #3979 axiom
[assign] #2508 clause 430 -652
[assign] #2517 bin 430
[decide-and-or] #3509 #2531
[push] 2
[assign] (not #2521) decision axiom
[eq-expl] #824 root
[new-match] 0x61d50e691eb8 #29 #28 #824 ; #2521
[mk-app] #3974 = #2521 #2520
[mk-app] #4063 not #29
[mk-app] #4064 or #4063 #3974
[instance] 0x61d50e691eb8 ; 1
[assign] (not #3974) justification -1: 432 -433
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #3974
[conflict] #3974
[pop] 1 3
[assign] #3974 axiom
[assign] #2521 clause 433 -653
[assign] #3512 bin 433
[decide-and-or] #2541 #2540
[push] 2
[assign] (not #2534) decision axiom
[eq-expl] #827 root
[new-match] 0x61d50e691f48 #29 #28 #827 ; #2534
[mk-app] #4063 = #2534 #2533
[mk-app] #4064 not #29
[mk-app] #4065 or #4064 #4063
[instance] 0x61d50e691f48 ; 1
[assign] (not #4063) justification -1: 435 -436
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #4063
[conflict] #4063
[pop] 1 3
[assign] #4063 axiom
[assign] #2534 clause 436 -654
[assign] #2538 bin 436
[decide-and-or] #3510 #2563
[push] 2
[assign] (not #2549) decision axiom
[eq-expl] #831 root
[new-match] 0x61d50e691fd8 #29 #28 #831 ; #2549
[mk-app] #4064 = #2549 #2542
[mk-app] #4065 not #29
[mk-app] #4066 or #4065 #4064
[instance] 0x61d50e691fd8 ; 1
[assign] (not #4064) justification -1: 438 -440
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #4064
[conflict] #4064
[pop] 1 3
[assign] #4064 axiom
[assign] #2549 clause 440 -655
[assign] #3515 bin 440
[decide-and-or] #2581 #2580
[push] 2
[assign] (not #2572) decision axiom
[eq-expl] #838 root
[new-match] 0x61d50e692050 #29 #28 #838 ; #2572
[mk-app] #4065 = #2572 #2571
[mk-app] #4066 not #29
[mk-app] #4067 or #4066 #4065
[instance] 0x61d50e692050 ; 1
[assign] (not #4065) justification -1: 443 -444
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #4065
[conflict] #4065
[pop] 1 3
[assign] #4065 axiom
[assign] #2572 clause 444 -656
[assign] #2578 bin 444
[eq-expl] #1276 root
[new-match] 0x61d50e692098 #2578 #2577 #1276 #1396 ; #2899
[mk-app] #4066 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #3934 #1276
[mk-app] #4067 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #3934 #2574 #1276
[mk-app] #4068 = #4066 #4067
[mk-app] #4069 not #2578
[mk-app] #4070 or #4069 #4068
[instance] 0x61d50e692098 ; 1
[attach-enode] #4066 1
[attach-enode] #4067 1
[attach-enode] #4068 1
[assign] #4068 justification -1: 445
[end-of-instance]
[eq-expl] #2574 root
[new-match] 0x61d50e692310 #3440 #2356 #1276 #2574 #1396 ; #782
[new-match] 0x61d50e692350 #4034 #4023 #1276 #2574 ; #782 (#1396 #3934)
[eq-expl] #341 root
[new-match] 0x61d50e692388 #174 #173 #341 ; #2574
[mk-app] #4071 has_type #2574 #189
[mk-app] #4072 not #4071
[mk-app] #4073 has_type #1276 #189
[mk-app] #4074 not #4073
[mk-app] #4075 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #2574 #1276 #2352
[mk-app] #4076 = #4067 #4075
[mk-app] #4077 or #3966 #4072 #4074 #4076
[inst-discovered] theory-solving 0 basic# ; #4077
[mk-app] #4078 = #4077 #4077
[instance] 0 #4078
[attach-enode] #4078 0
[end-of-instance]
[mk-app] #4078 not #3440
[mk-app] #4079 or #4078 #3966 #4072 #4074 #4076
[instance] 0x61d50e692310 ; 1
[attach-enode] #4071 1
[attach-enode] #4073 1
[attach-enode] #2351 1
[attach-enode] #2352 1
[attach-enode] #4075 1
[attach-enode] #4076 1
[end-of-instance]
[mk-app] #4080 %I #2574
[mk-app] #4081 >= #4080 #341
[mk-app] #4082 not #4081
[mk-app] #4083 * #370 #4080
[mk-app] #4084 + #3869 #4083
[mk-app] #4085 >= #4084 #341
[mk-app] #4086 not #4085
[mk-app] #4087 + #3869 #4001
[mk-app] #4088 <= #4087 #341
[mk-app] #4089 not #4088
[mk-app] #4090 >= #4067 #317
[mk-app] #4091 not #4090
[mk-app] #4092 <= #4067 #2373
[mk-app] #4093 not #4092
[mk-app] #4094 or #4091 #4093
[mk-app] #4095 not #4094
[mk-app] #4096 or #4072 #4074 #4082 #4086 #4089 #4095
[inst-discovered] theory-solving 0 basic# ; #4094
[mk-app] #4097 = #4094 #4094
[instance] 0 #4097
[attach-enode] #4097 0
[end-of-instance]
[mk-app] #4097 not #4034
[mk-app] #4098 or #4097 #4072 #4074 #4082 #4086 #4089 #4095
[instance] 0x61d50e692350 ; 2
[attach-enode] #4080 2
[attach-enode] #4083 2
[attach-enode] #4084 2
[attach-enode] #4087 2
[end-of-instance]
[mk-app] #4099 = #341 #4080
[attach-meaning] #370 arith (- 1)
[mk-app] #4100 = #4080 #341
[inst-discovered] theory-solving 0 arith# ; #4099
[mk-app] #4101 = #4099 #4100
[instance] 0 #4101
[attach-enode] #4101 0
[end-of-instance]
[mk-app] #4101 or #3871 #4100
[instance] 0x61d50e692388 ; 1
[attach-enode] #4100 1
[mk-app] #4102 <= #4080 #341
[assign] #4100 justification -1: 26
[end-of-instance]
[assign] #4102 clause 668 -667
[assign] #4081 clause 661 -667
[attach-meaning] #370 arith (- 1)
[mk-app] #4103 * #370 #3869
[mk-app] #4104 + #1454 #4103
[mk-app] #4105 <= #4104 #341
[mk-app] #4106 >= #4104 #341
[attach-enode] #4103 0
[attach-enode] #4104 0
[assign] #4105 justification -1: 568
[assign] #4106 justification -1: 568
[mk-app] #4107 = #2899 #4067
[attach-meaning] #370 arith (- 1)
[mk-app] #4108 * #370 #4067
[mk-app] #4109 + #2899 #4108
[mk-app] #4110 <= #4109 #341
[mk-app] #4111 >= #4109 #341
[assign] #4107 justification -1: 657 592
[attach-enode] #4107 0
[attach-enode] #4108 0
[attach-enode] #4109 0
[assign] #4110 justification -1: 671
[assign] #4111 justification -1: 671
[decide-and-or] #3516 #2630
[push] 2
[assign] (not #2606) decision axiom
[eq-expl] #839 root
[new-match] 0x61d50e693180 #29 #28 #839 ; #2606
[mk-app] #4112 = #2606 #2582
[mk-app] #4113 not #29
[mk-app] #4114 or #4113 #4112
[instance] 0x61d50e693180 ; 1
[assign] (not #4112) justification -1: 446 -449
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #4112
[conflict] #4112
[pop] 1 3
[assign] #4112 axiom
[assign] #2606 clause 449 -674
[assign] #3523 bin 449
[decide-and-or] #3767 #778
[push] 2
[assign] #778 decision axiom
[eq-expl] #2892 lit #1024 ; #1014
[eq-expl] #1025 root
[new-match] 0x61d50e6931e0 #2724 #2723 #1025 #341 #2892 ; #778
[mk-app] #4113 ens%lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_snoc. #1014 #341 #1025
[mk-app] #4114 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #3934 #2574 #3638
[mk-app] #4115 Sub #1025 #296
[mk-app] #4116 I #4115
[mk-app] #4117 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #3934 #2574 #4116
[mk-app] #4118 vstd!seq.Seq.index.? #125 #1167 #3934 #4116
[mk-app] #4119 %I #4118
[mk-app] #4120 Add #4117 #4119
[mk-app] #4121 = #4114 #4120
[mk-app] #4122 = #4113 #4121
[mk-app] #4123 not #2724
[mk-app] #4124 or #4123 #4122
[instance] 0x61d50e6931e0 ; 1
[attach-enode] #4113 1
[attach-enode] #4114 1
[attach-enode] #4115 1
[attach-enode] #4116 1
[attach-enode] #4117 1
[attach-enode] #4118 1
[attach-enode] #4119 1
[attach-enode] #4120 1
[attach-enode] #4121 1
[assign] #4122 justification -1: 460
[end-of-instance]
[assign] #4113 justification -1: 487 485
[eq-expl] #4117 root
[eq-expl] #4119 root
[new-match] 0x61d50e693768 #552 #550 #4119 #4117 ; #4120
[eq-expl] #4116 root
[new-match] 0x61d50e6937a0 #3440 #2356 #4116 #2574 #3934 ; #4117
[new-match] 0x61d50e6937e0 #4034 #4023 #4116 #2574 ; #4117 (#3934 #3934)
[eq-expl] #3638 root
[new-match] 0x61d50e693818 #3440 #2356 #3638 #2574 #1396 ; #3639
[new-match] 0x61d50e693858 #4034 #4023 #3638 #2574 ; #3639 (#1396 #3934)
[new-match] 0x61d50e693890 #174 #173 #1025 ; #3638
[eq-expl] #4115 root
[new-match] 0x61d50e6938c0 #174 #173 #4115 ; #4116
[new-match] 0x61d50e6938f0 #3286 #1569 #4116 #3934 #1167 #125 ; #4118
[new-match] 0x61d50e693938 #567 #559 #296 #1025 ; #4115
[mk-app] #4125 * #370 #4120
[mk-app] #4126 + #4119 #4117 #4125
[mk-app] #4127 = #4126 #341
[mk-app] #4128 + #4117 #4119 #4125
[inst-discovered] theory-solving 0 arith# ; #4126
[mk-app] #4129 = #4126 #4128
[instance] 0 #4129
[attach-enode] #4129 0
[end-of-instance]
[mk-app] #4129 = #4128 #341
[mk-app] #4130 or #3887 #4129
[instance] 0x61d50e693768 ; 2
[attach-enode] #4125 2
[attach-enode] #4128 2
[attach-enode] #4129 2
[mk-app] #4131 <= #4128 #341
[mk-app] #4132 >= #4128 #341
[assign] #4129 justification -1: 77
[end-of-instance]
[mk-app] #4133 has_type #4116 #189
[mk-app] #4134 not #4133
[mk-app] #4135 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #2574 #4116 #2352
[mk-app] #4136 = #4117 #4135
[mk-app] #4137 or #3966 #4072 #4134 #4136
[inst-discovered] theory-solving 0 basic# ; #4137
[mk-app] #4138 = #4137 #4137
[instance] 0 #4138
[attach-enode] #4138 0
[end-of-instance]
[mk-app] #4138 or #4078 #3966 #4072 #4134 #4136
[instance] 0x61d50e6937a0 ; 3
[attach-enode] #4133 3
[attach-enode] #4135 3
[attach-enode] #4136 3
[end-of-instance]
[mk-app] #4139 %I #4116
[mk-app] #4140 + #4139 #4083
[mk-app] #4141 >= #4140 #341
[mk-app] #4142 not #4141
[mk-app] #4143 + #4139 #4001
[mk-app] #4144 <= #4143 #341
[mk-app] #4145 not #4144
[mk-app] #4146 >= #4117 #317
[mk-app] #4147 not #4146
[mk-app] #4148 <= #4117 #2373
[mk-app] #4149 not #4148
[mk-app] #4150 or #4147 #4149
[mk-app] #4151 not #4150
[mk-app] #4152 or #4072 #4134 #4082 #4142 #4145 #4151
[mk-app] #4153 + #4083 #4139
[inst-discovered] theory-solving 0 arith# ; #4140
[mk-app] #4154 = #4140 #4153
[instance] 0 #4154
[attach-enode] #4154 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4154 * #370 #4139
[mk-app] #4155 + #4080 #4154
[mk-app] #4156 <= #4155 #341
[mk-app] #4157 >= #4153 #341
[inst-discovered] theory-solving 0 arith# ; #4157
[mk-app] #4158 = #4157 #4156
[instance] 0 #4158
[attach-enode] #4158 0
[end-of-instance]
[mk-app] #4157 not #4156
[mk-app] #4158 + #4001 #4139
[inst-discovered] theory-solving 0 arith# ; #4143
[mk-app] #4159 = #4143 #4158
[instance] 0 #4159
[attach-enode] #4159 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4159 + #3967 #4154
[mk-app] #4160 >= #4159 #341
[mk-app] #4161 <= #4158 #341
[inst-discovered] theory-solving 0 arith# ; #4161
[mk-app] #4162 = #4161 #4160
[instance] 0 #4162
[attach-enode] #4162 0
[end-of-instance]
[mk-app] #4158 not #4160
[inst-discovered] theory-solving 0 basic# ; #4150
[mk-app] #4161 = #4150 #4150
[instance] 0 #4161
[attach-enode] #4161 0
[end-of-instance]
[mk-app] #4161 or #4072 #4134 #4082 #4157 #4158 #4151
[inst-discovered] theory-solving 0 basic# ; #4161
[mk-app] #4162 = #4161 #4161
[instance] 0 #4162
[attach-enode] #4162 0
[end-of-instance]
[mk-app] #4162 or #4097 #4072 #4134 #4082 #4157 #4158 #4151
[instance] 0x61d50e6937e0 ; 2
[attach-enode] #4139 2
[attach-enode] #4154 2
[attach-enode] #4155 2
[attach-enode] #4159 2
[end-of-instance]
[mk-app] #4161 has_type #3638 #189
[mk-app] #4163 not #4161
[mk-app] #4164 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #2574 #3638 #2352
[mk-app] #4165 = #4114 #4164
[mk-app] #4166 or #3966 #4072 #4163 #4165
[inst-discovered] theory-solving 0 basic# ; #4166
[mk-app] #4167 = #4166 #4166
[instance] 0 #4167
[attach-enode] #4167 0
[end-of-instance]
[mk-app] #4167 or #4078 #3966 #4072 #4163 #4165
[instance] 0x61d50e693818 ; 1
[attach-enode] #4161 1
[attach-enode] #4164 1
[attach-enode] #4165 1
[end-of-instance]
[mk-app] #4168 %I #3638
[mk-app] #4169 + #4168 #4083
[mk-app] #4170 >= #4169 #341
[mk-app] #4171 not #4170
[mk-app] #4172 + #4168 #4001
[mk-app] #4173 <= #4172 #341
[mk-app] #4174 not #4173
[mk-app] #4175 >= #4114 #317
[mk-app] #4176 not #4175
[mk-app] #4177 <= #4114 #2373
[mk-app] #4178 not #4177
[mk-app] #4179 or #4176 #4178
[mk-app] #4180 not #4179
[mk-app] #4181 or #4072 #4163 #4082 #4171 #4174 #4180
[mk-app] #4182 + #4083 #4168
[inst-discovered] theory-solving 0 arith# ; #4169
[mk-app] #4183 = #4169 #4182
[instance] 0 #4183
[attach-enode] #4183 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4183 * #370 #4168
[mk-app] #4184 + #4080 #4183
[mk-app] #4185 <= #4184 #341
[mk-app] #4186 >= #4182 #341
[inst-discovered] theory-solving 0 arith# ; #4186
[mk-app] #4187 = #4186 #4185
[instance] 0 #4187
[attach-enode] #4187 0
[end-of-instance]
[mk-app] #4186 not #4185
[mk-app] #4187 + #4001 #4168
[inst-discovered] theory-solving 0 arith# ; #4172
[mk-app] #4188 = #4172 #4187
[instance] 0 #4188
[attach-enode] #4188 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4188 + #3967 #4183
[mk-app] #4189 >= #4188 #341
[mk-app] #4190 <= #4187 #341
[inst-discovered] theory-solving 0 arith# ; #4190
[mk-app] #4191 = #4190 #4189
[instance] 0 #4191
[attach-enode] #4191 0
[end-of-instance]
[mk-app] #4187 not #4189
[inst-discovered] theory-solving 0 basic# ; #4179
[mk-app] #4190 = #4179 #4179
[instance] 0 #4190
[attach-enode] #4190 0
[end-of-instance]
[mk-app] #4190 or #4072 #4163 #4082 #4186 #4187 #4180
[mk-app] #4191 or #4097 #4072 #4163 #4082 #4186 #4187 #4180
[instance] 0x61d50e693858 ; 2
[attach-enode] #4168 2
[attach-enode] #4183 2
[attach-enode] #4184 2
[attach-enode] #4188 2
[end-of-instance]
[mk-app] #4190 = #1025 #4168
[mk-app] #4192 or #3871 #4190
[instance] 0x61d50e693890 ; 1
[attach-enode] #4190 1
[attach-meaning] #370 arith (- 1)
[mk-app] #4193 + #1025 #4183
[mk-app] #4194 <= #4193 #341
[mk-app] #4195 >= #4193 #341
[attach-enode] #4193 1
[assign] #4190 justification -1: 26
[end-of-instance]
[mk-app] #4196 = #4115 #4139
[mk-app] #4197 or #3871 #4196
[instance] 0x61d50e6938c0 ; 2
[attach-enode] #4196 2
[assign] #4196 justification -1: 26
[end-of-instance]
[mk-app] #4198 has_type #4118 #1167
[mk-app] #4199 or #3966 #4134 #4198
[inst-discovered] theory-solving 0 basic# ; #4199
[mk-app] #4200 = #4199 #4199
[instance] 0 #4200
[attach-enode] #4200 0
[end-of-instance]
[mk-app] #4200 not #3286
[mk-app] #4201 or #4200 #3966 #4134 #4198
[instance] 0x61d50e6938f0 ; 3
[attach-enode] #4198 3
[end-of-instance]
[mk-app] #4202 * #370 #1025
[mk-app] #4203 + #296 #4202 #4115
[mk-app] #4204 = #4203 #341
[attach-meaning] #370 arith (- 1)
[mk-app] #4205 + #4202 #4115
[attach-meaning] #370 arith (- 1)
[mk-app] #4206 * #370 #4115
[mk-app] #4207 + #1025 #4206
[mk-app] #4205 = #4207 #296
[inst-discovered] theory-solving 0 arith# ; #4204
[mk-app] #4208 = #4204 #4205
[instance] 0 #4208
[attach-enode] #4208 0
[end-of-instance]
[mk-app] #4208 or #3877 #4205
[instance] 0x61d50e693938 ; 2
[attach-enode] #4206 2
[attach-enode] #4207 2
[attach-enode] #4205 2
[mk-app] #4209 <= #4207 #296
[mk-app] #4210 >= #4207 #296
[assign] #4205 justification -1: 78
[end-of-instance]
[assign] #4121 clause 676 -675 -677
[assign] #4131 clause 679 -678
[assign] #4132 clause 680 -678
[assign] #4194 clause 696 -695
[assign] #4195 clause 697 -695
[assign] #4209 clause 701 -700
[assign] #4210 clause 702 -700
[mk-app] #4211 = #1025 #1273
[attach-meaning] #370 arith (- 1)
[mk-app] #4212 * #370 #1273
[mk-app] #4213 + #1025 #4212
[mk-app] #4214 <= #4213 #341
[mk-app] #4215 >= #4213 #341
[assign] #4211 justification -1: 480
[attach-enode] #4211 0
[attach-enode] #4212 0
[attach-enode] #4213 0
[assign] #4214 justification -1: 703
[assign] #4215 justification -1: 703
[attach-meaning] #370 arith (- 1)
[mk-app] #4216 + #4115 #4154
[mk-app] #4217 <= #4216 #341
[mk-app] #4218 >= #4216 #341
[attach-enode] #4216 0
[assign] #4217 justification -1: 698
[assign] #4218 justification -1: 698
[attach-meaning] #370 arith (- 1)
[mk-app] #4219 + #4114 #4125
[mk-app] #4220 <= #4219 #341
[mk-app] #4221 >= #4219 #341
[attach-enode] #4219 0
[assign] #4220 justification -1: 676
[assign] #4221 justification -1: 676
[assign] #4185 clause 690 -696 -584 -668 -476 -574 -705
[assign] #4189 clause 691 -697 -577 -583 -647 484 -573 -704
[assign] #3630 justification -1: 676 592 698 707 706 702 701 705 704 584 583 574 573
[mk-app] #4222 = #4067 #4117
[attach-meaning] #370 arith (- 1)
[mk-app] #4223 * #370 #4117
[mk-app] #4224 + #4067 #4223
[mk-app] #4225 <= #4224 #341
[mk-app] #4226 >= #4224 #341
[assign] #4222 justification -1: 698 592 707 706 702 701 705 704 584 583 574 573
[attach-enode] #4222 0
[attach-enode] #4223 0
[attach-enode] #4224 0
[assign] #4225 justification -1: 710
[assign] #4226 justification -1: 710
[new-match] 0x61d50e67b760 #3286 #1569 #1276 #1396 #1167 #125 ; #748
[eq-expl] #782 cg (#1396 #3934) (#2574 #2574) (#1276 #1276) ; #4067
[eq-expl] #4067 lit #4068 ; #4066
[eq-expl] #4066 cg (#3934 #1396) (#1276 #1276) ; #2899
[eq-expl] #749 root
[new-match] 0x61d50e67b7a8 #552 #550 #749 #782 ; #750
[mk-app] #4227 vstd!seq.Seq.index.? #125 #1167 #3934 #1276
[mk-app] #4228 has_type #4227 #1167
[mk-app] #4229 or #3966 #4074 #4228
[inst-discovered] theory-solving 0 basic# ; #4229
[mk-app] #4230 = #4229 #4229
[instance] 0 #4230
[attach-enode] #4230 0
[end-of-instance]
[mk-app] #4230 or #4200 #3966 #4074 #4228
[instance] 0x61d50e67b760 ; 1
[attach-enode] #4227 1
[attach-enode] #4228 1
[end-of-instance]
[mk-app] #4231 Add #2899 #749
[mk-app] #4232 * #370 #4231
[mk-app] #4233 + #749 #2899 #4232
[mk-app] #4234 = #4233 #341
[mk-app] #4235 or #3887 #4234
[instance] 0x61d50e67b7a8 ; 1
[attach-enode] #4231 1
[attach-enode] #4232 1
[attach-enode] #4233 1
[attach-enode] #4234 1
[mk-app] #4236 <= #4233 #341
[mk-app] #4237 >= #4233 #341
[assign] #4234 justification -1: 77
[end-of-instance]
[assign] #4236 clause 715 -714
[assign] #4237 clause 716 -714
[mk-app] #4238 = #749 #4119
[attach-meaning] #370 arith (- 1)
[mk-app] #4239 * #370 #4119
[mk-app] #4240 + #749 #4239
[mk-app] #4241 <= #4240 #341
[mk-app] #4242 >= #4240 #341
[assign] #4238 justification -1: 698 592 707 706 702 701 705 704 584 583 574 573
[attach-enode] #4238 0
[attach-enode] #4239 0
[attach-enode] #4240 0
[assign] #4241 justification -1: 717
[assign] #4242 justification -1: 717
[mk-app] #4243 = #4120 #4231
[attach-meaning] #370 arith (- 1)
[mk-app] #4244 + #4120 #4232
[mk-app] #4245 <= #4244 #341
[mk-app] #4246 >= #4244 #341
[assign] #4243 justification -1: 698 592 657 707 706 702 701 705 704 584 583 574 573
[attach-enode] #4243 0
[attach-enode] #4244 0
[assign] #4245 justification -1: 720
[assign] #4246 justification -1: 720
[decide-and-or] #3660 #3706
[push] 3
[assign] #3706 decision axiom
[assign] #779 clause 490 -492 -491
[assign] #3645 clause 494 -490
[decide-and-or] #3770 #3689
[push] 4
[assign] #3689 decision axiom
[assign] (not #3714) clause -536 -537 -494
[assign] #696 clause 495 536
[assign] #3712 clause 535 536
[decide-and-or] #3712 #3678
[push] 5
[assign] (not #3647) decision axiom
[assign] #727 clause 496 498
[assign] (not #728) clause -497 498
[new-match] 0x61d50e60eb68 #2200 #2193 #1276 #1533 #1167 #125 #1534 #125 ; #728
[mk-app] #4247 req%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth. #125 #1534 #125 #1167 #3765 #1276
[mk-app] #4248 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_len.? #125 #1534 #125 #1167 #3765
[mk-app] #4249 %I #4248
[mk-app] #4250 * #370 #4249
[mk-app] #4251 + #3869 #4250
[mk-app] #4252 >= #4251 #341
[mk-app] #4253 not #4252
[mk-app] #4254 or #2197 #4253
[mk-app] #4255 = #4247 #4254
[mk-app] #4256 not #2200
[mk-app] #4257 or #4256 #4255
[instance] 0x61d50e60eb68 ; 1
[attach-enode] #4247 1
[attach-enode] #4248 1
[attach-enode] #4249 1
[attach-enode] #4250 1
[attach-enode] #4251 1
[assign] #4255 justification -1: 396
[end-of-instance]
[assign] (not #4247) justification -1: -497 539
[assign] (not #4254) clause -726 723 -727
[assign] #2190 clause 724 726
[assign] #4252 clause 725 726
[new-match] 0x61d50e60ef58 #2158 #2155 #3765 #1167 #125 #1534 #125 ; #4248
[new-match] 0x61d50e60efa8 #2252 #2248 #3765 #1167 #125 ; #4248 (#125 #125) (#1534 #1534)
[mk-app] #4258 has_type #3765 #1534
[mk-app] #4259 not #4258
[mk-app] #4260 has_type #4248 #200
[mk-app] #4261 or #4259 #4260
[mk-app] #4262 not #2158
[mk-app] #4263 or #4262 #4259 #4260
[instance] 0x61d50e60ef58 ; 2
[attach-enode] #4258 2
[attach-enode] #4260 2
[end-of-instance]
[mk-app] #4264 %Poly%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS. #3765
[mk-app] #4265 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/seq #4264
[mk-app] #4266 vstd!view.View.view.? #125 #1168 #4265
[mk-app] #4267 vstd!seq.Seq.len.? #125 #1167 #4266
[mk-app] #4268 I #4267
[mk-app] #4269 = #4248 #4268
[mk-app] #4270 or #3930 #4269
[mk-app] #4271 not #2252
[mk-app] #4272 or #4271 #3930 #4269
[instance] 0x61d50e60efa8 ; 2
[attach-enode] #4264 2
[mk-app] #4273 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #4264
[mk-app] #4274 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS #4273
[attach-enode] #4273 2
[attach-enode] #4274 2
[mk-app] #4275 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #4274
[mk-app] #4276 = #4273 #4275
[new-match] 0 datatype#21 datatype#18 #4273 ; #4274
[instance] 0 #4276
[attach-enode] #4275 2
[end-of-instance]
[attach-enode] #4265 2
[attach-enode] #4266 2
[attach-enode] #4267 2
[attach-enode] #4268 2
[attach-enode] #4269 2
[assign] #4269 justification -1: 18 401
[end-of-instance]
[assign] #4258 justification -1: 469 539
[eq-expl] #4264 cg (#3765 #1533) ; #1393
[eq-expl] #4265 cg (#4264 #1393) ; #1395
[eq-expl] #4266 cg (#125 #125) (#1168 #1168) (#4265 #1395) ; #1396
[eq-expl] #4267 cg (#125 #125) (#1167 #1167) (#4266 #1396) ; #1379
[new-match] 0x61d50e60f7a0 #174 #173 #4267 ; #4268
[mk-app] #4276 I #1379
[mk-app] #4277 %I #4276
[mk-app] #4278 = #1379 #4277
[mk-app] #4279 or #3871 #4278
[instance] 0x61d50e60f7a0 ; 3
[attach-enode] #4276 3
[attach-enode] #4277 3
[attach-enode] #4278 3
[assign] #4278 justification -1: 26
[end-of-instance]
[assign] #4260 clause 729 -728
[mk-app] #4280 = #1379 #4249
[attach-meaning] #370 arith (- 1)
[mk-app] #4281 + #1379 #4250
[mk-app] #4282 <= #4281 #341
[mk-app] #4283 >= #4281 #341
[assign] #4280 justification -1: 731 730 539
[attach-enode] #4280 0
[attach-enode] #4281 0
[assign] #4282 justification -1: 732
[assign] #4283 justification -1: 732
[eq-expl] #4248 lit #4269 ; #4268
[eq-expl] #4268 root
[eq-expl] #200 root
[new-match] 0x61d50e60fbe0 #522 #203 #4248 ; #4260 (#200 #200)
[new-match] 0x61d50e60fc10 #207 #203 #4248 ; #4260 (#200 #200)
[new-match] 0x61d50e60fc40 #473 #470 #4267 ; #4260 (#200 #200) (#4248 #4268)
[mk-app] #4284 has_type #4268 #200
[mk-app] #4285 not #4284
[mk-app] #4286 %I #4268
[mk-app] #4287 >= #4286 #341
[mk-app] #4288 or #4285 #4287
[mk-app] #4289 not #522
[mk-app] #4290 or #4289 #4285 #4287
[instance] 0x61d50e60fbe0 ; 3
[attach-enode] #4284 3
[attach-enode] #4286 3
[end-of-instance]
[assign] #4284 justification -1: 729 730
[mk-app] #4291 = #4249 #4286
[attach-meaning] #370 arith (- 1)
[mk-app] #4292 * #370 #4286
[mk-app] #4293 + #4249 #4292
[mk-app] #4294 <= #4293 #341
[mk-app] #4295 >= #4293 #341
[assign] #4291 justification -1: 730
[attach-enode] #4291 0
[attach-enode] #4292 0
[attach-enode] #4293 0
[assign] #4294 justification -1: 737
[assign] #4295 justification -1: 737
[resolve-process] true
[resolve-lit] 0 (not #4252)
[resolve-lit] 0 (not #4282)
[resolve-lit] 4 (not #4106)
[resolve-process] (not #4282)
[resolve-lit] 0 (not #4280)
[resolve-process] (not #4280)
[resolve-lit] 0 (not #4278)
[resolve-lit] 0 (not #4269)
[resolve-process] (not #4278)
[resolve-process] (not #4269)
[conflict] (not #4252)
[pop] 1 6
[attach-enode] #4248 0
[attach-enode] #4249 0
[attach-enode] #4250 0
[attach-enode] #4251 0
[assign] (not #4252) axiom
[new-match] 0x61d50e60ed90 #2158 #2155 #3765 #1167 #125 #1534 #125 ; #4248
[new-match] 0x61d50e60ede0 #2252 #2248 #3765 #1167 #125 ; #4248 (#125 #125) (#1534 #1534)
[mk-app] #4273 not #2158
[mk-app] #4274 or #4273 #4259 #4260
[instance] 0x61d50e60ed90 ; 2
[attach-enode] #4258 2
[attach-enode] #4260 2
[end-of-instance]
[mk-app] #4275 not #2252
[mk-app] #4280 or #4275 #3930 #4269
[instance] 0x61d50e60ede0 ; 2
[attach-enode] #4264 2
[mk-app] #4281 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #4264
[mk-app] #4282 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS #4281
[attach-enode] #4281 2
[attach-enode] #4282 2
[mk-app] #4283 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #4282
[mk-app] #4291 = #4281 #4283
[new-match] 0 datatype#21 datatype#18 #4281 ; #4282
[instance] 0 #4291
[attach-enode] #4283 2
[end-of-instance]
[attach-enode] #4265 2
[attach-enode] #4266 2
[attach-enode] #4267 2
[attach-enode] #4268 2
[attach-enode] #4269 2
[assign] #4269 justification -1: 18 401
[end-of-instance]
[assign] #4258 justification -1: 469 539
[eq-expl] #4264 cg (#3765 #1533) ; #1393
[eq-expl] #4265 cg (#4264 #1393) ; #1395
[eq-expl] #4266 cg (#125 #125) (#1168 #1168) (#4265 #1395) ; #1396
[eq-expl] #4267 cg (#125 #125) (#1167 #1167) (#4266 #1396) ; #1379
[new-match] 0x61d50e60f5d8 #174 #173 #4267 ; #4268
[mk-app] #4291 or #3871 #4278
[instance] 0x61d50e60f5d8 ; 3
[attach-enode] #4276 3
[attach-enode] #4277 3
[attach-enode] #4278 3
[assign] #4278 justification -1: 26
[end-of-instance]
[assign] #4260 clause 725 -724
[mk-app] #4292 = #1379 #4249
[attach-meaning] #370 arith (- 1)
[mk-app] #4293 + #1379 #4250
[mk-app] #4294 <= #4293 #341
[mk-app] #4295 >= #4293 #341
[assign] #4292 justification -1: 727 726 539
[attach-enode] #4292 0
[attach-enode] #4293 0
[assign] #4294 justification -1: 728
[assign] #4295 justification -1: 728
[eq-expl] #4248 lit #4269 ; #4268
[eq-expl] #4268 root
[new-match] 0x61d50e60fa18 #522 #203 #4248 ; #4260 (#200 #200)
[new-match] 0x61d50e60fa48 #207 #203 #4248 ; #4260 (#200 #200)
[new-match] 0x61d50e60fa78 #473 #470 #4267 ; #4260 (#200 #200) (#4248 #4268)
[mk-app] #4289 not #522
[mk-app] #4290 or #4289 #4285 #4287
[instance] 0x61d50e60fa18 ; 3
[attach-enode] #4284 3
[attach-enode] #4286 3
[end-of-instance]
[assign] #4284 justification -1: 725 726
[mk-app] #4279 = #4249 #4286
[attach-meaning] #370 arith (- 1)
[mk-app] #4271 * #370 #4286
[mk-app] #4272 + #4249 #4271
[mk-app] #4262 <= #4272 #341
[mk-app] #4263 >= #4272 #341
[assign] #4279 justification -1: 726
[attach-enode] #4279 0
[attach-enode] #4271 0
[attach-enode] #4272 0
[assign] #4262 justification -1: 733
[assign] #4263 justification -1: 733
[assign] #4287 clause 732 -731
[decide-and-or] #3712 #3678
[push] 5
[assign] (not #3647) decision axiom
[assign] #727 clause 496 498
[assign] (not #728) clause -497 498
[new-match] 0x61d50e610048 #2200 #2193 #1276 #1533 #1167 #125 #1534 #125 ; #728
[mk-app] #4256 not #2200
[mk-app] #4257 or #4256 #4255
[instance] 0x61d50e610048 ; 1
[attach-enode] #4247 1
[assign] #4254 justification -1: -723
[assign] #4255 justification -1: 396
[end-of-instance]
[assign] #4247 clause 736 -739
[resolve-lit] 0 (not #4247)
[resolve-process] #4247
[resolve-lit] 0 #728
[resolve-process] (not #4247)
[resolve-lit] 0 (not #4255)
[resolve-lit] 0 (not #4254)
[resolve-process] (not #4255)
[resolve-process] (not #4254)
[resolve-lit] 1 #4252
[conflict] #728 #4252
[pop] 1 6
[assign] #728 clause 497 723
[assign] #3647 clause 498 -497
[assign] (not #3708) clause -534 -498 -535
[assign] #721 clause 499 534
[assign] #3674 clause 533 534
[new-match] 0x61d50e610030 #2200 #2193 #1276 #1533 #1167 #125 #1534 #125 ; #728
[eq-expl] #720 root
[new-match] 0x61d50e610088 #3408 #2204 #720 #1276 #1533 #1167 #125 #1534 #125 ; #721
[mk-app] #4256 not #2200
[mk-app] #4257 or #4256 #4255
[instance] 0x61d50e610030 ; 1
[attach-enode] #4247 1
[assign] #4254 justification -1: -723
[assign] #4255 justification -1: 396
[end-of-instance]
[mk-app] #4296 has_type #720 #1167
[mk-app] #4297 not #4296
[mk-app] #4298 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.? #125 #1534 #125 #1167 #3765 #1276
[mk-app] #4299 = #720 #4298
[mk-app] #4300 not #4299
[mk-app] #4301 or #4297 #4300
[mk-app] #4302 ens%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth. #125 #1534 #125 #1167 #3765 #1276 #720
[mk-app] #4303 = #4301 #4302
[mk-app] #4304 not #4303
[mk-app] #4305 not #4301
[inst-discovered] theory-solving 0 basic# ; #4304
[mk-app] #4305 = #4304 #4304
[instance] 0 #4305
[attach-enode] #4305 0
[end-of-instance]
[mk-app] #4305 not #3408
[mk-app] #4306 or #4305 #4304
[instance] 0x61d50e610088 ; 1
[attach-enode] #4296 1
[attach-enode] #4298 1
[attach-enode] #4299 1
[attach-enode] #4302 1
[assign] (not #4303) justification -1: 397
[end-of-instance]
[assign] #4247 clause 736 -739
[assign] #4302 justification -1: 499 539
[assign] (not #4301) clause -742 -743 744
[assign] #4296 clause 740 742
[assign] #4299 clause 741 742
[eq-expl] #720 lit #4299 ; #4298
[eq-expl] #4298 root
[new-match] 0x61d50e6b4f18 #542 #236 #720 #275 ; #4296 (#1167 #1167)
[new-match] 0x61d50e6b4f50 #240 #236 #720 #275 ; #4296 (#1167 #1167)
[new-match] 0x61d50e6b4f88 #3402 #2179 #1276 #3765 #1167 #125 #1534 #125 ; #4298
[new-match] 0x61d50e6b4fe0 #2269 #2265 #1276 #3765 #1167 #125 ; #4298 (#125 #125) (#1534 #1534)
[mk-app] #4307 has_type #4298 #1167
[mk-app] #4308 not #4307
[mk-app] #4309 %I #4298
[mk-app] #4310 iInv #275 #4309
[mk-app] #4311 or #4308 #4310
[mk-app] #4312 not #542
[mk-app] #4313 or #4312 #4308 #4310
[instance] 0x61d50e6b4f18 ; 2
[attach-enode] #4307 2
[attach-enode] #4309 2
[attach-enode] #4310 2
[end-of-instance]
[mk-app] #4314 I #4309
[mk-app] #4315 = #4298 #4314
[mk-app] #4316 or #4308 #4315
[mk-app] #4317 not #240
[mk-app] #4318 or #4317 #4308 #4315
[instance] 0x61d50e6b4f50 ; 2
[attach-enode] #4314 2
[attach-enode] #4315 2
[end-of-instance]
[mk-app] #4319 vstd!seq.Seq.index.? #125 #1167 #4266 #1276
[mk-app] #4320 = #4298 #4319
[mk-app] #4321 or #3930 #4320
[mk-app] #4322 not #2269
[mk-app] #4323 or #4322 #3930 #4320
[instance] 0x61d50e6b4fe0 ; 2
[attach-enode] #4319 2
[attach-enode] #4320 2
[assign] #4320 justification -1: 18 404
[end-of-instance]
[assign] #4307 justification -1: 740 741
[assign] #4198 justification -1: 740 741 748 698 592 539 707 706 702 701 705 704 584 583 574 573
[assign] #4228 justification -1: 740 741 748 698 592 539 707 706 702 701 705 704 584 583 574 573
[eq-expl] #4115 lit #4196 ; #4139
[eq-expl] #4139 th arith ; #1454
[eq-expl] #1454 lit #3870 ; #3869
[eq-expl] #3869 root
[eq-expl] #4116 cg (#4115 #1454) ; #1276
[eq-expl] #4118 cg (#125 #125) (#1167 #1167) (#3934 #1396) (#4116 #1276) ; #748
[eq-expl] #748 root
[new-match] 0x61d50e6b5620 #542 #236 #4118 #275 ; #4198 (#1167 #1167)
[new-match] 0x61d50e6b5658 #240 #236 #4118 #275 ; #4198 (#1167 #1167)
[mk-app] #4324 has_type #748 #1167
[mk-app] #4325 not #4324
[mk-app] #4326 iInv #275 #749
[mk-app] #4327 or #4325 #4326
[mk-app] #4328 or #4312 #4325 #4326
[instance] 0x61d50e6b5620 ; 4
[attach-enode] #4324 4
[attach-enode] #4326 4
[end-of-instance]
[mk-app] #4329 I #749
[mk-app] #4330 = #748 #4329
[mk-app] #4331 or #4325 #4330
[mk-app] #4332 or #4317 #4325 #4330
[instance] 0x61d50e6b5658 ; 4
[attach-enode] #4329 4
[attach-enode] #4330 4
[end-of-instance]
[assign] #4310 clause 746 -745
[assign] #4315 clause 747 -745
[assign] #4324 justification -1: 699 698 592 707 706 702 701 705 704 584 583 574 573
[assign] #4326 justification -1: 746 741 748 539 741
[assign] #4330 justification -1: 747 748 539 741 748 741
[eq-expl] #4298 lit #4320 ; #4319
[eq-expl] #4319 cg (#125 #125) (#1167 #1167) (#4266 #1396) (#1276 #1276) ; #748
[eq-expl] #4309 cg (#4298 #720) ; #713
[eq-expl] #713 cg (#720 #748) ; #749
[new-match] 0x61d50e6b5a48 #1136 #455 #4309 #275 ; #4310
[new-match] 0x61d50e6b5a80 #174 #173 #4309 ; #4314
[eq-expl] #4314 lit #4315 ; #4298
[new-match] 0x61d50e6b5ab0 #503 #499 #4309 #275 ; #4198 (#4118 #4314) (#1167 #1167)
[mk-app] #4333 + #749 #3817
[mk-app] #4334 >= #4333 #341
[mk-app] #4335 not #4334
[mk-app] #4336 + #749 #3821
[mk-app] #4337 >= #4336 #341
[mk-app] #4338 or #4335 #4337
[mk-app] #4339 = #4338 #4326
[mk-app] #4340 not #4339
[mk-app] #4341 + #3817 #749
[inst-discovered] theory-solving 0 arith# ; #4333
[mk-app] #4342 = #4333 #4341
[instance] 0 #4342
[attach-enode] #4342 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4342 * #370 #749
[mk-app] #4343 + #313 #4342
[mk-app] #4344 <= #4343 #341
[mk-app] #4345 >= #4341 #341
[inst-discovered] theory-solving 0 arith# ; #4345
[mk-app] #4346 = #4345 #4344
[instance] 0 #4346
[attach-enode] #4346 0
[end-of-instance]
[mk-app] #4341 not #4344
[mk-app] #4345 + #3821 #749
[inst-discovered] theory-solving 0 arith# ; #4336
[mk-app] #4346 = #4336 #4345
[instance] 0 #4346
[attach-enode] #4346 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4346 + #335 #4342
[mk-app] #4347 <= #4346 #341
[mk-app] #4348 >= #4345 #341
[inst-discovered] theory-solving 0 arith# ; #4348
[mk-app] #4349 = #4348 #4347
[instance] 0 #4349
[attach-enode] #4349 0
[end-of-instance]
[mk-app] #4345 or #4341 #4347
[mk-app] #4348 = #4345 #4326
[mk-app] #4349 not #4345
[mk-app] #4350 not #4348
[inst-discovered] theory-solving 0 basic# ; #4350
[mk-app] #4349 = #4350 #4350
[instance] 0 #4349
[attach-enode] #4349 0
[end-of-instance]
[mk-app] #4349 or #3836 #4350
[instance] 0x61d50e6b5a48 ; 3
[attach-enode] #4342 3
[attach-enode] #4343 3
[attach-enode] #4346 3
[assign] (not #4348) justification -1: 61
[end-of-instance]
[assign] (not #4345) clause -754 755
[assign] #4344 clause 752 754
[assign] (not #4347) clause -753 754
[decide-and-or] #3674 #3658
[push] 5
[assign] (not #3649) decision axiom
[assign] #712 clause 500 502
[assign] (not #706) clause -501 502
[assign] #3682 clause 532 501
[eq-expl] #741 lit #696 ; #1462
[eq-expl] #705 cg (#741 #782) (#713 #749) ; #750
[eq-expl] #4117 cg (#3934 #1396) (#2574 #2574) (#4116 #1276) ; #782
[eq-expl] #4119 cg (#4118 #748) ; #749
[eq-expl] #750 cg (#782 #4117) (#749 #4119) ; #4120
[eq-expl] #4120 lit #4121 ; #4114
[eq-expl] #4114 cg (#3934 #1396) (#2574 #2574) (#3638 #3638) ; #3639
[eq-expl] #3639 root
[new-match] 0x61d50e6b5d80 #1136 #455 #705 #275 ; #706
[mk-app] #4351 + #3639 #3817
[mk-app] #4352 >= #4351 #341
[mk-app] #4353 not #4352
[mk-app] #4354 + #3639 #3821
[mk-app] #4355 >= #4354 #341
[mk-app] #4356 or #4353 #4355
[mk-app] #4357 iInv #275 #3639
[mk-app] #4358 = #4356 #4357
[mk-app] #4359 not #4358
[mk-app] #4360 + #3817 #3639
[inst-discovered] theory-solving 0 arith# ; #4351
[mk-app] #4361 = #4351 #4360
[instance] 0 #4361
[attach-enode] #4361 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4361 * #370 #3639
[mk-app] #4362 + #313 #4361
[mk-app] #4363 <= #4362 #341
[mk-app] #4364 >= #4360 #341
[inst-discovered] theory-solving 0 arith# ; #4364
[mk-app] #4365 = #4364 #4363
[instance] 0 #4365
[attach-enode] #4365 0
[end-of-instance]
[mk-app] #4360 not #4363
[mk-app] #4364 + #3821 #3639
[inst-discovered] theory-solving 0 arith# ; #4354
[mk-app] #4365 = #4354 #4364
[instance] 0 #4365
[attach-enode] #4365 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4365 + #335 #4361
[mk-app] #4366 <= #4365 #341
[mk-app] #4367 >= #4364 #341
[inst-discovered] theory-solving 0 arith# ; #4367
[mk-app] #4368 = #4367 #4366
[instance] 0 #4368
[attach-enode] #4368 0
[end-of-instance]
[mk-app] #4364 or #4360 #4366
[mk-app] #4367 = #4364 #4357
[mk-app] #4368 not #4364
[mk-app] #4369 not #4367
[inst-discovered] theory-solving 0 basic# ; #4369
[mk-app] #4368 = #4369 #4369
[instance] 0 #4368
[attach-enode] #4368 0
[end-of-instance]
[mk-app] #4368 or #3836 #4369
[instance] 0x61d50e6b5d80 ; 1
[attach-enode] #4361 1
[attach-enode] #4362 1
[attach-enode] #4365 1
[attach-enode] #4357 1
[assign] (not #4367) justification -1: 61
[end-of-instance]
[assign] (not #4357) justification -1: -501 676 592 698 741 748 539 495 481 657 707 706 702 701 705 704 584 583 574 573
[mk-app] #4370 = #3639 #4114
[attach-meaning] #370 arith (- 1)
[mk-app] #4371 * #370 #4114
[mk-app] #4372 + #3639 #4371
[mk-app] #4373 <= #4372 #341
[mk-app] #4374 >= #4372 #341
[assign] #4370 justification -1: 592
[attach-enode] #4370 0
[attach-enode] #4371 0
[attach-enode] #4372 0
[assign] #4373 justification -1: 761
[assign] #4374 justification -1: 761
[assign] #4364 clause 758 759 760
[decide-and-or] #3859 #3856
[push] 6
[assign] (not #3855) decision axiom
[new-match] 0x61d50e6b63d8 #1178 #1174 #1289 ; #3855 (#1168 #1168)
[eq-expl] #1288 lit #3866 ; #3865
[eq-expl] #3865 root
[new-match] 0x61d50e6b6408 #1181 #1180 #1288 ; #3855 (#1168 #1168) (#1289 #1289)
[mk-app] #4375 Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #3865
[mk-app] #4376 has_type #4375 #1168
[mk-app] #4377 not #1181
[mk-app] #4378 or #4377 #4376
[instance] 0x61d50e6b6408 ; 2
[attach-enode] #4375 2
[attach-enode] #4376 2
[assign] #4376 justification -1: 255
[end-of-instance]
[resolve-lit] 0 (not #4376)
[resolve-process] #4376
[resolve-lit] 0 #3855
[resolve-process] (not #4376)
[conflict] #3855
[pop] 1 7
[assign] #3855 axiom
[assign] #3857 clause 565 -564
[assign] #3844 justification -1: 565 591
[new-match] 0x61d50e6b63e0 #1178 #1174 #1289 ; #3855 (#1168 #1168)
[new-match] 0x61d50e6b6410 #1181 #1180 #1288 ; #3855 (#1168 #1168) (#1289 #1289)
[new-match] 0x61d50e6b6440 #1198 #1194 #1290 ; #3844 (#1188 #1188)
[mk-app] #4377 %Poly%vstd!seq.Seq<i32.>. #1290
[mk-app] #4378 Poly%vstd!seq.Seq<i32.>. #4377
[mk-app] #4379 = #1290 #4378
[mk-app] #4380 or #3845 #4379
[mk-app] #4381 or #3939 #3845 #4379
[instance] 0x61d50e6b6440 ; 2
[attach-enode] #4377 2
[attach-enode] #4378 2
[attach-enode] #4379 2
[assign] #4379 justification -1: 257 561
[end-of-instance]
[eq-expl] #4377 root
[new-match] 0x61d50e6b6648 #1187 #1186 #4377 ; #4378
[eq-expl] #1290 lit #4379 ; #4378
[eq-expl] #4378 root
[new-match] 0x61d50e6b6678 #1201 #1200 #4377 ; #3844 (#1188 #1188) (#1290 #4378)
[decide-and-or] #3942 #3969
[push] 6
[assign] (not #3968) decision axiom
[mk-app] #4382 <= #3967 #341
[mk-app] #4383 >= #3967 #341
[push] 7
[assign] (not #3975) decision axiom
[assign] (not #3980) clause -630 629
[decide-and-or] #4079 #4072
[push] 8
[assign] (not #4071) decision axiom
[eq-expl] #189 root
[new-match] 0x61d50e6b6770 #199 #195 #2574 ; #4071 (#189 #189)
[new-match] 0x61d50e6b67a0 #467 #466 #341 ; #4071 (#189 #189) (#2574 #2574)
[mk-app] #4384 not #467
[mk-app] #4385 or #4384 #4071
[instance] 0x61d50e6b67a0 ; 2
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #4071
[conflict] #4071
[pop] 1 9
[assign] #4071 axiom
[new-match] 0x61d50e6b6758 #199 #195 #2574 ; #4071 (#189 #189)
[new-match] 0x61d50e6b6788 #467 #466 #341 ; #4071 (#189 #189) (#2574 #2574)
[decide-and-or] #4079 #4074
[push] 8
[assign] (not #4073) decision axiom
[assign] (not #4133) justification -1: -659 698 707 706 702 701 705 704 584 583 574 573
[new-match] 0x61d50e6b67f0 #199 #195 #1276 ; #4073 (#189 #189)
[new-match] 0x61d50e6b6820 #467 #466 #1454 ; #4073 (#189 #189) (#1276 #1276)
[mk-app] #4384 I #3869
[mk-app] #4385 has_type #4384 #189
[mk-app] #4386 not #467
[mk-app] #4387 or #4386 #4385
[instance] 0x61d50e6b6820 ; 2
[attach-enode] #4384 2
[attach-enode] #4385 2
[assign] #4385 justification -1: 63
[end-of-instance]
[resolve-lit] 0 (not #4385)
[resolve-process] #4385
[resolve-lit] 0 #4073
[resolve-process] (not #4385)
[conflict] #4073
[pop] 1 9
[assign] #4073 axiom
[assign] #4076 clause 660 -659 -658
[assign] #4133 justification -1: 659 698 707 706 702 701 705 704 584 583 574 573
[assign] #4136 justification -1: 660 698 592 707 706 702 701 705 704 584 583 574 573
[new-match] 0x61d50e6b6820 #199 #195 #1276 ; #4073 (#189 #189)
[new-match] 0x61d50e6b6850 #467 #466 #1454 ; #4073 (#189 #189) (#1276 #1276)
[eq-expl] #2352 root
[new-match] 0x61d50e6b6880 #2320 #2319 #2352 #1276 #2574 #3934 ; #4075
[eq-expl] #2351 root
[new-match] 0x61d50e6b68c8 #3437 #2337 #2351 #1276 #2574 #3934 ; #4075 (#2352 #2352)
[mk-app] #4386 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #2574 #1276 #2316
[mk-app] #4387 = #4075 #4386
[mk-app] #4388 not #2320
[mk-app] #4389 or #4388 #4387
[instance] 0x61d50e6b6880 ; 3
[attach-enode] #2316 3
[attach-enode] #4386 3
[attach-enode] #4387 3
[assign] #4387 justification -1: 414
[end-of-instance]
[mk-app] #4390 + #4080 #4103
[mk-app] #4391 >= #4390 #341
[mk-app] #4392 vstd!seq.Seq.index.? #125 #1167 #3934 #2574
[mk-app] #4393 %I #4392
[mk-app] #4394 Add #4080 #296
[mk-app] #4395 I #4394
[mk-app] #4396 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #4395 #1276 #2351
[mk-app] #4397 Add #4393 #4396
[mk-app] #4398 if #4391 #341 #4397
[mk-app] #4399 = #4075 #4398
[mk-app] #4400 or #3966 #4072 #4074 #4399
[mk-app] #4401 + #4103 #4080
[inst-discovered] theory-solving 0 arith# ; #4390
[mk-app] #4402 = #4390 #4401
[instance] 0 #4402
[attach-enode] #4402 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4402 <= #4084 #341
[mk-app] #4403 >= #4401 #341
[inst-discovered] theory-solving 0 arith# ; #4403
[mk-app] #4404 = #4403 #4402
[instance] 0 #4404
[attach-enode] #4404 0
[end-of-instance]
[mk-app] #4401 if #4402 #341 #4397
[mk-app] #4403 = #4075 #4401
[mk-app] #4404 or #3966 #4072 #4074 #4403
[inst-discovered] theory-solving 0 basic# ; #4404
[mk-app] #4405 = #4404 #4404
[instance] 0 #4405
[attach-enode] #4405 0
[end-of-instance]
[mk-app] #4405 not #3437
[mk-app] #4406 or #4405 #3966 #4072 #4074 #4403
[instance] 0x61d50e6b68c8 ; 3
[mk-app] #4407 = #4401 #341
[mk-app] #4408 = #4397 #4401
[attach-enode] #4401 3
[attach-enode] #4392 3
[attach-enode] #4393 3
[attach-enode] #4394 3
[attach-enode] #4395 3
[attach-enode] #4396 3
[attach-enode] #4397 3
[attach-enode] #4407 3
[attach-enode] #4408 3
[attach-enode] #4403 3
[assign] #4403 justification -1: 415 626 658 659
[end-of-instance]
[eq-expl] #2316 root
[new-match] 0x61d50e6c0b70 #2320 #2319 #2316 #1276 #2574 #3934 ; #4386
[decide-and-or] #4098 #4086
[push] 8
[assign] (not #4085) decision axiom
[assign] #4402 clause 768 662
[assign] #4407 clause 769 -768
[mk-app] #4404 = #4067 #341
[mk-app] #4409 <= #4067 #341
[mk-app] #4410 >= #4067 #341
[assign] #4404 justification -1: 660 771 769
[attach-enode] #4404 0
[assign] #4409 justification -1: 772
[assign] #4410 justification -1: 772
[resolve-process] true
[resolve-lit] 7 (not #4102)
[resolve-lit] 0 #4085
[resolve-lit] 7 (not #4105)
[conflict] #4085
[pop] 1 9
[assign] #4085 axiom
[decide-and-or] #4098 #4089
[push] 8
[assign] (not #4088) decision axiom
[resolve-process] true
[resolve-lit] 0 #4088
[resolve-lit] 7 (not #4106)
[conflict] #4088
[pop] 1 9
[assign] #4088 axiom
[assign] (not #4094) clause -666 -663 -661 -658 -659 -662
[assign] #4090 clause 664 666
[assign] #4092 clause 665 666
[assign] #4146 clause 685 -664 -711
[assign] #4148 clause 686 -665 -712
[assign] (not #4150) clause -687 -686 -685
[decide-and-or] #4167 #4163
[push] 8
[assign] (not #4161) decision axiom
[new-match] 0x61d50e6c0cd8 #199 #195 #3638 ; #4161 (#189 #189)
[new-match] 0x61d50e6c0d08 #467 #466 #1025 ; #4161 (#189 #189) (#3638 #3638)
[mk-app] #4404 not #467
[mk-app] #4409 or #4404 #4161
[instance] 0x61d50e6c0d08 ; 2
[end-of-instance]
[resolve-process] true
[resolve-lit] 0 #4161
[conflict] #4161
[pop] 1 9
[assign] #4161 axiom
[assign] #4165 clause 689 -688 -658
[assign] (not #4179) clause -694 -688 -690 -691 -658
[assign] #4175 clause 692 694
[assign] #4177 clause 693 694
[new-match] 0x61d50e6c0ce8 #199 #195 #3638 ; #4161 (#189 #189)
[new-match] 0x61d50e6c0d18 #467 #466 #1025 ; #4161 (#189 #189) (#3638 #3638)
[new-match] 0x61d50e6c0d48 #2320 #2319 #2352 #3638 #2574 #3934 ; #4164
[new-match] 0x61d50e6c0d90 #3437 #2337 #2351 #3638 #2574 #3934 ; #4164 (#2352 #2352)
[mk-app] #4404 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #2574 #3638 #2316
[mk-app] #4409 = #4164 #4404
[mk-app] #4410 or #4388 #4409
[instance] 0x61d50e6c0d48 ; 3
[attach-enode] #4404 3
[attach-enode] #4409 3
[assign] #4409 justification -1: 414
[end-of-instance]
[mk-app] #4411 >= #4184 #341
[mk-app] #4412 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #4395 #3638 #2351
[mk-app] #4413 Add #4393 #4412
[mk-app] #4414 if #4411 #341 #4413
[mk-app] #4415 = #4164 #4414
[mk-app] #4416 or #3966 #4072 #4163 #4415
[inst-discovered] theory-solving 0 basic# ; #4416
[mk-app] #4417 = #4416 #4416
[instance] 0 #4417
[attach-enode] #4417 0
[end-of-instance]
[mk-app] #4417 or #4405 #3966 #4072 #4163 #4415
[instance] 0x61d50e6c0d90 ; 3
[mk-app] #4418 = #4414 #341
[mk-app] #4419 = #4413 #4414
[attach-enode] #4414 3
[attach-enode] #4412 3
[attach-enode] #4413 3
[attach-enode] #4418 3
[attach-enode] #4419 3
[attach-enode] #4415 3
[assign] #4415 justification -1: 415 626 658 688
[end-of-instance]
[new-match] 0x61d50e6c1310 #2320 #2319 #2316 #3638 #2574 #3934 ; #4404
[decide-and-or] #4364 #4360
[push] 8
[assign] (not #4363) decision axiom
[resolve-process] true
[resolve-lit] 3 (not #4374)
[resolve-lit] 0 #4363
[resolve-lit] 1 (not #4175)
[conflict] #4363 (not #4175)
[pop] 1 9
[assign] #4363 clause 756 -692
[assign] #4366 clause 757 -756 -758
[resolve-process] true
[resolve-lit] 2 (not #4373)
[resolve-lit] 0 (not #4366)
[resolve-lit] 0 (not #4177)
[resolve-process] (not #4366)
[resolve-lit] 0 (not #4363)
[resolve-lit] 2 (not #4364)
[resolve-process] (not #4363)
[resolve-lit] 0 (not #4175)
[resolve-process] (not #4177)
[resolve-lit] 0 #4179
[resolve-process] (not #4175)
[conflict] #4179 (not #4364)
[pop] 2 8
[assign] #4071 axiom
[assign] #4073 axiom
[assign] #4085 axiom
[assign] #4088 axiom
[assign] #4161 axiom
[assign] #4179 clause 694 -758
[assign] #4076 clause 660 -659 -658
[assign] (not #4094) clause -666 -663 -661 -658 -659 -662
[assign] #4165 clause 689 -688 -658
[resolve-process] true
[resolve-lit] 0 (not #4179)
[resolve-lit] 0 (not #4161)
[resolve-lit] 3 (not #4185)
[resolve-lit] 3 (not #4189)
[resolve-lit] 0 (not #4071)
[resolve-lit] 4 (not #4081)
[resolve-process] (not #4179)
[resolve-lit] 0 (not #4364)
[resolve-process] (not #4161)
[resolve-process] (not #4071)
[conflict] (not #4364)
[pop] 1 6
[attach-enode] #4361 0
[attach-enode] #4362 0
[attach-enode] #4365 0
[assign] #3855 axiom
[assign] #4071 axiom
[assign] #4073 axiom
[assign] #4085 axiom
[assign] #4088 axiom
[assign] #4161 axiom
[assign] (not #4364) axiom
[assign] #3857 clause 565 -564
[assign] #4076 clause 660 -659 -658
[assign] (not #4094) clause -666 -663 -661 -658 -659 -662
[assign] #4165 clause 689 -688 -658
[assign] (not #4179) clause -694 -688 -690 -691 -658
[assign] #4363 clause 756 758
[assign] (not #4366) clause -757 758
[assign] #4090 clause 664 666
[assign] #4092 clause 665 666
[assign] #4175 clause 692 694
[assign] #4177 clause 693 694
[assign] #4146 clause 685 -664 -711
[assign] #4148 clause 686 -665 -712
[assign] (not #4150) clause -687 -686 -685
[assign] #3844 justification -1: 565 591
[assign] #4136 justification -1: 660 698 592 707 706 702 701 705 704 584 583 574 573
[mk-app] #4367 = #3639 #4114
[attach-meaning] #370 arith (- 1)
[mk-app] #4370 * #370 #4114
[mk-app] #4371 + #3639 #4370
[mk-app] #4372 <= #4371 #341
[mk-app] #4373 >= #4371 #341
[assign] #4367 justification -1: 592
[attach-enode] #4367 0
[attach-enode] #4370 0
[attach-enode] #4371 0
[assign] #4372 justification -1: 759
[assign] #4373 justification -1: 759
[eq-expl] #1290 root
[new-match] 0x61d50e6b6258 #1198 #1194 #1290 ; #3844 (#1188 #1188)
[new-match] 0x61d50e6b6288 #2320 #2319 #2352 #1276 #2574 #3934 ; #4075
[new-match] 0x61d50e6b62d0 #3437 #2337 #2351 #1276 #2574 #3934 ; #4075 (#2352 #2352)
[new-match] 0x61d50e6b6318 #2320 #2319 #2352 #3638 #2574 #3934 ; #4164
[new-match] 0x61d50e6b6360 #3437 #2337 #2351 #3638 #2574 #3934 ; #4164 (#2352 #2352)
[mk-app] #4374 or #3939 #3845 #4379
[instance] 0x61d50e6b6258 ; 2
[attach-enode] #4377 2
[attach-enode] #4378 2
[attach-enode] #4379 2
[assign] #4379 justification -1: 257 561
[end-of-instance]
[mk-app] #4381 not #2320
[mk-app] #4369 or #4381 #4387
[instance] 0x61d50e6b6288 ; 3
[attach-enode] #2316 3
[attach-enode] #4386 3
[attach-enode] #4387 3
[assign] #4387 justification -1: 414
[end-of-instance]
[mk-app] #4368 + #4103 #4080
[inst-discovered] theory-solving 0 arith# ; #4390
[mk-app] #4382 = #4390 #4368
[instance] 0 #4382
[attach-enode] #4382 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4382 <= #4084 #341
[mk-app] #4383 >= #4368 #341
[inst-discovered] theory-solving 0 arith# ; #4383
[mk-app] #4402 = #4383 #4382
[instance] 0 #4402
[attach-enode] #4402 0
[end-of-instance]
[mk-app] #4368 if #4382 #341 #4397
[mk-app] #4383 = #4075 #4368
[mk-app] #4402 or #3966 #4072 #4074 #4383
[inst-discovered] theory-solving 0 basic# ; #4402
[mk-app] #4401 = #4402 #4402
[instance] 0 #4401
[attach-enode] #4401 0
[end-of-instance]
[mk-app] #4401 not #3437
[mk-app] #4407 or #4401 #3966 #4072 #4074 #4383
[instance] 0x61d50e6b62d0 ; 3
[mk-app] #4408 = #4368 #341
[mk-app] #4403 = #4368 #4397
[attach-enode] #4368 3
[attach-enode] #4392 3
[attach-enode] #4393 3
[attach-enode] #4394 3
[attach-enode] #4395 3
[attach-enode] #4396 3
[attach-enode] #4397 3
[attach-enode] #4408 3
[attach-enode] #4403 3
[attach-enode] #4383 3
[assign] #4383 justification -1: 415 626 658 659
[end-of-instance]
[mk-app] #4402 or #4381 #4409
[instance] 0x61d50e6b6318 ; 3
[attach-enode] #4404 3
[attach-enode] #4409 3
[assign] #4409 justification -1: 414
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4416
[mk-app] #4418 = #4416 #4416
[instance] 0 #4418
[attach-enode] #4418 0
[end-of-instance]
[mk-app] #4418 or #4401 #3966 #4072 #4163 #4415
[instance] 0x61d50e6b6360 ; 3
[mk-app] #4419 = #4414 #341
[mk-app] #4405 = #4413 #4414
[attach-enode] #4414 3
[attach-enode] #4412 3
[attach-enode] #4413 3
[attach-enode] #4419 3
[attach-enode] #4405 3
[attach-enode] #4415 3
[assign] #4415 justification -1: 415 626 658 688
[end-of-instance]
[eq-expl] #4377 root
[new-match] 0x61d50e6c0d90 #1187 #1186 #4377 ; #4378
[eq-expl] #2316 root
[new-match] 0x61d50e6c0dc0 #2320 #2319 #2316 #1276 #2574 #3934 ; #4386
[new-match] 0x61d50e6c0e08 #2320 #2319 #2316 #3638 #2574 #3934 ; #4404
[eq-expl] #1290 lit #4379 ; #4378
[eq-expl] #4378 root
[new-match] 0x61d50e6c0e50 #1201 #1200 #4377 ; #3844 (#1188 #1188) (#1290 #4378)
[decide-and-or] #3674 #3658
[push] 5
[assign] (not #3649) decision axiom
[assign] #712 clause 500 502
[assign] (not #706) clause -501 502
[assign] #3682 clause 532 501
[eq-expl] #2899 root
[eq-expl] #4067 lit #4068 ; #4066
[eq-expl] #4066 cg (#3934 #1396) (#1276 #1276) ; #2899
[new-match] 0x61d50e6c0ec0 #1136 #455 #705 #275 ; #706
[mk-app] #4417 = #4364 #4357
[mk-app] #4388 not #4364
[mk-app] #4410 not #4417
[inst-discovered] theory-solving 0 basic# ; #4410
[mk-app] #4388 = #4410 #4410
[instance] 0 #4388
[attach-enode] #4388 0
[end-of-instance]
[mk-app] #4388 or #3836 #4410
[instance] 0x61d50e6c0ec0 ; 1
[attach-enode] #4357 1
[assign] (not #4417) justification -1: 61
[end-of-instance]
[assign] #4357 clause 773 774
[resolve-lit] 0 (not #4357)
[resolve-process] #4357
[resolve-lit] 0 #706
[resolve-lit] 3 (not #4121)
[resolve-lit] 3 (not #4196)
[resolve-lit] 1 (not #4299)
[resolve-lit] 1 (not #4320)
[resolve-lit] 1 (not #696)
[resolve-lit] 4 (not #4068)
[resolve-lit] 3 (not #4218)
[resolve-lit] 3 (not #4217)
[resolve-lit] 3 (not #4210)
[resolve-lit] 3 (not #4209)
[resolve-lit] 3 (not #4215)
[resolve-lit] 3 (not #4214)
[resolve-process] (not #4357)
[resolve-lit] 0 #4417
[resolve-lit] 1 #4364
[resolve-process] #4417
[conflict] #706 (not #4121) (not #4299) (not #696) (not #4068) #4364
[pop] 1 6
[assign] #706 clause 501 -741 -676 -495 -657 758
[assign] #3649 clause 502 -501
[assign] (not #3682) clause -532 -502 -533
[assign] #687 clause 503 532
[assign] #673 clause 504 532
[assign] #3687 clause 531 532
[new-match] 0x61d50e6c0ed0 #1136 #455 #705 #275 ; #706
[eq-expl] #658 root
[eq-expl] #672 root
[new-match] 0x61d50e6c0f08 #3390 #2134 #672 #658 #1289 #1147 #125 #1167 #125 ; #673
[eq-expl] #698 lit #687 ; #686
[eq-expl] #686 root
[new-match] 0x61d50e6c0f68 #174 #173 #698 ; #672
[eq-expl] #652 root
[new-match] 0x61d50e6c0f98 #1166 #1165 #652 ; #658
[new-match] 0x61d50e6c0fc8 #2956 #392 #705 #275 ; #686
[mk-app] #4417 = #4364 #4357
[mk-app] #4410 not #4364
[mk-app] #4388 not #4417
[inst-discovered] theory-solving 0 basic# ; #4388
[mk-app] #4410 = #4388 #4388
[instance] 0 #4410
[attach-enode] #4410 0
[end-of-instance]
[mk-app] #4410 or #3836 #4388
[instance] 0x61d50e6c0ed0 ; 1
[attach-enode] #4357 1
[assign] (not #4417) justification -1: 61
[end-of-instance]
[mk-app] #4406 has_type #658 #1168
[mk-app] #4389 not #4406
[mk-app] #4420 vstd!seq.Seq.push.? #125 #1167 #1290 #672
[mk-app] #4421 = #459 #4420
[mk-app] #4422 not #4421
[mk-app] #4423 or #4389 #4422
[mk-app] #4424 = #4423 #673
[mk-app] #4425 not #4424
[mk-app] #4426 not #4423
[inst-discovered] theory-solving 0 basic# ; #4425
[mk-app] #4426 = #4425 #4425
[instance] 0 #4426
[attach-enode] #4426 0
[end-of-instance]
[mk-app] #4426 not #3390
[mk-app] #4427 or #4426 #4425
[instance] 0x61d50e6c0f08 ; 1
[attach-enode] #4406 1
[attach-enode] #4420 1
[attach-enode] #4421 1
[assign] (not #4424) justification -1: 389
[end-of-instance]
[mk-app] #4428 I #686
[mk-app] #4429 %I #4428
[mk-app] #4430 = #686 #4429
[mk-app] #4431 or #3871 #4430
[instance] 0x61d50e6c0f68 ; 1
[attach-enode] #4428 1
[attach-enode] #4429 1
[attach-enode] #4430 1
[assign] #4430 justification -1: 26
[end-of-instance]
[mk-app] #4432 %Poly%alloc!vec.Vec<i32./alloc!alloc.Global.>. #658
[mk-app] #4433 = #652 #4432
[mk-app] #4434 or #3867 #4433
[instance] 0x61d50e6c0f98 ; 1
[attach-enode] #4432 1
[attach-enode] #4433 1
[assign] #4433 justification -1: 253
[end-of-instance]
[mk-app] #4435 iClip #275 #3639
[mk-app] #4436 * #370 #4435
[mk-app] #4437 + #313 #4436
[mk-app] #4438 <= #4437 #341
[mk-app] #4439 not #4438
[mk-app] #4440 + #4435 #3821
[mk-app] #4441 >= #4440 #341
[mk-app] #4442 = #3639 #4435
[mk-app] #4443 or #4353 #4355 #4442
[mk-app] #4444 not #4443
[mk-app] #4445 or #4439 #4441 #4444
[mk-app] #4446 not #4445
[mk-app] #4447 + #3821 #4435
[inst-discovered] theory-solving 0 arith# ; #4440
[mk-app] #4448 = #4440 #4447
[instance] 0 #4448
[attach-enode] #4448 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4448 + #335 #4436
[mk-app] #4449 <= #4448 #341
[mk-app] #4450 >= #4447 #341
[inst-discovered] theory-solving 0 arith# ; #4450
[mk-app] #4451 = #4450 #4449
[instance] 0 #4451
[attach-enode] #4451 0
[end-of-instance]
[mk-app] #4447 or #4360 #4366 #4442
[inst-discovered] theory-solving 0 basic# ; #4447
[mk-app] #4450 = #4447 #4447
[instance] 0 #4450
[attach-enode] #4450 0
[end-of-instance]
[mk-app] #4450 not #4447
[mk-app] #4451 or #4439 #4449 #4450
[inst-discovered] theory-solving 0 basic# ; #4451
[mk-app] #4452 = #4451 #4451
[instance] 0 #4452
[attach-enode] #4452 0
[end-of-instance]
[mk-app] #4452 not #4451
[mk-app] #4453 not #2956
[mk-app] #4454 or #4453 #4452
[instance] 0x61d50e6c0fc8 ; 1
[attach-enode] #4435 1
[attach-enode] #4436 1
[attach-enode] #4437 1
[attach-enode] #4448 1
[attach-enode] #4442 1
[attach-meaning] #370 arith (- 1)
[mk-app] #4455 + #3639 #4436
[mk-app] #4456 <= #4455 #341
[mk-app] #4457 >= #4455 #341
[attach-enode] #4455 1
[assign] (not #4451) justification -1: 58
[end-of-instance]
[assign] #4357 clause 773 774
[assign] (not #4423) clause -777 778
[assign] #4438 clause 781 787
[assign] (not #4449) clause -782 787
[assign] #4447 clause 786 787
[assign] #4406 clause 775 777
[assign] #4421 clause 776 777
[assign] #4442 clause 783 -786
[assign] #4456 clause 784 -783
[assign] #4457 clause 785 -783
[new-match] 0x61d50e6c1c60 #1178 #1174 #658 ; #4406 (#1168 #1168)
[eq-expl] #652 lit #4433 ; #4432
[eq-expl] #4432 root
[new-match] 0x61d50e6c1c90 #1181 #1180 #652 ; #4406 (#1168 #1168) (#658 #658)
[new-match] 0x61d50e6c1cc0 #3310 #1690 #672 #1290 #1167 #125 ; #4420
[new-match] 0x61d50e6c1d08 #1810 #1807 #658 #1168 #125 ; #459
[mk-app] #4458 has_type #4378 #1188
[mk-app] #4459 not #4458
[mk-app] #4460 has_type #672 #1167
[mk-app] #4461 not #4460
[mk-app] #4462 vstd!seq.Seq.push.? #125 #1167 #4378 #672
[mk-app] #4463 has_type #4462 #1188
[mk-app] #4464 or #4459 #4461 #4463
[inst-discovered] theory-solving 0 basic# ; #4464
[mk-app] #4465 = #4464 #4464
[instance] 0 #4465
[attach-enode] #4465 0
[end-of-instance]
[mk-app] #4465 not #3310
[mk-app] #4466 or #4465 #4459 #4461 #4463
[instance] 0x61d50e6c1cc0 ; 2
[attach-enode] #4458 2
[attach-enode] #4460 2
[attach-enode] #4462 2
[attach-enode] #4463 2
[end-of-instance]
[mk-app] #4467 has_type #459 #3850
[mk-app] #4468 or #4389 #4467
[mk-app] #4469 or #3853 #4389 #4467
[instance] 0x61d50e6c1d08 ; 1
[attach-enode] #4467 1
[assign] #4467 justification -1: 332 775
[end-of-instance]
[assign] #4458 justification -1: 561 762
[assign] #4463 justification -1: 791 591 776 762
[eq-expl] #459 lit #4421 ; #4420
[eq-expl] #4420 root
[eq-expl] #3850 lit #3932 ; #1188
[new-match] 0x61d50e6c2168 #1198 #1194 #459 ; #4467 (#3850 #1188)
[mk-app] #4470 has_type #4420 #1188
[mk-app] #4471 not #4470
[mk-app] #4472 %Poly%vstd!seq.Seq<i32.>. #4420
[mk-app] #4473 Poly%vstd!seq.Seq<i32.>. #4472
[mk-app] #4474 = #4420 #4473
[mk-app] #4475 or #4471 #4474
[mk-app] #4476 or #3939 #4471 #4474
[instance] 0x61d50e6c2168 ; 2
[attach-enode] #4470 2
[attach-enode] #4472 2
[attach-enode] #4473 2
[attach-enode] #4474 2
[end-of-instance]
[assign] #4470 justification -1: 791 591 776
[assign] #4474 clause 793 -792
[eq-expl] #4472 root
[new-match] 0x61d50e6c2458 #1187 #1186 #4472 ; #4473
[eq-expl] #4473 lit #4474 ; #4420
[new-match] 0x61d50e6c2488 #1201 #1200 #4472 ; #4467 (#3850 #1188) (#459 #4473)
[decide-and-or] #3687 #3659
[push] 5
[assign] (not #3703) decision axiom
[assign] #674 clause 505 507
[assign] (not #3704) clause -506 507
[assign] #3705 clause 530 506
[eq-expl] #274 lit #276 ; #275
[new-match] 0x61d50e6c24d0 #1086 #449 #1025 #274 ; #3704
[mk-app] #4477 >= #1025 #341
[mk-app] #4478 not #4477
[mk-app] #4479 * #370 #288
[mk-app] #4480 + #1025 #4479
[mk-app] #4481 >= #4480 #341
[mk-app] #4482 or #4478 #4481
[mk-app] #4483 uInv #275 #1025
[mk-app] #4484 = #4482 #4483
[mk-app] #4485 not #4484
[mk-app] #4486 + #4479 #1025
[inst-discovered] theory-solving 0 arith# ; #4480
[mk-app] #4487 = #4480 #4486
[instance] 0 #4487
[attach-enode] #4487 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4487 + #288 #4202
[mk-app] #4488 <= #4487 #341
[mk-app] #4489 >= #4486 #341
[inst-discovered] theory-solving 0 arith# ; #4489
[mk-app] #4490 = #4489 #4488
[instance] 0 #4490
[attach-enode] #4490 0
[end-of-instance]
[mk-app] #4486 or #4478 #4488
[mk-app] #4489 = #4486 #4483
[mk-app] #4490 not #4486
[mk-app] #4491 not #4489
[inst-discovered] theory-solving 0 basic# ; #4491
[mk-app] #4490 = #4491 #4491
[instance] 0 #4490
[attach-enode] #4490 0
[end-of-instance]
[mk-app] #4490 or #3800 #4491
[instance] 0x61d50e6c24d0 ; 1
[attach-enode] #4202 1
[attach-enode] #4487 1
[attach-enode] #4483 1
[assign] (not #4489) justification -1: 60
[end-of-instance]
[assign] (not #4483) justification -1: -506 39
[mk-app] #4492 <= #288 #289
[mk-app] #4493 >= #288 #289
[assign] #4492 justification -1: 43
[assign] #4493 justification -1: 43
[assign] #4486 clause 796 797 798
[decide-and-or] #3942 #3969
[push] 6
[assign] (not #3968) decision axiom
[mk-app] #4494 <= #3967 #341
[mk-app] #4495 >= #3967 #341
[push] 7
[assign] (not #3975) decision axiom
[assign] (not #3980) clause -630 629
[push] 8
[assign] (not #4382) decision axiom
[assign] #4403 clause 766 764
[assign] (not #4494) clause -801 764 -647 -661 -670 -577 -477
[assign] #4495 clause 802 -662 -647 -661 -577 -670 -477
[assign] #4160 clause 684 -702 -577 -583 -647 -477 -573 -704 -707
[assign] #4477 clause 794 -662 -661 -584 -670 -705 -574
[assign] (not #4411) clause -769 -662 -584 -670 -696 -705 -574
[assign] #4156 clause 683 -662 -584 -670 -574 -701 -705 -706
[eq-expl] #4393 root
[eq-expl] #4396 root
[new-match] 0x61d50e6cab50 #552 #550 #4396 #4393 ; #4397
[eq-expl] #4080 lit #4100 ; #341
[new-match] 0x61d50e6cab88 #552 #550 #296 #4080 ; #4394
[eq-expl] #4395 root
[new-match] 0x61d50e6cabc0 #2320 #2319 #2351 #1276 #4395 #3934 ; #4396
[eq-expl] #4394 root
[new-match] 0x61d50e6cac08 #174 #173 #4394 ; #4395
[new-match] 0x61d50e6cac38 #3286 #1569 #2574 #3934 #1167 #125 ; #4392
[mk-app] #4496 * #370 #4397
[mk-app] #4497 + #4396 #4393 #4496
[mk-app] #4498 = #4497 #341
[mk-app] #4499 + #4393 #4396 #4496
[inst-discovered] theory-solving 0 arith# ; #4497
[mk-app] #4500 = #4497 #4499
[instance] 0 #4500
[attach-enode] #4500 0
[end-of-instance]
[mk-app] #4500 = #4499 #341
[mk-app] #4501 or #3887 #4500
[instance] 0x61d50e6cab50 ; 4
[attach-enode] #4496 4
[attach-enode] #4499 4
[attach-enode] #4500 4
[mk-app] #4502 <= #4499 #341
[mk-app] #4503 >= #4499 #341
[assign] #4500 justification -1: 77
[end-of-instance]
[mk-app] #4504 Add #341 #296
[mk-app] #4505 * #370 #4504
[mk-app] #4506 + #296 #341 #4505
[mk-app] #4507 = #4506 #341
[mk-app] #4508 + #296 #4505
[inst-discovered] theory-solving 0 arith# ; #4506
[mk-app] #4509 = #4506 #4508
[instance] 0 #4509
[attach-enode] #4509 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4509 = #4504 #296
[mk-app] #4510 = #4508 #341
[inst-discovered] theory-solving 0 arith# ; #4510
[mk-app] #4511 = #4510 #4509
[instance] 0 #4511
[attach-enode] #4511 0
[end-of-instance]
[mk-app] #4508 or #3887 #4509
[instance] 0x61d50e6cab88 ; 4
[attach-enode] #4504 4
[attach-enode] #4509 4
[assign] #4509 justification -1: 77
[end-of-instance]
[mk-app] #4510 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #4395 #1276 #2316
[mk-app] #4511 = #4396 #4510
[mk-app] #4512 or #4381 #4511
[instance] 0x61d50e6cabc0 ; 4
[attach-enode] #4510 4
[attach-enode] #4511 4
[assign] #4511 justification -1: 414
[end-of-instance]
[mk-app] #4513 %I #4395
[mk-app] #4514 = #4394 #4513
[mk-app] #4515 or #3871 #4514
[instance] 0x61d50e6cac08 ; 4
[attach-enode] #4513 4
[attach-enode] #4514 4
[assign] #4514 justification -1: 26
[end-of-instance]
[mk-app] #4516 has_type #4392 #1167
[mk-app] #4517 or #3966 #4072 #4516
[inst-discovered] theory-solving 0 basic# ; #4517
[mk-app] #4518 = #4517 #4517
[instance] 0 #4518
[attach-enode] #4518 0
[end-of-instance]
[mk-app] #4518 or #4200 #3966 #4072 #4516
[instance] 0x61d50e6cac38 ; 4
[attach-enode] #4516 4
[assign] #4516 justification -1: 305 626 658
[end-of-instance]
[assign] #4488 clause 795 -794 -796
[assign] #4405 clause 771 769
[assign] #4502 clause 804 -803
[assign] #4503 clause 805 -803
[mk-app] #4519 = #2899 #4397
[attach-meaning] #370 arith (- 1)
[mk-app] #4520 + #2899 #4496
[mk-app] #4521 <= #4520 #341
[mk-app] #4522 >= #4520 #341
[assign] #4519 justification -1: 766 767 660 657 592
[attach-enode] #4519 0
[attach-enode] #4520 0
[assign] #4521 justification -1: 810
[assign] #4522 justification -1: 810
[resolve-process] true
[resolve-lit] 7 (not #4054)
[resolve-lit] 3 (not #4493)
[resolve-lit] 0 (not #4488)
[resolve-lit] 6 (not #4214)
[conflict] (not #4488) (not #4054)
[pop] 7 9
[attach-enode] #4168 0
[attach-enode] #4183 0
[attach-enode] #4184 0
[attach-enode] #4193 0
[attach-enode] #4212 0
[attach-enode] #4213 0
[attach-enode] #4188 0
[attach-enode] #4248 0
[attach-enode] #4249 0
[attach-enode] #4250 0
[attach-enode] #4251 0
[attach-enode] #4361 0
[attach-enode] #4362 0
[attach-enode] #4114 0
[attach-enode] #4115 0
[attach-enode] #4116 0
[attach-enode] #4117 0
[attach-enode] #4223 0
[attach-enode] #4224 0
[attach-enode] #4365 0
[attach-enode] #4118 0
[attach-enode] #4119 0
[attach-enode] #4120 0
[attach-enode] #4121 0
[attach-enode] #4298 0
[attach-enode] #4299 0
[attach-enode] #4139 0
[attach-enode] #4154 0
[attach-enode] #4159 0
[attach-enode] #4206 0
[attach-enode] #4207 0
[attach-enode] #4216 0
[attach-enode] #4155 0
[assign] (not #4252) axiom
[assign] #3855 axiom
[assign] #4071 axiom
[assign] #4073 axiom
[assign] #4085 axiom
[assign] #4088 axiom
[attach-enode] #4161 0
[assign] #4161 axiom
[assign] (not #4364) axiom
[attach-enode] #4202 0
[attach-enode] #4487 0
[assign] (not #4488) clause -706 -650
[assign] #728 clause 497 681
[assign] #3857 clause 565 -564
[assign] #4076 clause 660 -659 -658
[assign] #4495 clause 696 -662 -647 -661 -577 -670 -477
[assign] (not #4094) clause -666 -663 -661 -658 -659 -662
[assign] #4363 clause 682 691
[assign] (not #4366) clause -690 691
[assign] #3647 clause 498 -497
[assign] #4090 clause 664 666
[assign] #4092 clause 665 666
[assign] #3844 justification -1: 565 591
[mk-app] #4128 = #1025 #1273
[attach-meaning] #370 arith (- 1)
[assign] #4128 justification -1: 480
[attach-enode] #4128 0
[assign] #4214 justification -1: 707
[assign] #4215 justification -1: 707
[mk-app] #4129 <= #288 #289
[mk-app] #4131 >= #288 #289
[assign] #4129 justification -1: 43
[assign] #4131 justification -1: 43
[mk-app] #4132 = #3639 #4114
[attach-meaning] #370 arith (- 1)
[mk-app] #4205 * #370 #4114
[mk-app] #4211 + #3639 #4205
[mk-app] #4219 <= #4211 #341
[mk-app] #4220 >= #4211 #341
[assign] #4132 justification -1: 592
[attach-enode] #4132 0
[attach-enode] #4205 0
[attach-enode] #4211 0
[assign] #4219 justification -1: 710
[assign] #4220 justification -1: 710
[eq-expl] #1290 root
[new-match] 0x61d50e67aa40 #1198 #1194 #1290 ; #3844 (#1188 #1188)
[new-match] 0x61d50e67aa70 #2320 #2319 #2352 #1276 #2574 #3934 ; #4075
[new-match] 0x61d50e67aab8 #3437 #2337 #2351 #1276 #2574 #3934 ; #4075 (#2352 #2352)
[mk-app] #4221 or #3939 #3845 #4379
[instance] 0x61d50e67aa40 ; 2
[attach-enode] #4377 2
[attach-enode] #4378 2
[attach-enode] #4379 2
[assign] #4379 justification -1: 257 561
[end-of-instance]
[mk-app] #4222 not #2320
[mk-app] #4236 or #4222 #4387
[instance] 0x61d50e67aa70 ; 3
[attach-enode] #2316 3
[attach-enode] #4386 3
[attach-enode] #4387 3
[assign] #4387 justification -1: 414
[end-of-instance]
[mk-app] #4237 + #4103 #4080
[inst-discovered] theory-solving 0 arith# ; #4390
[mk-app] #4238 = #4390 #4237
[instance] 0 #4238
[attach-enode] #4238 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4238 >= #4237 #341
[inst-discovered] theory-solving 0 arith# ; #4238
[mk-app] #4239 = #4238 #4382
[instance] 0 #4239
[attach-enode] #4239 0
[end-of-instance]
[mk-app] #4237 if #4382 #341 #4397
[mk-app] #4238 = #4075 #4237
[mk-app] #4239 or #3966 #4072 #4074 #4238
[inst-discovered] theory-solving 0 basic# ; #4239
[mk-app] #4240 = #4239 #4239
[instance] 0 #4240
[attach-enode] #4240 0
[end-of-instance]
[mk-app] #4240 not #3437
[mk-app] #4241 or #4240 #3966 #4072 #4074 #4238
[instance] 0x61d50e67aab8 ; 3
[mk-app] #4242 = #4237 #341
[mk-app] #4243 = #4237 #4397
[attach-enode] #4237 3
[attach-enode] #4392 3
[attach-enode] #4393 3
[attach-enode] #4394 3
[attach-enode] #4395 3
[attach-enode] #4396 3
[attach-enode] #4397 3
[attach-enode] #4242 3
[attach-enode] #4243 3
[attach-enode] #4238 3
[assign] #4238 justification -1: 415 626 658 659
[end-of-instance]
[assign] #4477 clause 700 -677 -661 -662 -670 -584 -574
[eq-expl] #4377 root
[new-match] 0x61d50e67b520 #1187 #1186 #4377 ; #4378
[eq-expl] #2316 root
[new-match] 0x61d50e67b550 #2320 #2319 #2316 #1276 #2574 #3934 ; #4386
[eq-expl] #1290 lit #4379 ; #4378
[eq-expl] #4378 root
[new-match] 0x61d50e67b598 #1201 #1200 #4377 ; #3844 (#1188 #1188) (#1290 #4378)
[new-match] 0x61d50e67b5c8 #3440 #2356 #3638 #2574 #1396 ; #3639
[new-match] 0x61d50e67b608 #4034 #4023 #3638 #2574 ; #3639 (#1396 #3934)
[new-match] 0x61d50e67b640 #174 #173 #1025 ; #3638
[inst-discovered] theory-solving 0 basic# ; #4166
[mk-app] #4239 = #4166 #4166
[instance] 0 #4239
[attach-enode] #4239 0
[end-of-instance]
[mk-app] #4239 or #4078 #3966 #4072 #4163 #4165
[instance] 0x61d50e67b5c8 ; 1
[attach-enode] #4164 1
[attach-enode] #4165 1
[assign] #4165 justification -1: 417 626 658 705
[end-of-instance]
[mk-app] #4244 + #4001 #4168
[inst-discovered] theory-solving 0 arith# ; #4172
[mk-app] #4245 = #4172 #4244
[instance] 0 #4245
[attach-enode] #4245 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4245 <= #4244 #341
[inst-discovered] theory-solving 0 arith# ; #4245
[mk-app] #4246 = #4245 #4189
[instance] 0 #4246
[attach-enode] #4246 0
[end-of-instance]
[mk-app] #4244 not #4189
[inst-discovered] theory-solving 0 basic# ; #4179
[mk-app] #4245 = #4179 #4179
[instance] 0 #4245
[attach-enode] #4245 0
[end-of-instance]
[mk-app] #4245 or #4072 #4163 #4082 #4186 #4244 #4180
[mk-app] #4246 or #4097 #4072 #4163 #4082 #4186 #4244 #4180
[instance] 0x61d50e67b608 ; 2
[end-of-instance]
[mk-app] #4245 or #3871 #4190
[instance] 0x61d50e67b640 ; 1
[attach-enode] #4190 1
[attach-meaning] #370 arith (- 1)
[assign] #4190 justification -1: 26
[end-of-instance]
[assign] #4194 clause 676 -719
[assign] #4195 clause 679 -719
[assign] #4185 clause 675 -676 -584 -668 -677 -574 -476
[assign] (not #4411) clause -701 -676 -662 -670 -677 -584 -574
[assign] #4189 clause 678 -679 -577 -583 -647 -680 -573 484
[assign] (not #4179) clause -689 -678 -675
[assign] #4175 clause 683 689
[assign] #4177 clause 688 689
[new-match] 0x61d50e67ba78 #2320 #2319 #2352 #3638 #2574 #3934 ; #4164
[new-match] 0x61d50e67bac0 #3437 #2337 #2351 #3638 #2574 #3934 ; #4164 (#2352 #2352)
[mk-app] #4281 or #4222 #4409
[instance] 0x61d50e67ba78 ; 3
[attach-enode] #4404 3
[attach-enode] #4409 3
[assign] #4409 justification -1: 414
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4416
[mk-app] #4282 = #4416 #4416
[instance] 0 #4282
[attach-enode] #4282 0
[end-of-instance]
[mk-app] #4282 or #4240 #3966 #4072 #4163 #4415
[instance] 0x61d50e67bac0 ; 3
[mk-app] #4283 = #4414 #341
[mk-app] #4292 = #4413 #4414
[attach-enode] #4414 3
[attach-enode] #4412 3
[attach-enode] #4413 3
[attach-enode] #4283 3
[attach-enode] #4292 3
[assign] #4292 justification -1: -701
[attach-enode] #4415 3
[assign] #4415 justification -1: 415 626 658 705
[end-of-instance]
[new-match] 0x61d50e60e8c8 #2320 #2319 #2316 #3638 #2574 #3934 ; #4404
[eq-expl] #4395 root
[new-match] 0x61d50e60e910 #2320 #2319 #2351 #3638 #4395 #3934 ; #4412
[eq-expl] #4393 root
[eq-expl] #4412 root
[new-match] 0x61d50e60e958 #552 #550 #4412 #4393 ; #4413
[new-match] 0x61d50e60e990 #552 #550 #296 #4080 ; #4394
[eq-expl] #4394 root
[new-match] 0x61d50e60e9c8 #174 #173 #4394 ; #4395
[new-match] 0x61d50e60e9f8 #3286 #1569 #2574 #3934 #1167 #125 ; #4392
[mk-app] #4293 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #4395 #3638 #2316
[mk-app] #4294 = #4412 #4293
[mk-app] #4295 or #4222 #4294
[instance] 0x61d50e60e910 ; 4
[attach-enode] #4293 4
[attach-enode] #4294 4
[assign] #4294 justification -1: 414
[end-of-instance]
[mk-app] #4279 * #370 #4413
[mk-app] #4271 + #4412 #4393 #4279
[mk-app] #4272 = #4271 #341
[mk-app] #4262 + #4393 #4412 #4279
[inst-discovered] theory-solving 0 arith# ; #4271
[mk-app] #4263 = #4271 #4262
[instance] 0 #4263
[attach-enode] #4263 0
[end-of-instance]
[mk-app] #4263 = #4262 #341
[mk-app] #4345 or #3887 #4263
[instance] 0x61d50e60e958 ; 4
[attach-enode] #4279 4
[attach-enode] #4262 4
[attach-enode] #4263 4
[mk-app] #4348 <= #4262 #341
[mk-app] #4367 >= #4262 #341
[assign] #4263 justification -1: 77
[end-of-instance]
[mk-app] #4370 + #296 #4505
[inst-discovered] theory-solving 0 arith# ; #4506
[mk-app] #4371 = #4506 #4370
[instance] 0 #4371
[attach-enode] #4371 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4371 = #4504 #296
[mk-app] #4372 = #4370 #341
[inst-discovered] theory-solving 0 arith# ; #4372
[mk-app] #4373 = #4372 #4371
[instance] 0 #4373
[attach-enode] #4373 0
[end-of-instance]
[mk-app] #4370 or #3887 #4371
[instance] 0x61d50e60e990 ; 4
[attach-enode] #4504 4
[attach-enode] #4371 4
[assign] #4371 justification -1: 77
[end-of-instance]
[mk-app] #4372 or #3871 #4514
[instance] 0x61d50e60e9c8 ; 4
[attach-enode] #4513 4
[attach-enode] #4514 4
[assign] #4514 justification -1: 26
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4517
[mk-app] #4373 = #4517 #4517
[instance] 0 #4373
[attach-enode] #4373 0
[end-of-instance]
[mk-app] #4373 not #3286
[mk-app] #4368 or #4373 #3966 #4072 #4516
[instance] 0x61d50e60e9f8 ; 4
[attach-enode] #4516 4
[assign] #4516 justification -1: 305 626 658
[end-of-instance]
[assign] #4348 clause 726 -725
[assign] #4367 clause 727 -725
[mk-app] #4408 = #3639 #4413
[attach-meaning] #370 arith (- 1)
[mk-app] #4403 + #3639 #4279
[mk-app] #4383 <= #4403 #341
[mk-app] #4419 >= #4403 #341
[assign] #4408 justification -1: 722 723 718 592
[attach-enode] #4408 0
[attach-enode] #4403 0
[assign] #4383 justification -1: 731
[assign] #4419 justification -1: 731
[new-match] 0x61d50e60f408 #2320 #2319 #2316 #3638 #4395 #3934 ; #4293
[eq-expl] #4392 root
[new-match] 0x61d50e60f450 #542 #236 #4392 #275 ; #4516 (#1167 #1167)
[new-match] 0x61d50e60f488 #240 #236 #4392 #275 ; #4516 (#1167 #1167)
[mk-app] #4405 not #4516
[mk-app] #4417 iInv #275 #4393
[mk-app] #4448 or #4405 #4417
[mk-app] #4449 not #542
[mk-app] #4455 or #4449 #4405 #4417
[instance] 0x61d50e60f450 ; 5
[attach-enode] #4417 5
[assign] #4417 justification -1: 75 730
[end-of-instance]
[mk-app] #4456 I #4393
[mk-app] #4457 = #4392 #4456
[mk-app] #4447 or #4405 #4457
[mk-app] #4450 not #240
[mk-app] #4451 or #4450 #4405 #4457
[instance] 0x61d50e60f488 ; 5
[attach-enode] #4456 5
[attach-enode] #4457 5
[assign] #4457 justification -1: 34 730
[end-of-instance]
[new-match] 0x61d50e60f758 #1136 #455 #4393 #275 ; #4417
[new-match] 0x61d50e60f790 #174 #173 #4393 ; #4456
[eq-expl] #4392 lit #4457 ; #4456
[eq-expl] #4456 root
[new-match] 0x61d50e60f7c0 #503 #499 #4393 #275 ; #4516 (#4392 #4456) (#1167 #1167)
[mk-app] #4486 + #4393 #3817
[mk-app] #4489 >= #4486 #341
[mk-app] #4492 not #4489
[mk-app] #4493 + #4393 #3821
[mk-app] #4499 >= #4493 #341
[mk-app] #4500 or #4492 #4499
[mk-app] #4502 = #4500 #4417
[mk-app] #4503 not #4502
[mk-app] #4509 + #3817 #4393
[inst-discovered] theory-solving 0 arith# ; #4486
[mk-app] #4519 = #4486 #4509
[instance] 0 #4519
[attach-enode] #4519 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4519 * #370 #4393
[mk-app] #4520 + #313 #4519
[mk-app] #4521 <= #4520 #341
[mk-app] #4522 >= #4509 #341
[inst-discovered] theory-solving 0 arith# ; #4522
[mk-app] #4200 = #4522 #4521
[instance] 0 #4200
[attach-enode] #4200 0
[end-of-instance]
[mk-app] #4509 not #4521
[mk-app] #4522 + #3821 #4393
[inst-discovered] theory-solving 0 arith# ; #4493
[mk-app] #4200 = #4493 #4522
[instance] 0 #4200
[attach-enode] #4200 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4200 + #335 #4519
[mk-app] #4518 <= #4200 #341
[mk-app] #4515 >= #4522 #341
[inst-discovered] theory-solving 0 arith# ; #4515
[mk-app] #4381 = #4515 #4518
[instance] 0 #4381
[attach-enode] #4381 0
[end-of-instance]
[mk-app] #4522 or #4509 #4518
[mk-app] #4515 = #4522 #4417
[mk-app] #4381 not #4522
[mk-app] #4512 not #4515
[inst-discovered] theory-solving 0 basic# ; #4512
[mk-app] #4381 = #4512 #4512
[instance] 0 #4381
[attach-enode] #4381 0
[end-of-instance]
[mk-app] #4381 or #3836 #4512
[instance] 0x61d50e60f758 ; 6
[attach-enode] #4519 6
[attach-enode] #4520 6
[attach-enode] #4200 6
[assign] (not #4515) justification -1: 61
[end-of-instance]
[assign] (not #4522) clause -738 739
[assign] #4521 clause 736 738
[assign] (not #4518) clause -737 738
[decide-and-or] #3767 #778
[push] 2
[assign] #778 decision axiom
[new-match] 0x61d50e60fab8 #2724 #2723 #1025 #341 #2892 ; #778
[mk-app] #4508 not #2724
[mk-app] #4501 or #4508 #4122
[instance] 0x61d50e60fab8 ; 1
[attach-enode] #4113 1
[assign] #4122 justification -1: 460
[end-of-instance]
[assign] #4113 justification -1: 487 485
[eq-expl] #4117 root
[eq-expl] #4119 root
[new-match] 0x61d50e60fc08 #552 #550 #4119 #4117 ; #4120
[eq-expl] #4116 root
[new-match] 0x61d50e60fc40 #3440 #2356 #4116 #2574 #3934 ; #4117
[new-match] 0x61d50e60fc80 #4034 #4023 #4116 #2574 ; #4117 (#3934 #3934)
[new-match] 0x61d50e60fcb8 #3286 #1569 #4116 #3934 #1167 #125 ; #4118
[eq-expl] #4115 root
[new-match] 0x61d50e60fd00 #174 #173 #4115 ; #4116
[new-match] 0x61d50e60fd30 #567 #559 #296 #1025 ; #4115
[mk-app] #4491 + #4117 #4119 #4125
[inst-discovered] theory-solving 0 arith# ; #4126
[mk-app] #4490 = #4126 #4491
[instance] 0 #4490
[attach-enode] #4490 0
[end-of-instance]
[mk-app] #4490 = #4491 #341
[mk-app] #4476 or #3887 #4490
[instance] 0x61d50e60fc08 ; 2
[attach-enode] #4125 2
[attach-enode] #4491 2
[attach-enode] #4490 2
[mk-app] #4469 <= #4491 #341
[mk-app] #4465 >= #4491 #341
[assign] #4490 justification -1: 77
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4137
[mk-app] #4466 = #4137 #4137
[instance] 0 #4466
[attach-enode] #4466 0
[end-of-instance]
[mk-app] #4466 or #4078 #3966 #4072 #4134 #4136
[instance] 0x61d50e60fc40 ; 3
[attach-enode] #4133 3
[attach-enode] #4135 3
[attach-enode] #4136 3
[end-of-instance]
[mk-app] #4452 + #4001 #4139
[inst-discovered] theory-solving 0 arith# ; #4143
[mk-app] #4453 = #4143 #4452
[instance] 0 #4453
[attach-enode] #4453 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4453 <= #4452 #341
[inst-discovered] theory-solving 0 arith# ; #4453
[mk-app] #4454 = #4453 #4160
[instance] 0 #4454
[attach-enode] #4454 0
[end-of-instance]
[mk-app] #4452 not #4160
[inst-discovered] theory-solving 0 basic# ; #4150
[mk-app] #4453 = #4150 #4150
[instance] 0 #4453
[attach-enode] #4453 0
[end-of-instance]
[mk-app] #4453 or #4072 #4134 #4082 #4157 #4452 #4151
[inst-discovered] theory-solving 0 basic# ; #4453
[mk-app] #4454 = #4453 #4453
[instance] 0 #4454
[attach-enode] #4454 0
[end-of-instance]
[mk-app] #4454 or #4097 #4072 #4134 #4082 #4157 #4452 #4151
[instance] 0x61d50e60fc80 ; 2
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4199
[mk-app] #4453 = #4199 #4199
[instance] 0 #4453
[attach-enode] #4453 0
[end-of-instance]
[mk-app] #4453 or #4373 #3966 #4134 #4198
[instance] 0x61d50e60fcb8 ; 3
[attach-enode] #4198 3
[end-of-instance]
[mk-app] #4434 or #3871 #4196
[instance] 0x61d50e60fd00 ; 2
[attach-enode] #4196 2
[attach-meaning] #370 arith (- 1)
[assign] #4196 justification -1: 26
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4431 + #4202 #4115
[attach-meaning] #370 arith (- 1)
[mk-app] #4431 = #4207 #296
[inst-discovered] theory-solving 0 arith# ; #4204
[mk-app] #4426 = #4204 #4431
[instance] 0 #4426
[attach-enode] #4426 0
[end-of-instance]
[mk-app] #4426 or #3877 #4431
[instance] 0x61d50e60fd30 ; 2
[attach-enode] #4431 2
[assign] #4431 justification -1: 78
[end-of-instance]
[assign] #4121 clause 692 -740 -741
[assign] #4469 clause 743 -742
[assign] #4465 clause 744 -742
[assign] #4217 clause 704 -749
[assign] #4218 clause 699 -749
[assign] #4209 clause 703 -750
[assign] #4210 clause 698 -750
[assign] #4156 clause 702 -704 -662 -670 -677 -703 -584 -574
[assign] #4160 clause 697 -699 -577 -583 -647 -680 -698 -573 -477
[attach-meaning] #370 arith (- 1)
[mk-app] #4427 + #4114 #4125
[mk-app] #4388 <= #4427 #341
[mk-app] #4410 >= #4427 #341
[attach-enode] #4427 0
[assign] #4388 justification -1: 692
[assign] #4410 justification -1: 692
[assign] #4133 justification -1: 659 568 749 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[assign] #4136 justification -1: 660 568 749 592 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[assign] #3630 justification -1: 692 592 568 749 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[mk-app] #4401 = #4067 #4117
[attach-meaning] #370 arith (- 1)
[assign] #4401 justification -1: 568 749 592 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[attach-enode] #4401 0
[assign] #4225 justification -1: 753
[assign] #4226 justification -1: 753
[new-match] 0x61d50e6b5658 #3286 #1569 #1276 #1396 #1167 #125 ; #748
[new-match] 0x61d50e6b56a0 #552 #550 #749 #782 ; #750
[inst-discovered] theory-solving 0 basic# ; #4229
[mk-app] #4418 = #4229 #4229
[instance] 0 #4418
[attach-enode] #4418 0
[end-of-instance]
[mk-app] #4418 or #4373 #3966 #4074 #4228
[instance] 0x61d50e6b5658 ; 1
[attach-enode] #4227 1
[attach-enode] #4228 1
[assign] #4228 justification -1: 305 626 659
[end-of-instance]
[mk-app] #4402 or #3887 #4234
[instance] 0x61d50e6b56a0 ; 1
[attach-enode] #4231 1
[attach-enode] #4232 1
[attach-enode] #4233 1
[attach-enode] #4234 1
[mk-app] #4407 <= #4233 #341
[mk-app] #4369 >= #4233 #341
[assign] #4234 justification -1: 77
[end-of-instance]
[assign] #4198 clause 748 -745
[assign] (not #4150) clause -747 -745 -702 -697
[assign] #4146 clause 684 -685 -664
[assign] #4148 clause 686 -687 -665
[assign] #4407 clause 756 -755
[assign] #4369 clause 757 -755
[mk-app] #4374 = #749 #4119
[attach-meaning] #370 arith (- 1)
[mk-app] #4350 * #370 #4119
[mk-app] #4349 + #749 #4350
[mk-app] #4317 <= #4349 #341
[mk-app] #4332 >= #4349 #341
[assign] #4374 justification -1: 568 749 592 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[attach-enode] #4374 0
[attach-enode] #4350 0
[attach-enode] #4349 0
[assign] #4317 justification -1: 758
[assign] #4332 justification -1: 758
[mk-app] #4312 = #4120 #4231
[attach-meaning] #370 arith (- 1)
[mk-app] #4328 + #4120 #4232
[mk-app] #4322 <= #4328 #341
[mk-app] #4323 >= #4328 #341
[assign] #4312 justification -1: 568 749 592 657 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[attach-enode] #4312 0
[attach-enode] #4328 0
[assign] #4322 justification -1: 761
[assign] #4323 justification -1: 761
[eq-expl] #4115 lit #4196 ; #4139
[eq-expl] #4139 th arith ; #4084
[eq-expl] #4084 th arith ; #3869
[eq-expl] #4116 cg (#4115 #1454) ; #1276
[eq-expl] #4118 cg (#125 #125) (#1167 #1167) (#3934 #1396) (#4116 #1276) ; #748
[new-match] 0x61d50e6b60b0 #542 #236 #4118 #275 ; #4198 (#1167 #1167)
[new-match] 0x61d50e6b60e8 #240 #236 #4118 #275 ; #4198 (#1167 #1167)
[mk-app] #4318 or #4449 #4325 #4326
[instance] 0x61d50e6b60b0 ; 4
[attach-enode] #4324 4
[attach-enode] #4326 4
[end-of-instance]
[mk-app] #4313 or #4450 #4325 #4330
[instance] 0x61d50e6b60e8 ; 4
[attach-enode] #4329 4
[attach-enode] #4330 4
[end-of-instance]
[assign] #4324 justification -1: 748 568 749 592 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[assign] #4326 clause 765 -764
[assign] #4330 clause 766 -764
[new-match] 0x61d50e6b65b0 #1136 #455 #749 #275 ; #4326
[new-match] 0x61d50e6b65e8 #174 #173 #749 ; #4329
[eq-expl] #4329 lit #4330 ; #748
[new-match] 0x61d50e6b6618 #503 #499 #749 #275 ; #4198 (#4118 #4329) (#1167 #1167)
[mk-app] #4305 or #4341 #4347
[mk-app] #4306 = #4305 #4326
[mk-app] #4256 not #4305
[mk-app] #4257 not #4306
[inst-discovered] theory-solving 0 basic# ; #4257
[mk-app] #4256 = #4257 #4257
[instance] 0 #4256
[attach-enode] #4256 0
[end-of-instance]
[mk-app] #4256 or #3836 #4257
[instance] 0x61d50e6b65b0 ; 5
[attach-enode] #4342 5
[attach-enode] #4343 5
[attach-enode] #4346 5
[assign] (not #4306) justification -1: 61
[end-of-instance]
[assign] (not #4305) clause -769 770
[assign] #4344 clause 767 769
[assign] (not #4347) clause -768 769
[decide-and-or] #3660 #3706
[push] 3
[assign] #3706 decision axiom
[assign] #779 clause 490 -492 -491
[assign] #3645 clause 494 -490
[decide-and-or] #3770 #3689
[push] 4
[assign] #3689 decision axiom
[assign] (not #3714) clause -536 -537 -494
[assign] #696 clause 495 536
[assign] #3712 clause 535 536
[assign] (not #3708) clause -534 -535 -498
[assign] #721 clause 499 534
[assign] #3674 clause 533 534
[eq-expl] #720 root
[new-match] 0x61d50e6b6928 #3408 #2204 #720 #1276 #1533 #1167 #125 #1534 #125 ; #721
[mk-app] #4289 not #4301
[inst-discovered] theory-solving 0 basic# ; #4304
[mk-app] #4289 = #4304 #4304
[instance] 0 #4289
[attach-enode] #4289 0
[end-of-instance]
[mk-app] #4289 not #3408
[mk-app] #4290 or #4289 #4304
[instance] 0x61d50e6b6928 ; 1
[attach-enode] #4296 1
[attach-enode] #4302 1
[assign] (not #4303) justification -1: 397
[end-of-instance]
[assign] #4302 justification -1: 499 539
[assign] (not #4301) clause -772 -773 774
[assign] #4296 clause 771 772
[assign] #4299 clause 693 772
[assign] #706 clause 501 -693 691 -495 -657 -692
[assign] #3649 clause 502 -501
[assign] (not #3682) clause -532 -502 -533
[assign] #687 clause 503 532
[assign] #673 clause 504 532
[assign] #3687 clause 531 532
[eq-expl] #720 lit #4299 ; #4298
[eq-expl] #4298 root
[new-match] 0x61d50e6b6bb8 #542 #236 #720 #275 ; #4296 (#1167 #1167)
[new-match] 0x61d50e6b6bf0 #240 #236 #720 #275 ; #4296 (#1167 #1167)
[new-match] 0x61d50e6b6c28 #3402 #2179 #1276 #3765 #1167 #125 #1534 #125 ; #4298
[new-match] 0x61d50e6b6c80 #2269 #2265 #1276 #3765 #1167 #125 ; #4298 (#125 #125) (#1534 #1534)
[new-match] 0x61d50e6b6cc8 #3390 #2134 #672 #658 #1289 #1147 #125 #1167 #125 ; #673
[eq-expl] #705 root
[new-match] 0x61d50e6b6d28 #1136 #455 #705 #275 ; #706
[eq-expl] #698 lit #687 ; #686
[eq-expl] #686 root
[new-match] 0x61d50e6b6d60 #174 #173 #698 ; #672
[eq-expl] #652 root
[new-match] 0x61d50e6c0800 #1166 #1165 #652 ; #658
[new-match] 0x61d50e6c0830 #2956 #392 #705 #275 ; #686
[eq-expl] #741 lit #696 ; #1462
[eq-expl] #713 root
[new-match] 0x61d50e6c0868 #552 #550 #713 #741 ; #705
[mk-app] #4291 or #4449 #4308 #4310
[instance] 0x61d50e6b6bb8 ; 2
[attach-enode] #4307 2
[attach-enode] #4309 2
[attach-enode] #4310 2
[end-of-instance]
[mk-app] #4275 or #4450 #4308 #4315
[instance] 0x61d50e6b6bf0 ; 2
[attach-enode] #4314 2
[attach-enode] #4315 2
[end-of-instance]
[mk-app] #4280 not #2269
[mk-app] #4273 or #4280 #3930 #4320
[instance] 0x61d50e6b6c80 ; 2
[attach-enode] #4264 2
[mk-app] #4274 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #4264
[mk-app] #4235 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS #4274
[attach-enode] #4274 2
[attach-enode] #4235 2
[mk-app] #4230 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #4235
[mk-app] #4208 = #4274 #4230
[new-match] 0 datatype#21 datatype#18 #4274 ; #4235
[instance] 0 #4208
[attach-enode] #4230 2
[end-of-instance]
[attach-enode] #4265 2
[attach-enode] #4266 2
[attach-enode] #4319 2
[attach-enode] #4320 2
[assign] #4320 justification -1: 18 404
[end-of-instance]
[mk-app] #4208 not #4423
[inst-discovered] theory-solving 0 basic# ; #4425
[mk-app] #4208 = #4425 #4425
[instance] 0 #4208
[attach-enode] #4208 0
[end-of-instance]
[mk-app] #4208 not #3390
[mk-app] #4201 or #4208 #4425
[instance] 0x61d50e6b6cc8 ; 1
[attach-enode] #4406 1
[attach-enode] #4420 1
[attach-enode] #4421 1
[assign] (not #4424) justification -1: 389
[end-of-instance]
[mk-app] #4197 + #705 #3817
[mk-app] #4192 >= #4197 #341
[mk-app] #4187 not #4192
[mk-app] #4191 + #705 #3821
[mk-app] #4167 >= #4191 #341
[mk-app] #4158 or #4187 #4167
[mk-app] #4162 = #4158 #706
[mk-app] #4138 not #4162
[mk-app] #4130 + #3817 #705
[inst-discovered] theory-solving 0 arith# ; #4197
[mk-app] #4123 = #4197 #4130
[instance] 0 #4123
[attach-enode] #4123 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4123 * #370 #705
[mk-app] #4124 + #313 #4123
[mk-app] #4523 <= #4124 #341
[mk-app] #4524 >= #4130 #341
[inst-discovered] theory-solving 0 arith# ; #4524
[mk-app] #4525 = #4524 #4523
[instance] 0 #4525
[attach-enode] #4525 0
[end-of-instance]
[mk-app] #4130 not #4523
[mk-app] #4524 + #3821 #705
[inst-discovered] theory-solving 0 arith# ; #4191
[mk-app] #4525 = #4191 #4524
[instance] 0 #4525
[attach-enode] #4525 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4525 + #335 #4123
[mk-app] #4526 <= #4525 #341
[mk-app] #4527 >= #4524 #341
[inst-discovered] theory-solving 0 arith# ; #4527
[mk-app] #4528 = #4527 #4526
[instance] 0 #4528
[attach-enode] #4528 0
[end-of-instance]
[mk-app] #4524 or #4130 #4526
[mk-app] #4527 = #4524 #706
[mk-app] #4528 not #4524
[mk-app] #4529 not #4527
[inst-discovered] theory-solving 0 basic# ; #4529
[mk-app] #4528 = #4529 #4529
[instance] 0 #4528
[attach-enode] #4528 0
[end-of-instance]
[mk-app] #4528 or #3836 #4529
[instance] 0x61d50e6b6d28 ; 1
[attach-enode] #4123 1
[attach-enode] #4124 1
[attach-enode] #4525 1
[assign] (not #4527) justification -1: 61
[end-of-instance]
[mk-app] #4530 or #3871 #4430
[instance] 0x61d50e6b6d60 ; 1
[attach-enode] #4428 1
[attach-enode] #4429 1
[attach-enode] #4430 1
[assign] #4430 justification -1: 26
[end-of-instance]
[mk-app] #4531 or #3867 #4433
[instance] 0x61d50e6c0800 ; 1
[attach-enode] #4432 1
[attach-enode] #4433 1
[assign] #4433 justification -1: 253
[end-of-instance]
[mk-app] #4532 * #370 #686
[mk-app] #4533 + #313 #4532
[mk-app] #4534 <= #4533 #341
[mk-app] #4535 not #4534
[mk-app] #4536 + #686 #3821
[mk-app] #4537 >= #4536 #341
[mk-app] #4538 = #705 #686
[mk-app] #4539 or #4187 #4167 #4538
[mk-app] #4540 not #4539
[mk-app] #4541 or #4535 #4537 #4540
[mk-app] #4542 not #4541
[mk-app] #4543 + #3821 #686
[inst-discovered] theory-solving 0 arith# ; #4536
[mk-app] #4544 = #4536 #4543
[instance] 0 #4544
[attach-enode] #4544 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4544 + #335 #4532
[mk-app] #4545 <= #4544 #341
[mk-app] #4546 >= #4543 #341
[inst-discovered] theory-solving 0 arith# ; #4546
[mk-app] #4547 = #4546 #4545
[instance] 0 #4547
[attach-enode] #4547 0
[end-of-instance]
[mk-app] #4543 or #4130 #4526 #4538
[inst-discovered] theory-solving 0 basic# ; #4543
[mk-app] #4546 = #4543 #4543
[instance] 0 #4546
[attach-enode] #4546 0
[end-of-instance]
[mk-app] #4546 not #4543
[mk-app] #4547 or #4535 #4545 #4546
[inst-discovered] theory-solving 0 basic# ; #4547
[mk-app] #4548 = #4547 #4547
[instance] 0 #4548
[attach-enode] #4548 0
[end-of-instance]
[mk-app] #4548 not #4547
[mk-app] #4549 not #2956
[mk-app] #4550 or #4549 #4548
[instance] 0x61d50e6c0830 ; 1
[attach-enode] #4532 1
[attach-enode] #4533 1
[attach-enode] #4544 1
[attach-enode] #4538 1
[mk-app] #4551 = #686 #705
[attach-meaning] #370 arith (- 1)
[mk-app] #4552 + #686 #4123
[mk-app] #4553 <= #4552 #341
[mk-app] #4554 >= #4552 #341
[attach-enode] #4551 1
[attach-enode] #4552 1
[assign] (not #4547) justification -1: 58
[end-of-instance]
[mk-app] #4555 Add #2899 #713
[mk-app] #4556 * #370 #4555
[mk-app] #4557 + #713 #2899 #4556
[mk-app] #4558 = #4557 #341
[mk-app] #4559 or #3887 #4558
[instance] 0x61d50e6c0868 ; 1
[attach-enode] #4555 1
[attach-enode] #4556 1
[attach-enode] #4557 1
[attach-enode] #4558 1
[mk-app] #4560 <= #4557 #341
[mk-app] #4561 >= #4557 #341
[assign] #4558 justification -1: 77
[end-of-instance]
[assign] (not #4423) clause -781 782
[assign] (not #4524) clause -785 786
[assign] #4534 clause 789 796
[assign] (not #4545) clause -790 796
[assign] #4543 clause 795 796
[assign] #4560 clause 798 -797
[assign] #4561 clause 799 -797
[assign] #4406 clause 779 781
[assign] #4421 clause 780 781
[assign] #4523 clause 783 785
[assign] (not #4526) clause -784 785
[assign] #4538 clause 791 784 -783 -795
[assign] #4307 justification -1: 771 693
[assign] #4551 justification -1: 791
[assign] #4310 justification -1: 765 693 778 539 693
[assign] #4315 justification -1: 778 766 693 778 693 539
[mk-app] #4562 = #705 #4555
[attach-meaning] #370 arith (- 1)
[mk-app] #4563 + #705 #4556
[mk-app] #4564 <= #4563 #341
[mk-app] #4565 >= #4563 #341
[assign] #4562 justification -1: 495 481
[attach-enode] #4562 0
[attach-enode] #4563 0
[assign] #4564 justification -1: 800
[assign] #4565 justification -1: 800
[mk-app] #4566 = #713 #749
[attach-meaning] #370 arith (- 1)
[mk-app] #4567 + #713 #4342
[mk-app] #4568 <= #4567 #341
[mk-app] #4569 >= #4567 #341
[assign] #4566 justification -1: 693 778 539
[attach-enode] #4566 0
[attach-enode] #4567 0
[assign] #4568 justification -1: 803
[assign] #4569 justification -1: 803
[mk-app] #4570 = #705 #4120
[attach-meaning] #370 arith (- 1)
[mk-app] #4571 + #705 #4125
[mk-app] #4572 <= #4571 #341
[mk-app] #4573 >= #4571 #341
[assign] #4570 justification -1: 568 749 592 693 778 539 495 481 657 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[attach-enode] #4570 0
[attach-enode] #4571 0
[assign] #4572 justification -1: 806
[assign] #4573 justification -1: 806
[new-match] 0x61d50e6cab10 #1810 #1807 #658 #1168 #125 ; #459
[new-match] 0x61d50e6cab50 #1178 #1174 #658 ; #4406 (#1168 #1168)
[eq-expl] #652 lit #4433 ; #4432
[eq-expl] #4432 root
[new-match] 0x61d50e6cab80 #1181 #1180 #652 ; #4406 (#1168 #1168) (#658 #658)
[new-match] 0x61d50e6cabb0 #3310 #1690 #672 #1290 #1167 #125 ; #4420
[mk-app] #4574 or #3853 #4389 #4467
[instance] 0x61d50e6cab10 ; 1
[attach-enode] #4467 1
[assign] #4467 justification -1: 332 779
[end-of-instance]
[assign] #4553 clause 793 -792
[assign] #4554 clause 794 -792
[eq-expl] #459 lit #4421 ; #4420
[eq-expl] #4420 root
[new-match] 0x61d50e6cae70 #1198 #1194 #459 ; #4467 (#3850 #1188)
[mk-app] #4575 or #3939 #4471 #4474
[instance] 0x61d50e6cae70 ; 2
[attach-enode] #4470 2
[attach-enode] #4472 2
[attach-enode] #4473 2
[attach-enode] #4474 2
[end-of-instance]
[assign] #4470 justification -1: 809 591 780
[assign] #4474 clause 811 -810
[eq-expl] #4472 root
[new-match] 0x61d50e6cb200 #1187 #1186 #4472 ; #4473
[eq-expl] #4473 lit #4474 ; #4420
[new-match] 0x61d50e6cb230 #1201 #1200 #4472 ; #4467 (#3850 #1188) (#459 #4473)
[decide-and-or] #3687 #3659
[push] 5
[assign] (not #3703) decision axiom
[assign] #674 clause 505 507
[assign] (not #3704) clause -506 507
[assign] #3705 clause 530 506
[new-match] 0x61d50e6cb278 #1086 #449 #1025 #274 ; #3704
[mk-app] #4576 or #4478 #4488
[mk-app] #4577 = #4576 #4483
[mk-app] #4578 not #4576
[mk-app] #4579 not #4577
[inst-discovered] theory-solving 0 basic# ; #4579
[mk-app] #4578 = #4579 #4579
[instance] 0 #4578
[attach-enode] #4578 0
[end-of-instance]
[mk-app] #4578 or #3800 #4579
[instance] 0x61d50e6cb278 ; 1
[assign] (not #4576) justification -1: 700 -706
[attach-enode] #4483 1
[assign] (not #4577) justification -1: 60
[end-of-instance]
[assign] #4483 clause 813 814
[resolve-lit] 0 (not #4483)
[resolve-process] #4483
[resolve-lit] 0 #3704
[resolve-lit] 4 (not #276)
[resolve-process] (not #4483)
[resolve-lit] 0 #4577
[resolve-lit] 0 #4576
[resolve-process] #4577
[resolve-process] #4576
[resolve-lit] 4 (not #4477)
[resolve-lit] 4 #4488
[conflict] #3704 (not #276) (not #4477)
[pop] 4 6
[assign] #3704 clause 506 -39 -700
[assign] #3703 clause 507 -506
[new-match] 0x61d50e60faa0 #1086 #449 #1025 #274 ; #3704
[mk-app] #4491 or #4478 #4488
[mk-app] #4490 = #4491 #4483
[mk-app] #4469 not #4491
[mk-app] #4465 not #4490
[inst-discovered] theory-solving 0 basic# ; #4465
[mk-app] #4469 = #4465 #4465
[instance] 0 #4469
[attach-enode] #4469 0
[end-of-instance]
[mk-app] #4469 or #3800 #4465
[instance] 0x61d50e60faa0 ; 1
[assign] (not #4491) justification -1: 700 -706
[attach-enode] #4483 1
[assign] (not #4490) justification -1: 60
[end-of-instance]
[assign] #4483 clause 741 742
[decide-and-or] #3767 #778
[push] 2
[assign] #778 decision axiom
[new-match] 0x61d50e60fc78 #2724 #2723 #1025 #341 #2892 ; #778
[mk-app] #4431 not #2724
[mk-app] #4427 or #4431 #4122
[instance] 0x61d50e60fc78 ; 1
[attach-enode] #4113 1
[assign] #4122 justification -1: 460
[end-of-instance]
[assign] #4113 justification -1: 487 485
[eq-expl] #4117 root
[eq-expl] #4119 root
[new-match] 0x61d50e60fdc8 #552 #550 #4119 #4117 ; #4120
[eq-expl] #4116 root
[new-match] 0x61d50e60fe00 #3440 #2356 #4116 #2574 #3934 ; #4117
[new-match] 0x61d50e60fe40 #4034 #4023 #4116 #2574 ; #4117 (#3934 #3934)
[new-match] 0x61d50e60fe78 #3286 #1569 #4116 #3934 #1167 #125 ; #4118
[eq-expl] #4115 root
[new-match] 0x61d50e60fec0 #174 #173 #4115 ; #4116
[new-match] 0x61d50e60fef0 #567 #559 #296 #1025 ; #4115
[mk-app] #4388 + #4117 #4119 #4125
[inst-discovered] theory-solving 0 arith# ; #4126
[mk-app] #4410 = #4126 #4388
[instance] 0 #4410
[attach-enode] #4410 0
[end-of-instance]
[mk-app] #4410 = #4388 #341
[mk-app] #4401 or #3887 #4410
[instance] 0x61d50e60fdc8 ; 2
[attach-enode] #4125 2
[attach-enode] #4388 2
[attach-enode] #4410 2
[mk-app] #4407 <= #4388 #341
[mk-app] #4369 >= #4388 #341
[assign] #4410 justification -1: 77
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4137
[mk-app] #4374 = #4137 #4137
[instance] 0 #4374
[attach-enode] #4374 0
[end-of-instance]
[mk-app] #4374 or #4078 #3966 #4072 #4134 #4136
[instance] 0x61d50e60fe00 ; 3
[attach-enode] #4133 3
[attach-enode] #4135 3
[attach-enode] #4136 3
[end-of-instance]
[mk-app] #4350 + #4001 #4139
[inst-discovered] theory-solving 0 arith# ; #4143
[mk-app] #4349 = #4143 #4350
[instance] 0 #4349
[attach-enode] #4349 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4349 <= #4350 #341
[inst-discovered] theory-solving 0 arith# ; #4349
[mk-app] #4317 = #4349 #4160
[instance] 0 #4317
[attach-enode] #4317 0
[end-of-instance]
[mk-app] #4350 not #4160
[inst-discovered] theory-solving 0 basic# ; #4150
[mk-app] #4349 = #4150 #4150
[instance] 0 #4349
[attach-enode] #4349 0
[end-of-instance]
[mk-app] #4349 or #4072 #4134 #4082 #4157 #4350 #4151
[inst-discovered] theory-solving 0 basic# ; #4349
[mk-app] #4317 = #4349 #4349
[instance] 0 #4317
[attach-enode] #4317 0
[end-of-instance]
[mk-app] #4317 or #4097 #4072 #4134 #4082 #4157 #4350 #4151
[instance] 0x61d50e60fe40 ; 2
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4199
[mk-app] #4349 = #4199 #4199
[instance] 0 #4349
[attach-enode] #4349 0
[end-of-instance]
[mk-app] #4349 or #4373 #3966 #4134 #4198
[instance] 0x61d50e60fe78 ; 3
[attach-enode] #4198 3
[end-of-instance]
[mk-app] #4332 or #3871 #4196
[instance] 0x61d50e60fec0 ; 2
[attach-enode] #4196 2
[attach-meaning] #370 arith (- 1)
[assign] #4196 justification -1: 26
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4312 + #4202 #4115
[attach-meaning] #370 arith (- 1)
[mk-app] #4312 = #4207 #296
[inst-discovered] theory-solving 0 arith# ; #4204
[mk-app] #4328 = #4204 #4312
[instance] 0 #4328
[attach-enode] #4328 0
[end-of-instance]
[mk-app] #4328 or #3877 #4312
[instance] 0x61d50e60fef0 ; 2
[attach-enode] #4312 2
[assign] #4312 justification -1: 78
[end-of-instance]
[assign] #4121 clause 692 -743 -744
[assign] #4407 clause 746 -745
[assign] #4369 clause 747 -745
[assign] #4217 clause 704 -752
[assign] #4218 clause 699 -752
[assign] #4209 clause 703 -753
[assign] #4210 clause 698 -753
[assign] #4156 clause 702 -704 -662 -670 -677 -703 -584 -574
[assign] #4160 clause 697 -699 -577 -583 -647 -680 -698 -573 -477
[attach-meaning] #370 arith (- 1)
[mk-app] #4322 + #4114 #4125
[mk-app] #4323 <= #4322 #341
[mk-app] #4305 >= #4322 #341
[attach-enode] #4322 0
[assign] #4323 justification -1: 692
[assign] #4305 justification -1: 692
[assign] #4133 justification -1: 659 568 752 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[assign] #4136 justification -1: 660 568 752 592 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[assign] #3630 justification -1: 692 592 568 752 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[mk-app] #4306 = #4067 #4117
[attach-meaning] #370 arith (- 1)
[assign] #4306 justification -1: 568 752 592 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[attach-enode] #4306 0
[assign] #4225 justification -1: 756
[assign] #4226 justification -1: 756
[new-match] 0x61d50e6b5858 #3286 #1569 #1276 #1396 #1167 #125 ; #748
[new-match] 0x61d50e6b58a0 #552 #550 #749 #782 ; #750
[inst-discovered] theory-solving 0 basic# ; #4229
[mk-app] #4274 = #4229 #4229
[instance] 0 #4274
[attach-enode] #4274 0
[end-of-instance]
[mk-app] #4274 or #4373 #3966 #4074 #4228
[instance] 0x61d50e6b5858 ; 1
[attach-enode] #4227 1
[attach-enode] #4228 1
[assign] #4228 justification -1: 305 626 659
[end-of-instance]
[mk-app] #4235 or #3887 #4234
[instance] 0x61d50e6b58a0 ; 1
[attach-enode] #4231 1
[attach-enode] #4232 1
[attach-enode] #4233 1
[attach-enode] #4234 1
[mk-app] #4230 <= #4233 #341
[mk-app] #4524 >= #4233 #341
[assign] #4234 justification -1: 77
[end-of-instance]
[assign] #4198 clause 751 -748
[assign] (not #4150) clause -750 -748 -702 -697
[assign] #4146 clause 684 -685 -664
[assign] #4148 clause 686 -687 -665
[assign] #4230 clause 759 -758
[assign] #4524 clause 760 -758
[mk-app] #4527 = #749 #4119
[attach-meaning] #370 arith (- 1)
[mk-app] #4544 * #370 #4119
[mk-app] #4545 + #749 #4544
[mk-app] #4551 <= #4545 #341
[mk-app] #4552 >= #4545 #341
[assign] #4527 justification -1: 568 752 592 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[attach-enode] #4527 0
[attach-enode] #4544 0
[attach-enode] #4545 0
[assign] #4551 justification -1: 761
[assign] #4552 justification -1: 761
[mk-app] #4553 = #4120 #4231
[attach-meaning] #370 arith (- 1)
[mk-app] #4554 + #4120 #4232
[mk-app] #4543 <= #4554 #341
[mk-app] #4546 >= #4554 #341
[assign] #4553 justification -1: 568 752 592 657 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[attach-enode] #4553 0
[attach-enode] #4554 0
[assign] #4543 justification -1: 764
[assign] #4546 justification -1: 764
[eq-expl] #4115 lit #4196 ; #4139
[eq-expl] #4139 th arith ; #4084
[eq-expl] #4116 cg (#4115 #1454) ; #1276
[eq-expl] #4118 cg (#125 #125) (#1167 #1167) (#3934 #1396) (#4116 #1276) ; #748
[new-match] 0x61d50e6b62b0 #542 #236 #4118 #275 ; #4198 (#1167 #1167)
[new-match] 0x61d50e6b62e8 #240 #236 #4118 #275 ; #4198 (#1167 #1167)
[mk-app] #4547 or #4449 #4325 #4326
[instance] 0x61d50e6b62b0 ; 4
[attach-enode] #4324 4
[attach-enode] #4326 4
[end-of-instance]
[mk-app] #4560 or #4450 #4325 #4330
[instance] 0x61d50e6b62e8 ; 4
[attach-enode] #4329 4
[attach-enode] #4330 4
[end-of-instance]
[assign] #4324 justification -1: 751 568 752 592 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[assign] #4326 clause 768 -767
[assign] #4330 clause 769 -767
[new-match] 0x61d50e6b67b0 #1136 #455 #749 #275 ; #4326
[new-match] 0x61d50e6b67e8 #174 #173 #749 ; #4329
[eq-expl] #4329 lit #4330 ; #748
[new-match] 0x61d50e6b6818 #503 #499 #749 #275 ; #4198 (#4118 #4329) (#1167 #1167)
[mk-app] #4561 or #4341 #4347
[mk-app] #4562 = #4561 #4326
[mk-app] #4563 not #4561
[mk-app] #4564 not #4562
[inst-discovered] theory-solving 0 basic# ; #4564
[mk-app] #4563 = #4564 #4564
[instance] 0 #4563
[attach-enode] #4563 0
[end-of-instance]
[mk-app] #4563 or #3836 #4564
[instance] 0x61d50e6b67b0 ; 5
[attach-enode] #4342 5
[attach-enode] #4343 5
[attach-enode] #4346 5
[assign] (not #4562) justification -1: 61
[end-of-instance]
[assign] (not #4561) clause -772 773
[assign] #4344 clause 770 772
[assign] (not #4347) clause -771 772
[decide-and-or] #3660 #3706
[push] 3
[assign] #3706 decision axiom
[assign] #779 clause 490 -492 -491
[assign] #3645 clause 494 -490
[decide-and-or] #3770 #3689
[push] 4
[assign] #3689 decision axiom
[assign] (not #3714) clause -536 -537 -494
[assign] #696 clause 495 536
[assign] #3712 clause 535 536
[assign] (not #3708) clause -534 -535 -498
[assign] #721 clause 499 534
[assign] #3674 clause 533 534
[eq-expl] #720 root
[new-match] 0x61d50e6b6b28 #3408 #2204 #720 #1276 #1533 #1167 #125 #1534 #125 ; #721
[mk-app] #4565 not #4301
[inst-discovered] theory-solving 0 basic# ; #4304
[mk-app] #4565 = #4304 #4304
[instance] 0 #4565
[attach-enode] #4565 0
[end-of-instance]
[mk-app] #4565 not #3408
[mk-app] #4566 or #4565 #4304
[instance] 0x61d50e6b6b28 ; 1
[attach-enode] #4296 1
[attach-enode] #4302 1
[assign] (not #4303) justification -1: 397
[end-of-instance]
[assign] #4302 justification -1: 499 539
[assign] (not #4301) clause -775 -776 777
[assign] #4296 clause 774 775
[assign] #4299 clause 693 775
[assign] #706 clause 501 -693 691 -495 -657 -692
[assign] #3649 clause 502 -501
[assign] (not #3682) clause -532 -502 -533
[assign] #687 clause 503 532
[assign] #673 clause 504 532
[assign] #3687 clause 531 532
[assign] (not #3705) clause -530 -531 -507
[assign] #3701 clause 508 530
[assign] #3735 clause 529 530
[eq-expl] #720 lit #4299 ; #4298
[eq-expl] #4298 root
[new-match] 0x61d50e6c0870 #542 #236 #720 #275 ; #4296 (#1167 #1167)
[new-match] 0x61d50e6c08a8 #240 #236 #720 #275 ; #4296 (#1167 #1167)
[new-match] 0x61d50e6c08e0 #3402 #2179 #1276 #3765 #1167 #125 #1534 #125 ; #4298
[new-match] 0x61d50e6c0938 #2269 #2265 #1276 #3765 #1167 #125 ; #4298 (#125 #125) (#1534 #1534)
[new-match] 0x61d50e6c0980 #3390 #2134 #672 #658 #1289 #1147 #125 #1167 #125 ; #673
[eq-expl] #705 root
[new-match] 0x61d50e6c09e0 #1136 #455 #705 #275 ; #706
[eq-expl] #698 lit #687 ; #686
[eq-expl] #686 root
[new-match] 0x61d50e6c0a18 #174 #173 #698 ; #672
[eq-expl] #652 root
[new-match] 0x61d50e6c0a48 #1166 #1165 #652 ; #658
[new-match] 0x61d50e6c0a78 #2956 #392 #705 #275 ; #686
[eq-expl] #741 lit #696 ; #1462
[eq-expl] #713 root
[new-match] 0x61d50e6c0ab0 #552 #550 #713 #741 ; #705
[new-match] 0x61d50e6c0ae8 #2432 #365 #1025 #274 ; #3699
[mk-app] #4567 or #4449 #4308 #4310
[instance] 0x61d50e6c0870 ; 2
[attach-enode] #4307 2
[attach-enode] #4309 2
[attach-enode] #4310 2
[end-of-instance]
[mk-app] #4568 or #4450 #4308 #4315
[instance] 0x61d50e6c08a8 ; 2
[attach-enode] #4314 2
[attach-enode] #4315 2
[end-of-instance]
[mk-app] #4569 not #2269
[mk-app] #4570 or #4569 #3930 #4320
[instance] 0x61d50e6c0938 ; 2
[attach-enode] #4264 2
[mk-app] #4571 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #4264
[mk-app] #4572 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS #4571
[attach-enode] #4571 2
[attach-enode] #4572 2
[mk-app] #4573 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #4572
[mk-app] #4576 = #4571 #4573
[new-match] 0 datatype#21 datatype#18 #4571 ; #4572
[instance] 0 #4576
[attach-enode] #4573 2
[end-of-instance]
[attach-enode] #4265 2
[attach-enode] #4266 2
[attach-enode] #4319 2
[attach-enode] #4320 2
[assign] #4320 justification -1: 18 404
[end-of-instance]
[mk-app] #4576 not #4423
[inst-discovered] theory-solving 0 basic# ; #4425
[mk-app] #4576 = #4425 #4425
[instance] 0 #4576
[attach-enode] #4576 0
[end-of-instance]
[mk-app] #4576 not #3390
[mk-app] #4577 or #4576 #4425
[instance] 0x61d50e6c0980 ; 1
[attach-enode] #4406 1
[attach-enode] #4420 1
[attach-enode] #4421 1
[assign] (not #4424) justification -1: 389
[end-of-instance]
[mk-app] #4579 or #4130 #4526
[mk-app] #4578 = #4579 #706
[mk-app] #4575 not #4579
[mk-app] #4574 not #4578
[inst-discovered] theory-solving 0 basic# ; #4574
[mk-app] #4575 = #4574 #4574
[instance] 0 #4575
[attach-enode] #4575 0
[end-of-instance]
[mk-app] #4575 or #3836 #4574
[instance] 0x61d50e6c09e0 ; 1
[attach-enode] #4123 1
[attach-enode] #4124 1
[attach-enode] #4525 1
[assign] (not #4578) justification -1: 61
[end-of-instance]
[mk-app] #4559 or #3871 #4430
[instance] 0x61d50e6c0a18 ; 1
[attach-enode] #4428 1
[attach-enode] #4429 1
[attach-enode] #4430 1
[assign] #4430 justification -1: 26
[end-of-instance]
[mk-app] #4548 or #3867 #4433
[instance] 0x61d50e6c0a48 ; 1
[attach-enode] #4432 1
[attach-enode] #4433 1
[assign] #4433 justification -1: 253
[end-of-instance]
[mk-app] #4549 + #3821 #686
[inst-discovered] theory-solving 0 arith# ; #4536
[mk-app] #4550 = #4536 #4549
[instance] 0 #4550
[attach-enode] #4550 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4550 + #335 #4532
[mk-app] #4531 <= #4550 #341
[mk-app] #4530 >= #4549 #341
[inst-discovered] theory-solving 0 arith# ; #4530
[mk-app] #4529 = #4530 #4531
[instance] 0 #4529
[attach-enode] #4529 0
[end-of-instance]
[mk-app] #4549 or #4130 #4526 #4538
[inst-discovered] theory-solving 0 basic# ; #4549
[mk-app] #4530 = #4549 #4549
[instance] 0 #4530
[attach-enode] #4530 0
[end-of-instance]
[mk-app] #4530 not #4549
[mk-app] #4529 or #4535 #4531 #4530
[inst-discovered] theory-solving 0 basic# ; #4529
[mk-app] #4528 = #4529 #4529
[instance] 0 #4528
[attach-enode] #4528 0
[end-of-instance]
[mk-app] #4528 not #4529
[mk-app] #4208 not #2956
[mk-app] #4201 or #4208 #4528
[instance] 0x61d50e6c0a78 ; 1
[attach-enode] #4532 1
[attach-enode] #4533 1
[attach-enode] #4550 1
[attach-enode] #4538 1
[mk-app] #4280 = #686 #705
[attach-meaning] #370 arith (- 1)
[mk-app] #4273 + #686 #4123
[mk-app] #4275 <= #4273 #341
[mk-app] #4291 >= #4273 #341
[attach-enode] #4280 1
[attach-enode] #4273 1
[assign] (not #4529) justification -1: 58
[end-of-instance]
[mk-app] #4289 or #3887 #4558
[instance] 0x61d50e6c0ab0 ; 1
[attach-enode] #4555 1
[attach-enode] #4556 1
[attach-enode] #4557 1
[attach-enode] #4558 1
[mk-app] #4290 <= #4557 #341
[mk-app] #4257 >= #4557 #341
[assign] #4558 justification -1: 77
[end-of-instance]
[mk-app] #4256 uClip #275 #1025
[mk-app] #4313 >= #4256 #341
[mk-app] #4318 not #4313
[mk-app] #4402 + #4256 #4479
[mk-app] #4418 >= #4402 #341
[mk-app] #4426 = #1025 #4256
[mk-app] #4434 or #4478 #4481 #4426
[mk-app] #4453 not #4434
[mk-app] #4452 or #4318 #4418 #4453
[mk-app] #4454 not #4452
[mk-app] #4466 + #4479 #4256
[inst-discovered] theory-solving 0 arith# ; #4402
[mk-app] #4476 = #4402 #4466
[instance] 0 #4476
[attach-enode] #4476 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4476 * #370 #4256
[mk-app] #4508 + #288 #4476
[mk-app] #4501 <= #4508 #341
[mk-app] #4580 >= #4466 #341
[inst-discovered] theory-solving 0 arith# ; #4580
[mk-app] #4581 = #4580 #4501
[instance] 0 #4581
[attach-enode] #4581 0
[end-of-instance]
[mk-app] #4466 or #4478 #4488 #4426
[inst-discovered] theory-solving 0 basic# ; #4466
[mk-app] #4580 = #4466 #4466
[instance] 0 #4580
[attach-enode] #4580 0
[end-of-instance]
[mk-app] #4580 not #4466
[mk-app] #4581 or #4318 #4501 #4580
[inst-discovered] theory-solving 0 basic# ; #4581
[mk-app] #4582 = #4581 #4581
[instance] 0 #4582
[attach-enode] #4582 0
[end-of-instance]
[mk-app] #4582 not #4581
[mk-app] #4583 not #2432
[mk-app] #4584 or #4583 #4582
[instance] 0x61d50e6c0ae8 ; 1
[attach-enode] #4256 1
[attach-enode] #4476 1
[attach-enode] #4508 1
[attach-enode] #4426 1
[attach-meaning] #370 arith (- 1)
[mk-app] #4585 + #1025 #4476
[mk-app] #4586 <= #4585 #341
[mk-app] #4587 >= #4585 #341
[attach-enode] #4585 1
[assign] (not #4581) justification -1: 57
[end-of-instance]
[assign] (not #4423) clause -784 785
[assign] (not #4579) clause -788 789
[assign] #4534 clause 792 799
[assign] (not #4531) clause -793 799
[assign] #4549 clause 798 799
[assign] #4290 clause 801 -800
[assign] #4257 clause 802 -800
[assign] #4313 clause 803 809
[assign] (not #4501) clause -804 809
[assign] #4466 clause 808 809
[assign] #4406 clause 782 784
[assign] #4421 clause 783 784
[assign] #4523 clause 786 788
[assign] (not #4526) clause -787 788
[assign] #4426 clause 805 -808
[assign] #4538 clause 794 787 -786 -798
[assign] #4586 clause 806 -805
[assign] #4587 clause 807 -805
[assign] #4307 justification -1: 774 693
[assign] #4280 justification -1: 794
[assign] #4310 justification -1: 768 693 781 539 693
[assign] #4315 justification -1: 781 769 693 781 693 539
[mk-app] #4588 = #705 #4555
[attach-meaning] #370 arith (- 1)
[mk-app] #4589 + #705 #4556
[mk-app] #4590 <= #4589 #341
[mk-app] #4591 >= #4589 #341
[assign] #4588 justification -1: 495 481
[attach-enode] #4588 0
[attach-enode] #4589 0
[assign] #4590 justification -1: 810
[assign] #4591 justification -1: 810
[mk-app] #4592 = #659 #4256
[attach-meaning] #370 arith (- 1)
[mk-app] #4593 + #659 #4476
[mk-app] #4594 <= #4593 #341
[mk-app] #4595 >= #4593 #341
[assign] #4592 justification -1: 508 39
[attach-enode] #4592 0
[attach-enode] #4593 0
[assign] #4594 justification -1: 813
[assign] #4595 justification -1: 813
[mk-app] #4596 = #713 #749
[attach-meaning] #370 arith (- 1)
[mk-app] #4597 + #713 #4342
[mk-app] #4598 <= #4597 #341
[mk-app] #4599 >= #4597 #341
[assign] #4596 justification -1: 693 781 539
[attach-enode] #4596 0
[attach-enode] #4597 0
[assign] #4598 justification -1: 816
[assign] #4599 justification -1: 816
[mk-app] #4600 = #705 #4120
[attach-meaning] #370 arith (- 1)
[mk-app] #4601 + #705 #4125
[mk-app] #4602 <= #4601 #341
[mk-app] #4603 >= #4601 #341
[assign] #4600 justification -1: 568 752 592 693 781 539 495 481 657 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[attach-enode] #4600 0
[attach-enode] #4601 0
[assign] #4602 justification -1: 819
[assign] #4603 justification -1: 819
[new-match] 0x61d50e6cb3c8 #1810 #1807 #658 #1168 #125 ; #459
[new-match] 0x61d50e6cb408 #1178 #1174 #658 ; #4406 (#1168 #1168)
[eq-expl] #652 lit #4433 ; #4432
[eq-expl] #4432 root
[new-match] 0x61d50e6cb438 #1181 #1180 #652 ; #4406 (#1168 #1168) (#658 #658)
[new-match] 0x61d50e6cb468 #3310 #1690 #672 #1290 #1167 #125 ; #4420
[mk-app] #4604 or #3853 #4389 #4467
[instance] 0x61d50e6cb3c8 ; 1
[attach-enode] #4467 1
[assign] #4467 justification -1: 332 782
[end-of-instance]
[assign] #4275 clause 796 -795
[assign] #4291 clause 797 -795
[assign] #3654 clause 511 -807 -573 -583 -680 484 -814
[eq-expl] #459 lit #4421 ; #4420
[eq-expl] #4420 root
[new-match] 0x61d50e6cb7c8 #1198 #1194 #459 ; #4467 (#3850 #1188)
[mk-app] #4605 or #3939 #4471 #4474
[instance] 0x61d50e6cb7c8 ; 2
[attach-enode] #4470 2
[attach-enode] #4472 2
[attach-enode] #4473 2
[attach-enode] #4474 2
[end-of-instance]
[assign] #4470 justification -1: 822 591 783
[assign] #4474 clause 824 -823
[eq-expl] #4472 root
[new-match] 0x61d50e6cbb80 #1187 #1186 #4472 ; #4473
[eq-expl] #4473 lit #4474 ; #4420
[new-match] 0x61d50e6cbbb0 #1201 #1200 #4472 ; #4467 (#3850 #1188) (#459 #4473)
[decide-and-or] #3735 #3669
[push] 5
[assign] (not #3668) decision axiom
[assign] #639 clause 509 513
[assign] #3721 clause 512 513
[assign] (not #3653) clause -510 -512 -511
[resolve-process] true
[resolve-lit] 0 #3653
[resolve-lit] 1 (not #4595)
[resolve-lit] 1 (not #4586)
[resolve-lit] 4 (not #4215)
[conflict] #3653 (not #4595) (not #4586)
[pop] 1 6
[assign] #3653 clause 510 -815 -806
[assign] (not #3721) clause -512 -510 -511
[assign] #3668 clause 513 512
[decide-and-or] #3735 #3661
[push] 5
[assign] (not #3663) decision axiom
[assign] #411 clause 514 516
[assign] (not #380) clause -515 516
[eq-expl] #659 lit #3701 ; #3699
[eq-expl] #3699 cg (#274 #275) (#1025 #1025) ; #4256
[eq-expl] #4256 lit #4426 ; #1025
[new-match] 0x61d50e6cbbf8 #552 #550 #296 #659 ; #378
[new-match] 0x61d50e6cbc30 #2018 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6cbc70 #1545 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6cbcb0 #3312 #1702 #672 #1290 #1167 #125 ; #460 (#459 #4420) (#125 #125) (#125 #125) (#1167 #1167) (#1167 #1167)
[mk-app] #4606 Add #1025 #296
[mk-app] #4607 * #370 #4606
[mk-app] #4608 + #296 #1025 #4607
[mk-app] #4609 = #4608 #341
[attach-meaning] #370 arith (- 1)
[mk-app] #4610 + #1025 #4607
[mk-app] #4611 = #4610 #370
[inst-discovered] theory-solving 0 arith# ; #4609
[mk-app] #4612 = #4609 #4611
[instance] 0 #4612
[attach-enode] #4612 0
[end-of-instance]
[mk-app] #4612 or #3887 #4611
[instance] 0x61d50e6cbbf8 ; 1
[attach-enode] #4606 1
[attach-enode] #4607 1
[attach-enode] #4610 1
[attach-enode] #4611 1
[mk-app] #4613 <= #4610 #370
[mk-app] #4614 >= #4610 #370
[assign] #4611 justification -1: 77
[end-of-instance]
[mk-app] #4615 vstd!seq.Seq.len.? #125 #1167 #4420
[mk-app] #4616 = #4615 #341
[mk-app] #4617 not #4616
[mk-app] #4618 ext_eq #2 #1188 #4420 #3970
[mk-app] #4619 or #4471 #3930 #4617 #4618
[inst-discovered] theory-solving 0 basic# ; #4619
[mk-app] #4620 = #4619 #4619
[instance] 0 #4620
[attach-enode] #4620 0
[end-of-instance]
[mk-app] #4620 or #3941 #4471 #3930 #4617 #4618
[instance] 0x61d50e6cbc30 ; 1
[attach-enode] #4615 1
[attach-enode] #4616 1
[attach-enode] #4618 1
[end-of-instance]
[mk-app] #4621 >= #4615 #341
[mk-app] #4622 or #4471 #4621
[mk-app] #4623 or #3842 #4471 #4621
[instance] 0x61d50e6cbc70 ; 1
[assign] #4621 justification -1: 303 823
[end-of-instance]
[mk-app] #4624 vstd!seq.Seq.len.? #125 #1167 #4462
[mk-app] #4625 vstd!seq.Seq.len.? #125 #1167 #4378
[mk-app] #4626 Add #4625 #296
[mk-app] #4627 nClip #4626
[mk-app] #4628 = #4624 #4627
[mk-app] #4629 or #4459 #4461 #3930 #4628
[inst-discovered] theory-solving 0 basic# ; #4629
[mk-app] #4630 = #4629 #4629
[instance] 0 #4630
[attach-enode] #4630 0
[end-of-instance]
[mk-app] #4630 not #3312
[mk-app] #4631 or #4630 #4459 #4461 #3930 #4628
[instance] 0x61d50e6cbcb0 ; 2
[attach-enode] #4458 2
[attach-enode] #4460 2
[attach-enode] #4462 2
[attach-enode] #4624 2
[attach-enode] #4625 2
[attach-enode] #4626 2
[attach-enode] #4627 2
[attach-enode] #4628 2
[end-of-instance]
[assign] #4613 clause 826 -825
[assign] #4614 clause 827 -825
[assign] #4458 justification -1: 561 713
[mk-app] #4632 = #4606 #4615
[attach-meaning] #370 arith (- 1)
[mk-app] #4633 * #370 #4615
[mk-app] #4634 + #4606 #4633
[mk-app] #4635 <= #4634 #341
[mk-app] #4636 >= #4634 #341
[attach-enode] #4632 0
[attach-enode] #4633 0
[attach-enode] #4634 0
[assign] (not #4632) justification -1: -515 783 508 805 39
[decide-and-or] #3942 #3969
[push] 6
[assign] (not #3968) decision axiom
[assign] (not #4494) justification -1: -627 696
[push] 7
[assign] (not #3975) decision axiom
[assign] (not #3980) clause -630 629
[push] 8
[assign] (not #4382) decision axiom
[assign] #4243 clause 716 694
[eq-expl] #4396 root
[new-match] 0x61d50e6d7cd8 #552 #550 #4396 #4393 ; #4397
[new-match] 0x61d50e6d7d10 #2320 #2319 #2351 #1276 #4395 #3934 ; #4396
[mk-app] #4637 + #4393 #4396 #4496
[inst-discovered] theory-solving 0 arith# ; #4497
[mk-app] #4638 = #4497 #4637
[instance] 0 #4638
[attach-enode] #4638 0
[end-of-instance]
[mk-app] #4638 = #4637 #341
[mk-app] #4639 or #3887 #4638
[instance] 0x61d50e6d7cd8 ; 4
[attach-enode] #4496 4
[attach-enode] #4637 4
[attach-enode] #4638 4
[mk-app] #4640 <= #4637 #341
[mk-app] #4641 >= #4637 #341
[assign] #4638 justification -1: 77
[end-of-instance]
[mk-app] #4642 or #4222 #4511
[instance] 0x61d50e6d7d10 ; 4
[attach-enode] #4510 4
[attach-enode] #4511 4
[assign] #4511 justification -1: 414
[end-of-instance]
[assign] #4640 clause 838 -837
[assign] #4641 clause 839 -837
[mk-app] #4643 = #2899 #4397
[attach-meaning] #370 arith (- 1)
[mk-app] #4644 + #2899 #4496
[mk-app] #4645 <= #4644 #341
[mk-app] #4646 >= #4644 #341
[assign] #4643 justification -1: 716 717 660 657 592
[attach-enode] #4643 0
[attach-enode] #4644 0
[assign] #4645 justification -1: 841
[assign] #4646 justification -1: 841
[new-match] 0x61d50e6d8350 #2320 #2319 #2316 #1276 #4395 #3934 ; #4510
[decide-and-or] #4620 #4617
[push] 9
[assign] (not #4616) decision axiom
[mk-app] #4647 <= #4615 #341
[assign] (not #4647) justification -1: -828 830
[decide-and-or] #4631 #4461
[push] 10
[assign] (not #4460) decision axiom
[new-match] 0x61d50e6d84e8 #542 #236 #672 #275 ; #4460 (#1167 #1167)
[new-match] 0x61d50e6d8520 #240 #236 #672 #275 ; #4460 (#1167 #1167)
[eq-expl] #686 lit #4538 ; #705
[eq-expl] #4298 lit #4320 ; #4319
[eq-expl] #4264 cg (#3765 #1533) ; #1393
[eq-expl] #4265 cg (#4264 #1393) ; #1395
[eq-expl] #4266 cg (#125 #125) (#1168 #1168) (#4265 #1395) ; #1396
[eq-expl] #4319 cg (#125 #125) (#1167 #1167) (#4266 #1396) (#1276 #1276) ; #748
[eq-expl] #713 cg (#720 #748) ; #749
[eq-expl] #705 cg (#741 #782) (#713 #749) ; #750
[eq-expl] #4117 cg (#3934 #1396) (#2574 #2574) (#4116 #1276) ; #782
[eq-expl] #4119 cg (#4118 #748) ; #749
[eq-expl] #750 cg (#782 #4117) (#749 #4119) ; #4120
[eq-expl] #4120 lit #4121 ; #4114
[eq-expl] #4114 cg (#3934 #1396) (#2574 #2574) (#3638 #3638) ; #3639
[new-match] 0x61d50e6d8558 #503 #499 #698 #275 ; #4460 (#1167 #1167) (#672 #672)
[mk-app] #4648 not #4357
[mk-app] #4649 I #3639
[mk-app] #4650 has_type #4649 #1167
[mk-app] #4651 or #4648 #4650
[mk-app] #4652 not #503
[mk-app] #4653 or #4652 #4648 #4650
[instance] 0x61d50e6d8558 ; 3
[attach-enode] #4357 3
[attach-enode] #4649 3
[attach-enode] #4650 3
[end-of-instance]
[assign] #4357 justification -1: 501 692 592 568 752 693 781 539 495 481 657 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[assign] (not #4650) justification -1: -832 503 794 692 592 568 752 693 781 539 495 481 657 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668
[resolve-process] true
[resolve-lit] 0 #4650
[resolve-lit] 0 (not #4357)
[resolve-process] #4650
[resolve-lit] 0 #4460
[resolve-lit] 6 (not #687)
[resolve-lit] 6 (not #4538)
[resolve-lit] 8 (not #4121)
[resolve-lit] 8 (not #4196)
[resolve-lit] 6 (not #4299)
[resolve-lit] 6 (not #4320)
[resolve-lit] 6 (not #696)
[resolve-lit] 9 (not #4068)
[resolve-lit] 8 (not #4218)
[resolve-lit] 8 (not #4217)
[resolve-lit] 8 (not #4210)
[resolve-lit] 8 (not #4209)
[resolve-lit] 9 (not #4215)
[resolve-lit] 9 (not #4214)
[resolve-lit] 9 (not #4106)
[resolve-lit] 9 (not #4105)
[resolve-lit] 9 (not #4081)
[resolve-lit] 9 (not #4102)
[resolve-process] (not #4357)
[resolve-lit] 6 (not #706)
[conflict] #4460 (not #687) (not #4121) (not #4299) (not #696) (not #4068) (not #706)
[pop] 6 11
[attach-enode] #4460 0
[assign] #4460 clause 825 -503 -692 -693 -495 -657 -501
[new-match] 0x61d50e6cbc60 #542 #236 #672 #275 ; #4460 (#1167 #1167)
[new-match] 0x61d50e6cbc98 #240 #236 #672 #275 ; #4460 (#1167 #1167)
[new-match] 0x61d50e6cbcd0 #503 #499 #698 #275 ; #4460 (#1167 #1167) (#672 #672)
[decide-and-or] #3735 #3661
[push] 5
[assign] (not #3663) decision axiom
[assign] #411 clause 514 516
[assign] (not #380) clause -515 516
[new-match] 0x61d50e6cbd20 #552 #550 #296 #659 ; #378
[new-match] 0x61d50e6cbd58 #2018 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6cbd98 #1545 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6cbdd8 #3312 #1702 #672 #1290 #1167 #125 ; #460 (#459 #4420) (#125 #125) (#125 #125) (#1167 #1167) (#1167 #1167)
[attach-meaning] #370 arith (- 1)
[mk-app] #4610 + #1025 #4607
[mk-app] #4611 = #4610 #370
[inst-discovered] theory-solving 0 arith# ; #4609
[mk-app] #4613 = #4609 #4611
[instance] 0 #4613
[attach-enode] #4613 0
[end-of-instance]
[mk-app] #4613 or #3887 #4611
[instance] 0x61d50e6cbd20 ; 1
[attach-enode] #4606 1
[attach-enode] #4607 1
[attach-enode] #4610 1
[attach-enode] #4611 1
[mk-app] #4614 <= #4610 #370
[mk-app] #4632 >= #4610 #370
[assign] #4611 justification -1: 77
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4619
[mk-app] #4633 = #4619 #4619
[instance] 0 #4633
[attach-enode] #4633 0
[end-of-instance]
[mk-app] #4633 or #3941 #4471 #3930 #4617 #4618
[instance] 0x61d50e6cbd58 ; 1
[attach-enode] #4615 1
[attach-enode] #4616 1
[attach-enode] #4618 1
[end-of-instance]
[mk-app] #4634 or #3842 #4471 #4621
[instance] 0x61d50e6cbd98 ; 1
[assign] #4621 justification -1: 303 823
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4629
[mk-app] #4635 = #4629 #4629
[instance] 0 #4635
[attach-enode] #4635 0
[end-of-instance]
[mk-app] #4635 not #3312
[mk-app] #4636 or #4635 #4459 #4461 #3930 #4628
[instance] 0x61d50e6cbdd8 ; 2
[attach-enode] #4458 2
[attach-enode] #4462 2
[attach-enode] #4624 2
[attach-enode] #4625 2
[attach-enode] #4626 2
[attach-enode] #4627 2
[attach-enode] #4628 2
[end-of-instance]
[assign] #4614 clause 827 -826
[assign] #4632 clause 828 -826
[assign] #4458 justification -1: 561 713
[mk-app] #4637 = #4606 #4615
[attach-meaning] #370 arith (- 1)
[mk-app] #4638 * #370 #4615
[mk-app] #4640 + #4606 #4638
[mk-app] #4641 <= #4640 #341
[mk-app] #4643 >= #4640 #341
[attach-enode] #4637 0
[attach-enode] #4638 0
[attach-enode] #4640 0
[assign] #4628 clause 833 -832
[assign] (not #4637) justification -1: -515 783 508 805 39
[eq-expl] #4625 cg (#125 #125) (#1167 #1167) (#4378 #1290) ; #1273
[eq-expl] #1273 lit #3694 ; #1025
[eq-expl] #4626 cg (#4625 #659) (#296 #296) ; #378
[eq-expl] #378 root
[new-match] 0x61d50e6d7d10 #2918 #348 #4626 ; #4627
[mk-app] #4644 nClip #378
[mk-app] #4645 >= #4644 #341
[mk-app] #4646 not #4645
[mk-app] #4647 >= #378 #341
[mk-app] #4652 not #4647
[mk-app] #4653 = #378 #4644
[mk-app] #4642 or #4652 #4653
[mk-app] #4639 not #4642
[mk-app] #4630 or #4646 #4639
[mk-app] #4631 not #4630
[mk-app] #4623 not #2918
[mk-app] #4620 or #4623 #4631
[instance] 0x61d50e6d7d10 ; 3
[attach-enode] #4644 3
[attach-enode] #4653 3
[attach-meaning] #370 arith (- 1)
[mk-app] #4612 * #370 #4644
[mk-app] #4654 + #378 #4612
[mk-app] #4655 <= #4654 #341
[mk-app] #4656 >= #4654 #341
[attach-enode] #4612 3
[attach-enode] #4654 3
[assign] (not #4630) justification -1: 56
[end-of-instance]
[assign] #4645 clause 837 843
[assign] #4642 clause 842 843
[assign] (not #4653) justification -1: -515 833 783 713 508 805 480 39
[mk-app] #4657 = #378 #4606
[attach-meaning] #370 arith (- 1)
[mk-app] #4658 + #378 #4607
[mk-app] #4659 <= #4658 #341
[mk-app] #4660 >= #4658 #341
[assign] #4657 justification -1: 508 805 39
[attach-enode] #4657 0
[attach-enode] #4658 0
[assign] #4659 justification -1: 844
[assign] #4660 justification -1: 844
[mk-app] #4661 = #4615 #4644
[attach-meaning] #370 arith (- 1)
[mk-app] #4662 + #4615 #4612
[mk-app] #4663 <= #4662 #341
[mk-app] #4664 >= #4662 #341
[assign] #4661 justification -1: 833 783 713 508 805 480 39 783
[attach-enode] #4661 0
[attach-enode] #4662 0
[assign] #4663 justification -1: 847
[assign] #4664 justification -1: 847
[assign] (not #4647) clause -838 839 -842
[resolve-process] true
[resolve-lit] 4 (not #4215)
[resolve-lit] 0 #4647
[resolve-lit] 0 (not #4660)
[resolve-lit] 0 (not #4614)
[resolve-process] #4647
[resolve-lit] 0 #4653
[resolve-lit] 0 (not #4642)
[resolve-process] (not #4660)
[resolve-lit] 0 (not #4657)
[resolve-process] (not #4657)
[resolve-lit] 1 (not #3701)
[resolve-lit] 1 (not #4426)
[resolve-lit] 4 (not #276)
[resolve-process] #4653
[resolve-lit] 0 #380
[resolve-lit] 0 (not #4628)
[resolve-lit] 1 (not #4421)
[resolve-lit] 4 (not #4379)
[resolve-process] (not #4642)
[resolve-lit] 0 #4630
[resolve-process] #4630
[resolve-process] (not #4628)
[resolve-lit] 0 (not #4458)
[resolve-lit] 1 (not #4460)
[resolve-process] (not #4458)
[resolve-lit] 4 (not #3844)
[resolve-process] (not #4614)
[resolve-lit] 0 (not #4611)
[resolve-process] (not #4611)
[conflict] #380 (not #3701) (not #4426) (not #276) (not #4421) (not #4460) (not #3844)
[pop] 1 6
[assign] #380 clause 515 -508 -805 -39 -783 -825 -561
[assign] #3663 clause 516 -515
[new-match] 0x61d50e6cbd30 #552 #550 #296 #659 ; #378
[new-match] 0x61d50e6cbd68 #2018 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6cbda8 #1545 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6cbde8 #3312 #1702 #672 #1290 #1167 #125 ; #460 (#459 #4420) (#125 #125) (#125 #125) (#1167 #1167) (#1167 #1167)
[attach-meaning] #370 arith (- 1)
[mk-app] #4610 + #1025 #4607
[mk-app] #4611 = #4610 #370
[inst-discovered] theory-solving 0 arith# ; #4609
[mk-app] #4614 = #4609 #4611
[instance] 0 #4614
[attach-enode] #4614 0
[end-of-instance]
[mk-app] #4614 or #3887 #4611
[instance] 0x61d50e6cbd30 ; 1
[attach-enode] #4606 1
[attach-enode] #4607 1
[attach-enode] #4610 1
[attach-enode] #4611 1
[mk-app] #4632 <= #4610 #370
[mk-app] #4637 >= #4610 #370
[assign] #4611 justification -1: 77
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4619
[mk-app] #4638 = #4619 #4619
[instance] 0 #4638
[attach-enode] #4638 0
[end-of-instance]
[mk-app] #4638 or #3941 #4471 #3930 #4617 #4618
[instance] 0x61d50e6cbd68 ; 1
[attach-enode] #4615 1
[attach-enode] #4616 1
[attach-enode] #4618 1
[end-of-instance]
[mk-app] #4640 or #3842 #4471 #4621
[instance] 0x61d50e6cbda8 ; 1
[assign] #4621 justification -1: 303 823
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4629
[mk-app] #4641 = #4629 #4629
[instance] 0 #4641
[attach-enode] #4641 0
[end-of-instance]
[mk-app] #4641 not #3312
[mk-app] #4643 or #4641 #4459 #4461 #3930 #4628
[instance] 0x61d50e6cbde8 ; 2
[attach-enode] #4458 2
[attach-enode] #4462 2
[attach-enode] #4624 2
[attach-enode] #4625 2
[attach-enode] #4626 2
[attach-enode] #4627 2
[attach-enode] #4628 2
[end-of-instance]
[assign] #4632 clause 827 -826
[assign] #4637 clause 828 -826
[assign] #4458 justification -1: 561 713
[mk-app] #4612 = #4606 #4615
[attach-meaning] #370 arith (- 1)
[mk-app] #4654 * #370 #4615
[mk-app] #4655 + #4606 #4654
[mk-app] #4656 <= #4655 #341
[mk-app] #4657 >= #4655 #341
[assign] #4612 justification -1: 515 783 508 805 39
[attach-enode] #4612 0
[attach-enode] #4654 0
[attach-enode] #4655 0
[assign] #4656 justification -1: 834
[assign] #4657 justification -1: 834
[assign] #4628 clause 833 -832
[eq-expl] #4625 cg (#125 #125) (#1167 #1167) (#4378 #1290) ; #1273
[eq-expl] #4626 cg (#4625 #659) (#296 #296) ; #378
[new-match] 0x61d50e6d7dc0 #2918 #348 #4626 ; #4627
[mk-app] #4658 not #2918
[mk-app] #4659 or #4658 #4631
[instance] 0x61d50e6d7dc0 ; 3
[attach-enode] #4644 3
[attach-enode] #4653 3
[attach-meaning] #370 arith (- 1)
[mk-app] #4660 * #370 #4644
[mk-app] #4661 + #378 #4660
[mk-app] #4662 <= #4661 #341
[mk-app] #4663 >= #4661 #341
[attach-enode] #4660 3
[attach-enode] #4661 3
[assign] (not #4630) justification -1: 56
[end-of-instance]
[assign] #4645 clause 837 843
[assign] #4642 clause 842 843
[assign] #4653 justification -1: 833 515 783 713 508 805 480 39
[mk-app] #4664 = #378 #4606
[attach-meaning] #370 arith (- 1)
[mk-app] #4623 + #378 #4607
[mk-app] #4620 <= #4623 #341
[mk-app] #4635 >= #4623 #341
[assign] #4664 justification -1: 508 805 39
[attach-enode] #4664 0
[attach-enode] #4623 0
[assign] #4620 justification -1: 844
[assign] #4635 justification -1: 844
[assign] #4662 clause 840 -839
[assign] #4663 clause 841 -839
[assign] #4647 clause 838 -807 -814 -827 -510 -846
[decide-and-or] #3735 #3677
[push] 5
[assign] (not #3665) decision axiom
[assign] #454 clause 517 519
[assign] (not #437) clause -518 519
[eq-expl] #434 cg (#659 #1025) ; #3638
[new-match] 0x61d50e6d84b8 #2578 #2577 #434 #1396 ; #436
[mk-app] #4636 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #3934 #3638
[mk-app] #4634 = #4636 #4114
[mk-app] #4633 or #4069 #4634
[instance] 0x61d50e6d84b8 ; 1
[attach-enode] #4636 1
[attach-enode] #4634 1
[assign] #4634 justification -1: 445
[end-of-instance]
[resolve-lit] 0 #437
[resolve-process] (not #437)
[resolve-lit] 0 (not #4634)
[resolve-lit] 1 (not #687)
[resolve-lit] 1 (not #4538)
[resolve-lit] 3 (not #4121)
[resolve-lit] 3 (not #4196)
[resolve-lit] 1 (not #4299)
[resolve-lit] 1 (not #4320)
[resolve-lit] 1 (not #696)
[resolve-lit] 4 (not #4068)
[resolve-lit] 1 (not #3701)
[resolve-lit] 1 (not #4426)
[resolve-lit] 4 (not #276)
[resolve-lit] 3 (not #4218)
[resolve-lit] 3 (not #4217)
[resolve-lit] 3 (not #4210)
[resolve-lit] 3 (not #4209)
[resolve-lit] 4 (not #4215)
[resolve-lit] 4 (not #4214)
[resolve-lit] 4 (not #4106)
[resolve-lit] 4 (not #4105)
[resolve-lit] 4 (not #4081)
[resolve-lit] 4 (not #4102)
[resolve-process] (not #4634)
[resolve-lit] 4 (not #2578)
[conflict] #437 (not #687) (not #4538) (not #4121) (not #4299) (not #696) (not #3701) (not #4426) (not #276) (not #2578)
[pop] 1 6
[assign] #437 clause 518 -503 -794 -692 -693 -495 -508 -805 -39 -445
[assign] #3665 clause 519 -518
[new-match] 0x61d50e6d84c8 #2578 #2577 #434 #1396 ; #436
[decide-and-or] #3735 #3729
[push] 5
[assign] (not #3728) decision axiom
[assign] #444 clause 520 525
[assign] #3679 clause 521 525
[assign] #3688 clause 522 525
[assign] #3739 clause 523 525
[assign] (not #3720) clause -524 525
[eq-expl] #3681 root
[new-match] 0x61d50e6d8518 #2578 #2577 #3681 #1396 ; #3719
[new-match] 0x61d50e6d8550 #199 #195 #3681 ; #3679 (#189 #189)
[new-match] 0x61d50e6d8580 #3320 #1722 #3681 #672 #1290 #1167 #125 ; #3695 (#459 #4420) (#125 #125) (#125 #125) (#1167 #1167) (#1167 #1167)
[new-match] 0x61d50e6d85d0 #3315 #1722 #3681 #672 #1290 #1167 #125 ; #3695 (#459 #4420) (#125 #125) (#125 #125) (#1167 #1167) (#1167 #1167)
[new-match] 0x61d50e6d8620 #3286 #1569 #3681 #459 #1167 #125 ; #3695
[mk-app] #4633 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #3934 #3681
[mk-app] #4613 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #3934 #2574 #3681
[mk-app] #4665 = #4633 #4613
[mk-app] #4666 or #4069 #4665
[instance] 0x61d50e6d8518 ; 1
[attach-enode] #4633 1
[attach-enode] #4613 1
[attach-enode] #4665 1
[assign] #4665 justification -1: 445
[end-of-instance]
[mk-app] #4667 I #3685
[mk-app] #4668 = #3681 #4667
[mk-app] #4669 or #3686 #4668
[mk-app] #4670 not #199
[mk-app] #4671 or #4670 #3686 #4668
[instance] 0x61d50e6d8550 ; 1
[attach-enode] #4667 1
[attach-enode] #4668 1
[assign] #4668 justification -1: 29 521
[end-of-instance]
[mk-app] #4672 = #3685 #4625
[mk-app] #4673 not #4672
[mk-app] #4674 vstd!seq.Seq.index.? #125 #1167 #4462 #3681
[mk-app] #4675 = #4674 #672
[mk-app] #4676 or #4459 #4461 #3686 #3930 #4673 #4675
[inst-discovered] theory-solving 0 basic# ; #4676
[mk-app] #4677 = #4676 #4676
[instance] 0 #4677
[attach-enode] #4677 0
[end-of-instance]
[mk-app] #4677 not #3320
[mk-app] #4678 or #4677 #4459 #4461 #3686 #3930 #4673 #4675
[instance] 0x61d50e6d8580 ; 2
[attach-enode] #4672 2
[attach-enode] #4674 2
[attach-enode] #4675 2
[end-of-instance]
[mk-app] #4679 * #370 #4625
[mk-app] #4680 + #3685 #4679
[mk-app] #4681 >= #4680 #341
[mk-app] #4682 vstd!seq.Seq.index.? #125 #1167 #4378 #3681
[mk-app] #4683 = #4674 #4682
[mk-app] #4684 or #4459 #4461 #3686 #3930 #3670 #4681 #4683
[inst-discovered] theory-solving 0 basic# ; #4684
[mk-app] #4685 = #4684 #4684
[instance] 0 #4685
[attach-enode] #4685 0
[end-of-instance]
[mk-app] #4685 not #3315
[mk-app] #4686 or #4685 #4459 #4461 #3686 #3930 #3670 #4681 #4683
[instance] 0x61d50e6d85d0 ; 2
[attach-enode] #4679 2
[attach-enode] #4680 2
[attach-enode] #4682 2
[attach-enode] #4683 2
[end-of-instance]
[mk-app] #4687 vstd!seq.Seq.index.? #125 #1167 #4420 #3681
[mk-app] #4688 has_type #4687 #1167
[mk-app] #4689 or #4471 #3686 #4688
[inst-discovered] theory-solving 0 basic# ; #4689
[mk-app] #4690 = #4689 #4689
[instance] 0 #4690
[attach-enode] #4690 0
[end-of-instance]
[mk-app] #4690 or #4373 #4471 #3686 #4688
[instance] 0x61d50e6d8620 ; 1
[attach-enode] #4687 1
[attach-enode] #4688 1
[assign] #4688 justification -1: 305 521 823
[end-of-instance]
[mk-app] #4691 = #1025 #4625
[attach-meaning] #370 arith (- 1)
[mk-app] #4692 + #1025 #4679
[mk-app] #4693 <= #4692 #341
[mk-app] #4694 >= #4692 #341
[assign] #4691 justification -1: 480 713
[attach-enode] #4691 0
[attach-enode] #4692 0
[assign] #4693 justification -1: 854
[assign] #4694 justification -1: 854
[eq-expl] #3681 lit #4668 ; #4667
[eq-expl] #4667 root
[new-match] 0x61d50e6d9220 #3440 #2356 #3681 #2574 #3934 ; #4613
[new-match] 0x61d50e6d9260 #4034 #4023 #3681 #2574 ; #4613 (#3934 #3934)
[eq-expl] #3685 root
[new-match] 0x61d50e6d9298 #174 #173 #3685 ; #4667
[eq-expl] #4687 cg (#125 #125) (#1167 #1167) (#4420 #459) (#3681 #3681) ; #3695
[eq-expl] #3695 root
[new-match] 0x61d50e6d92c8 #542 #236 #4687 #275 ; #4688 (#1167 #1167)
[new-match] 0x61d50e6d9300 #240 #236 #4687 #275 ; #4688 (#1167 #1167)
[new-match] 0x61d50e6d9338 #467 #466 #3685 ; #3679 (#189 #189) (#3681 #4667)
[mk-app] #4695 has_type #4667 #189
[mk-app] #4696 not #4695
[mk-app] #4697 lib!Chap28.MCSSSpec.MCSSSpec.spec_range_sum.? #3934 #2574 #4667
[mk-app] #4698 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #2574 #4667 #2352
[mk-app] #4699 = #4697 #4698
[mk-app] #4700 or #3966 #4072 #4696 #4699
[inst-discovered] theory-solving 0 basic# ; #4700
[mk-app] #4701 = #4700 #4700
[instance] 0 #4701
[attach-enode] #4701 0
[end-of-instance]
[mk-app] #4701 or #4078 #3966 #4072 #4696 #4699
[instance] 0x61d50e6d9220 ; 3
[attach-enode] #4695 3
[attach-enode] #4697 3
[attach-enode] #4698 3
[attach-enode] #4699 3
[end-of-instance]
[mk-app] #4702 %I #4667
[mk-app] #4703 + #4702 #4083
[mk-app] #4704 >= #4703 #341
[mk-app] #4705 not #4704
[mk-app] #4706 + #4702 #4001
[mk-app] #4707 <= #4706 #341
[mk-app] #4708 not #4707
[mk-app] #4709 >= #4697 #317
[mk-app] #4710 not #4709
[mk-app] #4711 <= #4697 #2373
[mk-app] #4712 not #4711
[mk-app] #4713 or #4710 #4712
[mk-app] #4714 not #4713
[mk-app] #4715 or #4072 #4696 #4082 #4705 #4708 #4714
[mk-app] #4716 + #4083 #4702
[inst-discovered] theory-solving 0 arith# ; #4703
[mk-app] #4717 = #4703 #4716
[instance] 0 #4717
[attach-enode] #4717 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4717 * #370 #4702
[mk-app] #4718 + #4080 #4717
[mk-app] #4719 <= #4718 #341
[mk-app] #4720 >= #4716 #341
[inst-discovered] theory-solving 0 arith# ; #4720
[mk-app] #4721 = #4720 #4719
[instance] 0 #4721
[attach-enode] #4721 0
[end-of-instance]
[mk-app] #4720 not #4719
[mk-app] #4721 + #4001 #4702
[inst-discovered] theory-solving 0 arith# ; #4706
[mk-app] #4722 = #4706 #4721
[instance] 0 #4722
[attach-enode] #4722 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4722 + #3967 #4717
[mk-app] #4723 >= #4722 #341
[mk-app] #4724 <= #4721 #341
[inst-discovered] theory-solving 0 arith# ; #4724
[mk-app] #4725 = #4724 #4723
[instance] 0 #4725
[attach-enode] #4725 0
[end-of-instance]
[mk-app] #4721 not #4723
[inst-discovered] theory-solving 0 basic# ; #4713
[mk-app] #4724 = #4713 #4713
[instance] 0 #4724
[attach-enode] #4724 0
[end-of-instance]
[mk-app] #4724 or #4072 #4696 #4082 #4720 #4721 #4714
[inst-discovered] theory-solving 0 basic# ; #4724
[mk-app] #4725 = #4724 #4724
[instance] 0 #4725
[attach-enode] #4725 0
[end-of-instance]
[mk-app] #4725 or #4097 #4072 #4696 #4082 #4720 #4721 #4714
[instance] 0x61d50e6d9260 ; 2
[attach-enode] #4702 2
[attach-enode] #4717 2
[attach-enode] #4718 2
[attach-enode] #4722 2
[end-of-instance]
[mk-app] #4724 has_type #3695 #1167
[mk-app] #4726 not #4724
[mk-app] #4727 iInv #275 #3709
[mk-app] #4728 or #4726 #4727
[mk-app] #4729 or #4449 #4726 #4727
[instance] 0x61d50e6d92c8 ; 2
[attach-enode] #4724 2
[attach-enode] #4727 2
[end-of-instance]
[mk-app] #4730 I #3709
[mk-app] #4731 = #3695 #4730
[mk-app] #4732 or #4726 #4731
[mk-app] #4733 or #4450 #4726 #4731
[instance] 0x61d50e6d9300 ; 2
[attach-enode] #4730 2
[attach-enode] #4731 2
[end-of-instance]
[assign] #4695 justification -1: 521 848
[assign] #4724 justification -1: 853 783
[mk-app] #4734 = #3685 #4702
[attach-meaning] #370 arith (- 1)
[mk-app] #4735 + #3685 #4717
[mk-app] #4736 <= #4735 #341
[mk-app] #4737 >= #4735 #341
[assign] #4734 justification -1: 848
[attach-enode] #4734 0
[attach-enode] #4735 0
[assign] #4736 justification -1: 867
[assign] #4737 justification -1: 867
[assign] #4699 clause 858 -857
[assign] #4727 clause 865 -864
[assign] #4731 clause 866 -864
[eq-expl] #3709 root
[new-match] 0x61d50e6df3a0 #1136 #455 #3709 #275 ; #4727
[new-match] 0x61d50e6df3d8 #2320 #2319 #2352 #4667 #2574 #3934 ; #4698
[new-match] 0x61d50e6df420 #3437 #2337 #2351 #4667 #2574 #3934 ; #4698 (#2352 #2352)
[new-match] 0x61d50e6df468 #174 #173 #3709 ; #4730
[eq-expl] #4730 lit #4731 ; #3695
[new-match] 0x61d50e6df498 #503 #499 #3709 #275 ; #4688 (#4687 #4730) (#1167 #1167)
[mk-app] #4738 + #3709 #3817
[mk-app] #4739 >= #4738 #341
[mk-app] #4740 not #4739
[mk-app] #4741 + #3709 #3821
[mk-app] #4742 >= #4741 #341
[mk-app] #4743 or #4740 #4742
[mk-app] #4744 = #4743 #4727
[mk-app] #4745 not #4744
[mk-app] #4746 + #3817 #3709
[inst-discovered] theory-solving 0 arith# ; #4738
[mk-app] #4747 = #4738 #4746
[instance] 0 #4747
[attach-enode] #4747 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4747 * #370 #3709
[mk-app] #4748 + #313 #4747
[mk-app] #4749 <= #4748 #341
[mk-app] #4750 >= #4746 #341
[inst-discovered] theory-solving 0 arith# ; #4750
[mk-app] #4751 = #4750 #4749
[instance] 0 #4751
[attach-enode] #4751 0
[end-of-instance]
[mk-app] #4746 not #4749
[mk-app] #4750 + #3821 #3709
[inst-discovered] theory-solving 0 arith# ; #4741
[mk-app] #4751 = #4741 #4750
[instance] 0 #4751
[attach-enode] #4751 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4751 + #335 #4747
[mk-app] #4752 <= #4751 #341
[mk-app] #4753 >= #4750 #341
[inst-discovered] theory-solving 0 arith# ; #4753
[mk-app] #4754 = #4753 #4752
[instance] 0 #4754
[attach-enode] #4754 0
[end-of-instance]
[mk-app] #4750 or #4746 #4752
[mk-app] #4753 = #4750 #4727
[mk-app] #4754 not #4750
[mk-app] #4755 not #4753
[inst-discovered] theory-solving 0 basic# ; #4755
[mk-app] #4754 = #4755 #4755
[instance] 0 #4754
[attach-enode] #4754 0
[end-of-instance]
[mk-app] #4754 or #3836 #4755
[instance] 0x61d50e6df3a0 ; 3
[attach-enode] #4747 3
[attach-enode] #4748 3
[attach-enode] #4751 3
[assign] (not #4753) justification -1: 61
[end-of-instance]
[mk-app] #4756 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #2574 #4667 #2316
[mk-app] #4757 = #4698 #4756
[mk-app] #4758 or #4222 #4757
[instance] 0x61d50e6df3d8 ; 4
[attach-enode] #4756 4
[attach-enode] #4757 4
[assign] #4757 justification -1: 414
[end-of-instance]
[mk-app] #4759 >= #4718 #341
[mk-app] #4760 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #4395 #4667 #2351
[mk-app] #4761 Add #4393 #4760
[mk-app] #4762 if #4759 #341 #4761
[mk-app] #4763 = #4698 #4762
[mk-app] #4764 or #3966 #4072 #4696 #4763
[inst-discovered] theory-solving 0 basic# ; #4764
[mk-app] #4765 = #4764 #4764
[instance] 0 #4765
[attach-enode] #4765 0
[end-of-instance]
[mk-app] #4765 or #4240 #3966 #4072 #4696 #4763
[instance] 0x61d50e6df420 ; 4
[mk-app] #4766 = #4762 #341
[mk-app] #4767 = #4761 #4762
[attach-enode] #4762 4
[attach-enode] #4760 4
[attach-enode] #4761 4
[attach-enode] #4766 4
[attach-enode] #4767 4
[attach-enode] #4763 4
[assign] #4763 justification -1: 415 626 658 857
[end-of-instance]
[assign] (not #4750) clause -872 873
[assign] #4749 clause 870 872
[assign] (not #4752) clause -871 872
[mk-app] #4768 = #3709 #4697
[attach-meaning] #370 arith (- 1)
[mk-app] #4769 * #370 #4697
[mk-app] #4770 + #3709 #4769
[mk-app] #4771 <= #4770 #341
[mk-app] #4772 >= #4770 #341
[attach-enode] #4768 0
[attach-enode] #4769 0
[attach-enode] #4770 0
[new-match] 0x61d50e6dfeb0 #2320 #2319 #2316 #4667 #2574 #3934 ; #4756
[assign] (not #4768) justification -1: -524 847 848 592
[decide-and-or] #3942 #3969
[push] 6
[assign] (not #3968) decision axiom
[assign] (not #4494) justification -1: -627 696
[push] 7
[assign] (not #3975) decision axiom
[assign] (not #3980) clause -630 629
[push] 8
[assign] (not #4382) decision axiom
[assign] #4243 clause 716 694
[eq-expl] #4396 root
[new-match] 0x61d50e6e0030 #552 #550 #4396 #4393 ; #4397
[new-match] 0x61d50e6e0068 #2320 #2319 #2351 #1276 #4395 #3934 ; #4396
[mk-app] #4773 + #4393 #4396 #4496
[inst-discovered] theory-solving 0 arith# ; #4497
[mk-app] #4774 = #4497 #4773
[instance] 0 #4774
[attach-enode] #4774 0
[end-of-instance]
[mk-app] #4774 = #4773 #341
[mk-app] #4775 or #3887 #4774
[instance] 0x61d50e6e0030 ; 4
[attach-enode] #4496 4
[attach-enode] #4773 4
[attach-enode] #4774 4
[mk-app] #4776 <= #4773 #341
[mk-app] #4777 >= #4773 #341
[assign] #4774 justification -1: 77
[end-of-instance]
[mk-app] #4778 or #4222 #4511
[instance] 0x61d50e6e0068 ; 4
[attach-enode] #4510 4
[attach-enode] #4511 4
[assign] #4511 justification -1: 414
[end-of-instance]
[assign] #4776 clause 883 -882
[assign] #4777 clause 884 -882
[mk-app] #4779 = #2899 #4397
[attach-meaning] #370 arith (- 1)
[mk-app] #4780 + #2899 #4496
[mk-app] #4781 <= #4780 #341
[mk-app] #4782 >= #4780 #341
[assign] #4779 justification -1: 716 717 660 657 592
[attach-enode] #4779 0
[attach-enode] #4780 0
[assign] #4781 justification -1: 886
[assign] #4782 justification -1: 886
[new-match] 0x61d50e6e06a8 #2320 #2319 #2316 #1276 #4395 #3934 ; #4510
[decide-and-or] #4638 #4617
[push] 9
[assign] (not #4616) decision axiom
[mk-app] #4783 <= #4615 #341
[assign] (not #4783) justification -1: -829 831
[decide-and-or] #4678 #4673
[push] 10
[assign] (not #4672) decision axiom
[attach-meaning] #370 arith (- 1)
[mk-app] #4784 <= #4680 #341
[decide-and-or] #4686 #4681
[push] 11
[assign] #4681 decision axiom
[assign] (not #4784) clause -890 -851
[resolve-process] true
[resolve-lit] 0 #4784
[resolve-lit] 6 (not #4693)
[resolve-lit] 7 (not #4594)
[resolve-lit] 7 (not #4587)
[resolve-lit] 6 (not #3739)
[conflict] #4784 (not #4693) (not #4594) (not #4587) (not #3739)
[pop] 6 12
[assign] #4784 clause 882 -855 -814 -807 -523
[decide-and-or] #3942 #3969
[push] 6
[assign] (not #3968) decision axiom
[assign] (not #4494) justification -1: -627 696
[push] 7
[assign] (not #3975) decision axiom
[assign] (not #3980) clause -630 629
[push] 8
[assign] (not #4382) decision axiom
[assign] #4243 clause 716 694
[eq-expl] #4396 root
[new-match] 0x61d50e6e0030 #552 #550 #4396 #4393 ; #4397
[new-match] 0x61d50e6e0068 #2320 #2319 #2351 #1276 #4395 #3934 ; #4396
[mk-app] #4773 + #4393 #4396 #4496
[inst-discovered] theory-solving 0 arith# ; #4497
[mk-app] #4774 = #4497 #4773
[instance] 0 #4774
[attach-enode] #4774 0
[end-of-instance]
[mk-app] #4774 = #4773 #341
[mk-app] #4776 or #3887 #4774
[instance] 0x61d50e6e0030 ; 4
[attach-enode] #4496 4
[attach-enode] #4773 4
[attach-enode] #4774 4
[mk-app] #4777 <= #4773 #341
[mk-app] #4779 >= #4773 #341
[assign] #4774 justification -1: 77
[end-of-instance]
[mk-app] #4780 or #4222 #4511
[instance] 0x61d50e6e0068 ; 4
[attach-enode] #4510 4
[attach-enode] #4511 4
[assign] #4511 justification -1: 414
[end-of-instance]
[assign] #4777 clause 884 -883
[assign] #4779 clause 885 -883
[mk-app] #4781 = #2899 #4397
[attach-meaning] #370 arith (- 1)
[mk-app] #4782 + #2899 #4496
[mk-app] #4783 <= #4782 #341
[mk-app] #4778 >= #4782 #341
[assign] #4781 justification -1: 716 717 660 657 592
[attach-enode] #4781 0
[attach-enode] #4782 0
[assign] #4783 justification -1: 887
[assign] #4778 justification -1: 887
[new-match] 0x61d50e6e06a8 #2320 #2319 #2316 #1276 #4395 #3934 ; #4510
[decide-and-or] #4638 #4617
[push] 9
[assign] (not #4616) decision axiom
[mk-app] #4775 <= #4615 #341
[assign] (not #4775) justification -1: -829 831
[decide-and-or] #4678 #4673
[push] 10
[assign] (not #4672) decision axiom
[attach-meaning] #370 arith (- 1)
[assign] (not #4681) justification -1: -849 882
[assign] #4683 clause 852 851
[new-match] 0x61d50e6e08e8 #3286 #1569 #3681 #4378 #1167 #125 ; #4682
[new-match] 0x61d50e6e0930 #3726 #1104 #3681 ; #4682 (#125 #125) (#1167 #1167) (#4378 #1290)
[mk-app] #4785 >= #4702 #341
[mk-app] #4786 not #4785
[mk-app] #4787 + #4702 #3631
[mk-app] #4788 <= #4787 #341
[mk-app] #4789 not #4788
[mk-app] #4790 vstd!seq.Seq.index.? #125 #1167 #1290 #4667
[mk-app] #4791 %I #4790
[mk-app] #4792 lib!Chap28.MCSSSpec.MCSSSpec.spec_prefix_sum.? #1396 #4667
[mk-app] #4793 = #4791 #4792
[mk-app] #4794 or #4696 #4786 #4789 #4793
[mk-app] #4795 + #3631 #4702
[inst-discovered] theory-solving 0 arith# ; #4787
[mk-app] #4796 = #4787 #4795
[instance] 0 #4796
[attach-enode] #4796 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4796 + #1454 #4717
[mk-app] #4797 >= #4796 #341
[mk-app] #4798 <= #4795 #341
[inst-discovered] theory-solving 0 arith# ; #4798
[mk-app] #4799 = #4798 #4797
[instance] 0 #4799
[attach-enode] #4799 0
[end-of-instance]
[mk-app] #4795 not #4797
[mk-app] #4798 or #4696 #4786 #4795 #4793
[mk-app] #4799 not #3726
[mk-app] #4800 or #4799 #4696 #4786 #4795 #4793
[instance] 0x61d50e6e0930 ; 3
[attach-enode] #4796 3
[attach-enode] #4790 3
[attach-enode] #4791 3
[attach-enode] #4792 3
[attach-enode] #4793 3
[end-of-instance]
[assign] (not #4793) justification -1: -524 848 852 783 713
[decide-and-or] #4725 #4720
[push] 11
[assign] (not #4719) decision axiom
[assign] #4759 clause 875 859
[assign] #4766 clause 876 -875
[mk-app] #4798 = #4697 #341
[mk-app] #4801 <= #4697 #341
[mk-app] #4802 >= #4697 #341
[assign] #4798 justification -1: 858 878 876
[attach-enode] #4798 0
[assign] #4801 justification -1: 894
[assign] #4802 justification -1: 894
[resolve-process] true
[resolve-lit] 6 (not #4736)
[resolve-lit] 10 (not #4102)
[resolve-lit] 0 #4719
[resolve-lit] 6 (not #3688)
[conflict] #4719 (not #4736) (not #3688)
[pop] 6 12
[assign] #4719 clause 859 -868 -522
[decide-and-or] #3942 #3969
[push] 6
[assign] (not #3968) decision axiom
[assign] (not #4494) justification -1: -627 696
[push] 7
[assign] (not #3975) decision axiom
[assign] (not #3980) clause -630 629
[push] 8
[assign] (not #4382) decision axiom
[assign] #4243 clause 716 694
[eq-expl] #4396 root
[new-match] 0x61d50e6e0030 #552 #550 #4396 #4393 ; #4397
[new-match] 0x61d50e6e0068 #2320 #2319 #2351 #1276 #4395 #3934 ; #4396
[mk-app] #4773 + #4393 #4396 #4496
[inst-discovered] theory-solving 0 arith# ; #4497
[mk-app] #4774 = #4497 #4773
[instance] 0 #4774
[attach-enode] #4774 0
[end-of-instance]
[mk-app] #4774 = #4773 #341
[mk-app] #4777 or #3887 #4774
[instance] 0x61d50e6e0030 ; 4
[attach-enode] #4496 4
[attach-enode] #4773 4
[attach-enode] #4774 4
[mk-app] #4779 <= #4773 #341
[mk-app] #4781 >= #4773 #341
[assign] #4774 justification -1: 77
[end-of-instance]
[mk-app] #4782 or #4222 #4511
[instance] 0x61d50e6e0068 ; 4
[attach-enode] #4510 4
[attach-enode] #4511 4
[assign] #4511 justification -1: 414
[end-of-instance]
[assign] #4779 clause 884 -883
[assign] #4781 clause 885 -883
[mk-app] #4783 = #2899 #4397
[attach-meaning] #370 arith (- 1)
[mk-app] #4778 + #2899 #4496
[mk-app] #4775 <= #4778 #341
[mk-app] #4798 >= #4778 #341
[assign] #4783 justification -1: 716 717 660 657 592
[attach-enode] #4783 0
[attach-enode] #4778 0
[assign] #4775 justification -1: 887
[assign] #4798 justification -1: 887
[new-match] 0x61d50e6e06a8 #2320 #2319 #2316 #1276 #4395 #3934 ; #4510
[decide-and-or] #4638 #4617
[push] 9
[assign] (not #4616) decision axiom
[mk-app] #4801 <= #4615 #341
[assign] (not #4801) justification -1: -829 831
[decide-and-or] #4678 #4673
[push] 10
[assign] (not #4672) decision axiom
[attach-meaning] #370 arith (- 1)
[assign] (not #4681) justification -1: -849 882
[assign] #4683 clause 852 851
[new-match] 0x61d50e6e08e8 #3286 #1569 #3681 #4378 #1167 #125 ; #4682
[new-match] 0x61d50e6e0930 #3726 #1104 #3681 ; #4682 (#125 #125) (#1167 #1167) (#4378 #1290)
[mk-app] #4802 or #4696 #4786 #4795 #4793
[mk-app] #4799 not #3726
[mk-app] #4800 or #4799 #4696 #4786 #4795 #4793
[instance] 0x61d50e6e0930 ; 3
[attach-enode] #4796 3
[attach-enode] #4790 3
[attach-enode] #4791 3
[attach-enode] #4792 3
[attach-enode] #4793 3
[end-of-instance]
[assign] (not #4793) justification -1: -524 848 852 783 713
[decide-and-or] #4725 #4721
[push] 11
[assign] (not #4723) decision axiom
[resolve-process] true
[resolve-lit] 6 (not #4737)
[resolve-lit] 0 #4723
[resolve-lit] 10 (not #4214)
[resolve-lit] 7 (not #4594)
[resolve-lit] 7 (not #4587)
[resolve-lit] 6 (not #3739)
[conflict] #4723 (not #4737) (not #4594) (not #4587) (not #3739)
[pop] 6 12
[assign] #4723 clause 860 -869 -814 -807 -523
[assign] (not #4713) clause -863 -860 -857 -859
[assign] #4709 clause 861 863
[assign] #4711 clause 862 863
[decide-and-or] #3942 #3969
[push] 6
[assign] (not #3968) decision axiom
[assign] (not #4494) justification -1: -627 696
[push] 7
[assign] (not #3975) decision axiom
[assign] (not #3980) clause -630 629
[push] 8
[assign] (not #4382) decision axiom
[assign] #4243 clause 716 694
[eq-expl] #4396 root
[new-match] 0x61d50e6e0030 #552 #550 #4396 #4393 ; #4397
[new-match] 0x61d50e6e0068 #2320 #2319 #2351 #1276 #4395 #3934 ; #4396
[mk-app] #4773 + #4393 #4396 #4496
[inst-discovered] theory-solving 0 arith# ; #4497
[mk-app] #4774 = #4497 #4773
[instance] 0 #4774
[attach-enode] #4774 0
[end-of-instance]
[mk-app] #4774 = #4773 #341
[mk-app] #4779 or #3887 #4774
[instance] 0x61d50e6e0030 ; 4
[attach-enode] #4496 4
[attach-enode] #4773 4
[attach-enode] #4774 4
[mk-app] #4781 <= #4773 #341
[mk-app] #4783 >= #4773 #341
[assign] #4774 justification -1: 77
[end-of-instance]
[mk-app] #4778 or #4222 #4511
[instance] 0x61d50e6e0068 ; 4
[attach-enode] #4510 4
[attach-enode] #4511 4
[assign] #4511 justification -1: 414
[end-of-instance]
[assign] #4781 clause 884 -883
[assign] #4783 clause 885 -883
[mk-app] #4775 = #2899 #4397
[attach-meaning] #370 arith (- 1)
[mk-app] #4798 + #2899 #4496
[mk-app] #4801 <= #4798 #341
[mk-app] #4799 >= #4798 #341
[assign] #4775 justification -1: 716 717 660 657 592
[attach-enode] #4775 0
[attach-enode] #4798 0
[assign] #4801 justification -1: 887
[assign] #4799 justification -1: 887
[new-match] 0x61d50e6e06a8 #2320 #2319 #2316 #1276 #4395 #3934 ; #4510
[decide-and-or] #4638 #4617
[push] 9
[assign] (not #4616) decision axiom
[mk-app] #4800 <= #4615 #341
[assign] (not #4800) justification -1: -829 831
[decide-and-or] #4678 #4673
[push] 10
[assign] (not #4672) decision axiom
[attach-meaning] #370 arith (- 1)
[assign] (not #4681) justification -1: -849 882
[assign] #4683 clause 852 851
[new-match] 0x61d50e6e08e8 #3286 #1569 #3681 #4378 #1167 #125 ; #4682
[new-match] 0x61d50e6e0930 #3726 #1104 #3681 ; #4682 (#125 #125) (#1167 #1167) (#4378 #1290)
[mk-app] #4782 or #4696 #4786 #4795 #4793
[mk-app] #4777 not #3726
[mk-app] #4802 or #4777 #4696 #4786 #4795 #4793
[instance] 0x61d50e6e0930 ; 3
[attach-enode] #4796 3
[attach-enode] #4790 3
[attach-enode] #4791 3
[attach-enode] #4792 3
[attach-enode] #4793 3
[end-of-instance]
[assign] (not #4793) justification -1: -524 848 852 783 713
[push] 11
[assign] (not #4759) decision axiom
[assign] #4767 clause 877 875
[assign] #4785 clause 891 -859 -661
[eq-expl] #4760 root
[new-match] 0x61d50e6e0d70 #552 #550 #4760 #4393 ; #4761
[new-match] 0x61d50e6e0da8 #2320 #2319 #2351 #4667 #4395 #3934 ; #4760
[mk-app] #4782 * #370 #4761
[mk-app] #4780 + #4760 #4393 #4782
[mk-app] #4776 = #4780 #341
[mk-app] #4803 + #4393 #4760 #4782
[inst-discovered] theory-solving 0 arith# ; #4780
[mk-app] #4804 = #4780 #4803
[instance] 0 #4804
[attach-enode] #4804 0
[end-of-instance]
[mk-app] #4804 = #4803 #341
[mk-app] #4805 or #3887 #4804
[instance] 0x61d50e6e0d70 ; 5
[attach-enode] #4782 5
[attach-enode] #4803 5
[attach-enode] #4804 5
[mk-app] #4806 <= #4803 #341
[mk-app] #4807 >= #4803 #341
[assign] #4804 justification -1: 77
[end-of-instance]
[mk-app] #4808 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #4395 #4667 #2316
[mk-app] #4809 = #4760 #4808
[mk-app] #4810 or #4222 #4809
[instance] 0x61d50e6e0da8 ; 5
[attach-enode] #4808 5
[attach-enode] #4809 5
[assign] #4809 justification -1: 414
[end-of-instance]
[assign] (not #4797) clause -892 -891 893
[assign] #4806 clause 895 -894
[assign] #4807 clause 896 -894
[mk-app] #4811 = #4697 #4761
[attach-meaning] #370 arith (- 1)
[mk-app] #4812 + #4697 #4782
[mk-app] #4813 <= #4812 #341
[mk-app] #4814 >= #4812 #341
[assign] #4811 justification -1: 877 878 858
[attach-enode] #4811 0
[attach-enode] #4812 0
[assign] #4813 justification -1: 898
[assign] #4814 justification -1: 898
[resolve-process] true
[resolve-lit] 6 (not #4737)
[resolve-lit] 6 (not #4694)
[resolve-lit] 0 #4797
[resolve-lit] 10 (not #4214)
[resolve-lit] 1 #4681
[conflict] #4797 (not #4737) (not #4694) #4681
[pop] 1 12
[assign] #4785 clause 891 -859 -661
[assign] #4797 clause 892 851 -856 -869
[resolve-process] true
[resolve-lit] 0 (not #4797)
[resolve-lit] 0 (not #4785)
[resolve-lit] 0 #4793
[resolve-lit] 5 (not #4695)
[resolve-process] (not #4797)
[resolve-lit] 0 #4681
[resolve-lit] 5 (not #4694)
[resolve-lit] 5 (not #4737)
[resolve-process] (not #4785)
[resolve-lit] 5 (not #4719)
[resolve-lit] 9 (not #4081)
[resolve-process] #4793
[resolve-lit] 5 #3720
[resolve-lit] 5 (not #4668)
[resolve-lit] 0 (not #4683)
[resolve-lit] 6 (not #4421)
[resolve-lit] 9 (not #4379)
[resolve-process] (not #4683)
[resolve-lit] 5 (not #3679)
[resolve-lit] 5 (not #3688)
[resolve-lit] 6 (not #4460)
[resolve-lit] 6 (not #4458)
[conflict] #4681 #3720 (not #4421) (not #4379) (not #3679) (not #3688) (not #4460) (not #4458)
[pop] 5 11
[assign] #4785 clause 883 -859 -661
[attach-enode] #4796 0
[assign] #4681 clause 851 524 -783 -713 -521 -522 -825 -832
[assign] (not #4797) clause -884 -851 -677 -584 -855 -868 -574
[assign] #4672 justification -1: 508 805 480 713 39 848 869 868 851 882 856 855 815 814 807 806
[assign] #4767 justification -1: 722 723 718 878 858 847 518 503 794 692 568 752 693 781 539 495 481 657 848 508 805 39 508 805 592 848 699 704 698 703 677 680 584 583 574 573 670 669 661 668 661 668 869 868 851 882 856 855 815 814 807 806
[mk-app] #4773 = #686 #4697
[attach-meaning] #370 arith (- 1)
[mk-app] #4774 + #686 #4769
[mk-app] #4781 <= #4774 #341
[mk-app] #4783 >= #4774 #341
[assign] #4773 justification -1: 847 518 503 848 508 805 39 508 805 592 848 869 868 851 882 856 855 815 814 807 806
[attach-enode] #4773 0
[attach-enode] #4774 0
[assign] #4781 justification -1: 885
[assign] #4783 justification -1: 885
[new-match] 0x61d50e6e0680 #199 #195 #3638 ; #4161 (#189 #189)
[new-match] 0x61d50e6e06b0 #467 #466 #1025 ; #4161 (#189 #189) (#3638 #3638)
[assign] #4675 clause 850 -849
[resolve-lit] 0 #3720
[resolve-process] (not #3720)
[resolve-lit] 1 (not #4430)
[resolve-lit] 1 (not #437)
[resolve-lit] 1 (not #687)
[resolve-lit] 0 (not #4668)
[resolve-lit] 1 (not #3701)
[resolve-lit] 1 (not #4426)
[resolve-lit] 4 (not #276)
[resolve-lit] 0 (not #4675)
[resolve-lit] 1 (not #4421)
[resolve-lit] 4 (not #4379)
[resolve-lit] 0 (not #4737)
[resolve-lit] 0 (not #4736)
[resolve-lit] 0 (not #4681)
[resolve-lit] 0 (not #4784)
[resolve-lit] 0 (not #4694)
[resolve-lit] 0 (not #4693)
[resolve-lit] 1 (not #4595)
[resolve-lit] 1 (not #4594)
[resolve-lit] 1 (not #4587)
[resolve-lit] 1 (not #4586)
[resolve-process] (not #4675)
[resolve-lit] 0 (not #4672)
[resolve-lit] 0 (not #3679)
[resolve-lit] 1 (not #4460)
[resolve-lit] 1 (not #4458)
[resolve-process] (not #4672)
[resolve-process] (not #4681)
[resolve-lit] 0 (not #3688)
[resolve-process] (not #4784)
[resolve-lit] 0 (not #3739)
[resolve-process] (not #4737)
[resolve-lit] 0 (not #4734)
[resolve-process] (not #4736)
[resolve-process] (not #4734)
[resolve-process] (not #4694)
[resolve-lit] 0 (not #4691)
[resolve-process] (not #4693)
[resolve-process] (not #4691)
[resolve-process] (not #4668)
[resolve-process] #3720
[resolve-lit] 0 #3728
[resolve-process] (not #3739)
[resolve-process] (not #3688)
[resolve-process] (not #3679)
[conflict] #3728 (not #437) (not #687) (not #3701) (not #4426) (not #276) (not #4421) (not #4379) (not #4460) (not #4458)
[pop] 1 6
[attach-enode] #4679 0
[attach-enode] #4680 0
[attach-enode] #4692 0
[attach-enode] #4667 0
[attach-enode] #4702 0
[attach-enode] #4717 0
[attach-enode] #4718 0
[attach-enode] #4735 0
[attach-enode] #4722 0
[attach-enode] #4796 0
[assign] #3728 clause 525 -518 -503 -508 -805 -39 -783 -713 -825 -832
[assign] (not #3676) clause -528 -525 -513 -516 -519 -529
[assign] #383 clause 526 528
[assign] (not #354) clause -527 528
[mk-app] #4691 = #1025 #4625
[attach-meaning] #370 arith (- 1)
[assign] #4691 justification -1: 480 713
[attach-enode] #4691 0
[assign] #4693 justification -1: 857
[assign] #4694 justification -1: 857
[eq-expl] #384 root
[eq-expl] #2886 lit #2890 ; #2889
[eq-expl] #2889 root
[eq-expl] #2 root
[new-match] 0x61d50e6d8c00 #1090 #770 #2 #2886 #384 ; #354
[new-match] 0x61d50e6d8c40 #567 #559 #659 #1487 ; #384
[mk-app] #4734 check_decrease_int #384 #2889 #2
[mk-app] #4750 >= #384 #341
[mk-app] #4753 not #4750
[mk-app] #4766 * #370 #384
[mk-app] #4767 + #2889 #4766
[mk-app] #4768 <= #4767 #341
[mk-app] #4769 or #4753 #4768
[mk-app] #4770 not #4769
[mk-app] #4771 = #384 #2889
[mk-app] #4772 not #4771
[mk-app] #4773 not #2
[mk-app] #4774 or #4772 #4773
[mk-app] #4781 not #4774
[mk-app] #4783 or #4770 #4781
[mk-app] #4765 = #4734 #4783
[mk-app] #4758 + #4766 #2889
[inst-discovered] theory-solving 0 arith# ; #4767
[mk-app] #4755 = #4767 #4758
[instance] 0 #4755
[attach-enode] #4755 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4755 * #370 #2889
[mk-app] #4754 + #384 #4755
[mk-app] #4733 >= #4754 #341
[mk-app] #4729 <= #4758 #341
[inst-discovered] theory-solving 0 arith# ; #4729
[mk-app] #4721 = #4729 #4733
[instance] 0 #4721
[attach-enode] #4721 0
[end-of-instance]
[mk-app] #4758 or #4753 #4733
[mk-app] #4729 not #4758
[inst-discovered] theory-solving 0 basic# ; #4773
[mk-app] #4721 = #4773 #1
[instance] 0 #4721
[attach-enode] #4721 0
[end-of-instance]
[mk-app] #4721 or #4772 #1
[inst-discovered] theory-solving 0 basic# ; #4721
[mk-app] #4725 = #4721 #1
[instance] 0 #4725
[attach-enode] #4725 0
[end-of-instance]
[mk-app] #4721 not #1
[inst-discovered] theory-solving 0 basic# ; #4721
[mk-app] #4725 = #4721 #2
[instance] 0 #4725
[attach-enode] #4725 0
[end-of-instance]
[mk-app] #4721 or #4729 #2
[inst-discovered] theory-solving 0 basic# ; #4721
[mk-app] #4725 = #4721 #4729
[instance] 0 #4725
[attach-enode] #4725 0
[end-of-instance]
[mk-app] #4721 = #4758 #4734
[mk-app] #4725 not #4721
[mk-app] #4701 = #4734 #4729
[inst-discovered] theory-solving 0 basic# ; #4701
[mk-app] #4690 = #4701 #4725
[instance] 0 #4690
[attach-enode] #4690 0
[end-of-instance]
[mk-app] #4729 not #4758
[inst-discovered] theory-solving 0 basic# ; #4725
[mk-app] #4729 = #4725 #4725
[instance] 0 #4729
[attach-enode] #4729 0
[end-of-instance]
[mk-app] #4729 not #1090
[mk-app] #4701 or #4729 #4725
[instance] 0x61d50e6d8c00 ; 1
[attach-enode] #4755 1
[attach-enode] #4754 1
[attach-enode] #4734 1
[assign] (not #4721) justification -1: 98
[end-of-instance]
[mk-app] #4690 Sub #1379 #1025
[mk-app] #4685 + #1025 #3873 #4690
[mk-app] #4686 = #4685 #341
[mk-app] #4677 or #3877 #4686
[instance] 0x61d50e6d8c40 ; 1
[attach-enode] #3873 1
[attach-enode] #4690 1
[attach-enode] #4685 1
[attach-enode] #4686 1
[mk-app] #4678 <= #4685 #341
[mk-app] #4670 >= #4685 #341
[assign] #4686 justification -1: 78
[end-of-instance]
[assign] #4678 clause 864 -863
[assign] #4670 clause 865 -863
[assign] (not #4734) justification -1: -527 483
[mk-app] #4671 = #2889 #3874
[attach-meaning] #370 arith (- 1)
[mk-app] #4666 + #2889 #3878
[mk-app] #4775 <= #4666 #341
[mk-app] #4798 >= #4666 #341
[assign] #4671 justification -1: 478
[attach-enode] #4671 0
[attach-enode] #4666 0
[assign] #4775 justification -1: 866
[assign] #4798 justification -1: 866
[mk-app] #4801 = #384 #4690
[attach-meaning] #370 arith (- 1)
[mk-app] #4799 * #370 #4690
[mk-app] #4800 + #384 #4799
[mk-app] #4777 <= #4800 #341
[mk-app] #4802 >= #4800 #341
[assign] #4801 justification -1: 508 805 39 478
[attach-enode] #4801 0
[attach-enode] #4799 0
[attach-enode] #4800 0
[assign] #4777 justification -1: 869
[assign] #4802 justification -1: 869
[assign] #4758 clause 860 861 862
[decide-and-or] #3728 #3723
[push] 5
[assign] (not #444) decision axiom
[decide-and-or] #3942 #3969
[push] 6
[assign] (not #3968) decision axiom
[assign] (not #4494) justification -1: -627 696
[push] 7
[assign] (not #3975) decision axiom
[assign] (not #3980) clause -630 629
[push] 8
[assign] (not #4382) decision axiom
[assign] #4243 clause 716 694
[eq-expl] #4396 root
[new-match] 0x61d50e6d9960 #552 #550 #4396 #4393 ; #4397
[new-match] 0x61d50e6d9998 #2320 #2319 #2351 #1276 #4395 #3934 ; #4396
[mk-app] #4778 + #4393 #4396 #4496
[inst-discovered] theory-solving 0 arith# ; #4497
[mk-app] #4779 = #4497 #4778
[instance] 0 #4779
[attach-enode] #4779 0
[end-of-instance]
[mk-app] #4779 = #4778 #341
[mk-app] #4803 or #3887 #4779
[instance] 0x61d50e6d9960 ; 4
[attach-enode] #4496 4
[attach-enode] #4778 4
[attach-enode] #4779 4
[mk-app] #4804 <= #4778 #341
[mk-app] #4806 >= #4778 #341
[assign] #4779 justification -1: 77
[end-of-instance]
[mk-app] #4807 or #4222 #4511
[instance] 0x61d50e6d9998 ; 4
[attach-enode] #4510 4
[attach-enode] #4511 4
[assign] #4511 justification -1: 414
[end-of-instance]
[assign] #4804 clause 873 -872
[assign] #4806 clause 874 -872
[mk-app] #4811 = #2899 #4397
[attach-meaning] #370 arith (- 1)
[mk-app] #4812 + #2899 #4496
[mk-app] #4813 <= #4812 #341
[mk-app] #4814 >= #4812 #341
[assign] #4811 justification -1: 716 717 660 657 592
[attach-enode] #4811 0
[attach-enode] #4812 0
[assign] #4813 justification -1: 876
[assign] #4814 justification -1: 876
[new-match] 0x61d50e6df360 #2320 #2319 #2316 #1276 #4395 #3934 ; #4510
[decide-and-or] #4638 #4617
[push] 9
[assign] (not #4616) decision axiom
[mk-app] #4810 <= #4615 #341
[assign] (not #4810) justification -1: -829 831
[decide-and-or] #4758 #4753
[push] 10
[assign] (not #4750) decision axiom
[resolve-process] true
[resolve-lit] 9 (not #4214)
[resolve-lit] 0 #4750
[resolve-lit] 6 (not #4802)
[resolve-lit] 6 (not #4670)
[conflict] #4750 (not #4802)
[pop] 6 11
[assign] #4750 clause 858 -871
[assign] #4733 clause 859 -858 -860
[resolve-process] true
[resolve-lit] 3 (not #4215)
[resolve-lit] 0 (not #4777)
[resolve-lit] 0 (not #4733)
[resolve-lit] 0 (not #4798)
[resolve-lit] 0 (not #4678)
[resolve-process] (not #4733)
[resolve-lit] 0 (not #4750)
[resolve-lit] 0 (not #4758)
[resolve-process] (not #4750)
[resolve-lit] 0 (not #4802)
[resolve-process] (not #4758)
[resolve-lit] 0 #4734
[resolve-lit] 0 #4721
[resolve-process] (not #4802)
[resolve-lit] 0 (not #4801)
[resolve-process] (not #4777)
[resolve-process] (not #4801)
[resolve-lit] 0 (not #3701)
[resolve-lit] 0 (not #4426)
[resolve-lit] 3 (not #276)
[resolve-process] (not #4798)
[resolve-lit] 0 (not #4671)
[resolve-process] (not #4671)
[resolve-process] #4734
[resolve-lit] 0 #354
[resolve-process] (not #4678)
[resolve-lit] 0 (not #4686)
[resolve-process] (not #4686)
[resolve-process] #4721
[resolve-process] #354
[resolve-lit] 0 #3676
[resolve-process] #3676
[resolve-lit] 0 (not #3728)
[resolve-lit] 0 (not #3668)
[resolve-lit] 0 (not #3663)
[resolve-lit] 0 (not #3665)
[resolve-lit] 0 (not #3735)
[resolve-process] (not #3728)
[resolve-lit] 0 (not #437)
[resolve-lit] 0 (not #687)
[resolve-lit] 0 (not #4421)
[resolve-lit] 3 (not #4379)
[resolve-lit] 0 (not #4460)
[resolve-lit] 0 (not #4458)
[resolve-process] (not #3665)
[resolve-process] (not #437)
[resolve-lit] 0 (not #4538)
[resolve-lit] 2 (not #4121)
[resolve-lit] 0 (not #4299)
[resolve-lit] 0 (not #696)
[resolve-lit] 3 (not #2578)
[resolve-process] (not #4458)
[resolve-lit] 3 (not #3844)
[resolve-process] (not #3663)
[resolve-lit] 0 (not #380)
[resolve-process] (not #380)
[resolve-process] (not #4460)
[resolve-lit] 3 (not #4068)
[resolve-lit] 0 (not #706)
[resolve-process] (not #3668)
[resolve-lit] 0 #3721
[resolve-process] #3721
[resolve-lit] 0 (not #3653)
[resolve-lit] 0 (not #3654)
[resolve-process] (not #3653)
[resolve-lit] 0 (not #4595)
[resolve-lit] 0 (not #4586)
[resolve-process] (not #3654)
[resolve-lit] 0 (not #4587)
[resolve-lit] 3 (not #4214)
[resolve-lit] 0 (not #4594)
[resolve-process] (not #4595)
[resolve-lit] 0 (not #4592)
[resolve-process] (not #4594)
[resolve-process] (not #4592)
[resolve-process] (not #4587)
[resolve-process] (not #4586)
[resolve-process] (not #4538)
[resolve-lit] 0 #4526
[resolve-lit] 0 (not #4523)
[resolve-lit] 0 (not #4549)
[resolve-process] (not #4426)
[resolve-lit] 0 (not #4466)
[resolve-lit] 3 (not #4477)
[resolve-lit] 3 #4488
[resolve-process] #4526
[resolve-lit] 0 #4579
[resolve-process] (not #4523)
[resolve-process] (not #4421)
[resolve-lit] 0 #4423
[resolve-process] (not #4466)
[resolve-lit] 0 #4581
[resolve-process] (not #4549)
[resolve-lit] 0 #4529
[resolve-process] #4579
[resolve-lit] 0 #4578
[resolve-process] #4423
[resolve-lit] 0 #4424
[resolve-lit] 0 (not #673)
[resolve-process] #4581
[resolve-process] #4529
[resolve-process] #4578
[resolve-process] #4424
[resolve-process] (not #3735)
[resolve-lit] 0 #3705
[resolve-process] (not #3701)
[resolve-process] #3705
[resolve-lit] 0 (not #3687)
[resolve-lit] 3 (not #3703)
[resolve-process] (not #3687)
[resolve-lit] 0 #3682
[resolve-process] (not #673)
[resolve-process] (not #687)
[resolve-process] #3682
[resolve-lit] 0 (not #3649)
[resolve-lit] 0 (not #3674)
[resolve-process] (not #3649)
[resolve-process] (not #706)
[resolve-lit] 3 #4364
[resolve-process] (not #4299)
[resolve-lit] 0 #4301
[resolve-process] #4301
[resolve-lit] 0 (not #4302)
[resolve-lit] 0 #4303
[resolve-process] (not #4302)
[resolve-lit] 0 (not #721)
[resolve-process] #4303
[resolve-process] (not #3674)
[resolve-lit] 0 #3708
[resolve-process] (not #721)
[resolve-process] #3708
[resolve-lit] 0 (not #3712)
[resolve-lit] 3 (not #3647)
[resolve-process] (not #3712)
[resolve-lit] 0 #3714
[resolve-process] (not #696)
[conflict] #3714 (not #276) (not #4121) (not #2578) (not #3844) (not #4477) #4364 (not #3647)
[pop] 2 5
[attach-enode] #4256 0
[attach-enode] #4476 0
[attach-enode] #4585 0
[attach-enode] #4593 0
[attach-enode] #4460 0
[attach-enode] #4426 0
[attach-meaning] #370 arith (- 1)
[attach-enode] #4420 0
[attach-enode] #4421 0
[attach-enode] #4606 0
[attach-enode] #4607 0
[attach-enode] #4610 0
[attach-enode] #4623 0
[attach-enode] #4538 0
[attach-enode] #4625 0
[attach-enode] #4679 0
[attach-enode] #4680 0
[attach-enode] #4692 0
[attach-enode] #4667 0
[attach-enode] #4702 0
[attach-enode] #4717 0
[attach-enode] #4718 0
[attach-enode] #4735 0
[attach-enode] #4722 0
[attach-enode] #4796 0
[attach-enode] #4458 0
[attach-enode] #4690 0
[attach-enode] #4799 0
[attach-enode] #4800 0
[assign] #3714 clause 536 -692 -39 -445 -561 -700 691 -498
[assign] #4458 justification -1: 561 713
[mk-app] #4571 = #1273 #4625
[attach-meaning] #370 arith (- 1)
[mk-app] #4572 + #1273 #4679
[mk-app] #4573 <= #4572 #341
[mk-app] #4579 >= #4572 #341
[assign] #4571 justification -1: 713
[attach-enode] #4571 0
[attach-enode] #4572 0
[assign] #4573 justification -1: 798
[assign] #4579 justification -1: 798
[assign] #4694 clause 793 -800 -677
[assign] #4693 clause 786 -799 -680
[decide-and-or] #3660 #3706
[push] 3
[assign] #3706 decision axiom
[assign] #779 clause 490 -492 -491
[assign] #3645 clause 494 -490
[assign] (not #3689) clause -537 -494 -536
[assign] #1016 clause 488 537
[assign] (not #940) clause -489 537
[new-match] 0x61d50e6c1980 #3535 #2695 #1025 #341 #2892 ; #940
[mk-app] #4578 >= #341 #341
[mk-app] #4550 or #2697 #4578
[mk-app] #4531 not #4550
[mk-app] #4280 * #370 #341
[mk-app] #4273 + #1025 #4280
[mk-app] #4275 <= #4273 #341
[mk-app] #4291 not #4275
[mk-app] #4549 or #2704 #4291
[mk-app] #4530 not #4549
[mk-app] #4529 + #1025 #4001
[mk-app] #4290 <= #4529 #341
[mk-app] #4257 or #2709 #4290
[mk-app] #4508 not #4257
[mk-app] #4501 or #4531 #4530 #4508
[mk-app] #4466 req%lib!Chap28.MCSSSpec.MCSSSpec.lemma_range_sum_snoc. #1014 #341 #1025
[mk-app] #4580 = #4501 #4466
[mk-app] #4581 not #4580
[inst-discovered] theory-solving 0 arith# ; #4578
[mk-app] #4588 = #4578 #1
[instance] 0 #4588
[attach-enode] #4588 0
[end-of-instance]
[mk-app] #4588 or #2697 #1
[inst-discovered] theory-solving 0 basic# ; #4588
[mk-app] #4589 = #4588 #1
[instance] 0 #4589
[attach-enode] #4589 0
[end-of-instance]
[mk-app] #4588 not #1
[inst-discovered] theory-solving 0 basic# ; #4588
[mk-app] #4589 = #4588 #2
[instance] 0 #4589
[attach-enode] #4589 0
[end-of-instance]
[inst-discovered] theory-solving 0 arith# ; #4280
[mk-app] #4588 = #4280 #341
[instance] 0 #4588
[attach-enode] #4588 0
[end-of-instance]
[mk-app] #4588 + #1025 #341
[inst-discovered] theory-solving 0 arith# ; #4588
[mk-app] #4589 = #4588 #1025
[instance] 0 #4589
[attach-enode] #4589 0
[end-of-instance]
[mk-app] #4588 <= #1025 #341
[mk-app] #4589 not #4588
[mk-app] #4590 or #2704 #4589
[mk-app] #4591 not #4590
[mk-app] #4592 or #4591 #4508
[mk-app] #4596 or #2 #4591 #4508
[inst-discovered] theory-solving 0 basic# ; #4596
[mk-app] #4597 = #4596 #4592
[instance] 0 #4597
[attach-enode] #4597 0
[end-of-instance]
[mk-app] #4596 = #4592 #4466
[mk-app] #4597 not #4592
[mk-app] #4598 not #4596
[inst-discovered] theory-solving 0 basic# ; #4598
[mk-app] #4597 = #4598 #4598
[instance] 0 #4597
[attach-enode] #4597 0
[end-of-instance]
[mk-app] #4597 not #3535
[mk-app] #4599 or #4597 #4598
[instance] 0x61d50e6c1980 ; 1
[attach-enode] #4529 1
[attach-enode] #4466 1
[assign] (not #4596) justification -1: 459
[end-of-instance]
[assign] (not #4466) justification -1: -489 485
[assign] #4290 clause 805 -680 -577 -583 -647 -573 484
[assign] #4592 clause 807 808 809
[assign] #4257 clause 806 -805
[assign] (not #4590) clause -803 -806 -807
[assign] #2685 clause 801 803
[assign] #4588 clause 802 803
[resolve-process] true
[resolve-lit] 0 (not #4588)
[resolve-lit] 2 (not #4215)
[conflict] (not #4588)
[pop] 1 4
[attach-enode] #4529 0
[assign] #4290 clause 801 -680 -577 -583 -647 -573 484
[assign] (not #4588) axiom
[assign] (not #3980) clause -630 802 -680
[assign] (not #3975) clause -629 630
[decide-and-or] #3660 #3706
[push] 3
[assign] #3706 decision axiom
[assign] #779 clause 490 -492 -491
[assign] #3645 clause 494 -490
[assign] (not #3689) clause -537 -494 -536
[assign] #1016 clause 488 537
[assign] (not #940) clause -489 537
[new-match] 0x61d50e6c1a18 #3535 #2695 #1025 #341 #2892 ; #940
[mk-app] #4590 or #2697 #1
[inst-discovered] theory-solving 0 basic# ; #4590
[mk-app] #4591 = #4590 #1
[instance] 0 #4591
[attach-enode] #4591 0
[end-of-instance]
[mk-app] #4590 not #1
[inst-discovered] theory-solving 0 basic# ; #4590
[mk-app] #4591 = #4590 #2
[instance] 0 #4591
[attach-enode] #4591 0
[end-of-instance]
[mk-app] #4590 or #2704 #4589
[mk-app] #4591 not #4590
[mk-app] #4592 or #4591 #4508
[mk-app] #4596 or #2 #4591 #4508
[inst-discovered] theory-solving 0 basic# ; #4596
[mk-app] #4598 = #4596 #4592
[instance] 0 #4598
[attach-enode] #4598 0
[end-of-instance]
[mk-app] #4596 = #4592 #4466
[mk-app] #4598 not #4592
[mk-app] #4597 not #4596
[inst-discovered] theory-solving 0 basic# ; #4597
[mk-app] #4598 = #4597 #4597
[instance] 0 #4598
[attach-enode] #4598 0
[end-of-instance]
[mk-app] #4598 not #3535
[mk-app] #4599 or #4598 #4597
[instance] 0x61d50e6c1a18 ; 1
[assign] #4590 justification -1: -802
[assign] #4257 justification -1: 801
[assign] (not #4592) justification -1: 804 806
[attach-enode] #4466 1
[assign] (not #4596) justification -1: 459
[end-of-instance]
[assign] #4466 clause 808 809
[resolve-lit] 0 (not #4466)
[resolve-process] #4466
[resolve-lit] 0 #940
[resolve-process] (not #4466)
[resolve-lit] 0 #4596
[resolve-lit] 0 #4592
[resolve-process] #4596
[resolve-process] #4592
[resolve-lit] 0 (not #4590)
[resolve-lit] 0 (not #4257)
[resolve-process] (not #4257)
[resolve-lit] 1 (not #4290)
[resolve-process] (not #4590)
[resolve-lit] 1 #4588
[conflict] #940 (not #4290) #4588
[pop] 1 4
[assign] #940 clause 489 -801 802
[assign] #3706 clause 492 -489
[assign] #3689 clause 537 -489
[assign] #779 clause 490 -492 -491
[assign] (not #3645) clause -494 -537 -536
[resolve-process] true
[resolve-lit] 0 #3645
[resolve-lit] 0 (not #779)
[resolve-process] #3645
[resolve-lit] 0 (not #3689)
[resolve-lit] 0 (not #3714)
[resolve-process] (not #779)
[resolve-lit] 0 (not #3706)
[resolve-lit] 0 (not #3630)
[resolve-process] (not #3689)
[resolve-lit] 0 (not #940)
[resolve-process] (not #3706)
[resolve-process] (not #940)
[resolve-lit] 0 (not #4290)
[resolve-lit] 0 #4588
[resolve-process] #4588
[resolve-process] (not #4290)
[resolve-lit] 1 (not #4214)
[resolve-process] (not #3714)
[resolve-lit] 0 (not #4121)
[resolve-lit] 1 (not #276)
[resolve-lit] 1 (not #2578)
[resolve-lit] 1 (not #3844)
[resolve-lit] 1 (not #4477)
[resolve-lit] 1 #4364
[resolve-lit] 1 (not #3647)
[resolve-process] (not #3630)
[resolve-lit] 0 (not #4196)
[resolve-lit] 0 (not #4218)
[resolve-lit] 0 (not #4217)
[resolve-lit] 0 (not #4210)
[resolve-lit] 0 (not #4209)
[resolve-lit] 1 (not #4215)
[resolve-lit] 1 (not #4106)
[resolve-lit] 1 (not #4105)
[resolve-lit] 1 (not #4081)
[resolve-lit] 1 (not #4102)
[resolve-process] (not #4210)
[resolve-lit] 0 (not #4312)
[resolve-process] (not #4209)
[resolve-process] (not #4218)
[resolve-process] (not #4217)
[resolve-process] (not #4121)
[resolve-lit] 0 (not #4113)
[resolve-lit] 0 (not #4122)
[resolve-process] (not #4312)
[resolve-process] (not #4196)
[resolve-process] (not #4113)
[resolve-lit] 0 (not #778)
[resolve-process] (not #4122)
[conflict] (not #778) (not #276) (not #2578) (not #3844) (not #4477) #4364 (not #3647)
[pop] 1 3
[attach-enode] #4256 0
[attach-enode] #4476 0
[attach-enode] #4585 0
[attach-enode] #4593 0
[attach-enode] #4460 0
[attach-enode] #4426 0
[attach-meaning] #370 arith (- 1)
[attach-enode] #4420 0
[attach-enode] #4421 0
[attach-enode] #4606 0
[attach-enode] #4607 0
[attach-enode] #4610 0
[attach-enode] #4623 0
[attach-enode] #4538 0
[attach-enode] #4625 0
[attach-enode] #4679 0
[attach-enode] #4680 0
[attach-enode] #4692 0
[attach-enode] #4667 0
[attach-enode] #4702 0
[attach-enode] #4717 0
[attach-enode] #4718 0
[attach-enode] #4735 0
[attach-enode] #4722 0
[attach-enode] #4796 0
[attach-enode] #4458 0
[attach-enode] #4690 0
[attach-enode] #4799 0
[attach-enode] #4800 0
[attach-enode] #4572 0
[attach-enode] #4529 0
[assign] #4290 clause 769 -680 -577 -583 -647 -573 484
[assign] (not #4588) axiom
[assign] (not #778) clause -487 -39 -445 -561 -700 691 -498
[assign] (not #3980) clause -630 770 -680
[assign] #940 clause 489 770 -769
[assign] #1016 clause 488 487
[resolve-process] true
[resolve-lit] 0 (not #940)
[resolve-lit] 0 #778
[resolve-process] (not #940)
[resolve-lit] 0 #4588
[resolve-lit] 0 (not #4290)
[resolve-process] #778
[resolve-lit] 0 (not #276)
[resolve-lit] 0 (not #2578)
[resolve-lit] 0 (not #3844)
[resolve-lit] 0 (not #4477)
[resolve-lit] 0 #4364
[resolve-lit] 0 (not #3647)
[resolve-process] #4588
[resolve-process] (not #4290)
[resolve-lit] 0 (not #4214)
[resolve-process] (not #4477)
[resolve-lit] 0 (not #4215)
[resolve-lit] 0 (not #4081)
[resolve-lit] 0 (not #4085)
[resolve-lit] 0 (not #4106)
[resolve-process] (not #4215)
[resolve-lit] 0 (not #4128)
[resolve-process] (not #4214)
[resolve-process] (not #4128)
[resolve-process] (not #3844)
[resolve-lit] 0 (not #3857)
[resolve-process] (not #3647)
[resolve-lit] 0 (not #728)
[resolve-process] (not #3857)
[resolve-lit] 0 (not #3855)
[resolve-process] (not #728)
[resolve-lit] 0 #4252
[resolve-process] #4364
[resolve-process] (not #4085)
[resolve-process] (not #3855)
[resolve-process] #4252
[resolve-process] (not #4106)
[resolve-process] (not #4081)
[resolve-lit] 0 (not #4100)
[resolve-process] (not #4100)
[resolve-process] (not #2578)
[resolve-lit] 0 (not #2572)
[resolve-process] (not #2572)
[resolve-lit] 0 (not #4065)
[resolve-process] (not #4065)
[conflict] (not #276)
[pop] 1 2
[attach-enode] #4080 0
[attach-enode] #4168 0
[attach-enode] #4183 0
[attach-enode] #4184 0
[attach-enode] #4193 0
[attach-enode] #4212 0
[attach-enode] #4213 0
[attach-enode] #4188 0
[attach-enode] #4248 0
[attach-enode] #4249 0
[attach-enode] #4250 0
[attach-enode] #4251 0
[attach-enode] #4361 0
[attach-enode] #4362 0
[attach-enode] #4114 0
[attach-enode] #4067 0
[attach-enode] #4115 0
[attach-enode] #4116 0
[attach-enode] #4117 0
[attach-enode] #4223 0
[attach-enode] #4224 0
[attach-enode] #4365 0
[attach-enode] #4118 0
[attach-enode] #4119 0
[attach-enode] #4120 0
[attach-enode] #4121 0
[attach-enode] #4298 0
[attach-enode] #4299 0
[attach-enode] #4066 0
[attach-enode] #4068 0
[attach-enode] #4103 0
[attach-enode] #4104 0
[attach-enode] #4083 0
[attach-enode] #4084 0
[attach-enode] #4139 0
[attach-enode] #4154 0
[attach-enode] #4159 0
[attach-enode] #4206 0
[attach-enode] #4207 0
[attach-enode] #4216 0
[attach-enode] #4155 0
[attach-enode] #4202 0
[attach-enode] #4487 0
[attach-enode] #4256 0
[attach-enode] #4476 0
[attach-enode] #4585 0
[attach-enode] #4593 0
[attach-enode] #4460 0
[attach-enode] #4426 0
[attach-meaning] #370 arith (- 1)
[attach-enode] #4420 0
[attach-enode] #4421 0
[attach-enode] #4606 0
[attach-enode] #4607 0
[attach-enode] #4610 0
[attach-enode] #4623 0
[attach-enode] #4538 0
[attach-enode] #4377 0
[attach-enode] #4378 0
[attach-enode] #4625 0
[attach-enode] #4679 0
[attach-enode] #4680 0
[attach-enode] #4692 0
[attach-enode] #4667 0
[attach-enode] #4702 0
[attach-enode] #4717 0
[attach-enode] #4718 0
[attach-enode] #4735 0
[attach-enode] #4722 0
[attach-enode] #4796 0
[attach-enode] #4379 0
[attach-enode] #4458 0
[attach-enode] #4690 0
[attach-enode] #4799 0
[attach-enode] #4800 0
[attach-enode] #4572 0
[attach-enode] #4529 0
[assign] #3979 axiom
[assign] #3974 axiom
[assign] #4063 axiom
[assign] #4064 axiom
[assign] #4065 axiom
[assign] #4112 axiom
[assign] (not #4252) axiom
[assign] #3855 axiom
[attach-enode] #4071 0
[assign] #4071 axiom
[attach-enode] #4073 0
[assign] #4073 axiom
[assign] #4085 axiom
[attach-enode] #4087 0
[assign] #4088 axiom
[attach-enode] #4161 0
[assign] #4161 axiom
[assign] (not #4364) axiom
[assign] (not #4588) axiom
[assign] (not #276) axiom
[assign] #2508 clause 430 -717
[assign] #2521 clause 433 -718
[assign] #2534 clause 436 -719
[assign] #2549 clause 440 -720
[assign] #2572 clause 444 -721
[assign] #2606 clause 449 -722
[assign] #728 clause 497 656
[assign] #3857 clause 565 -564
[assign] #4363 clause 657 668
[assign] (not #4366) clause -667 668
[assign] #4477 clause 681 716
[assign] #278 bin -39
[assign] #2517 bin 430
[assign] #3512 bin 433
[assign] #2538 bin 436
[assign] #3515 bin 440
[assign] #2578 bin 444
[assign] #3523 bin 449
[assign] #3647 clause 498 -497
[assign] #3844 justification -1: 565 591
[mk-app] #4053 = #1025 #1273
[attach-meaning] #370 arith (- 1)
[assign] #4053 justification -1: 480
[attach-enode] #4053 0
[assign] #4214 justification -1: 727
[assign] #4215 justification -1: 727
[attach-meaning] #370 arith (- 1)
[mk-app] #3973 <= #4104 #341
[assign] #3973 justification -1: 568
[assign] #4106 justification -1: 568
[mk-app] #4100 <= #288 #289
[mk-app] #4105 >= #288 #289
[assign] #4100 justification -1: 43
[assign] #4105 justification -1: 43
[mk-app] #4107 = #3639 #4114
[attach-meaning] #370 arith (- 1)
[mk-app] #4108 * #370 #4114
[mk-app] #4109 + #3639 #4108
[mk-app] #4110 <= #4109 #341
[mk-app] #4111 >= #4109 #341
[assign] #4107 justification -1: 592
[attach-enode] #4107 0
[attach-enode] #4108 0
[attach-enode] #4109 0
[assign] #4110 justification -1: 731
[assign] #4111 justification -1: 731
[mk-app] #4128 = #2449 #292
[mk-app] #4129 <= #2449 #292
[mk-app] #4131 >= #2449 #292
[assign] #4128 justification -1: 44 40
[attach-enode] #4128 0
[assign] #4129 justification -1: 734
[assign] #4131 justification -1: 734
[eq-expl] #1290 root
[new-match] 0x61d50e67b398 #1198 #1194 #1290 ; #3844 (#1188 #1188)
[new-match] 0x61d50e67b3c8 #2578 #2577 #1276 #1396 ; #2899
[mk-app] #4132 or #3939 #3845 #4379
[instance] 0x61d50e67b398 ; 2
[assign] #4379 justification -1: 257 561
[end-of-instance]
[mk-app] #4205 not #2578
[mk-app] #4211 or #4205 #4068
[instance] 0x61d50e67b3c8 ; 1
[assign] #4068 justification -1: 445
[end-of-instance]
[assign] #4290 clause 715 -655 -577 -583 -647 -573 484
[assign] (not #3980) clause -630 -655 716
[assign] (not #4054) clause -687 -736
[assign] #940 clause 489 -715 716
[assign] (not #3975) clause -629 630
[assign] #778 clause 487 -489
[assign] #3706 clause 492 -489
[assign] #3689 clause 537 -489
[assign] #4458 justification -1: 561 709
[mk-app] #4219 = #2899 #4067
[attach-meaning] #370 arith (- 1)
[mk-app] #4220 * #370 #4067
[mk-app] #4237 + #2899 #4220
[mk-app] #4242 <= #4237 #341
[mk-app] #4243 >= #4237 #341
[assign] #4219 justification -1: 671 592
[attach-enode] #4219 0
[attach-enode] #4220 0
[attach-enode] #4237 0
[assign] #4242 justification -1: 737
[assign] #4243 justification -1: 737
[mk-app] #4238 = #1273 #4625
[attach-meaning] #370 arith (- 1)
[assign] #4238 justification -1: 709
[attach-enode] #4238 0
[assign] #4573 justification -1: 740
[assign] #4579 justification -1: 740
[eq-expl] #4377 root
[new-match] 0x61d50e67bac0 #1187 #1186 #4377 ; #4378
[new-match] 0x61d50e67baf0 #3440 #2356 #1276 #2574 #1396 ; #782
[new-match] 0x61d50e67bb30 #4034 #4023 #1276 #2574 ; #782 (#1396 #3934)
[new-match] 0x61d50e67bb68 #3440 #2356 #3638 #2574 #1396 ; #3639
[new-match] 0x61d50e67bba8 #4034 #4023 #3638 #2574 ; #3639 (#1396 #3934)
[new-match] 0x61d50e67bbe0 #174 #173 #341 ; #2574
[new-match] 0x61d50e67bc10 #174 #173 #1025 ; #3638
[new-match] 0x61d50e67bc40 #2724 #2723 #1025 #341 #2892 ; #778
[eq-expl] #782 cg (#1396 #3934) (#2574 #2574) (#1276 #1276) ; #4067
[eq-expl] #4067 lit #4068 ; #4066
[eq-expl] #4066 cg (#3934 #1396) (#1276 #1276) ; #2899
[new-match] 0x61d50e67bc80 #552 #550 #749 #782 ; #750
[new-match] 0x61d50e67bcb8 #3286 #1569 #1276 #1396 #1167 #125 ; #748
[eq-expl] #4378 root
[new-match] 0x61d50e60e4e8 #2018 #1540 #4378 #1167 #125 ; #4625
[new-match] 0x61d50e60e528 #1545 #1540 #4378 #1167 #125 ; #4625
[new-match] 0x61d50e60e568 #1198 #1194 #4378 ; #4458 (#1188 #1188)
[new-match] 0x61d50e60e598 #1201 #1200 #4377 ; #4458 (#1188 #1188) (#4378 #4378)
[inst-discovered] theory-solving 0 basic# ; #4077
[mk-app] #4283 = #4077 #4077
[instance] 0 #4283
[attach-enode] #4283 0
[end-of-instance]
[mk-app] #4283 not #3440
[mk-app] #4292 or #4283 #3966 #4072 #4074 #4076
[instance] 0x61d50e67baf0 ; 1
[attach-enode] #2351 1
[attach-enode] #2352 1
[attach-enode] #4075 1
[attach-enode] #4076 1
[assign] #4076 justification -1: 417 626 723 724
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4094
[mk-app] #4262 = #4094 #4094
[instance] 0 #4262
[attach-enode] #4262 0
[end-of-instance]
[mk-app] #4262 not #4034
[mk-app] #4263 or #4262 #4072 #4074 #4082 #4086 #4089 #4095
[instance] 0x61d50e67bb30 ; 2
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4166
[mk-app] #4348 = #4166 #4166
[instance] 0 #4348
[attach-enode] #4348 0
[end-of-instance]
[mk-app] #4348 or #4283 #3966 #4072 #4163 #4165
[instance] 0x61d50e67bb68 ; 1
[attach-enode] #4164 1
[attach-enode] #4165 1
[assign] #4165 justification -1: 417 626 723 726
[end-of-instance]
[mk-app] #4367 + #4001 #4168
[inst-discovered] theory-solving 0 arith# ; #4172
[mk-app] #4371 = #4172 #4367
[instance] 0 #4371
[attach-enode] #4371 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4371 <= #4367 #341
[inst-discovered] theory-solving 0 arith# ; #4371
[mk-app] #4408 = #4371 #4189
[instance] 0 #4408
[attach-enode] #4408 0
[end-of-instance]
[mk-app] #4367 not #4189
[mk-app] #4371 or #4072 #4163 #4082 #4186 #4367 #4180
[mk-app] #4408 or #4262 #4072 #4163 #4082 #4186 #4367 #4180
[instance] 0x61d50e67bba8 ; 2
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4371 = #4080 #341
[inst-discovered] theory-solving 0 arith# ; #4099
[mk-app] #4403 = #4099 #4371
[instance] 0 #4403
[attach-enode] #4403 0
[end-of-instance]
[mk-app] #4403 or #3871 #4371
[instance] 0x61d50e67bbe0 ; 1
[attach-enode] #4371 1
[assign] #4371 justification -1: 26
[end-of-instance]
[mk-app] #4383 or #3871 #4190
[instance] 0x61d50e67bc10 ; 1
[attach-enode] #4190 1
[attach-meaning] #370 arith (- 1)
[assign] #4190 justification -1: 26
[end-of-instance]
[mk-app] #4419 not #2724
[mk-app] #4522 or #4419 #4122
[instance] 0x61d50e67bc40 ; 1
[attach-enode] #4113 1
[assign] #4122 justification -1: 460
[end-of-instance]
[mk-app] #4515 or #3887 #4234
[instance] 0x61d50e67bc80 ; 1
[attach-enode] #4231 1
[attach-enode] #4232 1
[attach-enode] #4233 1
[attach-enode] #4234 1
[mk-app] #4491 <= #4233 #341
[mk-app] #4490 >= #4233 #341
[assign] #4234 justification -1: 77
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4229
[mk-app] #4465 = #4229 #4229
[instance] 0 #4465
[attach-enode] #4465 0
[end-of-instance]
[mk-app] #4465 not #3286
[mk-app] #4469 or #4465 #3966 #4074 #4228
[instance] 0x61d50e67bcb8 ; 1
[attach-enode] #4227 1
[attach-enode] #4228 1
[assign] #4228 justification -1: 305 626 724
[end-of-instance]
[mk-app] #4512 >= #4625 #341
[mk-app] #4381 or #4459 #4512
[mk-app] #4450 or #3842 #4459 #4512
[instance] 0x61d50e60e528 ; 3
[assign] #4512 justification -1: 303 710
[end-of-instance]
[assign] #4693 clause 700 -714 -655
[assign] #4694 clause 707 -713 -652
[assign] #4102 clause 649 -744
[assign] #4081 clause 672 -744
[assign] #4194 clause 651 -745
[assign] #4195 clause 654 -745
[assign] #4491 clause 749 -748
[assign] #4490 clause 750 -748
[assign] #4495 clause 677 -672 -647 -673 -676 -577 -477
[assign] (not #4094) clause -742 -672
[assign] #4185 clause 650 -651 -584 -652 -649 -574 -476
[assign] (not #4411) clause -682 -651 -676 -673 -652 -584 -574
[assign] #4189 clause 653 -654 -577 -583 -647 -655 -573 484
[assign] #4090 clause 659 742
[assign] #4092 clause 662 742
[assign] (not #4179) clause -666 -653 -650 -672
[assign] #4175 clause 658 666
[assign] #4177 clause 665 666
[assign] #4113 justification -1: 487 485
[eq-expl] #2352 root
[new-match] 0x61d50e60f5b0 #2320 #2319 #2352 #1276 #2574 #3934 ; #4075
[eq-expl] #2351 root
[new-match] 0x61d50e60f5f8 #3437 #2337 #2351 #1276 #2574 #3934 ; #4075 (#2352 #2352)
[new-match] 0x61d50e60f640 #2320 #2319 #2352 #3638 #2574 #3934 ; #4164
[new-match] 0x61d50e60f688 #3437 #2337 #2351 #3638 #2574 #3934 ; #4164 (#2352 #2352)
[eq-expl] #4117 root
[eq-expl] #4119 root
[new-match] 0x61d50e60f6d0 #552 #550 #4119 #4117 ; #4120
[eq-expl] #4116 root
[new-match] 0x61d50e60f708 #3440 #2356 #4116 #2574 #3934 ; #4117
[new-match] 0x61d50e60f748 #4034 #4023 #4116 #2574 ; #4117 (#3934 #3934)
[new-match] 0x61d50e60f780 #3286 #1569 #4116 #3934 #1167 #125 ; #4118
[eq-expl] #4115 root
[new-match] 0x61d50e60f7c8 #174 #173 #4115 ; #4116
[new-match] 0x61d50e60f7f8 #567 #559 #296 #1025 ; #4115
[eq-expl] #4227 cg (#125 #125) (#1167 #1167) (#3934 #1396) (#1276 #1276) ; #748
[new-match] 0x61d50e60f830 #542 #236 #4227 #275 ; #4228 (#1167 #1167)
[new-match] 0x61d50e60f868 #240 #236 #4227 #275 ; #4228 (#1167 #1167)
[mk-app] #4451 not #2320
[mk-app] #4449 or #4451 #4387
[instance] 0x61d50e60f5b0 ; 3
[attach-enode] #2316 3
[attach-enode] #4386 3
[attach-enode] #4387 3
[assign] #4387 justification -1: 414
[end-of-instance]
[mk-app] #4455 + #4103 #4080
[inst-discovered] theory-solving 0 arith# ; #4390
[mk-app] #4373 = #4390 #4455
[instance] 0 #4373
[attach-enode] #4373 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4373 >= #4455 #341
[inst-discovered] theory-solving 0 arith# ; #4373
[mk-app] #4368 = #4373 #4382
[instance] 0 #4368
[attach-enode] #4368 0
[end-of-instance]
[mk-app] #4455 if #4382 #341 #4397
[mk-app] #4373 = #4075 #4455
[mk-app] #4368 or #3966 #4072 #4074 #4373
[inst-discovered] theory-solving 0 basic# ; #4368
[mk-app] #4372 = #4368 #4368
[instance] 0 #4372
[attach-enode] #4372 0
[end-of-instance]
[mk-app] #4372 not #3437
[mk-app] #4370 or #4372 #3966 #4072 #4074 #4373
[instance] 0x61d50e60f5f8 ; 3
[mk-app] #4345 = #4455 #341
[mk-app] #4222 = #4397 #4455
[attach-enode] #4455 3
[attach-enode] #4392 3
[attach-enode] #4393 3
[attach-enode] #4394 3
[attach-enode] #4395 3
[attach-enode] #4396 3
[attach-enode] #4397 3
[attach-enode] #4345 3
[attach-enode] #4222 3
[attach-enode] #4373 3
[assign] #4373 justification -1: 415 626 723 724
[end-of-instance]
[mk-app] #4368 or #4451 #4409
[instance] 0x61d50e60f640 ; 3
[attach-enode] #4404 3
[attach-enode] #4409 3
[assign] #4409 justification -1: 414
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4416
[mk-app] #4295 = #4416 #4416
[instance] 0 #4295
[attach-enode] #4295 0
[end-of-instance]
[mk-app] #4295 or #4372 #3966 #4072 #4163 #4415
[instance] 0x61d50e60f688 ; 3
[mk-app] #4240 = #4414 #341
[mk-app] #4282 = #4413 #4414
[attach-enode] #4414 3
[attach-enode] #4412 3
[attach-enode] #4413 3
[attach-enode] #4240 3
[attach-enode] #4282 3
[assign] #4282 justification -1: -682
[attach-enode] #4415 3
[assign] #4415 justification -1: 415 626 723 726
[end-of-instance]
[mk-app] #4281 + #4117 #4119 #4125
[inst-discovered] theory-solving 0 arith# ; #4126
[mk-app] #4245 = #4126 #4281
[instance] 0 #4245
[attach-enode] #4245 0
[end-of-instance]
[mk-app] #4245 = #4281 #341
[mk-app] #4097 or #3887 #4245
[instance] 0x61d50e60f6d0 ; 2
[attach-enode] #4125 2
[attach-enode] #4281 2
[attach-enode] #4245 2
[mk-app] #4244 <= #4281 #341
[mk-app] #4246 >= #4281 #341
[assign] #4245 justification -1: 77
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4137
[mk-app] #4078 = #4137 #4137
[instance] 0 #4078
[attach-enode] #4078 0
[end-of-instance]
[mk-app] #4078 or #4283 #3966 #4072 #4134 #4136
[instance] 0x61d50e60f708 ; 3
[attach-enode] #4133 3
[attach-enode] #4135 3
[attach-enode] #4136 3
[end-of-instance]
[mk-app] #4239 + #4001 #4139
[inst-discovered] theory-solving 0 arith# ; #4143
[mk-app] #4241 = #4143 #4239
[instance] 0 #4241
[attach-enode] #4241 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4241 <= #4239 #341
[inst-discovered] theory-solving 0 arith# ; #4241
[mk-app] #4236 = #4241 #4160
[instance] 0 #4236
[attach-enode] #4236 0
[end-of-instance]
[mk-app] #4239 not #4160
[inst-discovered] theory-solving 0 basic# ; #4150
[mk-app] #4241 = #4150 #4150
[instance] 0 #4241
[attach-enode] #4241 0
[end-of-instance]
[mk-app] #4241 or #4072 #4134 #4082 #4157 #4239 #4151
[inst-discovered] theory-solving 0 basic# ; #4241
[mk-app] #4236 = #4241 #4241
[instance] 0 #4236
[attach-enode] #4236 0
[end-of-instance]
[mk-app] #4236 or #4262 #4072 #4134 #4082 #4157 #4239 #4151
[instance] 0x61d50e60f748 ; 2
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4199
[mk-app] #4241 = #4199 #4199
[instance] 0 #4241
[attach-enode] #4241 0
[end-of-instance]
[mk-app] #4241 or #4465 #3966 #4134 #4198
[instance] 0x61d50e60f780 ; 3
[attach-enode] #4198 3
[end-of-instance]
[mk-app] #4221 or #3871 #4196
[instance] 0x61d50e60f7c8 ; 2
[attach-enode] #4196 2
[attach-meaning] #370 arith (- 1)
[assign] #4196 justification -1: 26
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4101 + #4202 #4115
[attach-meaning] #370 arith (- 1)
[mk-app] #4101 = #4207 #296
[inst-discovered] theory-solving 0 arith# ; #4204
[mk-app] #4098 = #4204 #4101
[instance] 0 #4098
[attach-enode] #4098 0
[end-of-instance]
[mk-app] #4098 or #3877 #4101
[instance] 0x61d50e60f7f8 ; 2
[attach-enode] #4101 2
[assign] #4101 justification -1: 78
[end-of-instance]
[mk-app] #4079 not #542
[mk-app] #4069 or #4079 #4325 #4326
[instance] 0x61d50e60f830 ; 2
[attach-enode] #4324 2
[attach-enode] #4326 2
[end-of-instance]
[mk-app] #4070 not #240
[mk-app] #4388 or #4070 #4325 #4330
[instance] 0x61d50e60f868 ; 2
[attach-enode] #4329 2
[attach-enode] #4330 2
[end-of-instance]
[assign] #4121 clause 669 -746 -747
[assign] #4244 clause 762 -761
[assign] #4246 clause 763 -761
[assign] #4217 clause 685 -768
[assign] #4218 clause 680 -768
[assign] #4209 clause 684 -769
[assign] #4210 clause 679 -769
[assign] #4156 clause 683 -685 -676 -673 -652 -684 -584 -574
[assign] #4160 clause 678 -680 -577 -583 -647 -655 -679 -573 -477
[assign] #4324 justification -1: 751 592
[attach-meaning] #370 arith (- 1)
[mk-app] #4410 + #4114 #4125
[mk-app] #4407 <= #4410 #341
[mk-app] #4369 >= #4410 #341
[attach-enode] #4410 0
[assign] #4407 justification -1: 669
[assign] #4369 justification -1: 669
[eq-expl] #2316 root
[new-match] 0x61d50e6b5cb0 #2320 #2319 #2316 #1276 #2574 #3934 ; #4386
[new-match] 0x61d50e6b5cf8 #2320 #2319 #2316 #3638 #2574 #3934 ; #4404
[eq-expl] #4395 root
[new-match] 0x61d50e6b5d40 #2320 #2319 #2351 #3638 #4395 #3934 ; #4412
[eq-expl] #4393 root
[eq-expl] #4412 root
[new-match] 0x61d50e6b5d88 #552 #550 #4412 #4393 ; #4413
[eq-expl] #4080 lit #4371 ; #341
[new-match] 0x61d50e6b5dc0 #552 #550 #296 #4080 ; #4394
[eq-expl] #4394 root
[new-match] 0x61d50e6b5df8 #174 #173 #4394 ; #4395
[new-match] 0x61d50e6b5e28 #3286 #1569 #2574 #3934 #1167 #125 ; #4392
[mk-app] #4312 or #4451 #4294
[instance] 0x61d50e6b5d40 ; 4
[attach-enode] #4293 4
[attach-enode] #4294 4
[assign] #4294 justification -1: 414
[end-of-instance]
[mk-app] #4322 + #4393 #4412 #4279
[inst-discovered] theory-solving 0 arith# ; #4271
[mk-app] #4323 = #4271 #4322
[instance] 0 #4323
[attach-enode] #4323 0
[end-of-instance]
[mk-app] #4323 = #4322 #341
[mk-app] #4305 or #3887 #4323
[instance] 0x61d50e6b5d88 ; 4
[attach-enode] #4279 4
[attach-enode] #4322 4
[attach-enode] #4323 4
[mk-app] #4306 <= #4322 #341
[mk-app] #4230 >= #4322 #341
[assign] #4323 justification -1: 77
[end-of-instance]
[mk-app] #4524 + #296 #4505
[inst-discovered] theory-solving 0 arith# ; #4506
[mk-app] #4527 = #4506 #4524
[instance] 0 #4527
[attach-enode] #4527 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4527 = #4504 #296
[mk-app] #4544 = #4524 #341
[inst-discovered] theory-solving 0 arith# ; #4544
[mk-app] #4545 = #4544 #4527
[instance] 0 #4545
[attach-enode] #4545 0
[end-of-instance]
[mk-app] #4524 or #3887 #4527
[instance] 0x61d50e6b5dc0 ; 4
[attach-enode] #4504 4
[attach-enode] #4527 4
[assign] #4527 justification -1: 77
[end-of-instance]
[mk-app] #4544 or #3871 #4514
[instance] 0x61d50e6b5df8 ; 4
[attach-enode] #4513 4
[attach-enode] #4514 4
[assign] #4514 justification -1: 26
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4517
[mk-app] #4545 = #4517 #4517
[instance] 0 #4545
[attach-enode] #4545 0
[end-of-instance]
[mk-app] #4545 or #4465 #3966 #4072 #4516
[instance] 0x61d50e6b5e28 ; 4
[attach-enode] #4516 4
[assign] #4516 justification -1: 305 626 723
[end-of-instance]
[assign] #4326 clause 771 -770
[assign] #4330 clause 772 -770
[assign] #4306 clause 777 -776
[assign] #4230 clause 778 -776
[mk-app] #4551 = #3639 #4413
[attach-meaning] #370 arith (- 1)
[mk-app] #4552 + #3639 #4279
[mk-app] #4553 <= #4552 #341
[mk-app] #4554 >= #4552 #341
[assign] #4551 justification -1: 759 760 743 592
[attach-enode] #4551 0
[attach-enode] #4552 0
[assign] #4553 justification -1: 782
[assign] #4554 justification -1: 782
[new-match] 0x61d50e6b68d8 #2320 #2319 #2316 #3638 #4395 #3934 ; #4293
[eq-expl] #4392 root
[new-match] 0x61d50e6b6920 #542 #236 #4392 #275 ; #4516 (#1167 #1167)
[new-match] 0x61d50e6b6958 #240 #236 #4392 #275 ; #4516 (#1167 #1167)
[new-match] 0x61d50e6b6990 #1136 #455 #749 #275 ; #4326
[new-match] 0x61d50e6b69c8 #174 #173 #749 ; #4329
[eq-expl] #4329 lit #4330 ; #748
[new-match] 0x61d50e6b69f8 #503 #499 #749 #275 ; #4228 (#4227 #4329) (#1167 #1167)
[mk-app] #4543 or #4079 #4405 #4417
[instance] 0x61d50e6b6920 ; 5
[attach-enode] #4417 5
[assign] #4417 justification -1: 75 781
[end-of-instance]
[mk-app] #4546 or #4070 #4405 #4457
[instance] 0x61d50e6b6958 ; 5
[attach-enode] #4456 5
[attach-enode] #4457 5
[assign] #4457 justification -1: 34 781
[end-of-instance]
[mk-app] #4561 or #4341 #4347
[mk-app] #4562 = #4561 #4326
[mk-app] #4571 not #4561
[mk-app] #4564 not #4562
[inst-discovered] theory-solving 0 basic# ; #4564
[mk-app] #4571 = #4564 #4564
[instance] 0 #4571
[attach-enode] #4571 0
[end-of-instance]
[mk-app] #4571 or #3836 #4564
[instance] 0x61d50e6b6990 ; 3
[attach-enode] #4342 3
[attach-enode] #4343 3
[attach-enode] #4346 3
[assign] (not #4562) justification -1: 61
[end-of-instance]
[assign] (not #4561) clause -789 790
[assign] #4344 clause 787 789
[assign] (not #4347) clause -788 789
[new-match] 0x61d50e6c0a28 #1136 #455 #4393 #275 ; #4417
[new-match] 0x61d50e6c0a60 #174 #173 #4393 ; #4456
[eq-expl] #4392 lit #4457 ; #4456
[eq-expl] #4456 root
[new-match] 0x61d50e6c0a90 #503 #499 #4393 #275 ; #4516 (#4392 #4456) (#1167 #1167)
[mk-app] #4563 or #4509 #4518
[mk-app] #4560 = #4563 #4417
[mk-app] #4547 not #4563
[mk-app] #4235 not #4560
[inst-discovered] theory-solving 0 basic# ; #4235
[mk-app] #4547 = #4235 #4235
[instance] 0 #4547
[attach-enode] #4547 0
[end-of-instance]
[mk-app] #4547 or #3836 #4235
[instance] 0x61d50e6c0a28 ; 6
[attach-enode] #4519 6
[attach-enode] #4520 6
[attach-enode] #4200 6
[assign] (not #4560) justification -1: 61
[end-of-instance]
[assign] (not #4563) clause -793 794
[assign] #4521 clause 791 793
[assign] (not #4518) clause -792 793
[decide-and-or] #3689 #3680
[push] 1
[assign] (not #3645) decision axiom
[assign] #739 clause 493 494
[assign] (not #779) clause -490 494
[assign] (not #3630) clause -491 490
[assign] #3714 clause 536 490
[mk-app] #4274 = #3639 #4231
[attach-meaning] #370 arith (- 1)
[mk-app] #4328 + #3639 #4232
[mk-app] #4332 <= #4328 #341
[mk-app] #4349 >= #4328 #341
[attach-enode] #4274 0
[attach-enode] #4328 0
[assign] (not #4274) justification -1: -491 671 592
[decide-and-or] #3942 #3969
[push] 2
[assign] (not #3968) decision axiom
[assign] (not #4494) justification -1: -627 677
[push] 3
[assign] (not #4382) decision axiom
[assign] #4222 clause 755 674
[eq-expl] #4396 root
[new-match] 0x61d50e6c1010 #552 #550 #4396 #4393 ; #4397
[new-match] 0x61d50e6c1048 #2320 #2319 #2351 #1276 #4395 #3934 ; #4396
[mk-app] #4350 + #4393 #4396 #4496
[inst-discovered] theory-solving 0 arith# ; #4497
[mk-app] #4317 = #4497 #4350
[instance] 0 #4317
[attach-enode] #4317 0
[end-of-instance]
[mk-app] #4317 = #4350 #341
[mk-app] #4374 or #3887 #4317
[instance] 0x61d50e6c1010 ; 4
[attach-enode] #4496 4
[attach-enode] #4350 4
[attach-enode] #4317 4
[mk-app] #4401 <= #4350 #341
[mk-app] #4431 >= #4350 #341
[assign] #4317 justification -1: 77
[end-of-instance]
[mk-app] #4427 or #4451 #4511
[instance] 0x61d50e6c1048 ; 4
[attach-enode] #4510 4
[attach-enode] #4511 4
[assign] #4511 justification -1: 414
[end-of-instance]
[assign] #4401 clause 799 -798
[assign] #4431 clause 800 -798
[mk-app] #4590 = #2899 #4397
[attach-meaning] #370 arith (- 1)
[mk-app] #4591 + #2899 #4496
[mk-app] #4592 <= #4591 #341
[mk-app] #4596 >= #4591 #341
[assign] #4590 justification -1: 755 756 741 671 592
[attach-enode] #4590 0
[attach-enode] #4591 0
[assign] #4592 justification -1: 802
[assign] #4596 justification -1: 802
[new-match] 0x61d50e6c1688 #2320 #2319 #2316 #1276 #4395 #3934 ; #4510
[decide-and-or] #4078 #4134
[push] 4
[assign] (not #4133) decision axiom
[new-match] 0x61d50e6c1788 #199 #195 #4116 ; #4133 (#189 #189)
[eq-expl] #4115 lit #4196 ; #4139
[eq-expl] #4139 root
[new-match] 0x61d50e6c17b8 #467 #466 #4115 ; #4133 (#189 #189) (#4116 #4116)
[mk-app] #4597 I #4139
[mk-app] #4598 has_type #4597 #189
[mk-app] #4599 not #467
[mk-app] #4600 or #4599 #4598
[instance] 0x61d50e6c17b8 ; 4
[attach-enode] #4597 4
[attach-enode] #4598 4
[assign] #4598 justification -1: 63
[end-of-instance]
[resolve-lit] 0 (not #4598)
[resolve-process] #4598
[resolve-lit] 0 #4133
[resolve-process] (not #4598)
[conflict] #4133
[pop] 1 5
[assign] #4133 axiom
[assign] #4136 clause 765 -764
[assign] #4198 clause 767 -764
[assign] (not #4150) clause -766 -764
[assign] #4146 clause 660 766
[assign] #4148 clause 663 766
[new-match] 0x61d50e6c1798 #199 #195 #4116 ; #4133 (#189 #189)
[new-match] 0x61d50e6c17c8 #467 #466 #4115 ; #4133 (#189 #189) (#4116 #4116)
[eq-expl] #4118 root
[new-match] 0x61d50e6c17f8 #542 #236 #4118 #275 ; #4198 (#1167 #1167)
[new-match] 0x61d50e6c1830 #240 #236 #4118 #275 ; #4198 (#1167 #1167)
[new-match] 0x61d50e6c1868 #2320 #2319 #2352 #4116 #2574 #3934 ; #4135
[new-match] 0x61d50e6c18b0 #3437 #2337 #2351 #4116 #2574 #3934 ; #4135 (#2352 #2352)
[mk-app] #4599 not #4198
[mk-app] #4600 iInv #275 #4119
[mk-app] #4601 or #4599 #4600
[mk-app] #4602 or #4079 #4599 #4600
[instance] 0x61d50e6c17f8 ; 4
[attach-enode] #4600 4
[assign] #4600 justification -1: 75 767
[end-of-instance]
[mk-app] #4603 I #4119
[mk-app] #4611 = #4118 #4603
[mk-app] #4637 or #4599 #4611
[mk-app] #4612 or #4070 #4599 #4611
[instance] 0x61d50e6c1830 ; 4
[attach-enode] #4603 4
[attach-enode] #4611 4
[assign] #4611 justification -1: 34 767
[end-of-instance]
[mk-app] #4654 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #2574 #4116 #2316
[mk-app] #4655 = #4135 #4654
[mk-app] #4656 or #4451 #4655
[instance] 0x61d50e6c1868 ; 4
[attach-enode] #4654 4
[attach-enode] #4655 4
[assign] #4655 justification -1: 414
[end-of-instance]
[mk-app] #4657 >= #4155 #341
[mk-app] #4660 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #4395 #4116 #2351
[mk-app] #4661 Add #4393 #4660
[mk-app] #4662 if #4657 #341 #4661
[mk-app] #4663 = #4135 #4662
[mk-app] #4664 or #3966 #4072 #4134 #4663
[inst-discovered] theory-solving 0 basic# ; #4664
[mk-app] #4620 = #4664 #4664
[instance] 0 #4620
[attach-enode] #4620 0
[end-of-instance]
[mk-app] #4620 or #4372 #3966 #4072 #4134 #4663
[instance] 0x61d50e6c18b0 ; 4
[mk-app] #4691 = #4662 #341
[mk-app] #4678 = #4661 #4662
[attach-enode] #4662 4
[attach-enode] #4660 4
[attach-enode] #4661 4
[attach-enode] #4691 4
[attach-enode] #4678 4
[attach-enode] #4663 4
[assign] #4663 justification -1: 415 626 723 764
[end-of-instance]
[new-match] 0x61d50e6c2050 #1136 #455 #4119 #275 ; #4600
[new-match] 0x61d50e6c2088 #174 #173 #4119 ; #4603
[new-match] 0x61d50e6c20b8 #2320 #2319 #2316 #4116 #2574 #3934 ; #4654
[eq-expl] #4118 lit #4611 ; #4603
[eq-expl] #4603 root
[new-match] 0x61d50e6c2100 #503 #499 #4119 #275 ; #4198 (#4118 #4603) (#1167 #1167)
[mk-app] #4670 + #4119 #3817
[mk-app] #4671 >= #4670 #341
[mk-app] #4666 not #4671
[mk-app] #4775 + #4119 #3821
[mk-app] #4798 >= #4775 #341
[mk-app] #4801 or #4666 #4798
[mk-app] #4777 = #4801 #4600
[mk-app] #4677 not #4777
[mk-app] #4729 + #3817 #4119
[inst-discovered] theory-solving 0 arith# ; #4670
[mk-app] #4701 = #4670 #4729
[instance] 0 #4701
[attach-enode] #4701 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4701 * #370 #4119
[mk-app] #4658 + #313 #4701
[mk-app] #4659 <= #4658 #341
[mk-app] #4641 >= #4729 #341
[inst-discovered] theory-solving 0 arith# ; #4641
[mk-app] #4643 = #4641 #4659
[instance] 0 #4643
[attach-enode] #4643 0
[end-of-instance]
[mk-app] #4729 not #4659
[mk-app] #4641 + #3821 #4119
[inst-discovered] theory-solving 0 arith# ; #4775
[mk-app] #4643 = #4775 #4641
[instance] 0 #4643
[attach-enode] #4643 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4643 + #335 #4701
[mk-app] #4640 <= #4643 #341
[mk-app] #4638 >= #4641 #341
[inst-discovered] theory-solving 0 arith# ; #4638
[mk-app] #4614 = #4638 #4640
[instance] 0 #4614
[attach-enode] #4614 0
[end-of-instance]
[mk-app] #4641 or #4729 #4640
[mk-app] #4638 = #4641 #4600
[mk-app] #4614 not #4641
[mk-app] #4605 not #4638
[inst-discovered] theory-solving 0 basic# ; #4605
[mk-app] #4614 = #4605 #4605
[instance] 0 #4614
[attach-enode] #4614 0
[end-of-instance]
[mk-app] #4614 or #3836 #4605
[instance] 0x61d50e6c2050 ; 5
[attach-enode] #4701 5
[attach-enode] #4658 5
[attach-enode] #4643 5
[assign] (not #4638) justification -1: 61
[end-of-instance]
[assign] (not #4641) clause -814 815
[assign] #4659 clause 812 814
[assign] (not #4640) clause -813 814
[push] 4
[assign] (not #4332) decision axiom
[assign] #4349 clause 797 796
[push] 5
[assign] (not #4657) decision axiom
[assign] #4678 clause 810 808
[eq-expl] #4660 root
[new-match] 0x61d50e6c2410 #552 #550 #4660 #4393 ; #4661
[new-match] 0x61d50e6c2448 #2320 #2319 #2351 #4116 #4395 #3934 ; #4660
[mk-app] #4604 * #370 #4661
[mk-app] #4582 + #4660 #4393 #4604
[mk-app] #4583 = #4582 #341
[mk-app] #4584 + #4393 #4660 #4604
[inst-discovered] theory-solving 0 arith# ; #4582
[mk-app] #4289 = #4582 #4584
[instance] 0 #4289
[attach-enode] #4289 0
[end-of-instance]
[mk-app] #4289 = #4584 #341
[mk-app] #4528 or #3887 #4289
[instance] 0x61d50e6c2410 ; 5
[attach-enode] #4604 5
[attach-enode] #4584 5
[attach-enode] #4289 5
[mk-app] #4208 <= #4584 #341
[mk-app] #4201 >= #4584 #341
[assign] #4289 justification -1: 77
[end-of-instance]
[mk-app] #4548 lib!Chap28.MCSSSpec.MCSSSpec.rec%spec_range_sum.? #3934 #4395 #4116 #2316
[mk-app] #4559 = #4660 #4548
[mk-app] #4574 or #4451 #4559
[instance] 0x61d50e6c2448 ; 5
[attach-enode] #4548 5
[attach-enode] #4559 5
[assign] #4559 justification -1: 414
[end-of-instance]
[assign] #4208 clause 817 -816
[assign] #4201 clause 818 -816
[mk-app] #4575 = #4117 #4661
[attach-meaning] #370 arith (- 1)
[mk-app] #4576 + #4117 #4604
[mk-app] #4577 <= #4576 #341
[mk-app] #4569 >= #4576 #341
[assign] #4575 justification -1: 765 810 811
[attach-enode] #4575 0
[attach-enode] #4576 0
[assign] #4577 justification -1: 820
[assign] #4569 justification -1: 820
[new-match] 0x61d50e6cac40 #2320 #2319 #2316 #4116 #4395 #3934 ; #4548
[mk-app] #4570 = #1454 #4139
[attach-enode] #4570 0
[attach-meaning] #370 arith (- 1)
[mk-app] #4568 + #1454 #4154
[mk-app] #4567 <= #4568 #341
[mk-app] #4565 >= #4568 #341
[attach-enode] #4568 0
[assign] #4565 clause 825 -583 -655 -679 -680 -573
[assign] #4567 clause 824 -584 -652 -684 -685 -574
[assign] #4570 clause 823 -824 -825
[resolve-lit] 4 #3630
[resolve-process] (not #3630)
[resolve-lit] 0 (not #4570)
[conflict] (not #4570) #3630
[pop] 4 6
[attach-enode] #4568 0
[assign] #4565 clause 798 -583 -655 -679 -680 -573
[assign] #4567 clause 799 -584 -652 -684 -685 -574
[assign] #4133 axiom
[attach-enode] #4570 0
[attach-meaning] #370 arith (- 1)
[assign] #4570 justification -1: 798 799
[resolve-process] true
[resolve-lit] 0 (not #4570)
[resolve-lit] 0 #3630
[resolve-process] (not #4570)
[resolve-lit] 0 (not #4565)
[resolve-lit] 0 (not #4567)
[resolve-process] (not #4567)
[resolve-process] (not #4565)
[conflict] #3630
[pop] 1 2
[attach-enode] #4568 0
[assign] #4565 clause 795 -583 -655 -679 -680 -573
[assign] #4567 clause 796 -584 -652 -684 -685 -574
[attach-enode] #4570 0
[attach-meaning] #370 arith (- 1)
[assign] #4570 justification -1: 795 796
[assign] #3630 clause 491 -797
[assign] #4133 axiom
[assign] #779 clause 490 -491
[assign] #4136 clause 765 -764
[assign] #4198 clause 767 -764
[assign] (not #4150) clause -766 -764
[assign] #3645 clause 494 -490
[assign] #4146 clause 660 766
[assign] #4148 clause 663 766
[assign] (not #3714) clause -536 -494
[assign] #696 clause 495 536
[assign] #3712 clause 535 536
[assign] (not #3708) clause -534 -535
[assign] #721 clause 499 534
[assign] #3674 clause 533 534
[mk-app] #4274 = #3639 #4231
[attach-meaning] #370 arith (- 1)
[mk-app] #4328 + #3639 #4232
[mk-app] #4332 <= #4328 #341
[mk-app] #4349 >= #4328 #341
[assign] #4274 justification -1: 491 671 592
[attach-enode] #4274 0
[attach-enode] #4328 0
[assign] #4332 justification -1: 798
[assign] #4349 justification -1: 798
[mk-app] #4350 = #4067 #4117
[attach-meaning] #370 arith (- 1)
[assign] #4350 justification -1: 797 768 592
[attach-enode] #4350 0
[assign] #4225 justification -1: 801
[assign] #4226 justification -1: 801
[mk-app] #4317 = #749 #4119
[attach-meaning] #370 arith (- 1)
[mk-app] #4401 + #749 #4701
[mk-app] #4431 <= #4401 #341
[mk-app] #4590 >= #4401 #341
[assign] #4317 justification -1: 797 768 592
[attach-enode] #4317 0
[attach-enode] #4701 0
[attach-enode] #4401 0
[assign] #4431 justification -1: 802
[assign] #4590 justification -1: 802
[new-match] 0x61d50e6c1688 #2320 #2319 #2352 #4116 #2574 #3934 ; #4135
[new-match] 0x61d50e6c16d0 #3437 #2337 #2351 #4116 #2574 #3934 ; #4135 (#2352 #2352)
[eq-expl] #1454 lit #4570 ; #4139
[eq-expl] #1276 cg (#1454 #4115) ; #4116
[eq-expl] #720 root
[new-match] 0x61d50e6c1718 #3408 #2204 #720 #1276 #1533 #1167 #125 #1534 #125 ; #721
[inst-discovered] theory-solving 0 basic# ; #4664
[mk-app] #4591 = #4664 #4664
[instance] 0 #4591
[attach-enode] #4591 0
[end-of-instance]
[mk-app] #4591 or #4372 #3966 #4072 #4134 #4663
[instance] 0x61d50e6c16d0 ; 4
[mk-app] #4592 = #4662 #341
[mk-app] #4596 = #4661 #4662
[attach-enode] #4662 4
[attach-enode] #4660 4
[attach-enode] #4661 4
[attach-enode] #4592 4
[attach-enode] #4596 4
[attach-enode] #4663 4
[assign] #4663 justification -1: 415 626 723 764
[end-of-instance]
[mk-app] #4691 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.spec_index.? #125 #1534 #125 #1167 #3765 #4116
[mk-app] #4678 = #720 #4691
[mk-app] #4641 not #4678
[mk-app] #4638 or #4297 #4641
[mk-app] #4584 ens%lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphTrait.nth. #125 #1534 #125 #1167 #3765 #4116 #720
[mk-app] #4289 = #4638 #4584
[mk-app] #4208 not #4289
[mk-app] #4201 not #4638
[inst-discovered] theory-solving 0 basic# ; #4208
[mk-app] #4201 = #4208 #4208
[instance] 0 #4201
[attach-enode] #4201 0
[end-of-instance]
[mk-app] #4201 not #3408
[mk-app] #4575 or #4201 #4208
[instance] 0x61d50e6c1718 ; 1
[attach-enode] #4296 1
[attach-enode] #4691 1
[attach-enode] #4678 1
[attach-enode] #4584 1
[assign] (not #4289) justification -1: 397
[end-of-instance]
[assign] #4584 justification -1: 499 797 768 539
[assign] (not #4638) clause -811 -812 813
[assign] #4296 clause 809 811
[assign] #4678 clause 810 811
[assign] #4299 justification -1: 810 797 768
[eq-expl] #720 lit #4678 ; #4691
[eq-expl] #4691 cg (#125 #125) (#1534 #1534) (#125 #125) (#1167 #1167) (#3765 #3765) (#4116 #1276) ; #4298
[eq-expl] #4298 root
[new-match] 0x61d50e6c2178 #542 #236 #720 #275 ; #4296 (#1167 #1167)
[new-match] 0x61d50e6c21b0 #240 #236 #720 #275 ; #4296 (#1167 #1167)
[new-match] 0x61d50e6c21e8 #3402 #2179 #1276 #3765 #1167 #125 #1534 #125 ; #4298
[new-match] 0x61d50e6c2240 #2269 #2265 #1276 #3765 #1167 #125 ; #4298 (#125 #125) (#1534 #1534)
[mk-app] #4576 or #4079 #4308 #4310
[instance] 0x61d50e6c2178 ; 2
[attach-enode] #4307 2
[attach-enode] #4309 2
[attach-enode] #4310 2
[end-of-instance]
[mk-app] #4577 or #4070 #4308 #4315
[instance] 0x61d50e6c21b0 ; 2
[attach-enode] #4314 2
[attach-enode] #4315 2
[end-of-instance]
[mk-app] #4569 vstd!seq.Seq.index.? #125 #1167 #4266 #4116
[mk-app] #4574 = #4691 #4569
[mk-app] #4528 or #3930 #4574
[mk-app] #4605 not #2269
[mk-app] #4614 or #4605 #3930 #4574
[instance] 0x61d50e6c2240 ; 2
[attach-enode] #4264 2
[mk-app] #4620 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #4264
[mk-app] #4656 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS #4620
[attach-enode] #4620 2
[attach-enode] #4656 2
[mk-app] #4612 lib!Chap19.ArraySeqStEph.ArraySeqStEph.ArraySeqStEphS./ArraySeqStEphS/?seq #4656
[mk-app] #4602 = #4620 #4612
[new-match] 0 datatype#21 datatype#18 #4620 ; #4656
[instance] 0 #4602
[attach-enode] #4612 2
[end-of-instance]
[attach-enode] #4265 2
[attach-enode] #4266 2
[attach-enode] #4569 2
[attach-enode] #4574 2
[assign] #4574 justification -1: 18 404
[end-of-instance]
[assign] #706 clause 501 -670 -495
[assign] #3649 clause 502 -501
[assign] (not #3682) clause -532 -502 -533
[assign] #687 clause 503 532
[assign] #673 clause 504 532
[assign] #3687 clause 531 532
[assign] #4460 clause 692 -503 -670 -495 -501
[assign] #4307 justification -1: 809 810 797 768
[assign] #4310 justification -1: 771 810 817 797 768 592 592 539 810
[assign] #4315 justification -1: 817 772 810 817 810 592 592 539 797 768
[eq-expl] #672 root
[new-match] 0x61d50e6cae78 #3390 #2134 #672 #658 #1289 #1147 #125 #1167 #125 ; #673
[eq-expl] #741 lit #696 ; #1462
[eq-expl] #4691 lit #4574 ; #4569
[eq-expl] #4264 cg (#3765 #1533) ; #1393
[eq-expl] #4265 cg (#4264 #1393) ; #1395
[eq-expl] #4266 cg (#125 #125) (#1168 #1168) (#4265 #1395) ; #1396
[eq-expl] #4569 cg (#125 #125) (#1167 #1167) (#4266 #3934) (#4116 #4116) ; #4118
[eq-expl] #4118 cg (#125 #125) (#1167 #1167) (#3934 #1396) (#4116 #1276) ; #748
[eq-expl] #713 cg (#720 #748) ; #749
[eq-expl] #705 cg (#741 #782) (#713 #749) ; #750
[eq-expl] #750 lit #3630 ; #3639
[new-match] 0x61d50e6caed8 #1136 #455 #705 #275 ; #706
[eq-expl] #698 lit #687 ; #686
[eq-expl] #686 root
[new-match] 0x61d50e6caf10 #174 #173 #698 ; #672
[eq-expl] #652 root
[new-match] 0x61d50e6caf40 #1166 #1165 #652 ; #658
[new-match] 0x61d50e6caf70 #2956 #392 #705 #275 ; #686
[mk-app] #4602 not #4423
[inst-discovered] theory-solving 0 basic# ; #4425
[mk-app] #4602 = #4425 #4425
[instance] 0 #4602
[attach-enode] #4602 0
[end-of-instance]
[mk-app] #4602 not #3390
[mk-app] #4427 or #4602 #4425
[instance] 0x61d50e6cae78 ; 1
[attach-enode] #4406 1
[assign] (not #4424) justification -1: 389
[end-of-instance]
[mk-app] #4374 = #4364 #4357
[mk-app] #4566 not #4364
[mk-app] #4778 not #4374
[inst-discovered] theory-solving 0 basic# ; #4778
[mk-app] #4566 = #4778 #4778
[instance] 0 #4566
[attach-enode] #4566 0
[end-of-instance]
[mk-app] #4566 or #3836 #4778
[instance] 0x61d50e6caed8 ; 1
[attach-enode] #4357 1
[assign] (not #4374) justification -1: 61
[end-of-instance]
[mk-app] #4779 or #3871 #4430
[instance] 0x61d50e6caf10 ; 1
[attach-enode] #4428 1
[attach-enode] #4429 1
[attach-enode] #4430 1
[assign] #4430 justification -1: 26
[end-of-instance]
[mk-app] #4804 or #3867 #4433
[instance] 0x61d50e6caf40 ; 1
[attach-enode] #4432 1
[attach-enode] #4433 1
[assign] #4433 justification -1: 253
[end-of-instance]
[mk-app] #4806 + #3821 #4435
[inst-discovered] theory-solving 0 arith# ; #4440
[mk-app] #4811 = #4440 #4806
[instance] 0 #4811
[attach-enode] #4811 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4811 + #335 #4436
[mk-app] #4812 <= #4811 #341
[mk-app] #4813 >= #4806 #341
[inst-discovered] theory-solving 0 arith# ; #4813
[mk-app] #4814 = #4813 #4812
[instance] 0 #4814
[attach-enode] #4814 0
[end-of-instance]
[mk-app] #4806 or #4360 #4366 #4442
[inst-discovered] theory-solving 0 basic# ; #4806
[mk-app] #4813 = #4806 #4806
[instance] 0 #4813
[attach-enode] #4813 0
[end-of-instance]
[mk-app] #4813 not #4806
[mk-app] #4814 or #4439 #4812 #4813
[inst-discovered] theory-solving 0 basic# ; #4814
[mk-app] #4810 = #4814 #4814
[instance] 0 #4810
[attach-enode] #4810 0
[end-of-instance]
[mk-app] #4810 not #4814
[mk-app] #4807 not #2956
[mk-app] #4803 or #4807 #4810
[instance] 0x61d50e6caf70 ; 1
[attach-enode] #4435 1
[attach-enode] #4436 1
[attach-enode] #4437 1
[attach-enode] #4811 1
[attach-enode] #4442 1
[attach-meaning] #370 arith (- 1)
[mk-app] #4805 + #3639 #4436
[mk-app] #4815 <= #4805 #341
[mk-app] #4816 >= #4805 #341
[attach-enode] #4805 1
[assign] (not #4814) justification -1: 58
[end-of-instance]
[assign] (not #4423) clause -819 820
[assign] #4357 clause 821 822
[assign] #4438 clause 825 831
[assign] (not #4812) clause -826 831
[assign] #4806 clause 830 831
[assign] #4406 clause 818 819
[assign] #4421 clause 694 819
[assign] #4442 clause 827 -830
[assign] #4815 clause 828 -827
[assign] #4816 clause 829 -827
[assign] #4538 justification -1: 827 491 810 817 797 768 592 592 539 495 481 671 491
[new-match] 0x61d50e6cbb18 #1178 #1174 #658 ; #4406 (#1168 #1168)
[eq-expl] #652 lit #4433 ; #4432
[eq-expl] #4432 root
[new-match] 0x61d50e6cbb48 #1181 #1180 #652 ; #4406 (#1168 #1168) (#658 #658)
[eq-expl] #1290 lit #4379 ; #4378
[new-match] 0x61d50e6cbb78 #3310 #1690 #672 #1290 #1167 #125 ; #4420
[new-match] 0x61d50e6cbbc0 #1810 #1807 #658 #1168 #125 ; #459
[inst-discovered] theory-solving 0 basic# ; #4464
[mk-app] #4817 = #4464 #4464
[instance] 0 #4817
[attach-enode] #4817 0
[end-of-instance]
[mk-app] #4817 not #3310
[mk-app] #4818 or #4817 #4459 #4461 #4463
[instance] 0x61d50e6cbb78 ; 2
[attach-enode] #4462 2
[attach-enode] #4463 2
[assign] #4463 justification -1: 321 692 710
[end-of-instance]
[mk-app] #4819 or #3853 #4389 #4467
[instance] 0x61d50e6cbbc0 ; 1
[attach-enode] #4467 1
[assign] #4467 justification -1: 332 818
[end-of-instance]
[eq-expl] #459 lit #4421 ; #4420
[eq-expl] #4420 root
[new-match] 0x61d50e6cbe90 #1198 #1194 #459 ; #4467 (#3850 #1188)
[mk-app] #4820 or #3939 #4471 #4474
[instance] 0x61d50e6cbe90 ; 2
[attach-enode] #4470 2
[attach-enode] #4472 2
[attach-enode] #4473 2
[attach-enode] #4474 2
[end-of-instance]
[assign] #4470 justification -1: 833 591 694
[assign] #4474 clause 835 -834
[eq-expl] #4472 root
[new-match] 0x61d50e6cc180 #1187 #1186 #4472 ; #4473
[eq-expl] #4473 lit #4474 ; #4420
[new-match] 0x61d50e6cc1b0 #1201 #1200 #4472 ; #4467 (#3850 #1188) (#459 #4473)
[decide-and-or] #3687 #3659
[push] 1
[assign] (not #3703) decision axiom
[assign] #674 clause 505 507
[assign] (not #3704) clause -506 507
[assign] #3705 clause 530 506
[eq-expl] #274 lit #278 ; #277
[eq-expl] #277 root
[new-match] 0x61d50e6cc1f8 #1086 #449 #1025 #274 ; #3704
[mk-app] #4821 * #370 #291
[mk-app] #4822 + #1025 #4821
[mk-app] #4823 >= #4822 #341
[mk-app] #4824 or #4478 #4823
[mk-app] #4825 uInv #277 #1025
[mk-app] #4826 = #4824 #4825
[mk-app] #4827 not #4826
[mk-app] #4828 + #4821 #1025
[inst-discovered] theory-solving 0 arith# ; #4822
[mk-app] #4829 = #4822 #4828
[instance] 0 #4829
[attach-enode] #4829 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4829 + #291 #4202
[mk-app] #4830 <= #4829 #341
[mk-app] #4831 >= #4828 #341
[inst-discovered] theory-solving 0 arith# ; #4831
[mk-app] #4832 = #4831 #4830
[instance] 0 #4832
[attach-enode] #4832 0
[end-of-instance]
[mk-app] #4828 or #4478 #4830
[mk-app] #4831 = #4828 #4825
[mk-app] #4832 not #4828
[mk-app] #4833 not #4831
[inst-discovered] theory-solving 0 basic# ; #4833
[mk-app] #4832 = #4833 #4833
[instance] 0 #4832
[attach-enode] #4832 0
[end-of-instance]
[mk-app] #4832 or #3800 #4833
[instance] 0x61d50e6cc1f8 ; 1
[attach-enode] #4829 1
[attach-enode] #4825 1
[assign] (not #4831) justification -1: 60
[end-of-instance]
[assign] (not #4825) justification -1: -506 40
[mk-app] #4834 <= #291 #292
[mk-app] #4835 >= #291 #292
[assign] #4834 justification -1: 44
[assign] #4835 justification -1: 44
[assign] #4828 clause 837 838 839
[assign] #4830 clause 836 -837
[resolve-process] true
[resolve-lit] 0 (not #4835)
[resolve-lit] 0 (not #4830)
[resolve-process] (not #4830)
[resolve-lit] 0 (not #4828)
[resolve-process] (not #4828)
[resolve-lit] 0 #4825
[resolve-lit] 0 #4831
[resolve-process] (not #4835)
[resolve-process] #4825
[resolve-lit] 0 #3704
[resolve-process] #4831
[conflict] #3704
[pop] 1 2
[assign] #3704 axiom
[assign] #3703 clause 507 -506
[assign] (not #3705) clause -530 -507 -531
[assign] #3701 clause 508 530
[assign] #3735 clause 529 530
[new-match] 0x61d50e6cc208 #1086 #449 #1025 #274 ; #3704
[new-match] 0x61d50e6cc240 #2432 #365 #1025 #274 ; #3699
[mk-app] #4828 or #4478 #4830
[mk-app] #4831 = #4828 #4825
[mk-app] #4834 not #4828
[mk-app] #4835 not #4831
[inst-discovered] theory-solving 0 basic# ; #4835
[mk-app] #4834 = #4835 #4835
[instance] 0 #4834
[attach-enode] #4834 0
[end-of-instance]
[mk-app] #4834 or #3800 #4835
[instance] 0x61d50e6cc208 ; 1
[attach-enode] #4829 1
[attach-enode] #4825 1
[assign] (not #4831) justification -1: 60
[end-of-instance]
[mk-app] #4833 uClip #277 #1025
[mk-app] #4832 >= #4833 #341
[mk-app] #4836 not #4832
[mk-app] #4837 + #4833 #4821
[mk-app] #4838 >= #4837 #341
[mk-app] #4839 = #1025 #4833
[mk-app] #4840 or #4478 #4823 #4839
[mk-app] #4841 not #4840
[mk-app] #4842 or #4836 #4838 #4841
[mk-app] #4843 not #4842
[mk-app] #4844 + #4821 #4833
[inst-discovered] theory-solving 0 arith# ; #4837
[mk-app] #4845 = #4837 #4844
[instance] 0 #4845
[attach-enode] #4845 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4845 * #370 #4833
[mk-app] #4846 + #291 #4845
[mk-app] #4847 <= #4846 #341
[mk-app] #4848 >= #4844 #341
[inst-discovered] theory-solving 0 arith# ; #4848
[mk-app] #4849 = #4848 #4847
[instance] 0 #4849
[attach-enode] #4849 0
[end-of-instance]
[mk-app] #4844 or #4478 #4830 #4839
[inst-discovered] theory-solving 0 basic# ; #4844
[mk-app] #4848 = #4844 #4844
[instance] 0 #4848
[attach-enode] #4848 0
[end-of-instance]
[mk-app] #4848 not #4844
[mk-app] #4849 or #4836 #4847 #4848
[inst-discovered] theory-solving 0 basic# ; #4849
[mk-app] #4850 = #4849 #4849
[instance] 0 #4850
[attach-enode] #4850 0
[end-of-instance]
[mk-app] #4850 not #4849
[mk-app] #4851 not #2432
[mk-app] #4852 or #4851 #4850
[instance] 0x61d50e6cc240 ; 1
[attach-enode] #4833 1
[attach-enode] #4845 1
[attach-enode] #4846 1
[attach-enode] #4839 1
[attach-meaning] #370 arith (- 1)
[mk-app] #4853 + #1025 #4845
[mk-app] #4854 <= #4853 #341
[mk-app] #4855 >= #4853 #341
[attach-enode] #4853 1
[assign] (not #4849) justification -1: 57
[end-of-instance]
[assign] #4832 clause 840 846
[assign] (not #4847) clause -841 846
[assign] #4844 clause 845 846
[assign] #4825 justification -1: 506 40
[mk-app] #4856 <= #291 #292
[mk-app] #4857 >= #291 #292
[assign] #4856 justification -1: 44
[assign] #4857 justification -1: 44
[mk-app] #4858 = #659 #4833
[attach-meaning] #370 arith (- 1)
[mk-app] #4859 + #659 #4845
[mk-app] #4860 <= #4859 #341
[mk-app] #4861 >= #4859 #341
[assign] #4858 justification -1: 508 40
[attach-enode] #4858 0
[attach-enode] #4859 0
[assign] #4860 justification -1: 849
[assign] #4861 justification -1: 849
[assign] (not #4828) clause -837 -838 839
[assign] (not #4830) clause -836 837
[assign] #4839 clause 842 836 -845
[assign] #4854 clause 843 -842
[assign] #4855 clause 844 -842
[mk-app] #4862 = #378 #4606
[attach-meaning] #370 arith (- 1)
[mk-app] #4863 <= #4623 #341
[assign] #4862 justification -1: 508 842 40
[attach-enode] #4862 0
[assign] #4863 justification -1: 852
[assign] #4635 justification -1: 852
[mk-app] #4864 = #384 #4690
[attach-meaning] #370 arith (- 1)
[mk-app] #4865 <= #4800 #341
[assign] #4864 justification -1: 508 842 40 478
[attach-enode] #4864 0
[assign] #4865 justification -1: 854
[assign] #4802 justification -1: 854
[assign] #3654 clause 511 484 -573 -583 -655 -844 -850
[assign] #3653 clause 510 -840 -851
[assign] #4750 clause 711 -712
[assign] (not #3721) clause -512 -510 -511
[assign] #3668 clause 513 512
[new-match] 0x61d50e6d8338 #552 #550 #296 #1025 ; #4606
[new-match] 0x61d50e6d8370 #567 #559 #1025 #1379 ; #4690
[attach-meaning] #370 arith (- 1)
[mk-app] #4866 = #4610 #370
[inst-discovered] theory-solving 0 arith# ; #4609
[mk-app] #4867 = #4609 #4866
[instance] 0 #4867
[attach-enode] #4867 0
[end-of-instance]
[mk-app] #4867 or #3887 #4866
[instance] 0x61d50e6d8338 ; 2
[attach-enode] #4866 2
[mk-app] #4868 >= #4610 #370
[assign] #4866 justification -1: 77
[end-of-instance]
[mk-app] #4869 or #3877 #4686
[instance] 0x61d50e6d8370 ; 2
[attach-enode] #3873 2
[attach-enode] #4685 2
[attach-enode] #4686 2
[mk-app] #4870 <= #4685 #341
[mk-app] #4871 >= #4685 #341
[assign] #4686 justification -1: 78
[end-of-instance]
[assign] #4632 clause 695 -856
[assign] #4868 clause 857 -856
[assign] #4870 clause 859 -858
[assign] #4871 clause 860 -858
[decide-and-or] #3735 #3661
[push] 1
[assign] (not #3663) decision axiom
[assign] #411 clause 514 516
[assign] (not #380) clause -515 516
[new-match] 0x61d50e6d8860 #2018 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6d88a0 #1545 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6d88e0 #3312 #1702 #672 #1290 #1167 #125 ; #460 (#459 #4420) (#125 #125) (#125 #125) (#1167 #1167) (#1167 #1167)
[inst-discovered] theory-solving 0 basic# ; #4619
[mk-app] #4872 = #4619 #4619
[instance] 0 #4872
[attach-enode] #4872 0
[end-of-instance]
[mk-app] #4872 or #3941 #4471 #3930 #4617 #4618
[instance] 0x61d50e6d8860 ; 1
[attach-enode] #4615 1
[attach-enode] #4616 1
[attach-enode] #4618 1
[end-of-instance]
[mk-app] #4873 or #3842 #4471 #4621
[instance] 0x61d50e6d88a0 ; 1
[assign] #4621 justification -1: 303 834
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4629
[mk-app] #4874 = #4629 #4629
[instance] 0 #4874
[attach-enode] #4874 0
[end-of-instance]
[mk-app] #4874 not #3312
[mk-app] #4875 or #4874 #4459 #4461 #3930 #4628
[instance] 0x61d50e6d88e0 ; 2
[attach-enode] #4624 2
[attach-enode] #4626 2
[attach-enode] #4627 2
[attach-enode] #4628 2
[assign] #4628 justification -1: 18 323 692 710
[end-of-instance]
[mk-app] #4876 = #378 #4615
[attach-meaning] #370 arith (- 1)
[mk-app] #4877 * #370 #4615
[mk-app] #4878 + #378 #4877
[mk-app] #4879 <= #4878 #341
[mk-app] #4880 >= #4878 #341
[attach-enode] #4876 0
[attach-enode] #4877 0
[attach-enode] #4878 0
[eq-expl] #4625 cg (#125 #125) (#1167 #1167) (#4378 #1290) ; #1273
[eq-expl] #4626 cg (#4625 #1025) (#296 #296) ; #4606
[eq-expl] #4606 root
[new-match] 0x61d50e6d9050 #2918 #348 #4626 ; #4627
[mk-app] #4881 nClip #4606
[mk-app] #4882 >= #4881 #341
[mk-app] #4883 not #4882
[mk-app] #4884 >= #4606 #341
[mk-app] #4885 not #4884
[mk-app] #4886 = #4606 #4881
[mk-app] #4887 or #4885 #4886
[mk-app] #4888 not #4887
[mk-app] #4889 or #4883 #4888
[mk-app] #4890 not #4889
[mk-app] #4891 not #2918
[mk-app] #4892 or #4891 #4890
[instance] 0x61d50e6d9050 ; 3
[attach-enode] #4881 3
[attach-enode] #4886 3
[attach-meaning] #370 arith (- 1)
[mk-app] #4893 * #370 #4881
[mk-app] #4894 + #4606 #4893
[mk-app] #4895 <= #4894 #341
[mk-app] #4896 >= #4894 #341
[attach-enode] #4893 3
[attach-enode] #4894 3
[assign] (not #4889) justification -1: 56
[end-of-instance]
[assign] #4882 clause 868 874
[assign] #4887 clause 873 874
[assign] (not #4876) justification -1: -515 694
[assign] (not #4886) justification -1: -515 864 694 709 480 508 842 40
[mk-app] #4897 = #4615 #4881
[attach-meaning] #370 arith (- 1)
[mk-app] #4898 + #4615 #4893
[mk-app] #4899 <= #4898 #341
[mk-app] #4900 >= #4898 #341
[assign] #4897 justification -1: 864 694 709 480 694
[attach-enode] #4897 0
[attach-enode] #4898 0
[assign] #4899 justification -1: 875
[assign] #4900 justification -1: 875
[assign] (not #4884) clause -869 870 -873
[resolve-process] true
[resolve-lit] 0 #4884
[conflict] #4884
[pop] 1 2
[assign] #4884 axiom
[assign] #4647 clause 696 -697 -861
[decide-and-or] #3735 #3661
[push] 1
[assign] (not #3663) decision axiom
[assign] #411 clause 514 516
[assign] (not #380) clause -515 516
[new-match] 0x61d50e6d8860 #2018 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6d88a0 #1545 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6d88e0 #3312 #1702 #672 #1290 #1167 #125 ; #460 (#459 #4420) (#125 #125) (#125 #125) (#1167 #1167) (#1167 #1167)
[inst-discovered] theory-solving 0 basic# ; #4619
[mk-app] #4876 = #4619 #4619
[instance] 0 #4876
[attach-enode] #4876 0
[end-of-instance]
[mk-app] #4876 or #3941 #4471 #3930 #4617 #4618
[instance] 0x61d50e6d8860 ; 1
[attach-enode] #4615 1
[attach-enode] #4616 1
[attach-enode] #4618 1
[end-of-instance]
[mk-app] #4877 or #3842 #4471 #4621
[instance] 0x61d50e6d88a0 ; 1
[assign] #4621 justification -1: 303 834
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4629
[mk-app] #4878 = #4629 #4629
[instance] 0 #4878
[attach-enode] #4878 0
[end-of-instance]
[mk-app] #4878 not #3312
[mk-app] #4879 or #4878 #4459 #4461 #3930 #4628
[instance] 0x61d50e6d88e0 ; 2
[attach-enode] #4624 2
[attach-enode] #4626 2
[attach-enode] #4627 2
[attach-enode] #4628 2
[assign] #4628 justification -1: 18 323 692 710
[end-of-instance]
[mk-app] #4880 = #378 #4615
[attach-meaning] #370 arith (- 1)
[mk-app] #4893 * #370 #4615
[mk-app] #4894 + #378 #4893
[mk-app] #4895 <= #4894 #341
[mk-app] #4896 >= #4894 #341
[attach-enode] #4880 0
[attach-enode] #4893 0
[attach-enode] #4894 0
[eq-expl] #4626 cg (#4625 #1025) (#296 #296) ; #4606
[new-match] 0x61d50e6d9050 #2918 #348 #4626 ; #4627
[mk-app] #4897 not #2918
[mk-app] #4898 or #4897 #4890
[instance] 0x61d50e6d9050 ; 3
[attach-enode] #4881 3
[attach-enode] #4886 3
[attach-meaning] #370 arith (- 1)
[mk-app] #4899 * #370 #4881
[mk-app] #4900 + #4606 #4899
[mk-app] #4891 <= #4900 #341
[mk-app] #4892 >= #4900 #341
[attach-enode] #4899 3
[attach-enode] #4900 3
[assign] (not #4889) justification -1: 56
[end-of-instance]
[assign] #4882 clause 869 874
[assign] #4887 clause 873 874
[assign] #4886 clause 870 -873
[assign] #4891 clause 871 -870
[assign] #4892 clause 872 -870
[assign] (not #4880) justification -1: -515 694
[resolve-lit] 0 #380
[resolve-process] (not #380)
[resolve-lit] 0 (not #4886)
[resolve-lit] 0 (not #4628)
[resolve-process] (not #4886)
[resolve-lit] 0 (not #4887)
[resolve-process] (not #4887)
[resolve-lit] 0 #4889
[resolve-process] #4889
[resolve-process] (not #4628)
[conflict] #380
[pop] 1 2
[assign] #380 axiom
[assign] #3663 clause 516 -515
[new-match] 0x61d50e6d8870 #2018 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6d88b0 #1545 #1540 #459 #1167 #125 ; #460
[new-match] 0x61d50e6d88f0 #3312 #1702 #672 #1290 #1167 #125 ; #460 (#459 #4420) (#125 #125) (#125 #125) (#1167 #1167) (#1167 #1167)
[inst-discovered] theory-solving 0 basic# ; #4619
[mk-app] #4880 = #4619 #4619
[instance] 0 #4880
[attach-enode] #4880 0
[end-of-instance]
[mk-app] #4880 or #3941 #4471 #3930 #4617 #4618
[instance] 0x61d50e6d8870 ; 1
[attach-enode] #4615 1
[attach-enode] #4616 1
[attach-enode] #4618 1
[end-of-instance]
[mk-app] #4893 or #3842 #4471 #4621
[instance] 0x61d50e6d88b0 ; 1
[assign] #4621 justification -1: 303 834
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4629
[mk-app] #4894 = #4629 #4629
[instance] 0 #4894
[attach-enode] #4894 0
[end-of-instance]
[mk-app] #4894 not #3312
[mk-app] #4895 or #4894 #4459 #4461 #3930 #4628
[instance] 0x61d50e6d88f0 ; 2
[attach-enode] #4624 2
[attach-enode] #4626 2
[attach-enode] #4627 2
[attach-enode] #4628 2
[assign] #4628 justification -1: 18 323 692 710
[end-of-instance]
[mk-app] #4896 = #378 #4615
[attach-meaning] #370 arith (- 1)
[mk-app] #4899 * #370 #4615
[mk-app] #4900 + #378 #4899
[mk-app] #4891 <= #4900 #341
[mk-app] #4892 >= #4900 #341
[assign] #4896 justification -1: 515 694
[attach-enode] #4896 0
[attach-enode] #4899 0
[attach-enode] #4900 0
[assign] #4891 justification -1: 866
[assign] #4892 justification -1: 866
[eq-expl] #4626 cg (#4625 #1025) (#296 #296) ; #4606
[eq-expl] #4606 root
[new-match] 0x61d50e6d90e0 #2918 #348 #4626 ; #4627
[mk-app] #4897 not #2918
[mk-app] #4898 or #4897 #4890
[instance] 0x61d50e6d90e0 ; 3
[attach-enode] #4881 3
[attach-enode] #4886 3
[attach-meaning] #370 arith (- 1)
[mk-app] #4878 * #370 #4881
[mk-app] #4879 + #4606 #4878
[mk-app] #4877 <= #4879 #341
[mk-app] #4876 >= #4879 #341
[attach-enode] #4878 3
[attach-enode] #4879 3
[assign] (not #4889) justification -1: 56
[end-of-instance]
[assign] #4882 clause 869 874
[assign] #4887 clause 873 874
[assign] #4886 clause 870 -873
[assign] #4877 clause 871 -870
[assign] #4876 clause 872 -870
[decide-and-or] #3735 #3677
[push] 1
[assign] (not #3665) decision axiom
[assign] #454 clause 517 519
[assign] (not #437) clause -518 519
[eq-expl] #659 lit #3701 ; #3699
[eq-expl] #3699 cg (#274 #277) (#1025 #1025) ; #4833
[eq-expl] #4833 lit #4839 ; #1025
[eq-expl] #434 cg (#659 #1025) ; #3638
[new-match] 0x61d50e6d95b0 #2578 #2577 #434 #1396 ; #436
[mk-app] #4874 or #4205 #4634
[instance] 0x61d50e6d95b0 ; 1
[attach-enode] #4636 1
[attach-enode] #4634 1
[assign] #4634 justification -1: 445
[end-of-instance]
[resolve-lit] 0 #437
[resolve-process] (not #437)
[resolve-lit] 0 (not #4634)
[resolve-process] (not #4634)
[conflict] #437
[pop] 1 2
[assign] #437 axiom
[assign] #3665 clause 519 -518
[new-match] 0x61d50e6d95c0 #2578 #2577 #434 #1396 ; #436
[decide-and-or] #3735 #3729
[push] 1
[assign] (not #3728) decision axiom
[assign] #444 clause 520 525
[assign] #3679 clause 521 525
[assign] #3688 clause 522 525
[assign] #3739 clause 523 525
[assign] (not #3720) clause -524 525
[assign] #4681 clause 708 -521 -694 -522 -692 524
[eq-expl] #3681 root
[new-match] 0x61d50e6d9610 #2578 #2577 #3681 #1396 ; #3719
[new-match] 0x61d50e6d9648 #199 #195 #3681 ; #3679 (#189 #189)
[new-match] 0x61d50e6d9678 #3320 #1722 #3681 #672 #1290 #1167 #125 ; #3695 (#459 #4420) (#125 #125) (#125 #125) (#1167 #1167) (#1167 #1167)
[new-match] 0x61d50e6d96c8 #3315 #1722 #3681 #672 #1290 #1167 #125 ; #3695 (#459 #4420) (#125 #125) (#125 #125) (#1167 #1167) (#1167 #1167)
[new-match] 0x61d50e6d9718 #3286 #1569 #3681 #459 #1167 #125 ; #3695
[mk-app] #4874 or #4205 #4665
[instance] 0x61d50e6d9610 ; 1
[attach-enode] #4633 1
[attach-enode] #4613 1
[attach-enode] #4665 1
[assign] #4665 justification -1: 445
[end-of-instance]
[mk-app] #4875 not #199
[mk-app] #4873 or #4875 #3686 #4668
[instance] 0x61d50e6d9648 ; 1
[attach-enode] #4668 1
[assign] #4668 justification -1: 29 521
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4676
[mk-app] #4872 = #4676 #4676
[instance] 0 #4872
[attach-enode] #4872 0
[end-of-instance]
[mk-app] #4872 not #3320
[mk-app] #4901 or #4872 #4459 #4461 #3686 #3930 #4673 #4675
[instance] 0x61d50e6d9678 ; 2
[attach-enode] #4672 2
[attach-meaning] #370 arith (- 1)
[attach-enode] #4674 2
[attach-enode] #4675 2
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4684
[mk-app] #4902 = #4684 #4684
[instance] 0 #4902
[attach-enode] #4902 0
[end-of-instance]
[mk-app] #4902 not #3315
[mk-app] #4903 or #4902 #4459 #4461 #3686 #3930 #3670 #4681 #4683
[instance] 0x61d50e6d96c8 ; 2
[attach-enode] #4682 2
[attach-enode] #4683 2
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4689
[mk-app] #4904 = #4689 #4689
[instance] 0 #4904
[attach-enode] #4904 0
[end-of-instance]
[mk-app] #4904 or #4465 #4471 #3686 #4688
[instance] 0x61d50e6d9718 ; 1
[attach-enode] #4687 1
[attach-enode] #4688 1
[assign] #4688 justification -1: 305 521 834
[end-of-instance]
[mk-app] #4905 = #3685 #4702
[attach-meaning] #370 arith (- 1)
[assign] #4905 justification -1: 876
[attach-enode] #4905 0
[assign] #4736 justification -1: 881
[assign] #4737 justification -1: 881
[assign] #4784 clause 699 -523 -655 -714 -844 -850
[eq-expl] #3681 lit #4668 ; #4667
[eq-expl] #4667 root
[new-match] 0x61d50e6df490 #3440 #2356 #3681 #2574 #3934 ; #4613
[new-match] 0x61d50e6df4d0 #4034 #4023 #3681 #2574 ; #4613 (#3934 #3934)
[eq-expl] #3685 cg (#3681 #4667) ; #4702
[eq-expl] #4702 root
[new-match] 0x61d50e6df508 #174 #173 #3685 ; #4667
[eq-expl] #4687 cg (#125 #125) (#1167 #1167) (#4420 #459) (#3681 #3681) ; #3695
[new-match] 0x61d50e6df538 #542 #236 #4687 #275 ; #4688 (#1167 #1167)
[new-match] 0x61d50e6df570 #240 #236 #4687 #275 ; #4688 (#1167 #1167)
[new-match] 0x61d50e6df5a8 #467 #466 #3685 ; #3679 (#189 #189) (#3681 #4667)
[inst-discovered] theory-solving 0 basic# ; #4700
[mk-app] #4906 = #4700 #4700
[instance] 0 #4906
[attach-enode] #4906 0
[end-of-instance]
[mk-app] #4906 or #4283 #3966 #4072 #4696 #4699
[instance] 0x61d50e6df490 ; 3
[attach-enode] #4695 3
[attach-enode] #4697 3
[attach-enode] #4698 3
[attach-enode] #4699 3
[end-of-instance]
[mk-app] #4907 + #4001 #4702
[inst-discovered] theory-solving 0 arith# ; #4706
[mk-app] #4908 = #4706 #4907
[instance] 0 #4908
[attach-enode] #4908 0
[end-of-instance]
[attach-meaning] #370 arith (- 1)
[mk-app] #4908 <= #4907 #341
[inst-discovered] theory-solving 0 arith# ; #4908
[mk-app] #4909 = #4908 #4723
[instance] 0 #4909
[attach-enode] #4909 0
[end-of-instance]
[mk-app] #4907 not #4723
[inst-discovered] theory-solving 0 basic# ; #4713
[mk-app] #4908 = #4713 #4713
[instance] 0 #4908
[attach-enode] #4908 0
[end-of-instance]
[mk-app] #4908 or #4072 #4696 #4082 #4720 #4907 #4714
[inst-discovered] theory-solving 0 basic# ; #4908
[mk-app] #4909 = #4908 #4908
[instance] 0 #4909
[attach-enode] #4909 0
[end-of-instance]
[mk-app] #4909 or #4262 #4072 #4696 #4082 #4720 #4907 #4714
[instance] 0x61d50e6df4d0 ; 2
[end-of-instance]
[mk-app] #4908 or #4079 #4726 #4727
[instance] 0x61d50e6df538 ; 2
[attach-enode] #4724 2
[attach-enode] #4727 2
[end-of-instance]
[mk-app] #4910 or #4070 #4726 #4731
[instance] 0x61d50e6df570 ; 2
[attach-enode] #4730 2
[attach-enode] #4731 2
[end-of-instance]
[assign] (not #4797) clause -706 -702 -708
[assign] #4719 clause 701 -702 -522
[assign] #4672 clause 877 -699
[assign] #4785 clause 705 -701
[assign] #4675 clause 878 -877
[assign] #4695 justification -1: 521 876
[assign] #4724 justification -1: 880 694
[resolve-lit] 0 #3720
[resolve-process] (not #3720)
[resolve-lit] 0 (not #4668)
[resolve-lit] 0 (not #4672)
[resolve-lit] 0 (not #4675)
[resolve-process] (not #4675)
[resolve-lit] 0 (not #3679)
[resolve-process] (not #4672)
[resolve-lit] 0 (not #4784)
[resolve-lit] 0 (not #4681)
[resolve-process] (not #4784)
[resolve-lit] 0 (not #3739)
[resolve-process] (not #4668)
[resolve-process] (not #4681)
[resolve-lit] 0 (not #3688)
[resolve-process] #3720
[resolve-lit] 0 #3728
[resolve-process] (not #3739)
[resolve-process] (not #3688)
[resolve-process] (not #3679)
[conflict] #3728
[pop] 1 2
[assign] #3728 axiom
[assign] (not #3676) clause -528 -525 -513 -516 -519 -529
[assign] #383 clause 526 528
[assign] (not #354) clause -527 528
[eq-expl] #384 cg (#1487 #1379) (#659 #1025) ; #4690
[eq-expl] #4690 root
[new-match] 0x61d50e6d95f8 #1090 #770 #2 #2886 #384 ; #354
[mk-app] #4905 check_decrease_int #4690 #2889 #2
[mk-app] #4910 >= #4690 #341
[mk-app] #4908 not #4910
[mk-app] #4907 + #2889 #4799
[mk-app] #4909 <= #4907 #341
[mk-app] #4906 or #4908 #4909
[mk-app] #4904 not #4906
[mk-app] #4902 = #4690 #2889
[mk-app] #4903 not #4902
[mk-app] #4872 or #4903 #4773
[mk-app] #4901 not #4872
[mk-app] #4875 or #4904 #4901
[mk-app] #4873 = #4905 #4875
[mk-app] #4874 or #4903 #1
[inst-discovered] theory-solving 0 basic# ; #4874
[mk-app] #4911 = #4874 #1
[instance] 0 #4911
[attach-enode] #4911 0
[end-of-instance]
[mk-app] #4874 not #1
[inst-discovered] theory-solving 0 basic# ; #4874
[mk-app] #4911 = #4874 #2
[instance] 0 #4911
[attach-enode] #4911 0
[end-of-instance]
[mk-app] #4874 or #4904 #2
[inst-discovered] theory-solving 0 basic# ; #4874
[mk-app] #4911 = #4874 #4904
[instance] 0 #4911
[attach-enode] #4911 0
[end-of-instance]
[mk-app] #4874 = #4906 #4905
[mk-app] #4911 not #4874
[mk-app] #4912 = #4905 #4904
[inst-discovered] theory-solving 0 basic# ; #4912
[mk-app] #4913 = #4912 #4911
[instance] 0 #4913
[attach-enode] #4913 0
[end-of-instance]
[inst-discovered] theory-solving 0 basic# ; #4911
[mk-app] #4912 = #4911 #4911
[instance] 0 #4912
[attach-enode] #4912 0
[end-of-instance]
[mk-app] #4912 not #1090
[mk-app] #4913 or #4912 #4911
[instance] 0x61d50e6d95f8 ; 1
[attach-enode] #4907 1
[attach-enode] #4905 1
[assign] (not #4874) justification -1: 98
[end-of-instance]
[assign] (not #4905) justification -1: -527 483 508 842 40 478
[mk-app] #4914 = #2889 #3874
[attach-meaning] #370 arith (- 1)
[mk-app] #4915 + #2889 #3878
[mk-app] #4916 <= #4915 #341
[mk-app] #4917 >= #4915 #341
[assign] #4914 justification -1: 478
[attach-enode] #4914 0
[attach-enode] #4915 0
[assign] #4916 justification -1: 880
[assign] #4917 justification -1: 880
[assign] #4906 clause 877 878 879
[decide-and-or] #3728 #3723
[push] 1
[assign] (not #444) decision axiom
[decide-and-or] #3942 #3969
[push] 2
[assign] (not #3968) decision axiom
[assign] (not #4494) justification -1: -627 677
[push] 3
[assign] (not #4382) decision axiom
[assign] #4222 clause 755 674
[assign] #4596 justification -1: 755 756 741 808 765 797 768 592
[eq-expl] #4396 root
[new-match] 0x61d50e6def10 #552 #550 #4396 #4393 ; #4397
[new-match] 0x61d50e6def48 #2320 #2319 #2351 #1276 #4395 #3934 ; #4396
[mk-app] #4918 + #4393 #4396 #4496
[inst-discovered] theory-solving 0 arith# ; #4497
[mk-app] #4919 = #4497 #4918
[instance] 0 #4919
[attach-enode] #4919 0
[end-of-instance]
[mk-app] #4919 = #4918 #341
[mk-app] #4920 or #3887 #4919
[instance] 0x61d50e6def10 ; 4
[attach-enode] #4496 4
[attach-enode] #4918 4
[attach-enode] #4919 4
[mk-app] #4921 <= #4918 #341
[mk-app] #4922 >= #4918 #341
[assign] #4919 justification -1: 77
[end-of-instance]
[mk-app] #4923 or #4451 #4559
[instance] 0x61d50e6def48 ; 4
[attach-enode] #4548 4
[attach-enode] #4559 4
[assign] #4559 justification -1: 414
[end-of-instance]
[assign] #4921 clause 884 -883
[assign] #4922 clause 885 -883
[mk-app] #4924 = #2899 #4397
[attach-meaning] #370 arith (- 1)
[mk-app] #4925 + #2899 #4496
[mk-app] #4926 <= #4925 #341
[mk-app] #4927 >= #4925 #341
[assign] #4924 justification -1: 755 756 741 671 592
[attach-enode] #4924 0
[attach-enode] #4925 0
[assign] #4926 justification -1: 887
[assign] #4927 justification -1: 887
[new-match] 0x61d50e6df588 #2320 #2319 #2316 #4116 #4395 #3934 ; #4548
[push] 4
[assign] (not #4657) decision axiom
[decide-and-or] #4880 #4617
[push] 5
[assign] (not #4616) decision axiom
[mk-app] #4928 <= #4615 #341
[assign] (not #4928) justification -1: -862 864
[decide-and-or] #4906 #4908
[push] 6
[assign] (not #4910) decision axiom
[resolve-process] true
[resolve-lit] 0 #4910
[conflict] #4910
[pop] 1 7
[assign] #4910 axiom
[assign] #4909 clause 876 -875 -877
[resolve-process] true
[resolve-lit] 0 (not #4909)
[conflict] (not #4909)
[pop] 1 6
[assign] #4910 axiom
[assign] (not #4909) axiom
[resolve-process] true
[resolve-lit] 0 #4909
[resolve-lit] 0 (not #4910)
[resolve-process] #4909
[conflict] (not #4910)
[pop] 1 5
[assign] #4910 axiom
[assign] (not #4909) axiom
[resolve-lit] 0 (not #4910)
[resolve-process] #4910
[conflict] (not #4910)
[pop] 1 4
[assign] #4910 axiom
[assign] (not #4909) axiom
[resolve-lit] 0 (not #4910)
[resolve-process] #4910
[conflict] (not #4910)
[pop] 1 3
[assign] #4910 axiom
[assign] (not #4909) axiom
[resolve-lit] 0 (not #4910)
[resolve-process] #4910
[conflict] (not #4910)
[pop] 1 2
[assign] #4910 axiom
[assign] (not #4909) axiom
[pop] 1 1
[eof]
