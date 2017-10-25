use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 217], OperandSize::Dword)
}

#[test]
fn pmovzxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1358786042, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 44, 221, 250, 109, 253, 80], OperandSize::Dword)
}

#[test]
fn pmovzxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 208], OperandSize::Qword)
}

#[test]
fn pmovzxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 4, 135], OperandSize::Qword)
}

