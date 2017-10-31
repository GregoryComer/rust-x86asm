use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 2007434569, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 22, 182, 73, 5, 167, 119], OperandSize::Dword)
}

#[test]
fn movhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1487517043, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 22, 140, 115, 115, 181, 169, 88], OperandSize::Qword)
}

#[test]
fn movhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 80112913, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 23, 148, 193, 17, 109, 198, 4], OperandSize::Dword)
}

#[test]
fn movhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(IndirectDisplaced(RDX, 160563745, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 23, 162, 33, 2, 146, 9], OperandSize::Qword)
}

