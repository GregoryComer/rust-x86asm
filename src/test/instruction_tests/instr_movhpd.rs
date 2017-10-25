use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 22, 28, 134], OperandSize::Dword)
}

#[test]
fn movhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 22, 11], OperandSize::Qword)
}

#[test]
fn movhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(IndirectDisplaced(ECX, 1347745100, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 23, 161, 76, 245, 84, 80], OperandSize::Dword)
}

#[test]
fn movhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(IndirectScaledDisplaced(RSI, Four, 18070299, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 23, 52, 181, 27, 187, 19, 1], OperandSize::Qword)
}

