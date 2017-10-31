use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 212], OperandSize::Dword)
}

#[test]
fn punpcklwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1785213393, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 12, 149, 209, 49, 104, 106], OperandSize::Dword)
}

#[test]
fn punpcklwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 254], OperandSize::Qword)
}

#[test]
fn punpcklwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 671040190, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 172, 139, 190, 66, 255, 39], OperandSize::Qword)
}

#[test]
fn punpcklwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 253], OperandSize::Dword)
}

#[test]
fn punpcklwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 39], OperandSize::Dword)
}

#[test]
fn punpcklwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 232], OperandSize::Qword)
}

#[test]
fn punpcklwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 36, 143], OperandSize::Qword)
}

