use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1nexte_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 235], OperandSize::Dword)
}

#[test]
fn sha1nexte_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 2006683317, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 4, 197, 181, 142, 155, 119], OperandSize::Dword)
}

#[test]
fn sha1nexte_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 230], OperandSize::Qword)
}

#[test]
fn sha1nexte_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDX, 614736727, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 170, 87, 35, 164, 36], OperandSize::Qword)
}

