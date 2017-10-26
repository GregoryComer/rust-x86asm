use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 201, 29], OperandSize::Dword)
}

#[test]
fn cmpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 39, 25], OperandSize::Dword)
}

#[test]
fn cmpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 249, 6], OperandSize::Qword)
}

#[test]
fn cmpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 2121292956, Some(OperandSize::Qword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 180, 150, 156, 92, 112, 126, 89], OperandSize::Qword)
}

