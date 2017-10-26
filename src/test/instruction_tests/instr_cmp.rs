use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 219], OperandSize::Word)
}

#[test]
fn cmp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 9], OperandSize::Word)
}

#[test]
fn cmp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 211], OperandSize::Dword)
}

#[test]
fn cmp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 9790029, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 28, 245, 77, 98, 149, 0], OperandSize::Dword)
}

#[test]
fn cmp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 210], OperandSize::Qword)
}

#[test]
fn cmp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 786097511, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 140, 192, 103, 229, 218, 46], OperandSize::Qword)
}

#[test]
fn cmp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 201], OperandSize::Qword)
}

#[test]
fn cmp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 28, 248], OperandSize::Qword)
}

#[test]
fn cmp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 253], OperandSize::Word)
}

#[test]
fn cmp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 15], OperandSize::Word)
}

#[test]
fn cmp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 220], OperandSize::Dword)
}

#[test]
fn cmp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1393080810, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 140, 112, 234, 185, 8, 83], OperandSize::Dword)
}

#[test]
fn cmp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 205], OperandSize::Qword)
}

#[test]
fn cmp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(RSI, 461135295, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 142, 191, 93, 124, 27], OperandSize::Qword)
}

#[test]
fn cmp_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 234], OperandSize::Word)
}

#[test]
fn cmp_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 40], OperandSize::Word)
}

#[test]
fn cmp_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 247], OperandSize::Dword)
}

#[test]
fn cmp_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1846369404, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 20, 189, 124, 92, 13, 110], OperandSize::Dword)
}

#[test]
fn cmp_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 204], OperandSize::Qword)
}

#[test]
fn cmp_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 12, 214], OperandSize::Qword)
}

#[test]
fn cmp_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 253], OperandSize::Qword)
}

#[test]
fn cmp_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RDX, Two, 976791527, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 52, 85, 231, 167, 56, 58], OperandSize::Qword)
}

#[test]
fn cmp_23() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 219], OperandSize::Word)
}

#[test]
fn cmp_24() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 132, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 153, 132, 0], OperandSize::Word)
}

#[test]
fn cmp_25() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 211], OperandSize::Dword)
}

#[test]
fn cmp_26() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 12, 202], OperandSize::Dword)
}

#[test]
fn cmp_27() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 218], OperandSize::Qword)
}

#[test]
fn cmp_28() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 759621741, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 28, 117, 109, 232, 70, 45], OperandSize::Qword)
}

#[test]
fn cmp_29() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 219], OperandSize::Qword)
}

#[test]
fn cmp_30() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1007282838, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 28, 221, 150, 234, 9, 60], OperandSize::Qword)
}

#[test]
fn cmp_31() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 228], OperandSize::Word)
}

#[test]
fn cmp_32() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(DI, 14917, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 181, 69, 58], OperandSize::Word)
}

#[test]
fn cmp_33() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 210], OperandSize::Dword)
}

#[test]
fn cmp_34() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 2050589852, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 20, 117, 156, 132, 57, 122], OperandSize::Dword)
}

#[test]
fn cmp_35() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 238], OperandSize::Qword)
}

#[test]
fn cmp_36() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 34], OperandSize::Qword)
}

#[test]
fn cmp_37() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 230], OperandSize::Word)
}

#[test]
fn cmp_38() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 106, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 89, 106], OperandSize::Word)
}

#[test]
fn cmp_39() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 235], OperandSize::Dword)
}

#[test]
fn cmp_40() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(EAX, 1740046987, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 160, 139, 2, 183, 103], OperandSize::Dword)
}

#[test]
fn cmp_41() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 241], OperandSize::Qword)
}

#[test]
fn cmp_42() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 20, 126], OperandSize::Qword)
}

#[test]
fn cmp_43() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RSI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 206], OperandSize::Qword)
}

#[test]
fn cmp_44() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 213758215, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 59, 28, 157, 7, 177, 189, 12], OperandSize::Qword)
}

#[test]
fn cmp_45() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 118], OperandSize::Word)
}

#[test]
fn cmp_46() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 44], OperandSize::Dword)
}

#[test]
fn cmp_47() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 4], OperandSize::Qword)
}

#[test]
fn cmp_48() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(2688)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 128, 10], OperandSize::Word)
}

#[test]
fn cmp_49() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(21861)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 101, 85], OperandSize::Dword)
}

#[test]
fn cmp_50() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(10105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 121, 39], OperandSize::Qword)
}

#[test]
fn cmp_51() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1211152485)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 101, 184, 48, 72], OperandSize::Word)
}

#[test]
fn cmp_52() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(893926335)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 191, 59, 72, 53], OperandSize::Dword)
}

#[test]
fn cmp_53() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(226624691)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 179, 4, 130, 13], OperandSize::Qword)
}

#[test]
fn cmp_54() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1942871200)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 61, 160, 220, 205, 115], OperandSize::Qword)
}

#[test]
fn cmp_55() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 249, 75], OperandSize::Word)
}

#[test]
fn cmp_56() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(SI, 77, Some(OperandSize::Byte), None)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 124, 77, 30], OperandSize::Word)
}

#[test]
fn cmp_57() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 251, 93], OperandSize::Dword)
}

#[test]
fn cmp_58() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 57, 63], OperandSize::Dword)
}

#[test]
fn cmp_59() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 251, 24], OperandSize::Qword)
}

#[test]
fn cmp_60() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 58, 5], OperandSize::Qword)
}

#[test]
fn cmp_61() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 249, 113], OperandSize::Qword)
}

#[test]
fn cmp_62() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(RDX, 353229575, Some(OperandSize::Byte), None)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 186, 7, 219, 13, 21, 96], OperandSize::Qword)
}

#[test]
fn cmp_63() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(Literal16(21275)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 254, 27, 83], OperandSize::Word)
}

#[test]
fn cmp_64() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 206, Some(OperandSize::Word), None)), operand2: Some(Literal16(7357)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 185, 206, 0, 189, 28], OperandSize::Word)
}

#[test]
fn cmp_65() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BX)), operand2: Some(Literal16(30040)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 251, 88, 117], OperandSize::Dword)
}

#[test]
fn cmp_66() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal16(3839)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 60, 138, 255, 14], OperandSize::Dword)
}

#[test]
fn cmp_67() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(Literal16(3623)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 254, 39, 14], OperandSize::Qword)
}

#[test]
fn cmp_68() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Literal16(1066)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 63, 42, 4], OperandSize::Qword)
}

#[test]
fn cmp_69() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDI)), operand2: Some(Literal32(2102515860)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 255, 148, 216, 81, 125], OperandSize::Word)
}

#[test]
fn cmp_70() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(BP, 189, Some(OperandSize::Dword), None)), operand2: Some(Literal32(978787370)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 190, 189, 0, 42, 28, 87, 58], OperandSize::Word)
}

#[test]
fn cmp_71() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(Literal32(853577530)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 254, 58, 143, 224, 50], OperandSize::Dword)
}

#[test]
fn cmp_72() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(EAX, 290349393, Some(OperandSize::Dword), None)), operand2: Some(Literal32(765412767)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 184, 81, 97, 78, 17, 159, 69, 159, 45], OperandSize::Dword)
}

#[test]
fn cmp_73() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDI)), operand2: Some(Literal32(1042144837)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 255, 69, 222, 29, 62], OperandSize::Qword)
}

#[test]
fn cmp_74() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1565625122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 63, 34, 139, 81, 93], OperandSize::Qword)
}

#[test]
fn cmp_75() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RCX)), operand2: Some(Literal32(1166023352)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 249, 184, 26, 128, 69], OperandSize::Qword)
}

#[test]
fn cmp_76() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 261630550, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1907496698)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 188, 194, 86, 42, 152, 15, 250, 22, 178, 113], OperandSize::Qword)
}

#[test]
fn cmp_77() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CX)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 249, 62], OperandSize::Word)
}

#[test]
fn cmp_78() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 80, Some(OperandSize::Word), None)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 122, 80, 62], OperandSize::Word)
}

#[test]
fn cmp_79() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 250, 26], OperandSize::Dword)
}

#[test]
fn cmp_80() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(EBX, 1175449277, Some(OperandSize::Word), None)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 187, 189, 238, 15, 70, 61], OperandSize::Dword)
}

#[test]
fn cmp_81() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BX)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 251, 60], OperandSize::Qword)
}

#[test]
fn cmp_82() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 62, 70], OperandSize::Qword)
}

#[test]
fn cmp_83() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 252, 45], OperandSize::Word)
}

#[test]
fn cmp_84() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(SI, 7706, Some(OperandSize::Dword), None)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 188, 26, 30, 100], OperandSize::Word)
}

#[test]
fn cmp_85() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDI)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 255, 47], OperandSize::Dword)
}

#[test]
fn cmp_86() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(EBX, 796192862, Some(OperandSize::Dword), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 187, 94, 240, 116, 47, 107], OperandSize::Dword)
}

#[test]
fn cmp_87() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 254, 100], OperandSize::Qword)
}

#[test]
fn cmp_88() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 56, 29], OperandSize::Qword)
}

#[test]
fn cmp_89() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RDX)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 250, 78], OperandSize::Qword)
}

#[test]
fn cmp_90() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1447514775, Some(OperandSize::Qword), None)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 60, 253, 151, 82, 71, 86, 27], OperandSize::Qword)
}

