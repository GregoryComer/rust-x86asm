declare -A modes
modes=(
["PD"]="avx_template_pd"
["PS"]="avx_template_ps"
["SD"]="avx_template_sd"
["SS"]="avx_template_ss"
["MOV"]="avx_template_mov"
["MOV_V"]="avx_template_mov_vex"
["MOV_E"]="avx_template_mov_evex"
["XMM"]="avx_template_xmm"
["XMM64"]="avx_template_xmm64"
["MOVMSK"]="avx_template_movmsk"
["MEM"]="avx_template_mem"
["RMEM"]="avx_template_rmem"
["XMMOR64"]="avx_template_xmm_or_64"
["STD"]="avx_template_std"
["MOVS"]="avx_template_movs"
["STD2"]="avx_template_std2"
["IMM8"]="avx_template_imm8"
["IMM83"]="avx_template_imm83"
["STDA"]="avx_template_stda"
["STDA2"]="avx_template_stda2"
["VEX4"]="avx_template_vex4"
["IMM82"]="avx_template_imm82"
["CMP"]="avx_template_cmp"
["CMP32"]="avx_template_cmp32"
["CMP64"]="avx_template_cmp64"
["EXTR"]="avx_template_extr"
["XMM2"]="avx_template_xmm2"
["INSR"]="avx_template_insr"
["MOVSX"]="avx_template_movsx"
["STDE"]="avx_template_stde"
["PSE"]="avx_template_pse"
["PDE"]="avx_template_pde"
)

while read line; do
    if [[ ${line:0:1} == '#' ]]
    then
        continue
    fi
    lineparts=($line)
    mnemonic=${lineparts[0]}
    mode=${lineparts[1]}
    primary_opcode=${lineparts[2]}
    secondary_opcode=${lineparts[3]}
    fixed_prefix=${lineparts[4]}
    op_size_prefix=${lineparts[5]}
    force_64=${lineparts[6]}
    param1=${lineparts[7]}
    param2=${lineparts[8]}
    param3=${lineparts[9]}

    file=${modes[$mode]}
    if [ -z $file ]; then
        echo "Unknown mode $mode" 1>&2
        exit 1
    fi

    file="templates/$file"

    entry=$(sed "
    s/%MNEMONIC%/$mnemonic/g
    s/%PRIMARY_OPCODE%/$primary_opcode/g
    s/%SECONDARY_OPCODE%/$secondary_opcode/g
    s/%FIXED_PREFIX%/$fixed_prefix/g
    s/%OP_SIZE_PREFIX%/$op_size_prefix/g
    s/%FORCE_64%/$force_64/g
    s/%PARAM1%/$param1/g
    s/%PARAM2%/$param2/g
    s/%PARAM3%/$param3/g
    " $file)

    echo $entry >> instruction_defs_raw.rs
done < avx_extra_defs
