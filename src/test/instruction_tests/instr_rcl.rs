use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 210, 64], OperandSize::Word)
}

#[test]
fn rcl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(DI, 1, Some(OperandSize::Byte), None)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 85, 1, 97], OperandSize::Word)
}

#[test]
fn rcl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 211, 88], OperandSize::Dword)
}

#[test]
fn rcl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 20, 64, 89], OperandSize::Dword)
}

#[test]
fn rcl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CL)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 209, 103], OperandSize::Qword)
}

#[test]
fn rcl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RAX, 1130823697, Some(OperandSize::Byte), None)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 144, 17, 0, 103, 67, 31], OperandSize::Qword)
}

#[test]
fn rcl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 211, 73], OperandSize::Qword)
}

#[test]
fn rcl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 20, 143, 71], OperandSize::Qword)
}

#[test]
fn rcl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DI)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 215, 28], OperandSize::Word)
}

#[test]
fn rcl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 244, Some(OperandSize::Word), None)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 144, 244, 0, 37], OperandSize::Word)
}

#[test]
fn rcl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DI)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 215, 77], OperandSize::Dword)
}

#[test]
fn rcl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(EAX, Four, 491271839, Some(OperandSize::Word), None)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 20, 133, 159, 54, 72, 29, 11], OperandSize::Dword)
}

#[test]
fn rcl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BX)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 211, 72], OperandSize::Qword)
}

#[test]
fn rcl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 20, 121, 113], OperandSize::Qword)
}

#[test]
fn rcl_15() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EDX)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 210, 10], OperandSize::Word)
}

#[test]
fn rcl_16() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(BP, 117, Some(OperandSize::Dword), None)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 86, 117, 55], OperandSize::Word)
}

#[test]
fn rcl_17() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 211, 96], OperandSize::Dword)
}

#[test]
fn rcl_18() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(ECX, 1889690784, Some(OperandSize::Dword), None)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 145, 160, 100, 162, 112, 72], OperandSize::Dword)
}

#[test]
fn rcl_19() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 209, 107], OperandSize::Qword)
}

#[test]
fn rcl_20() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 20, 119, 40], OperandSize::Qword)
}

#[test]
fn rcl_21() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RBP)), operand2: Some(Literal8(124)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 213, 124], OperandSize::Qword)
}

#[test]
fn rcl_22() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RBX, Four, 592463477, Some(OperandSize::Qword), None)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 20, 157, 117, 70, 80, 35, 69], OperandSize::Qword)
}

#[test]
fn rcl_23() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 210], OperandSize::Word)
}

#[test]
fn rcl_24() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 149, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 144, 149, 0], OperandSize::Word)
}

#[test]
fn rcl_25() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 210], OperandSize::Dword)
}

#[test]
fn rcl_26() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(ESI, 1002442743, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 150, 247, 15, 192, 59], OperandSize::Dword)
}

#[test]
fn rcl_27() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 211], OperandSize::Qword)
}

#[test]
fn rcl_28() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 16], OperandSize::Qword)
}

#[test]
fn rcl_29() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 211], OperandSize::Qword)
}

#[test]
fn rcl_30() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 289787640, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 148, 202, 248, 206, 69, 17], OperandSize::Qword)
}

#[test]
fn rcl_31() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 215], OperandSize::Word)
}

#[test]
fn rcl_32() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 23], OperandSize::Word)
}

#[test]
fn rcl_33() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 213], OperandSize::Dword)
}

#[test]
fn rcl_34() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 18], OperandSize::Dword)
}

#[test]
fn rcl_35() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 212], OperandSize::Qword)
}

#[test]
fn rcl_36() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 20, 194], OperandSize::Qword)
}

#[test]
fn rcl_37() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 209], OperandSize::Word)
}

#[test]
fn rcl_38() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(BX, 25329, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 151, 241, 98], OperandSize::Word)
}

#[test]
fn rcl_39() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 215], OperandSize::Dword)
}

#[test]
fn rcl_40() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1712413696, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 20, 221, 0, 92, 17, 102], OperandSize::Dword)
}

#[test]
fn rcl_41() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 214], OperandSize::Qword)
}

#[test]
fn rcl_42() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RDI, 2131987119, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 151, 175, 138, 19, 127], OperandSize::Qword)
}

#[test]
fn rcl_43() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 211], OperandSize::Qword)
}

#[test]
fn rcl_44() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 530526052, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 148, 218, 100, 47, 159, 31], OperandSize::Qword)
}

#[test]
fn rcl_45() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 210], OperandSize::Word)
}

#[test]
fn rcl_46() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Memory(3262, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 22, 190, 12], OperandSize::Word)
}

#[test]
fn rcl_47() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 209], OperandSize::Dword)
}

#[test]
fn rcl_48() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 533740914, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 148, 194, 114, 61, 208, 31], OperandSize::Dword)
}

#[test]
fn rcl_49() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 210], OperandSize::Qword)
}

#[test]
fn rcl_50() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RSI, Two, 683765200, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 20, 117, 208, 109, 193, 40], OperandSize::Qword)
}

#[test]
fn rcl_51() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 209], OperandSize::Qword)
}

#[test]
fn rcl_52() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RDX, 688912239, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 146, 111, 247, 15, 41], OperandSize::Qword)
}

#[test]
fn rcl_53() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 211], OperandSize::Word)
}

#[test]
fn rcl_54() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(BX, 4221, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 151, 125, 16], OperandSize::Word)
}

#[test]
fn rcl_55() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 210], OperandSize::Dword)
}

#[test]
fn rcl_56() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(ECX, Four, 990694031, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 20, 141, 143, 202, 12, 59], OperandSize::Dword)
}

#[test]
fn rcl_57() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 209], OperandSize::Qword)
}

#[test]
fn rcl_58() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RAX, 349561077, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 144, 245, 224, 213, 20], OperandSize::Qword)
}

#[test]
fn rcl_59() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 210], OperandSize::Word)
}

#[test]
fn rcl_60() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 20], OperandSize::Word)
}

#[test]
fn rcl_61() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 214], OperandSize::Dword)
}

#[test]
fn rcl_62() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 22], OperandSize::Dword)
}

#[test]
fn rcl_63() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 212], OperandSize::Qword)
}

#[test]
fn rcl_64() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RDI, Four, 91079116, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 20, 189, 204, 193, 109, 5], OperandSize::Qword)
}

#[test]
fn rcl_65() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 213], OperandSize::Qword)
}

#[test]
fn rcl_66() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 20, 219], OperandSize::Qword)
}

