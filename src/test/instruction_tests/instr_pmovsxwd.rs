use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 204], OperandSize::Dword)
}

#[test]
fn pmovsxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 762596721, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 172, 158, 113, 77, 116, 45], OperandSize::Dword)
}

#[test]
fn pmovsxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 215], OperandSize::Qword)
}

#[test]
fn pmovsxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 1465853878, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 172, 218, 182, 39, 95, 87], OperandSize::Qword)
}

