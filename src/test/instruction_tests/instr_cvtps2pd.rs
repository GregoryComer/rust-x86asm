use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 255], OperandSize::Dword)
}

#[test]
fn cvtps2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1639361118, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 148, 153, 94, 170, 182, 97], OperandSize::Dword)
}

#[test]
fn cvtps2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 217], OperandSize::Qword)
}

#[test]
fn cvtps2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1139383744, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 52, 157, 192, 157, 233, 67], OperandSize::Qword)
}

