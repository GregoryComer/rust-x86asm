use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn comiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 219], OperandSize::Dword)
}

#[test]
fn comiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EBX, 2126705310, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 155, 158, 242, 194, 126], OperandSize::Dword)
}

#[test]
fn comiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 210], OperandSize::Qword)
}

#[test]
fn comiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1803355332, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 148, 66, 196, 4, 125, 107], OperandSize::Qword)
}

