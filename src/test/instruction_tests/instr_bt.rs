use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 210], OperandSize::Word)
}

#[test]
fn bt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 73, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 98, 73], OperandSize::Word)
}

#[test]
fn bt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 246], OperandSize::Dword)
}

#[test]
fn bt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1753253577, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 52, 125, 201, 134, 128, 104], OperandSize::Dword)
}

#[test]
fn bt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 213], OperandSize::Qword)
}

#[test]
fn bt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1963374055, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 12, 93, 231, 181, 6, 117], OperandSize::Qword)
}

#[test]
fn bt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 202], OperandSize::Word)
}

#[test]
fn bt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 36], OperandSize::Word)
}

#[test]
fn bt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 201], OperandSize::Dword)
}

#[test]
fn bt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 28, 179], OperandSize::Dword)
}

#[test]
fn bt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 230], OperandSize::Qword)
}

#[test]
fn bt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 57], OperandSize::Qword)
}

#[test]
fn bt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 163, 210], OperandSize::Qword)
}

#[test]
fn bt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 10772608, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 163, 44, 197, 128, 96, 164, 0], OperandSize::Qword)
}

#[test]
fn bt_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(BX)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 227, 11], OperandSize::Word)
}

#[test]
fn bt_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 26710, Some(OperandSize::Word), None)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 161, 86, 104, 123], OperandSize::Word)
}

#[test]
fn bt_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DI)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 231, 87], OperandSize::Dword)
}

#[test]
fn bt_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1192576244, Some(OperandSize::Word), None)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 164, 177, 244, 68, 21, 71, 109], OperandSize::Dword)
}

#[test]
fn bt_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(BX)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 227, 62], OperandSize::Qword)
}

#[test]
fn bt_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 34, 52], OperandSize::Qword)
}

#[test]
fn bt_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ECX)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 225, 18], OperandSize::Word)
}

#[test]
fn bt_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Memory(26458, Some(OperandSize::Dword), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 38, 90, 103, 113], OperandSize::Word)
}

#[test]
fn bt_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ECX)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 225, 121], OperandSize::Dword)
}

#[test]
fn bt_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 36, 183, 74], OperandSize::Dword)
}

#[test]
fn bt_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDI)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 231, 87], OperandSize::Qword)
}

#[test]
fn bt_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1257663439, Some(OperandSize::Dword), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 36, 189, 207, 107, 246, 74, 81], OperandSize::Qword)
}

#[test]
fn bt_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(RSP)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 228, 7], OperandSize::Qword)
}

#[test]
fn bt_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 36, 248, 56], OperandSize::Qword)
}

