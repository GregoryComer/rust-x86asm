use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 211], OperandSize::Dword)
}

#[test]
fn punpckhbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 2054074839, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 36, 205, 215, 177, 110, 122], OperandSize::Dword)
}

#[test]
fn punpckhbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 223], OperandSize::Qword)
}

#[test]
fn punpckhbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1376030099, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 156, 206, 147, 141, 4, 82], OperandSize::Qword)
}

#[test]
fn punpckhbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 243], OperandSize::Dword)
}

#[test]
fn punpckhbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 20, 215], OperandSize::Dword)
}

#[test]
fn punpckhbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 197], OperandSize::Qword)
}

#[test]
fn punpckhbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDX, 603541429, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 154, 181, 79, 249, 35], OperandSize::Qword)
}

