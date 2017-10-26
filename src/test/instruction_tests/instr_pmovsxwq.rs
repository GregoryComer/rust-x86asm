use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 210], OperandSize::Dword)
}

#[test]
fn pmovsxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 271035545, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 188, 198, 153, 172, 39, 16], OperandSize::Dword)
}

#[test]
fn pmovsxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 241], OperandSize::Qword)
}

#[test]
fn pmovsxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 306738108, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 36, 157, 188, 115, 72, 18], OperandSize::Qword)
}

