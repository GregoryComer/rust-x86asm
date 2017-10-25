use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 2146368948, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 43, 164, 67, 180, 253, 238, 127], OperandSize::Dword)
}

#[test]
fn movntpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPD, operand1: Some(IndirectDisplaced(RBX, 1251891259, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 43, 187, 59, 88, 158, 74], OperandSize::Qword)
}

