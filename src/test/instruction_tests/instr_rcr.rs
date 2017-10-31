use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 217, 75], OperandSize::Word)
}

#[test]
fn rcr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 24, Some(OperandSize::Byte), None)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 89, 24, 32], OperandSize::Word)
}

#[test]
fn rcr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 217, 96], OperandSize::Dword)
}

#[test]
fn rcr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 304139684, Some(OperandSize::Byte), None)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 156, 187, 164, 205, 32, 18, 63], OperandSize::Dword)
}

#[test]
fn rcr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 218, 105], OperandSize::Qword)
}

#[test]
fn rcr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 26, 0], OperandSize::Qword)
}

#[test]
fn rcr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 219, 24], OperandSize::Qword)
}

#[test]
fn rcr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 26, 116], OperandSize::Qword)
}

#[test]
fn rcr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SI)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 222, 40], OperandSize::Word)
}

#[test]
fn rcr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 29561, Some(OperandSize::Word), None)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 154, 121, 115, 65], OperandSize::Word)
}

#[test]
fn rcr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DX)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 218, 60], OperandSize::Dword)
}

#[test]
fn rcr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 28, 91, 43], OperandSize::Dword)
}

#[test]
fn rcr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SI)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 222, 78], OperandSize::Qword)
}

#[test]
fn rcr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1198132790, Some(OperandSize::Word), None)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 28, 181, 54, 14, 106, 71, 17], OperandSize::Qword)
}

#[test]
fn rcr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 218, 10], OperandSize::Word)
}

#[test]
fn rcr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 25, 75], OperandSize::Word)
}

#[test]
fn rcr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 219, 32], OperandSize::Dword)
}

#[test]
fn rcr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1085597927, Some(OperandSize::Dword), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 156, 219, 231, 232, 180, 64, 105], OperandSize::Dword)
}

#[test]
fn rcr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 222, 6], OperandSize::Qword)
}

#[test]
fn rcr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 2060839871, Some(OperandSize::Dword), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 156, 144, 191, 235, 213, 122, 24], OperandSize::Qword)
}

#[test]
fn rcr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RBP)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 221, 77], OperandSize::Qword)
}

#[test]
fn rcr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 31, 86], OperandSize::Qword)
}

#[test]
fn rcr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 218], OperandSize::Word)
}

#[test]
fn rcr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 26], OperandSize::Word)
}

#[test]
fn rcr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 217], OperandSize::Dword)
}

#[test]
fn rcr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(EDI, Two, 267320202, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 28, 125, 138, 251, 238, 15], OperandSize::Dword)
}

#[test]
fn rcr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 217], OperandSize::Qword)
}

#[test]
fn rcr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 31], OperandSize::Qword)
}

#[test]
fn rcr_29() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 217], OperandSize::Qword)
}

#[test]
fn rcr_30() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 2059123315, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 156, 195, 115, 186, 187, 122], OperandSize::Qword)
}

#[test]
fn rcr_31() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 220], OperandSize::Word)
}

#[test]
fn rcr_32() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 29877, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 154, 181, 116], OperandSize::Word)
}

#[test]
fn rcr_33() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 221], OperandSize::Dword)
}

#[test]
fn rcr_34() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 303573504, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 156, 138, 0, 42, 24, 18], OperandSize::Dword)
}

#[test]
fn rcr_35() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 220], OperandSize::Qword)
}

#[test]
fn rcr_36() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 26], OperandSize::Qword)
}

#[test]
fn rcr_37() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 217], OperandSize::Word)
}

#[test]
fn rcr_38() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 24], OperandSize::Word)
}

#[test]
fn rcr_39() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 218], OperandSize::Dword)
}

#[test]
fn rcr_40() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 31], OperandSize::Dword)
}

#[test]
fn rcr_41() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 220], OperandSize::Qword)
}

#[test]
fn rcr_42() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(RAX, 248099561, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 152, 233, 178, 201, 14], OperandSize::Qword)
}

#[test]
fn rcr_43() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RSI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 222], OperandSize::Qword)
}

#[test]
fn rcr_44() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1495066522, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 156, 215, 154, 231, 28, 89], OperandSize::Qword)
}

#[test]
fn rcr_45() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 218], OperandSize::Word)
}

#[test]
fn rcr_46() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(BP, 13116, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 158, 60, 51], OperandSize::Word)
}

#[test]
fn rcr_47() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 218], OperandSize::Dword)
}

#[test]
fn rcr_48() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 1300180351, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 156, 136, 127, 45, 127, 77], OperandSize::Dword)
}

#[test]
fn rcr_49() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 219], OperandSize::Qword)
}

#[test]
fn rcr_50() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 28, 146], OperandSize::Qword)
}

#[test]
fn rcr_51() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 218], OperandSize::Qword)
}

#[test]
fn rcr_52() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 271752036, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 156, 185, 100, 155, 50, 16], OperandSize::Qword)
}

#[test]
fn rcr_53() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 221], OperandSize::Word)
}

#[test]
fn rcr_54() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(DI, 20, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 93, 20], OperandSize::Word)
}

#[test]
fn rcr_55() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 219], OperandSize::Dword)
}

#[test]
fn rcr_56() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1890924847, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 156, 206, 47, 57, 181, 112], OperandSize::Dword)
}

#[test]
fn rcr_57() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 223], OperandSize::Qword)
}

#[test]
fn rcr_58() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 126604525, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 156, 176, 237, 212, 139, 7], OperandSize::Qword)
}

#[test]
fn rcr_59() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 221], OperandSize::Word)
}

#[test]
fn rcr_60() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 119, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 89, 119], OperandSize::Word)
}

#[test]
fn rcr_61() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 222], OperandSize::Dword)
}

#[test]
fn rcr_62() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(EDX, 1108990974, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 154, 254, 219, 25, 66], OperandSize::Dword)
}

#[test]
fn rcr_63() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 222], OperandSize::Qword)
}

#[test]
fn rcr_64() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 25], OperandSize::Qword)
}

#[test]
fn rcr_65() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 223], OperandSize::Qword)
}

#[test]
fn rcr_66() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 25], OperandSize::Qword)
}

