use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 225], OperandSize::Dword)
}

#[test]
fn phsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 692930374, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 148, 184, 70, 71, 77, 41], OperandSize::Dword)
}

#[test]
fn phsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 204], OperandSize::Qword)
}

#[test]
fn phsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 522756629, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 36, 181, 21, 162, 40, 31], OperandSize::Qword)
}

#[test]
fn phsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 232], OperandSize::Dword)
}

#[test]
fn phsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 52, 251], OperandSize::Dword)
}

#[test]
fn phsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 220], OperandSize::Qword)
}

#[test]
fn phsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1564761966, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 4, 197, 110, 95, 68, 93], OperandSize::Qword)
}

