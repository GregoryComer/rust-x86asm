use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pavgw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 236], OperandSize::Dword)
}

#[test]
fn pavgw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 1212399691, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 140, 128, 75, 192, 67, 72], OperandSize::Dword)
}

#[test]
fn pavgw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 229], OperandSize::Qword)
}

#[test]
fn pavgw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 60, 139], OperandSize::Qword)
}

#[test]
fn pavgw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 229], OperandSize::Dword)
}

#[test]
fn pavgw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1691280848, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 44, 221, 208, 229, 206, 100], OperandSize::Dword)
}

#[test]
fn pavgw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 232], OperandSize::Qword)
}

#[test]
fn pavgw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1573116459, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 36, 117, 43, 218, 195, 93], OperandSize::Qword)
}

