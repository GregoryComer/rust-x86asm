use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn extractps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 231, 7], OperandSize::Dword)
}

#[test]
fn extractps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(IndirectScaledDisplaced(EBX, Four, 1342298592, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 36, 157, 224, 217, 1, 80, 25], OperandSize::Dword)
}

#[test]
fn extractps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 253, 80], OperandSize::Qword)
}

#[test]
fn extractps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 40, 12], OperandSize::Qword)
}

